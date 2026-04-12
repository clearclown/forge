<div align="center">

# Forge

**計算は通貨である。すべての電力は廃棄物でなく知能を生む。**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · **日本語** · [简体中文](../zh-CN/README.md) · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

Forge は、**計算能力そのものが通貨になる**分散型 LLM 推論 (inference) プロトコルです。ノードは他のノードのために LLM 推論を実行し、その対価として Compute Unit (CU) を獲得します。1 CU = 10⁹ FLOP の検証済み有用計算 (parameters.md §1)。OpenAI 互換 API を完全サポートし、426 テスト / 95 アサーション GREEN、5 層エコシステム全体で 326 テスト合格済みです。

分散推論エンジンは Michael Neale 氏の [mesh-llm](https://github.com/michaelneale/mesh-llm) を基盤とし、その上に Forge 独自の経済層 — CU 台帳、Proof of Useful Work、動的価格設定、AI 自律予算管理、フェイルセーフ制御 — を構築しています。詳細は [CREDITS.md](../../../CREDITS.md) を参照してください。

---

## 30 秒デモ

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

Apple Silicon Metal GPU で検証済み (2026-04-09)。SmolLM2-135M (~100 MB) を HuggingFace から自動ダウンロードし、リアルノードを起動して全 Phase 1–12 エンドポイントを実行します。設定・API キー・アカウント登録は不要です。

```
✓ ノード PID 26860、モデル読み込み完了 (SmolLM2-135M on Metal)
✓ 実際の推論 3 回完了 — contributed=41 CU
✓ マークルルート: 094f694...                  ← 改ざん検知可能な台帳証明
✓ Bitcoin OP_RETURN ペイロード生成済み        ← 外部アンカー可能
✓ Prometheus /metrics: forge_trade_count_total 3
✓ L2/L3/L4 全エンドポイント応答確認
```

デモ完了後、同じノードで OpenAI 互換クライアントがそのまま動作します。

```bash
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.forge/api_token 2>/dev/null || echo "$TOKEN")

# リアルタイムストリーミング (Phase 11 実装)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"こんにちは"}],"stream":true}'

# 残高確認 / Bitcoin アンカー / Prometheus メトリクス
curl $OPENAI_BASE_URL/forge/balance   -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/forge/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics    # 認証不要 (Prometheus スクレイプ用)
```

---

## なぜ Forge か 🔥

```
Bitcoin:  電力  →  無意味な SHA-256  →  BTC  (仕事は無価値)
Forge:    電力  →  有用な LLM 推論  →  CU   (仕事が価値そのもの)
```

Bitcoin は「電力 → 計算 → 通貨」を世界に証明しました。ただし Bitcoin の計算自体には目的がありません。Forge はその構造を反転させます。すべての CU は、誰かの実際の問いに答えた本物の知能の証拠です。

### 1. 計算 = 通貨

推論リクエスト 1 件が 1 件の取引 (trade) です。プロバイダーは CU を獲得し、コンシューマーは CU を支払います。ブロックチェーン不要、トークン不要、ICO なし。CU は取引所で売買できません。獲得するには有用な計算を実行するしかありません。これが投機を構造的に不可能にする設計です。

### 2. ブロックチェーンなしの改ざん耐性

すべての取引は両当事者が Ed25519 で二重署名し、メッシュ全体にゴシップ (gossip) 伝播します。全取引のマークルルートは Bitcoin に OP_RETURN アンカーできます。グローバル合意は不要で、双方向の暗号証明で十分です。

### 3. AI エージェントが自ら予算を管理

スマートフォン上のエージェントが夜間にアイドル計算を提供して CU を稼ぎ、翌朝に 70B モデルへのアクセスを買って賢くなり、さらに稼ぐ。このサイクルを `/v1/forge/balance` と `/v1/forge/pricing` を読んで自律的に回します。予算ポリシーとサーキットブレーカーが暴走支出を防ぎます。

```
エージェント (スマホ上 1.5B モデル)
  → 夜間に推論を提供して CU を獲得
  → 70B モデルに CU を支払う → より深い回答を得る
  → より良い判断 → さらに多くの CU を稼ぐ
  → このループが繰り返される → エージェントが自律的に成長
```

### 4. 計算マイクロファイナンス

アイドル CU を他のノードに利息付きで貸し出せます。小さなノードが CU を借りて大きなモデルにアクセスし、稼いで返済する。他のどの分散推論プロジェクトにも存在しない機能です。強力なハードウェアを持つ人だけでなく、誰でも自己改善ループに参加できる経済インフラです。

---

## 5 層アーキテクチャ

```
┌─────────────────────────────────────────────────────────┐
│  L4: ディスカバリ (forge-agora) ✅ v0.1                 │
│  エージェントマーケットプレイス、評判 (reputation) 集約、 │
│  Nostr NIP-90 データ自動販売機、A2A 決済拡張             │
├─────────────────────────────────────────────────────────┤
│  L3: インテリジェンス (forge-mind) ✅ v0.1              │
│  AutoAgent スタイルの自己改善ループ (CU で課金)、        │
│  ベンチマーク、メタオプティマイザ、ROI ゲート            │
├─────────────────────────────────────────────────────────┤
│  L2: ファイナンス (forge-bank) ✅ v0.1                  │
│  戦略 (Conservative / Balanced / HighYield)、           │
│  先物 (futures)、保険、リスクモデル、yield オプティマイザ │
├─────────────────────────────────────────────────────────┤
│  L1: 経済 (forge — このリポジトリ) ✅ Phase 1–12        │
│  CU 台帳、二重署名取引、動的価格設定、貸付、安全制御、    │
│  Prometheus /metrics、Bitcoin OP_RETURN アンカー         │
├─────────────────────────────────────────────────────────┤
│  L0: 推論 (forge-mesh / mesh-llm) ✅                    │
│  パイプライン並列化、MoE シャーディング、iroh QUIC メッシュ│
│  Nostr ディスカバリ、MLX / llama.cpp バックエンド         │
└─────────────────────────────────────────────────────────┘

5 層すべて実装済み。エコシステム全体で 326 テスト合格。
```

---

## クイックスタート

### Option 1: エンドツーエンドデモ (推奨 — 約 30 秒)

```bash
git clone https://github.com/clearclown/forge && cd forge
bash scripts/demo-e2e.sh
```

全 Phase 1–12 エンドポイントを実際のデータで確認できます。

### Option 2: Python SDK

```bash
pip install forge-sdk forge-cu-mcp
```

```python
from forge_sdk import ForgeClient

c = ForgeClient(base_url="http://localhost:3001")
print("残高:", c.balance())      # CU 残高を確認
print("推奨行動:", c.bank_tick()) # L2 ポートフォリオマネージャーの判断
```

[PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) — L2/L3/L4 を含む 20 メソッド  
[PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/) — Claude Code / Cursor 向け 20 MCP ツール

### Option 3: Rust CLI

**前提条件**: [Rust をインストール](https://rustup.rs/) (約 2 分)

```bash
cargo build --release

# ノードを起動 — モデルは HuggingFace から自動ダウンロード
./target/release/forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# ローカルでチャット
./target/release/forge chat -m "smollm2:135m" "重力とは何ですか？"

# シードノードとして起動 (P2P で推論を提供して CU を稼ぐ)
./target/release/forge seed -m "qwen2.5:1.5b"

# ワーカーとして接続 (P2P でシードから推論を購入)
./target/release/forge worker --seed <public_key>

# 登録済みモデル一覧を確認
./target/release/forge models
```

[Crates.io: forge](https://crates.io/crates/forge) · [互換性ドキュメント](../../compatibility.md) · [デモスクリプト](../../../scripts/demo-e2e.sh)

### Option 4: Docker / ビルド済みバイナリ

ビルド済みバイナリと `clearclown/forge:latest` Docker イメージは [リリースページ](https://github.com/clearclown/forge/releases) で管理されています。現時点では Option 1 がソースから 2 分以内でビルドできます。

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
| `GET /v1/forge/balance` | CU 残高、評判スコア、貢献履歴 |
| `GET /v1/forge/pricing` | 市場価格 (EMA 平滑化)、需給状況、コスト見積もり |
| `GET /v1/forge/trades` | 最近の取引一覧 (プロバイダー / CU 量 / トークン数) |
| `GET /v1/forge/network` | メッシュ全体の CU フロー + マークルルート |
| `GET /v1/forge/providers` | 評判・コスト調整済みのプロバイダーランキング (エージェントルーティング用) |
| `POST /v1/forge/invoice` | CU 残高から Lightning インボイスを作成 |
| `GET /v1/forge/route` | 最適プロバイダー選択 (cost / quality / balanced モード) |
| `GET /settlement` | 決済明細エクスポート (マークルルート付き) |
| `GET /v1/forge/anchor` | Bitcoin OP_RETURN アンカースクリプト生成 |
| `GET /metrics` | Prometheus スクレイプエンドポイント (認証不要) |

### 貸付 (Lending)

| エンドポイント | 説明 |
|---|---|
| `POST /v1/forge/lend` | 貸付プールに CU を提供 |
| `POST /v1/forge/borrow` | CU ローンを申請 |
| `POST /v1/forge/lend-to` | 貸し手主導で特定ノードにローンを提案 |
| `POST /v1/forge/repay` | 未返済ローンを返済 |
| `GET /v1/forge/credit` | 信用スコアと返済履歴 |
| `GET /v1/forge/pool` | 貸付プールの状態 (利用可能残高 / 利用率 / 平均金利) |
| `GET /v1/forge/loans` | 自ノードのアクティブローン一覧 |

### 安全制御

| エンドポイント | 説明 |
|---|---|
| `GET /v1/forge/safety` | キルスイッチ状態、サーキットブレーカー、予算ポリシー |
| `POST /v1/forge/kill` | 緊急停止 — 全 CU トランザクションを即時凍結 |
| `POST /v1/forge/policy` | エージェントごとの予算上限を設定 |

### L2/L3/L4 レイヤー

| エンドポイント | 説明 |
|---|---|
| `POST /v1/forge/bank/tick` | L2 ポートフォリオマネージャーを実行して最適行動を返す |
| `GET /v1/forge/bank/risk` | VaR 99% リスクモデルの現在値 |
| `POST /v1/forge/mind/improve` | L3 自己改善サイクルを 1 回実行 (CU を消費) |
| `GET /v1/forge/agora/find` | L4 エージェントマーケットプレイスでケーパビリティを検索 |
| `GET /v1/forge/collusion/{hex}` | 共謀検知スコアとトラストペナルティのデバッグ |
| `POST /v1/forge/admin/save-state` | L2/L3/L4 状態を JSON スナップショットに永続化 |

全エンドポイントはレート制限付き (トークンバケット、30 req/sec)。`/metrics` のみ制限除外。

---

## 安全設計

AI エージェントが自律的に計算を消費する設計は強力である反面、制御を誤ると致命的です。Forge は 5 層の安全機構を持ちます。

| 層 | メカニズム | 保護内容 |
|---|---|---|
| **キルスイッチ** | オペレーターが全取引をミリ秒単位で凍結 | エージェント暴走の即時停止 |
| **予算ポリシー** | リクエストごと・時間あたり・生涯の上限をエージェント単位で設定 | 総被曝量の制限 |
| **サーキットブレーカー** | 5 回のエラー or 毎分 30 件超の支出で自動トリップ | 異常パターンの検知 |
| **速度検出** | 1 分間スライディングウィンドウで支出レートを監視 | バースト支出の抑制 |
| **人間承認閾値** | 設定額を超えるトランザクションはオペレーター承認が必要 | 高額支出の最終防衛線 |

設計原則: **フェイルセーフ (fail-safe)**。安全性を判断できない場合、常にアクションを**拒否**します。

---

## 他ツールとの比較

| 特性 | **Forge** | Ollama | llama-server | Bittensor | Akash |
|---|---|---|---|---|---|
| LLM 推論 (GGUF / llama.cpp) | ✅ | ✅ | ✅ | 独自 | Docker |
| OpenAI 互換 API | ✅ | ✅ | ✅ | ❌ | ❌ |
| P2P メッシュ (iroh QUIC) | ✅ | ❌ | ❌ | 独自 | ❌ |
| リクエスト単位の CU 計量 | ✅ | ❌ | ❌ | ❌ | ❌ |
| 双方向署名取引台帳 | ✅ | ❌ | ❌ | ❌ | ❌ |
| 計算貸付 (lending pool) | ✅ | ❌ | ❌ | ❌ | ❌ |
| AI 自律予算管理 | ✅ | ❌ | ❌ | ❌ | ❌ |
| 評判ゴシップ + 共謀検知 | ✅ | ❌ | ❌ | △ | ❌ |
| Bitcoin OP_RETURN アンカー | ✅ | ❌ | ❌ | ❌ | ❌ |
| Prometheus /metrics | ✅ | ❌ | ❌ | ❌ | ❌ |
| 投機トークンなし | ✅ | — | — | ❌ (TAO) | ❌ (AKT) |

Forge の立場: 既存の競合はいずれも「電力を無意味な仕事に燃やす (Bitcoin)」「計算コストと切り離された投機トークンを使う (Bittensor, Akash)」「中央集権型商用サービスとして提供する (Together.ai)」のいずれかです。Forge だけが、計算の単位が FLOP であり、仕事の単位が推論であり、単位を増やす唯一の方法が有用な計算を実行することです。

---

## 理論的背景

| 時代 | 基軸 | 裏打ち |
|---|---|---|
| 古代 | 金本位 | 地質学的希少性 |
| 1944–1971 | ブレトン・ウッズ体制 | USD の金兌換 |
| 1971–現在 | ペトロダラー | 石油需要 + 軍事力 |
| 2009–現在 | Bitcoin | SHA-256 へのエネルギー (無意味な仕事) |
| **現在** | **計算本位制 (Compute Standard)** | **LLM 推論へのエネルギー (有用な仕事)** |

ソディ (Frederick Soddy) は 1920 年代に「富は流通中の実エネルギーである」と論じました。Bitcoin はそのエネルギー→通貨の変換を証明しましたが、仕事を無駄にしました。Forge は同じ変換を有用な仕事に向けます。CU は物理的な下限 (~$0.000001/CU 電力コスト) と上限 (~$0.000132/CU Mac Mini 運用コスト) を持ちます (parameters.md §9)。

Mac Mini M4 1 台が Qwen2.5-7B を動かすと、年間約 500 万 CU の生産能力があります (parameters.md §9)。オーナーが眠っている間も、有用な推論を提供することで継続的に価値を生みます。

---

## プロジェクト構造

```
forge/  (このリポジトリ — L1 経済層)
├── crates/
│   ├── forge-ledger/     # CU 台帳、貸付、安全、NIP-90、共謀検知、アンカー
│   ├── forge-node/       # ノードデーモン、HTTP API (45+ エンドポイント)、パイプライン
│   ├── forge-cli/        # CLI: chat / seed / worker / settle / wallet
│   ├── forge-lightning/  # CU ↔ Bitcoin Lightning 双方向ブリッジ
│   ├── forge-net/        # P2P: iroh QUIC + Noise + ゴシップ (取引・ローン)
│   ├── forge-proto/      # ワイヤプロトコル: 27 種類以上のメッセージ型
│   ├── forge-infer/      # 推論: llama.cpp / GGUF / Metal / CUDA / CPU
│   ├── forge-core/       # 共有型: NodeId, CU, Config
│   └── forge-shard/      # トポロジー: レイヤー割り当て
├── sdk/python/           # Python クライアント (全貸付 API 対応)
├── mcp/                  # MCP サーバー (Claude Code / Cursor 向け)
├── scripts/
│   ├── demo-e2e.sh       # エンドツーエンドデモ (Phase 1–12 全エンドポイント)
│   └── verify-impl.sh    # TDD 回帰テスト (95 アサーション)
└── docs/                 # 仕様、戦略、脅威モデル、ロードマップ
```

Rust 約 14,500 行。**426 テスト合格。** Phase 1–12 実装済み。

---

## 姉妹リポジトリ

| リポジトリ | レイヤー | テスト数 | 状態 |
|---|---|---|---|
| [clearclown/forge](https://github.com/clearclown/forge) (このリポジトリ) | L1 経済 | 426 | Phase 1–12 ✅ |
| [clearclown/forge-bank](https://github.com/clearclown/forge-bank) | L2 ファイナンス | 45 | v0.1 ✅ |
| [clearclown/forge-mind](https://github.com/clearclown/forge-mind) | L3 インテリジェンス | 40 | v0.1 ✅ |
| [clearclown/forge-agora](https://github.com/clearclown/forge-agora) | L4 ディスカバリ | 39 | v0.1 ✅ |
| [clearclown/forge-economics](https://github.com/clearclown/forge-economics) | 理論・仕様 | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 推論 | 686 | ✅ |

---

## ドキュメント一覧

| ドキュメント | 内容 |
|---|---|
| [strategy.md](../../strategy.md) | 競合ポジショニング、貸付仕様、5 層アーキテクチャ詳細 |
| [monetary-theory.md](../../monetary-theory.md) | CU が機能する理由: Soddy / Bitcoin / PoUW / AI 専用通貨論 |
| [concept.md](../../concept.md) | 計算が通貨になる根拠、ポスト・マーケティング経済 |
| [economy.md](../../economy.md) | CU 経済モデル、Proof of Useful Work、貸付の仕組み |
| [architecture.md](../../architecture.md) | 経済層と推論層の 2 層設計 |
| [agent-integration.md](../../agent-integration.md) | SDK / MCP / 借入ワークフロー / 信用スコア構築 |
| [protocol-spec.md](../../protocol-spec.md) | ワイヤプロトコル仕様 (17 メッセージ型) |
| [roadmap.md](../../roadmap.md) | 開発フェーズ (Phase 1–13+) |
| [threat-model.md](../../threat-model.md) | セキュリティ脅威・経済的攻撃シナリオ (T1–T17) |
| [bootstrap.md](../../bootstrap.md) | ノード起動、段階的劣化、障害回復 |
| [a2a-payment.md](../../a2a-payment.md) | A2A / MCP プロトコル向け CU 決済拡張 |
| [compatibility.md](../../compatibility.md) | llama.cpp / mesh-llm / Ollama / Bittensor / Akash との比較表 |
| [faq.md](../../faq.md) | よくある質問 12 件 |

---

## 貢献

コントリビューションを歓迎します。まず [CONTRIBUTING.md](../../../CONTRIBUTING.md) と [docs/developer-guide.md](../../developer-guide.md) をお読みください。

**3 つの原則:**

1. **テストファースト** — 新しい経済プリミティブは必ず仕様 (`forge-economics/spec/parameters.md`) を先に確定し、テストを書いてから実装します
2. **仕様準拠** — 数値定数は `parameters.md` の §N を唯一の情報源とします。ハードコードは禁止です
3. **パニック禁止** — `unwrap()` / `expect()` は本番コードパスに使用しません。エラーは `ForgeError` で伝播させます

小さな修正 (typo・テスト追加・軽微なバグ) は直接 PR を送ってください。大きな変更 (新クレート・レイヤー追加・プロトコル変更) はコードを書く前に Issue で設計を議論してください。

---

## ライセンスと謝辞

MIT ライセンス。

Forge の分散推論エンジンは Michael Neale 氏の [mesh-llm](https://github.com/michaelneale/mesh-llm) を基盤としています。詳細は [CREDITS.md](../../../CREDITS.md) を参照してください。
