<div align="center">

# Tirami

**计算即货币。每一瓦特都在产生智能，而非浪费。**

[![Crates.io](https://img.shields.io/crates/v/tirami-core?label=crates.io&color=e6522c)](https://crates.io/crates/tirami-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · **简体中文** · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Tirami 是一种分布式推理协议，其中计算即金钱。** 节点通过为他人执行有用的 LLM 推理来赚取TRM (Tirami Resource Merit) (TRM)。与比特币不同——在比特币中，电力被浪费在毫无意义的哈希计算上——在 Tirami 节点上花费的每一焦耳都会产生某人真正需要的真实智能。

分布式推理引擎基于 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 构建。Tirami 在其上添加了计算经济层：TRM 核算、有用工作证明 (Proof of Useful Work)、动态定价、自主代理预算和故障安全控制。参见 [CREDITS.md](../../../CREDITS.md)。

**集成分叉：** [tirami-mesh](https://github.com/nm-arealnormalman/mesh-llm) — 内置 Tirami 经济层的 mesh-llm。

## 现场演示

这是来自正在运行的 Tirami 节点的真实输出。每次推理都会消耗 TRM。每个 TRM 都是通过有用的计算赚取的。

```
$ tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**检查余额 — 每个新节点都会获得 1,000 TRM 的免费额度：**
```
$ curl localhost:3000/v1/tirami/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**提出问题 — 推理消耗 TRM：**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"Say hello in Japanese"}]}'
{
  "choices": [{"message": {"content": "こんにちは！ (konnichiwa!)"}}],
  "usage": {"completion_tokens": 9},
  "x_tirami": {
    "trm_cost": 9,
    "effective_balance": 1009
  }
}
```

每个响应都包含 `x_tirami` — **该计算的 TRM 成本**以及剩余余额。提供者赚取了 9 TRM，消费者花费了 9 TRM。物理学为每个单位提供了支撑。

**三次推理后 — 账本上的真实交易：**
```
$ curl localhost:3000/v1/tirami/trades
{
  "count": 3,
  "trades": [
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"trm_amount": 9, "tokens_processed": 9, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**每笔交易都有一个默克尔根 — 可锚定到比特币以获得不可篡改的证明：**
```
$ curl localhost:3000/v1/tirami/network
{
  "total_trades": 3,
  "total_contributed_trm": 19,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI 代理失控？紧急开关可在几毫秒内冻结一切：**
```
$ curl -X POST localhost:3000/v1/tirami/kill \
    -d '{"activate":true, "reason":"anomaly detected", "operator":"admin"}'
→ KILL SWITCH ACTIVATED
→ All TRM transactions frozen. No agent can spend.
```

**安全控制始终开启：**
```
$ curl localhost:3000/v1/tirami/safety
{
  "kill_switch_active": false,
  "circuit_tripped": false,
  "policy": {
    "max_trm_per_hour": 10000,
    "max_trm_per_request": 1000,
    "max_trm_lifetime": 1000000,
    "human_approval_threshold": 5000
  }
}
```

## 为什么选择 Tirami

```
比特币: 电力 → 毫无意义的 SHA-256 → BTC
Tirami: 电力 → 有用的 LLM 推理 → TRM
```

比特币证明了"电力 → 计算 → 金钱"。但比特币的计算是没有目的的。Tirami 将其反转：每个 TRM 都代表了解决某人实际问题的真实智能。

**其他项目不具备的四个特点：**

### 1. 计算 = 货币

每次推理都是一笔交易。提供者赚取 TRM，消费者支出 TRM。没有区块链，没有代币，没有 ICO。TRM 由物理学支撑——即为有用工作而消耗的电力。与 Bittensor (TAO)、Akash (AKT) 或 Golem (GLM) 不同，TRM 无法被投机——它通过执行有用计算来赚取。

### 2. 无需区块链的防篡改

每笔交易都由双方进行双重签名 (Ed25519)，并在网格中通过 gossip 同步。所有交易的默克尔根可以锚定到比特币进行不可篡改的审计。不需要全球共识——双边加密证明就足够了。

### 3. AI 代理管理自己的计算

手机上的代理在夜间借出空闲计算能力 → 赚取 TRM → 购买 70B 模型的访问权限 → 变得更聪明 → 赚得更多。代理自主检查 `/v1/tirami/balance` 和 `/v1/tirami/pricing`。预算政策和断路器可防止失控支出。

```
代理 (手机上的 1.5B 模型)
  → 通过提供推理在夜间赚取 TRM
  → 在 70B 模型上花费 TRM → 获得更聪明的回答
  → 更好的决策 → 赚取更多 TRM
  → 循环重复 → 代理成长
```

### 4. 计算微金融

节点可以将闲置 TRM 以利息借给其他节点。小型节点借入 TRM，访问更大的模型，赚取更多 TRM，并支付利息偿还。没有其他分布式推理项目提供计算借贷。这是让自我改进循环对每个人（而不仅仅是那些已经拥有强大硬件的人）在经济上都可行的引擎。

## 架构

```
┌─────────────────────────────────────────────────┐
│  L4: 发现 (tirami-agora) ✅ v0.1                 │
│  代理市场、声誉聚合、                             │
│  Nostr NIP-90、Google A2A 支付扩展               │
├─────────────────────────────────────────────────┤
│  L3: 智能 (tirami-mind) ✅ v0.1                  │
│  AutoAgent 自我改进循环、                        │
│  harness 市场、元优化                            │
├─────────────────────────────────────────────────┤
│  L2: 金融 (tirami-bank) ✅ v0.1                  │
│  策略、投资组合、期货、保险、                      │
│  风险模型、收益优化器                             │
├─────────────────────────────────────────────────┤
│  L1: 经济 (tirami — 本仓库) ✅ 第 1-13 阶段       │
│  TRM 账本、双重签名交易、动态定价、                 │
│  借贷原语、安全控制                               │
├─────────────────────────────────────────────────┤
│  L0: 推理 (tirami-mesh / mesh-llm) ✅            │
│  流水线并行、MoE 分片、                           │
│  iroh 网格、Nostr 发现、MLX/llama.cpp            │
└─────────────────────────────────────────────────┘

全部 5 层均已存在。生态系统中共有 785 个测试通过。
```

## 快速入门

### 方式一：一键端到端演示（Rust，冷启动约 30 秒）

```bash
git clone https://github.com/clearclown/tirami && cd tirami
bash scripts/demo-e2e.sh
```

此脚本会从 HuggingFace 下载 SmolLM2-135M（约 100 MB），启动带有 Metal/CUDA 加速的真实 Tirami 节点，运行三次真实的聊天完成，遍历第 1-13 阶段的所有端点，并打印彩色摘要。已于 2026-04-09 在 Apple Silicon Metal GPU 上验证。

完成后，同一节点还响应：

```bash
# 兼容 OpenAI 的客户端
export OPENAI_BASE_URL=http://127.0.0.1:3001/v1
export OPENAI_API_KEY=$(cat ~/.tirami/api_token 2>/dev/null || echo "$TOKEN")

# 真实的逐令牌流式传输 (第 11 阶段)
curl -N $OPENAI_BASE_URL/chat/completions \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{"model":"smollm2:135m","messages":[{"role":"user","content":"hi"}],"stream":true}'

# 第 8 阶段经济 / 第 9 阶段声誉 / 第 10 阶段指标 / 锚定
curl $OPENAI_BASE_URL/tirami/balance -H "Authorization: Bearer $OPENAI_API_KEY"
curl $OPENAI_BASE_URL/tirami/anchor?network=mainnet -H "Authorization: Bearer $OPENAI_API_KEY"
curl http://127.0.0.1:3001/metrics  # Prometheus，无需认证
```

完整功能矩阵（对比 llama.cpp / mesh-llm / Ollama / Bittensor / Akash）请参见 [`docs/compatibility.md`](../../../docs/compatibility.md)。

### 方式二：手动 Rust 命令

**前置条件**：[安装 Rust](https://rustup.rs/)（约 2 分钟）

```bash
cargo build --release

# 运行节点 — 自动从 HuggingFace 下载模型
./target/release/tirami node -m "qwen2.5:0.5b" --ledger tirami-ledger.json

# 或以下任意命令：
./target/release/tirami chat -m "smollm2:135m" "什么是重力？"
./target/release/tirami seed -m "qwen2.5:1.5b"               # 作为 P2P 提供者赚取 TRM
./target/release/tirami worker --seed <public_key>           # 作为 P2P 消费者花费 TRM
./target/release/tirami models                                # 列出目录（或使用 HF URL / 简写）
```

**[Crates.io: tirami-core](https://crates.io/crates/tirami-core)** ·
**[兼容性文档](../../../docs/compatibility.md)** ·
**[演示脚本](../../../scripts/demo-e2e.sh)**

### 方式三：预编译二进制 / Docker

预编译二进制文件和 `clearclown/tirami:latest` Docker 镜像在
[releases](../../../releases) 页面跟踪。在此之前，方式一可在两分钟内从源码构建。

## API 参考

### 推理（OpenAI 兼容）

| 端点 | 描述 |
|----------|-------------|
| `POST /v1/chat/completions` | 支持流式传输的聊天。每个响应包含 `x_tirami.cu_cost` |
| `GET /v1/models` | 列出已加载的模型 |

### 经济

| 端点 | 描述 |
|----------|-------------|
| `GET /v1/tirami/balance` | TRM 余额、声誉、贡献历史 |
| `GET /v1/tirami/pricing` | 市场价格 (EMA 平滑)、成本估算 |
| `GET /v1/tirami/trades` | 最近交易及其 TRM 金额 |
| `GET /v1/tirami/network` | 总 TRM 流量 + 默克尔根 |
| `GET /v1/tirami/providers` | 按声誉和成本排名的提供者 |
| `POST /v1/tirami/invoice` | 从 TRM 余额创建 Lightning 发票 |
| `GET /v1/tirami/route` | 最佳提供者选择（成本/质量/平衡） |
| `GET /settlement` | 可导出的结算报表 |

### 借贷

| 端点 | 描述 |
|----------|-------------|
| `POST /v1/tirami/lend` | 向借贷池提供 TRM |
| `POST /v1/tirami/borrow` | 申请 TRM 贷款 |
| `POST /v1/tirami/repay` | 还清未偿贷款 |
| `GET /v1/tirami/credit` | 信用分数与历史 |
| `GET /v1/tirami/pool` | 借贷池状态 |
| `GET /v1/tirami/loans` | 活动贷款 |

### 安全

| 端点 | 描述 |
|----------|-------------|
| `GET /v1/tirami/safety` | 紧急开关状态、断路器、预算政策 |
| `POST /v1/tirami/kill` | 紧急停机 — 冻结所有 TRM 交易 |
| `POST /v1/tirami/policy` | 为每个代理设置预算限制 |

## 安全设计

AI 代理自主花费计算资源虽然强大，但也非常危险。Tirami 拥有五层安全防护：

| 层级 | 机制 | 保护对象 |
|-------|-----------|------------|
| **紧急开关** | 人工操作员立即冻结所有交易 | 停止失控的代理 |
| **预算政策** | 每个代理的限制：单次请求、每小时、终生 | 限制总敞口 |
| **断路器** | 5 次错误或每分钟 30 次以上支出自动跳闸 | 捕捉异常 |
| **速率检测** | 1 分钟滑动窗口监控支出速率 | 防止突发支出 |
| **人工审批** | 超过阈值的交易需要人工确认 | 保护大额支出 |

设计原则：**故障安全 (fail-safe)**。如果任何检查无法确定安全性，它将**拒绝**该操作。

## 构想

| 时代 | 标准 | 支撑 |
|-----|----------|---------|
| 古代 | 黄金 | 地质稀缺性 |
| 1944–1971 | 布雷顿森林体系 | 美元挂钩黄金 |
| 1971–至今 | 石油美元 | 石油需求 + 军事力量 |
| 2009–至今 | 比特币 | SHA-256 上的能源（无用工作） |
| **现在** | **计算本位制 (Compute Standard)** | **LLM 推理上的能源（有用工作）** |

一间装满运行 Tirami 的 Mac Mini 的房间就像一栋公寓楼——通过在业主睡觉时执行有用工作来产生收益。

## 项目结构

```
tirami/  (本仓库 — 第 1 层)
├── crates/
│   ├── tirami-ledger/      # TRM 核算、借贷、agora (NIP-90)、安全
│   ├── tirami-node/        # 节点守护进程、HTTP API（借贷 + 路由）、流水线
│   ├── tirami-cli/         # CLI: 聊天、播种、工人、结算、钱包
│   ├── tirami-lightning/   # TRM ↔ 比特币 Lightning 桥接（双向）
│   ├── tirami-net/         # P2P: iroh QUIC + Noise + gossip（交易 + 贷款）
│   ├── tirami-proto/       # 线缆协议: 27+ 种消息类型，含 Loan*
│   ├── tirami-infer/       # 推理引擎: llama.cpp, GGUF, Metal/CPU
│   ├── tirami-core/        # 类型定义: NodeId, TRM, Config
│   └── tirami-shard/       # 拓扑: 层级分配
├── scripts/verify-impl.sh         # TDD 回归测试（24 个断言）
└── docs/                  # 规范、战略、威胁模型、路线图
```

约 20,000 行 Rust。**785 个测试通过。** 第 1-13 阶段完成。

## 姊妹仓库（完整生态系统）

| 仓库 | 层级 | 测试数 | 状态 |
|------|-------|-------|--------|
| [clearclown/tirami](https://github.com/clearclown/tirami)（本仓库） | L1 经济 | 785 | 第 1-13 阶段 ✅ |
| [clearclown/tirami-bank](https://github.com/clearclown/tirami-bank) | L2 金融 | — | archived |
| [clearclown/tirami-mind](https://github.com/clearclown/tirami-mind) | L3 智能 | — | archived |
| [clearclown/tirami-agora](https://github.com/clearclown/tirami-agora) | L4 发现 | — | archived |
| [clearclown/tirami-economics](https://github.com/clearclown/tirami-economics) | 理论 | 16/16 GREEN | ✅ |
| [nm-arealnormalman/mesh-llm](https://github.com/nm-arealnormalman/mesh-llm) | L0 推理 | 43 (tirami-economy) | ✅ |

## 文档

- [战略](../../../docs/strategy.md) — 竞争定位、借贷规范、5 层架构
- [货币理论](../../../docs/monetary-theory.md) — 为什么 TRM 可行：Soddy、比特币、PoUW、仅限 AI 的货币
- [概念与愿景](../../../docs/concept.md) — 为什么计算即金钱
- [经济模型](../../../docs/economy.md) — TRM 经济、有用工作证明、借贷
- [架构设计](../../../docs/architecture.md) — 双层设计
- [代理集成](../../../docs/agent-integration.md) — SDK、MCP、借贷工作流
- [线缆协议](../../../docs/protocol-spec.md) — 17 种消息类型
- [路线图](../../../docs/roadmap.md) — 开发阶段
- [威胁模型](../../../docs/threat-model.md) — 安全 + 经济攻击
- [引导启动](../../../docs/bootstrap.md) — 启动、降级、恢复
- [A2A 支付](../../../docs/a2a-payment.md) — 面向代理协议的 TRM 支付扩展
- [兼容性](../../../docs/compatibility.md) — 与 llama.cpp / Ollama / Bittensor 的功能矩阵

## 许可证

MIT

## 致谢

Tirami 的分布式推理基于 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 构建。参见 [CREDITS.md](../../../CREDITS.md)。
