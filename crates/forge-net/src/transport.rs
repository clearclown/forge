use crate::FORGE_ALPN;
use crate::connection::PeerConnection;
use forge_core::NodeId;
use forge_proto::Envelope;
use iroh::endpoint::presets;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tokio::sync::{Mutex, Notify, mpsc};

const MAX_RECENT_MSG_IDS_PER_PEER: usize = 2_048;

#[derive(Default)]
struct ReplayWindow {
    order: VecDeque<u64>,
    seen: HashSet<u64>,
}

impl ReplayWindow {
    fn record(&mut self, msg_id: u64) -> bool {
        if !self.seen.insert(msg_id) {
            return false;
        }

        self.order.push_back(msg_id);
        while self.order.len() > MAX_RECENT_MSG_IDS_PER_PEER {
            if let Some(evicted) = self.order.pop_front() {
                self.seen.remove(&evicted);
            }
        }

        true
    }
}

/// The Forge P2P transport layer built on Iroh.
pub struct ForgeTransport {
    endpoint: iroh::Endpoint,
    peers: Arc<Mutex<HashMap<String, PeerConnection>>>,
    recent_msg_ids: Arc<Mutex<HashMap<String, ReplayWindow>>>,
    incoming_tx: mpsc::Sender<(String, Envelope)>,
    incoming_rx: Arc<Mutex<mpsc::Receiver<(String, Envelope)>>>,
    shutdown: Arc<Notify>,
    closed: Arc<AtomicBool>,
}

impl ForgeTransport {
    /// Create a new transport with a fresh Iroh endpoint.
    pub async fn new() -> anyhow::Result<Self> {
        let endpoint = iroh::Endpoint::builder(presets::N0)
            .alpns(vec![FORGE_ALPN.to_vec()])
            .bind()
            .await?;

        let endpoint_id = endpoint.id();
        tracing::info!("Forge node started: {}", endpoint_id.fmt_short());
        let addr = endpoint.addr();
        tracing::info!("Endpoint address: {:?}", addr);

        let (incoming_tx, incoming_rx) = mpsc::channel(256);

        Ok(Self {
            endpoint,
            peers: Arc::new(Mutex::new(HashMap::new())),
            recent_msg_ids: Arc::new(Mutex::new(HashMap::new())),
            incoming_tx,
            incoming_rx: Arc::new(Mutex::new(incoming_rx)),
            shutdown: Arc::new(Notify::new()),
            closed: Arc::new(AtomicBool::new(false)),
        })
    }

    /// Get this node's Iroh EndpointId.
    pub fn endpoint_id(&self) -> iroh::EndpointId {
        self.endpoint.id()
    }

    /// Get this node's full address for sharing with peers.
    pub fn endpoint_addr(&self) -> iroh::EndpointAddr {
        self.endpoint.addr()
    }

    /// Get the forge-core NodeId derived from the Iroh identity.
    pub fn forge_node_id(&self) -> NodeId {
        let bytes: [u8; 32] = *self.endpoint.id().as_bytes();
        NodeId(bytes)
    }

    /// Connect to a peer by their EndpointAddr.
    ///
    /// Starts a background read loop so that messages sent by the remote
    /// peer on this connection are delivered to `recv()`.
    pub async fn connect(&self, addr: iroh::EndpointAddr) -> anyhow::Result<PeerConnection> {
        let peer_node_id = NodeId(*addr.id.as_bytes());
        tracing::info!("Connecting to peer: {}", peer_node_id);

        let conn = self.endpoint.connect(addr, FORGE_ALPN).await?;
        let peer_conn = PeerConnection::new(conn);
        let peer_id = peer_conn.peer_id().to_string();

        self.peers
            .lock()
            .await
            .insert(peer_id.clone(), peer_conn.clone());

        // Start reading messages from this peer in the background.
        // Without this, messages sent *back* by the remote side would
        // never be consumed because nobody calls accept_bi() on the
        // outgoing connection.
        let read_peer = peer_conn.clone();
        let read_tx = self.incoming_tx.clone();
        let read_id = peer_id;
        let peers = self.peers.clone();
        let recent_msg_ids = self.recent_msg_ids.clone();
        tokio::spawn(async move {
            Self::read_peer_messages(read_peer, read_id, read_tx, peers, recent_msg_ids).await;
        });

        Ok(peer_conn)
    }

