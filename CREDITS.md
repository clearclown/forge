# Credits & Acknowledgements

## mesh-llm

Forge's distributed inference engine is built on [mesh-llm](https://github.com/michaelneale/mesh-llm) by **Michael Neale**.

mesh-llm solved the hard problems of distributed LLM inference:

- Pipeline parallelism for dense models across multiple machines
- Expert sharding for Mixture-of-Experts architectures (Qwen3, Mixtral, DeepSeek)
- iroh-based mesh networking with Nostr discovery
- OpenAI-compatible API with multi-model routing
- Web management console with live topology visualization
- Plugin system with Blackboard agent coordination

Forge does not claim credit for any of this engineering. What Forge adds is an economic layer — CU accounting, Proof of Useful Work, dynamic pricing, and autonomous agent budgets — on top of mesh-llm's inference foundation.

The relationship is straightforward: mesh-llm makes distributed inference work. Forge makes it worth doing.

## Other Influences

- **Bitcoin** — Proved that `electricity → computation → monetary value` is a viable economic model. Forge inherits the insight but replaces useless PoW with useful inference.
- **BitTorrent** — Reciprocity-based resource sharing without central coordination. Forge's CU economy is a formalized version of BitTorrent's tit-for-tat.
- **llama.cpp** by Georgi Gerganov — The inference engine that makes local LLM execution practical on consumer hardware.
- **iroh** by n0 — The QUIC-based networking library that handles NAT traversal, relay fallback, and peer-to-peer connections.
- **LDK (Lightning Dev Kit)** — The embedded Lightning node that enables optional CU↔Bitcoin settlement.

## Philosophy

Sam Altman's thesis — intelligence scales with compute — is the economic foundation of Forge. If more electricity + more silicon = smarter AI, then compute is the most valuable commodity of the AI era. Forge creates a market for it.
