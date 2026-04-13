<div align="center">

# Tirami

**計算は通貨である。すべての電力は廃棄物でなく知能を生む。**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)
[![Tests](https://img.shields.io/badge/tests-785_passing-brightgreen)]()
[![verify-impl](https://img.shields.io/badge/verify--impl-123%2F123_GREEN-brightgreen)]()

---

[English](../../../README.md) · **日本語** · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami は、計算能力そのものが通貨になる分散型 LLM 推論プロトコルです。** ノードは他のノードのために LLM 推論を実行し、その対価として TRM (Tirami Resource Merit) を獲得します。Bitcoin は電力を無意味なハッシュに燃やしますが、Tirami では 1 ジュールの電力がすべて、誰かが実際に必要としている本物の知能を生み出します。

分散推論エンジンは Michael Neale 氏の [mesh-llm](https://github.com/michaelneale/mesh-llm) を基盤とし、その上に Tirami 独自の計算経済 — TRM 台帳、Proof of Useful Work、動的価格設定、自律エージェント予算管理、フェイルセーフ制御 — を構築しています。詳細は [CREDITS.md](../../../CREDITS.md) を参照してください。

**統合フォーク:** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — mesh-llm に Tirami 経済レイヤーを組み込んだプロダクションランタイム。

---

## ライブデモ

実際に動作する Tirami ノードの出力です。すべての推論に TRM がかかり、すべての TRM は有用な計算で稼がれます。

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**残高確認 — 新規ノードには 1,000 TRM のウェルカムローンが付与されます:**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**質問する — 推論には TRM がかかります:**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"こんにちは"}]}'
{
  "choices": [{"message": {"content": "こんにちは！お元気ですか？"}}],
  "usage": {"completion_tokens": 9},
  "x_forge": {
    "cu_cost": 9,
    "effective_balance": 1009
  }
}
```

すべてのレスポンスに `x_forge` が含まれます — **その計算にかかった TRM コスト**と残高です。プロバイダーは 9 TRM を獲得し、コンシューマーは 9 TRM を支払いました。物理法則がすべての単位を裏打ちしています。

**トケノミクス確認 — Bitcoin に着想を得た供給カーブ:**
```
$ tirami su supply
  Total Supply Cap:    21,000,000,000 TRM
  Total Minted:        0
  Supply Factor:       1.0 (genesis)
  Current Epoch:       0
  Yield Rate:          0.001/hr
```

**すべての取引にはマークルルートがあり、Bitcoin に OP_RETURN アンカーして不変の証拠にできます:**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_cu": 19,
  "merkle_root": "aac8db9f...38748"
}
```

**AI エージェントが暴走したら？ キルスイッチがミリ秒で全凍結します:**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

---

## なぜ Tirami か

```
Bitcoin:  電力  →  無意味な SHA-256  →  BTC
Tirami:   電力  →  有用な LLM 推論  →  TRM
```

Bitcoin は「電力 → 計算 → 通貨」を世界に証明しました。ただし Bitcoin の計算自体には目的がありません。Tirami はその構造を反転させます。すべての TRM は、誰かの実際の問いに答えた本物の知能の証拠です。

**他のどのプロジェクトも実現していない 4 つの特長:**

### 1. 計算 = 通貨 (210 億 TRM 供給上限)

推論リクエスト 1 件が 1 件の取引 (trade) です。プロバイダーは TRM を獲得し、コンシューマーは TRM を支払います。ブロックチェーン不要、トークン不要、ICO なし。TRM は物理法則 — 有用な仕事に消費された電力 — で裏打ちされています。Bitcoin に着想を得たトケノミクス: 210 億 TRM の供給上限、ハルビングエポック、ステーキング (7 日/30 日/90 日/365 日の倍率)、ネットワーク成長のためのリファラルボーナス。

### 2. ブロックチェーンなしの改ざん耐性

すべての取引は両当事者が Ed25519 で二重署名し、メッシュ全体にゴシップ伝播します。全取引のマークルルートは Bitcoin に OP_RETURN でアンカーして不変の監査記録にできます。グローバル合意は不要で、双方向の暗号証明で十分です。

### 3. AI エージェントが自ら計算を管理

スマートフォン上のエージェントが夜間にアイドル計算を提供して TRM を稼ぎ、翌朝に 70B モデルへのアクセスを買って賢くなり、さらに稼ぐ。このサイクルを `/v1/tirami/balance` と `/v1/tirami/pricing` を読んで自律的に回します。予算ポリシーとサーキットブレーカーが暴走支出を防ぎます。

```
エージェント (スマホ上 1.5B モデル)
  → 夜間に推論を提供して TRM を獲得
  → 70B モデルに TRM を支払う → より深い回答を得る
  → より良い判断 → さらに多くの TRM を稼ぐ
  → このループが繰り返される → エージェントが自律的に成長
```

### 4. 計算マイクロファイナンス

アイドル TRM を他のノードに利息付きで貸し出せます。小さなノードが TRM を借りて大きなモデルにアクセスし、稼いで返済する。強力なハードウェアを持つ人だけでなく、誰でも自己改善ループに参加できる経済インフラです。他のどの分散推論プロジェクトにも存在しない機能です。

---

## 5 層アーキテクチャ

```
┌─────────────────────────────────────────────────────────────┐
│  L4: ディスカバリ (tirami-agora) ✅                          │
│  エージェントマーケットプレイス、評判、Nostr NIP-90、         │
│  ガバナンス (ステーク加重投票)                                │
├─────────────────────────────────────────────────────────────┤
│  L3: インテリジェンス (tirami-mind) ✅                       │
│  AutoAgent 自己改善ループ (TRM で課金)、                     │
│  ハーネスマーケットプレイス、メタオプティマイザ、              │
│  フェデレーテッドラーニング足場                               │
├─────────────────────────────────────────────────────────────┤
│  L2: ファイナンス (tirami-bank) ✅                           │
│  戦略、ポートフォリオ、先物、保険、                           │
│  リスクモデル、yield オプティマイザ、ステーキング             │
├─────────────────────────────────────────────────────────────┤
│  L1: 経済 (tirami — このリポジトリ) ✅ Phase 1–13           │
│  TRM 台帳、二重署名取引、動的価格設定、貸付、                 │
│  トケノミクス (210 億上限、ハルビング)、                      │
│  安全制御、Prometheus、Bitcoin アンカー                       │
├─────────────────────────────────────────────────────────────┤
│  L0: 推論 (forge-mesh / mesh-llm) ✅                        │
│  パイプライン並列化、MoE シャーディング、                      │
│  iroh メッシュ、Nostr ディスカバリ、MLX / llama.cpp          │
└─────────────────────────────────────────────────────────────┘

5 層すべて Rust。785 テスト合格。123/123 verify-impl GREEN。
```

---

## クイックスタート

### Option 1: エンドツーエンドデモ (推奨 — コールドスタート約 30 秒)

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

SmolLM2-135M (~100 MB) を HuggingFace から自動ダウンロードし、Metal/CUDA アクセラレーション付きの実ノードを起動して、全 Phase 1–13 エンドポイントを実行し、カラーサマリーを表示します。

デモ完了後、同じノードで以下も動作します:

```bash
# OpenAI 互換クライアントとしてそのまま使える
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# リアルタイムトークンストリーミング
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"こんにちは"}],"stream":true}'

# 経済 / 評判 / メトリクス / アンカー
curl $OPENAI_BASE_URL/forge/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus スクレイプ用 (認証不要)
```

互換性の詳細は [`docs/compatibility.md`](../../compatibility.md) を参照してください。

### Option 2: Rust SDK + MCP (すべて Rust、Python 不要)

```bash
# SDK — 全 Tirami エンドポイント対応の非同期 HTTP クライアント
cargo add tirami-sdk

# MCP サーバー — Claude Code / Cursor / ChatGPT 向け 40 ツール
cargo install tirami-mcp
tirami-mcp  # stdio JSON-RPC サーバー
```

### Option 3: Rust CLI を直接操作

**前提条件**: [Rust をインストール](https://rustup.rs/) (約 2 分)

```bash
cargo build --release

# ノードを起動 — モデルは HuggingFace から自動ダウンロード
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# その他のコマンド:
./target/release/tirami chat -m "smollm2:135m" "重力とは何ですか？"
./target/release/tirami seed -m "qwen2.5:1.5b"               # P2P プロバイダーとして TRM を稼ぐ
./target/release/tirami worker --seed <public_key>            # P2P コンシューマーとして TRM を支払う
./target/release/tirami models                                 # モデルカタログを表示
./target/release/tirami su supply                              # トケノミクスを確認
./target/release/tirami su stake 10000 90d                     # 90 日間 TRM をステーク (2.0× 倍率)
```

---

## API リファレンス

### 推論 — OpenAI 互換

| エンドポイント | 説明 |
|---|---|
| `POST /v1/chat/completions` | ストリーミング対応チャット。レスポンスに `x_forge.cu_cost` を含む |
| `GET /v1/models` | ロード済みモデル一覧 |

### 経済

| エンドポイント | 説明 |
|---|---|
| `GET /v1/tirami/balance` | TRM 残高、評判スコア、貢献履歴 |
| `GET /v1/tirami/pricing` | 市場価格 (EMA 平滑化)、コスト見積もり |
| `GET /v1/tirami/trades` | 最近の取引一覧 (TRM 量) |
| `GET /v1/tirami/network` | TRM 総フロー + マークルルート |
| `GET /v1/tirami/providers` | 評判・コスト順のプロバイダーランキング |
| `POST /v1/tirami/invoice` | TRM 残高から Lightning インボイスを作成 |
| `GET /v1/tirami/route` | 最適プロバイダー選択 (cost / quality / balanced) |
| `GET /settlement` | 決済明細エクスポート |

### トケノミクス (Tirami Su)

| エンドポイント | 説明 |
|---|---|
| `GET /v1/tirami/su/supply` | 供給上限、発行済、エポック、利回り率 |
| `POST /v1/tirami/su/stake` | TRM をステーキング (7 日/30 日/90 日/365 日の倍率) |
| `POST /v1/tirami/su/unstake` | ステーキング解除 |
| `POST /v1/tirami/su/refer` | リファラルを登録 (100 TRM ボーナス) |
| `GET /v1/tirami/su/referrals` | リファラル統計 |

### ガバナンス

| エンドポイント | 説明 |
|---|---|
| `POST /v1/tirami/governance/propose` | ガバナンス提案を作成 |
| `POST /v1/tirami/governance/vote` | ステーク加重投票を行う |
| `GET /v1/tirami/governance/proposals` | アクティブな提案一覧 |
| `GET /v1/tirami/governance/tally/{id}` | 提案の投票集計 |

### 貸付 (Lending)

| エンドポイント | 説明 |
|---|---|
| `POST /v1/tirami/lend` | 貸付プールに TRM を提供 |
| `POST /v1/tirami/borrow` | TRM ローンを申請 |
| `POST /v1/tirami/repay` | 未返済ローンを返済 |
| `GET /v1/tirami/credit` | 信用スコアと返済履歴 |
| `GET /v1/tirami/pool` | 貸付プールの状態 |
| `GET /v1/tirami/loans` | アクティブローン一覧 |

### 安全制御

| エンドポイント | 説明 |
|---|---|
| `GET /v1/tirami/safety` | キルスイッチ状態、サーキットブレーカー、予算ポリシー |
| `POST /v1/tirami/kill` | 緊急停止 — 全 TRM トランザクションを即時凍結 |
| `POST /v1/tirami/policy` | エージェントごとの予算上限を設定 |

### L2/L3/L4 レイヤー

| エンドポイント | 説明 |
|---|---|
| `POST /v1/tirami/bank/tick` | L2 ポートフォリオマネージャーを実行して最適行動を返す |
| `GET /v1/tirami/bank/risk` | VaR 99% リスクモデルの現在値 |
| `POST /v1/tirami/mind/improve` | L3 自己改善サイクルを実行 (TRM を消費) |
| `GET /v1/tirami/agora/find` | L4 エージェントマーケットプレイスでケーパビリティを検索 |
| `GET /v1/tirami/collusion/{hex}` | 共謀検知スコア (Tarjan SCC + ボリュームスパイク) |
| `POST /v1/tirami/admin/save-state` | L2/L3/L4 状態を JSON スナップショットに永続化 |

### 可観測性 (Observability)

| エンドポイント | 説明 |
|---|---|
| `GET /metrics` | Prometheus/OpenMetrics (20+ ゲージ、トケノミクス・ガバナンス含む) |
| `GET /v1/tirami/anchor` | Bitcoin OP_RETURN アンカーペイロード (40 バイト FRGE ヘッダ + マークルルート) |

全エンドポイントはレート制限付き (トークンバケット、30 req/sec)。`/metrics` のみ制限除外。

---

## 安全設計

AI エージェントが自律的に計算を消費する設計は強力である反面、制御を誤ると致命的です。Tirami は 5 層の安全機構を持ちます。

| 層 | メカニズム | 保護内容 |
|---|---|---|
| **キルスイッチ** | オペレーターが全取引をミリ秒単位で凍結 | エージェント暴走の即時停止 |
| **予算ポリシー** | リクエストごと・時間あたり・生涯の上限をエージェント単位で設定 | 総被曝量の制限 |
| **サーキットブレーカー** | 5 回のエラー or 毎分 30 件超の支出で自動トリップ | 異常パターンの検知 |
| **速度検出** | 1 分間スライディングウィンドウで支出レートを監視 | バースト支出の抑制 |
| **人間承認閾値** | 設定額を超えるトランザクションはオペレーター承認が必要 | 高額支出の最終防衛線 |

設計原則: **フェイルセーフ (fail-safe)**。安全性を判断できない場合、常にアクションを**拒否**します。

---

## 理論的背景

| 時代 | 基軸 | 裏打ち |
|---|---|---|
| 古代 | 金本位 | 地質学的希少性 |
| 1944–1971 | ブレトン・ウッズ体制 | USD の金兌換 |
| 1971–現在 | ペトロダラー | 石油需要 + 軍事力 |
| 2009–現在 | Bitcoin | SHA-256 へのエネルギー (無意味な仕事) |
| **現在** | **計算本位制 (Compute Standard)** | **LLM 推論へのエネルギー (有用な仕事)** |

Mac Mini を並べた部屋はアパートのようなものです — オーナーが眠っている間も、有用な計算を実行することで継続的に価値を生みます。

---

## プロジェクト構造

```
tirami/  (このリポジトリ — 5 層すべて、14 の Rust クレート)
├── crates/
│   ├── tirami-ledger/      # TRM 台帳、貸付、トケノミクス、ステーキング、ガバナンス、共謀検知
│   ├── tirami-node/        # ノードデーモン、HTTP API (54+ エンドポイント)、パイプライン
│   ├── tirami-cli/         # CLI: chat, seed, worker, settle, wallet, su
│   ├── tirami-sdk/         # Rust 非同期 HTTP クライアント (54 メソッド)
│   ├── tirami-mcp/         # Rust MCP サーバー (Claude/Cursor 向け 40 ツール)
│   ├── tirami-bank/        # L2: 戦略、ポートフォリオ、先物、保険、リスク
│   ├── tirami-mind/        # L3: AutoAgent 自己改善、フェデレーテッドラーニング
│   ├── tirami-agora/       # L4: エージェントマーケットプレイス、評判、NIP-90
│   ├── tirami-lightning/   # TRM ↔ Bitcoin Lightning ブリッジ (双方向)
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + ゴシップ (取引・ローン・評判)
│   ├── tirami-proto/       # ワイヤプロトコル: 27+ メッセージ型 (署名付き評判含む)
│   ├── tirami-infer/       # 推論: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # 共有型: NodeId, TRM, Config
│   └── tirami-shard/       # トポロジー: レイヤー割り当て
├── scripts/verify-impl.sh  # TDD 準拠テスト (123 アサーション)
└── docs/                   # 仕様、戦略、脅威モデル、ロードマップ
```

Rust 約 20,000 行。**785 テスト合格。** Phase 1–13 実装済み。

---

## エコシステム

| リポジトリ | レイヤー | テスト数 | 状態 |
|---|---|---|---|
| [clearclown/tirami](https://github.com/clearclown/tirami) (このリポジトリ) | L1–L4 | 785 | Phase 1–13 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | 理論・仕様 | 16/16 GREEN | 仕様 + 論文 ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 推論 | 646 | forge-economy 移植済み ✅ |
| clearclown/tirami-bank | L2 ファイナンス | アーカイブ | `crates/tirami-bank/` に統合済み |
| clearclown/tirami-mind | L3 インテリジェンス | アーカイブ | `crates/tirami-mind/` に統合済み |
| clearclown/tirami-agora | L4 ディスカバリ | アーカイブ | `crates/tirami-agora/` に統合済み |

---

## ドキュメント一覧

| ドキュメント | 内容 |
|---|---|
| [Strategy](../../strategy.md) | 競合ポジショニング、貸付仕様、5 層アーキテクチャ詳細 |
| [Monetary Theory](../../monetary-theory.md) | TRM が機能する理由: Soddy / Bitcoin / PoUW / AI 専用通貨論 |
| [Concept & Vision](../../concept.md) | 計算が通貨になる根拠 |
| [Economic Model](../../economy.md) | TRM 経済モデル、Proof of Useful Work、貸付の仕組み |
| [Architecture](../../architecture.md) | 2 層設計 |
| [Agent Integration](../../agent-integration.md) | SDK / MCP / 借入ワークフロー |
| [Wire Protocol](../../protocol-spec.md) | 27+ メッセージ型 |
| [Roadmap](../../roadmap.md) | 開発フェーズ |
| [Threat Model](../../threat-model.md) | セキュリティ脅威・経済的攻撃シナリオ |
| [Bootstrap](../../bootstrap.md) | ノード起動、段階的劣化、障害回復 |
| [A2A Payment](../../a2a-payment.md) | A2A / MCP プロトコル向け TRM 決済拡張 |
| [Compatibility](../../compatibility.md) | llama.cpp / mesh-llm / Ollama / Bittensor 比較表 |
| [BitVM Design](../../bitvm-design.md) | 不正証明によるオプティミスティック検証 |
| [Operator Guide](../../operator-guide.md) | プロダクションでのノード運用ガイド |
| [Developer Guide](../../developer-guide.md) | コントリビューションガイド |
| [FAQ](../../faq.md) | よくある質問 |
| [Migration Guide](../../migration-guide.md) | llama-server / Ollama / Bittensor からの移行 |

---

## 貢献

コントリビューションを歓迎します。まず [CONTRIBUTING.md](../../../CONTRIBUTING.md) と [docs/developer-guide.md](../../developer-guide.md) をお読みください。

**3 つの原則:**

1. **テストファースト** — 新しい経済プリミティブは必ず仕様 (`forge-economics/spec/parameters.md`) を先に確定し、テストを書いてから実装します
2. **仕様準拠** — 数値定数は `parameters.md` の §N を唯一の情報源とします。ハードコードは禁止です
3. **パニック禁止** — `unwrap()` / `expect()` は本番コードパスに使用しません。エラーは `TiramiError` で伝播させます

小さな修正 (typo・テスト追加・軽微なバグ) は直接 PR を送ってください。大きな変更 (新クレート・レイヤー追加・プロトコル変更) はコードを書く前に Issue で設計を議論してください。

---

## ライセンスと謝辞

MIT ライセンス。

Tirami の分散推論エンジンは Michael Neale 氏の [mesh-llm](https://github.com/michaelneale/mesh-llm) を基盤としています。詳細は [CREDITS.md](../../../CREDITS.md) を参照してください。