    /// Start accepting incoming connections in the background.
    pub fn start_accepting(&self) -> tokio::task::JoinHandle<()> {
        let endpoint = self.endpoint.clone();
        let peers = self.peers.clone();
        let recent_msg_ids = self.recent_msg_ids.clone();
        let incoming_tx = self.incoming_tx.clone();

        tokio::spawn(async move {
            loop {
                match endpoint.accept().await {
                    Some(connecting) => {
                        let peers = peers.clone();
                        let recent_msg_ids = recent_msg_ids.clone();
                        let incoming_tx = incoming_tx.clone();

                        tokio::spawn(async move {
                            match connecting.await {
                                Ok(conn) => {
                                    let peer_conn = PeerConnection::new(conn);
                                    let peer_id = peer_conn.peer_id().to_string();
                                    tracing::info!(
                                        "Accepted connection from: {}",
                                        peer_conn.peer_node_id()
                                    );
                                    peers
                                        .lock()
                                        .await
                                        .insert(peer_id.clone(), peer_conn.clone());

                                    Self::read_peer_messages(
                                        peer_conn,
                                        peer_id,
                                        incoming_tx,
                                        peers,
                                        recent_msg_ids,
                                    )
                                    .await;
                                }
                                Err(e) => {
                                    tracing::warn!("Failed to accept connection: {}", e);
                                }
                            }
                        });
                    }
                    None => {
                        tracing::info!("Endpoint closed, stopping accept loop");
                        break;
                    }
                }
            }
        })
    }

    /// Read messages from a peer connection and forward to the incoming channel.
    async fn read_peer_messages(
        peer: PeerConnection,
        peer_id: String,
        tx: mpsc::Sender<(String, Envelope)>,
        peers: Arc<Mutex<HashMap<String, PeerConnection>>>,
        recent_msg_ids: Arc<Mutex<HashMap<String, ReplayWindow>>>,
    ) {
        loop {
            match peer.recv_message().await {
                Ok(envelope) => {
                    if let Err(err) = envelope.validate_for_peer(peer.peer_node_id()) {
                        tracing::warn!(
                            "Dropping invalid envelope from {}: {}",
                            peer.peer_node_id(),
                            err
                        );
                        continue;
                    }
                    let is_new_message = {
                        let mut recent_msg_ids = recent_msg_ids.lock().await;
                        recent_msg_ids
                            .entry(peer_id.clone())
                            .or_default()
                            .record(envelope.msg_id)
                    };
                    if !is_new_message {
                        tracing::warn!(
                            "Dropping duplicate envelope {} from {}",
                            envelope.msg_id,
                            peer.peer_node_id()
                        );
                        continue;
                    }
                    if tx.send((peer_id.clone(), envelope)).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::debug!("Peer {} disconnected: {}", peer_id, e);
                    break;
                }
            }
        }

        peers.lock().await.remove(&peer_id);
        recent_msg_ids.lock().await.remove(&peer_id);
    }

    /// Receive the next incoming message from any peer.
    pub async fn recv(&self) -> Option<(String, Envelope)> {
        if self.closed.load(Ordering::SeqCst) {
            return None;
        }

        let mut incoming_rx = self.incoming_rx.lock().await;
        if self.closed.load(Ordering::SeqCst) {
            return None;
        }

        tokio::select! {
            message = incoming_rx.recv() => message,
            _ = self.shutdown.notified() => None,
        }
    }

    /// Send a message to a specific peer.
    pub async fn send_to(&self, peer_id: &str, envelope: &Envelope) -> anyhow::Result<()> {
        let peers = self.peers.lock().await;
        let peer = peers
            .get(peer_id)
            .ok_or_else(|| anyhow::anyhow!("peer not found: {}", peer_id))?;
        peer.send_message(envelope).await
    }

    /// Get a peer connection by ID.
    pub async fn get_peer(&self, peer_id: &str) -> Option<PeerConnection> {
        self.peers.lock().await.get(peer_id).cloned()
    }

    /// Get the list of connected peer IDs.
    pub async fn connected_peers(&self) -> Vec<String> {
        self.peers.lock().await.keys().cloned().collect()
    }

    /// Gracefully close the transport.
    pub async fn close(&self) {
        self.closed.store(true, Ordering::SeqCst);
        self.shutdown.notify_waiters();
        self.endpoint.close().await;
    }
}
