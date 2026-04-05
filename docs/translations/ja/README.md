# Forge

> 計算は通貨である。すべてのワットは廃棄物ではなく知能を生み出す。

**Forgeは、計算能力がお金となる分散型推論プロトコルです。** ノードは他者のために有用なLLM推論を実行することで、計算ユニット（CU: Compute Unit）を獲得します。無意味なハッシュ計算に電力を消費するBitcoinとは異なり、Forgeノードで費やされるすべてのジュールは、誰かが実際に必要としている本物の知能を生み出します。

分散型推論エンジンは、Michael Neale氏による [mesh-llm](https://github.com/michaelneale/mesh-llm) に基づいて構築されています。Forgeはその上に、CU会計、有益な仕事の証明（Proof of Useful Work）、動的価格設定、自律エージェント予算、およびフェイルセーフ制御といった計算経済層を追加します。[CREDITS.md](CREDITS.md) を参照してください。

**統合フォーク:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — Forge経済層が組み込まれたmesh-llm。

## ライブデモ

これは稼働中のForgeノードからの実際の出力です。すべての推論にはCUコストがかかります。すべてのCUは有用な計算によって獲得されます。

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**残高確認 — すべての新しいノードは1,000 CUの無料枠を受け取ります:**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**質問する — 推論にはCUコストがかかります:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"日本語で挨拶して"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

すべてのレスポンスには `x_forge` — **その計算のCUコスト** と残高が含まれます。プロバイダーは9 CUを獲得し、消費者は9 CUを支払いました。すべてのユニットは物理学（電力消費）に裏打ちされています。

**3回の推論後 — 台帳上の実際の取引:**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**すべての取引はマークルルートを持ち、不変の証明のためにBitcoinにアンカー可能です:**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AIエージェントが暴走した？キルスイッチがミリ秒単位ですべてを凍結します:**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ すべてのCUトランザクションを凍結。エージェントは支出不能。
```

**安全制御は常にオン:**
```
$ curl localhost:3000/v1/forge/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_cu_per_hour": 10000,
    "max_cu_per_request": 1000,
    "max_cu_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## Forgeが存在する理由

```
Bitcoin:  電力  →  無意味なSHA-256  →  BTC
Forge:    電力  →  有用なLLM推論    →  CU
```

Bitcoinは「電力 → 計算 → お金」を証明しました。しかし、Bitcoinの計算には目的がありません。Forgeはそれを反転させます。すべてのCUは、誰かの実際の問題を解決した本物の知能を表しています。

**他のプロジェクトにはない3つの特徴:**

### 1. 計算 = 通貨

すべての推論は取引です。プロバイダーはCUを獲得し、消費者はCUを支払います。ブロックチェーンも、トークンも、ICOもありません。CUは物理学、つまり有用な仕事のために消費された電力によって裏打ちされています。

### 2. ブロックチェーンなしで改ざん防止

すべての取引は双方によって二重署名（Ed25519）され、メッシュ全体でゴシップ同期されます。すべての取引のマークルルートは、不変の監査のためにBitcoinにアンカーできます。グローバルなコンセンサスは不要で、二者間の暗号学的証明で十分です。

### 3. AIエージェントが自ら計算資源を管理

スマートフォンのエージェントが夜間にアイドル計算能力を貸し出す → CUを獲得 → 70Bモデルへのアクセスを購入 → より賢くなる → さらに稼ぐ。エージェントは自律的に `/v1/forge/balance` と `/v1/forge/pricing` を確認します。予算ポリシーとサーキットブレーカーが制御不能な支出を防ぎます。

```
エージェント (スマホ上の1.5Bモデル)
  → 夜間に推論を提供してCUを稼ぐ
  → 70BモデルにCUを支払う → より賢い回答を得る
  → より良い意思決定 → さらにCUを稼ぐ
  → サイクルが繰り返される → エージェントが成長する
```

## アーキテクチャ

```
┌─────────────────────────────────────────────────┐
│  推論層 (Inference Layer: mesh-llm)             │
│  パイプライン並列化、MoEエキスパートシャッディング、 │
│  irohメッシュ、Nostrディスカバリ、OpenAI API       │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  経済層 (Economic Layer: Forge)                 │
│  CU台帳、二重署名取引、ゴシップ、                 │
│  動的価格設定、マークルルート、安全制御           │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│  安全層 (Safety Layer)                          │
│  キルスイッチ、予算ポリシー、サーキットブレーカー、 │
│  速度検出、人間による承認しきい値                 │
└──────────────────┬──────────────────────────────┘
                   │ 任意
┌──────────────────▼──────────────────────────────┐
│  外部ブリッジ (External Bridges)                │
│  CU ↔ BTC (Lightning)、CU ↔ ステーブルコイン    │
└─────────────────────────────────────────────────┘
```

## クイックスタート

```bash
# ビルド
cargo build --release

# 自動ダウンロードされたモデルでノードを実行
forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# ローカルでチャット
forge chat -m "qwen2.5:0.5b" "重力とは何ですか？"

# シードを開始 (P2P、CUを稼ぐ)
forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# ワーカーとして接続 (P2P、CUを支払う)
forge worker --seed <public_key>

# モデル一覧を表示
forge models
```

## APIリファレンス

### 推論 (OpenAI互換)

| エンドポイント | 説明 |
|----------|-------------|
| `POST /v1/chat/completions` | ストリーミング対応チャット。すべてのレスポンスに `x_forge.cu_cost` を含む |
| `GET /v1/models` | ロードされたモデルの一覧 |

### 経済

| エンドポイント | 説明 |
|----------|-------------|
| `GET /v1/forge/balance` | CU残高、評判、貢献履歴 |
| `GET /v1/forge/pricing` | 市場価格 (EMA平滑化)、コスト見積もり |
| `GET /v1/forge/trades` | 最近の取引とCU量 |
| `GET /v1/forge/network` | 総CUフロー + マークルルート |
| `GET /v1/forge/providers` | 評判とコストでランク付けされたプロバイダー |
| `POST /v1/forge/invoice` | CU残高からLightningインボイスを作成 |
| `GET /settlement` | エクスポート可能な決済明細 |

### 安全

| エンドポイント | 説明 |
|----------|-------------|
| `GET /v1/forge/safety` | キルスイッチの状態、サーキットブレーカー、予算ポリシー |
| `POST /v1/forge/kill` | 緊急停止 — すべてのCUトランザクションを凍結 |
| `POST /v1/forge/policy` | エージェントごとの予算制限を設定 |

## 安全設計

AIエージェントが自律的に計算資源を消費することは強力ですが危険です。Forgeには5つの安全層があります。

| 層 | メカニズム | 保護対象 |
|-------|-----------|------------|
| **キルスイッチ** | オペレーターが即座にすべての取引を凍結 | エージェントの暴走を停止 |
| **予算ポリシー** | エージェントごとの制限：リクエストごと、時間、生涯 | 総露出額を制限 |
| **サーキットブレーカー** | 5回のエラーまたは毎分30回以上の支出で自動発動 | 異常を検知 |
| **速度検出** | 1分間のスライディングウィンドウによる支出率監視 | バースト支出を防止 |
| **人間の承認** | しきい値を超える取引には人間の承認が必要 | 高額支出を保護 |

設計原則: **フェイルセーフ**。安全性を判断できない場合は、アクションを**拒否**します。

## アイデア

| 時代 | 基準 | 裏打ち |
|-----|----------|---------|
| 古代 | 金 | 地質学的な希少性 |
| 1944–1971 | ブレトン・ウッズ | 金に固定された米ドル |
| 1971–現在 | ペトロダラー | 石油需要 + 軍事力 |
| 2009–現在 | Bitcoin | SHA-256へのエネルギー (無意味な仕事) |
| **現在** | **計算本位制 (Compute Standard)** | **LLM推論へのエネルギー (有用な仕事)** |

Forgeを実行しているMac Miniが並んだ部屋は、オーナーが眠っている間に有用な仕事を遂行して収益を生み出すアパートのようなものです。

## プロジェクト構造

```
forge/
├── crates/
│   ├── forge-ledger/      # CU会計、取引、価格設定、安全、マークルルート
│   ├── forge-node/        # ノードデーモン、HTTP API、パイプラインコーディネーター
│   ├── forge-cli/         # CLI: チャット、シード、ワーカー、決済、ウォレット
│   ├── forge-lightning/   # CU ↔ Bitcoin Lightningブリッジ
│   ├── forge-net/         # P2P: iroh QUIC + Noise + ゴシップ
│   ├── forge-proto/       # 通信プロトコル: 17種類のメッセージ
│   ├── forge-infer/       # 推論エンジン: llama.cpp、GGUF、Metal/CPU
│   ├── forge-core/        # 型定義: NodeId、CU、Config
│   └── forge-shard/       # トポロジー: レイヤー割り当て
└── docs/                  # 仕様書、脅威モデル、ロードマップ
```

Rustで約10,000行。76のテスト。2回のセキュリティ監査完了。

## ドキュメント

- [コンセプトとビジョン](docs/concept.md) — なぜ計算がお金なのか
- [経済モデル](docs/economy.md) — CU経済、有益な仕事の証明
- [アーキテクチャ](docs/architecture.md) — 二層設計
- [通信プロトコル](docs/protocol-spec.md) — 17種類のメッセージ
- [ロードマップ](docs/roadmap.md) — 開発フェーズ
- [脅威モデル](docs/threat-model.md) — セキュリティ + 経済的攻撃
- [ブートストラップ](docs/bootstrap.md) — 起動、劣化、回復

## ライセンス

MIT

## 謝辞

Forgeの分散型推論は、Michael Neale氏による [mesh-llm](https://github.com/michaelneale/mesh-llm) に基づいて構築されています。[CREDITS.md](CREDITS.md) を参照してください。
