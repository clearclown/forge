<div align="center">

# Forge

**计算即货币。每一瓦特都在产生智能，而非浪费。**

[![PyPI: forge-sdk](https://img.shields.io/pypi/v/forge-sdk?label=forge-sdk&color=3775A9)](https://pypi.org/project/forge-sdk/)
[![PyPI: forge-cu-mcp](https://img.shields.io/pypi/v/forge-cu-mcp?label=forge-cu-mcp&color=3775A9)](https://pypi.org/project/forge-cu-mcp/)
[![Crates.io](https://img.shields.io/crates/v/forge?label=crates.io&color=e6522c)](https://crates.io/crates/forge)
[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](../../../LICENSE)

---

[English](../../../README.md) · [日本語](../ja/README.md) · **简体中文** · [繁體中文](../zh-TW/README.md) · [Español](../es/README.md) · [Français](../fr/README.md) · [Русский](../ru/README.md) · [Українська](../uk/README.md) · [हिन्दी](../hi/README.md) · [العربية](../ar/README.md) · [فارسی](../fa/README.md) · [עברית](../he/README.md)

</div>

**Forge 是一种分布式推理协议，其中计算即金钱。** 节点通过为他人执行有用的 LLM 推理来赚取计算单位 (CU)。与比特币不同——在比特币中，电力被浪费在毫无意义的哈希计算上——在 Forge 节点上花费的每一焦耳都会产生某人真正需要的真实智能。

分布式推理引擎基于 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 构建。Forge 在其上添加了计算经济层：CU 核算、有用工作证明 (Proof of Useful Work)、动态定价、自主代理预算和故障安全控制。参见 [CREDITS.md](../../../CREDITS.md)。

**集成分叉：** [forge-mesh](https://github.com/nm-arealnormalman/mesh-llm) — 内置 Forge 经济层的 mesh-llm。

## 现场演示

这是来自正在运行的 Forge 节点的真实输出。每次推理都会消耗 CU。每个 CU 都是通过有用的计算赚取的。

```
$ forge node -m "qwen2.5:0.5b" --ledger forge-ledger.json
  Model loaded: Qwen2.5-0.5B (Metal-accelerated, 491MB)
  API server listening on 127.0.0.1:3000
```

**检查余额 — 每个新节点都会获得 1,000 CU 的免费额度：**
```
$ curl localhost:3000/v1/forge/balance
{
  "effective_balance": 1000,
  "contributed": 0,
  "consumed": 0,
  "reputation": 0.5
}
```

**提出问题 — 推理消耗 CU：**
```
$ curl localhost:3000/v1/chat/completions \
    -d '{"messages":[{"role":"user","content":"用中文打个招呼"}]}'
{
  "choices": [{"message": {"content": "你好！"}}],
  "usage": {"completion_tokens": 2},
  "x_forge": {
    "cu_cost": 2,
    "effective_balance": 1002
  }
}
```

每个响应都包含 `x_forge` — **该计算的 CU 成本**以及剩余余额。提供者赚取了 2 CU，消费者花费了 2 CU。物理学为每个单位提供了支撑。

**三次推理后 — 账本上的真实交易：**
```
$ curl localhost:3000/v1/forge/trades
{
  "count": 3,
  "trades": [
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 5, "tokens_processed": 5, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"},
    {"cu_amount": 2, "tokens_processed": 2, "model_id": "qwen2.5-0.5b-instruct-q4_k_m"}
  ]
}
```

**每笔交易都有一个默克尔根 — 可锚定到比特币以获得不可篡改的证明：**
```
$ curl localhost:3000/v1/forge/network
{
  "total_trades": 3,
  "total_contributed_cu": 12,
  "merkle_root": "aac8db9f62dd9ff23926195a70ed8fcfc188fc867d9f2adabd8e694beb338748"
}
```

**AI 代理失控？紧急开关可在几毫秒内冻结一切：**
```
$ curl -X POST localhost:3000/v1/forge/kill \
    -d '{"activate":true, "reason":"检测到异常", "operator":"admin"}'
→ 紧急开关已激活 (KILL SWITCH ACTIVATED)
→ 所有 CU 交易已冻结。代理无法支出。
```

**安全控制始终开启：**
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

## 为什么选择 Forge

```
比特币: 电力 → 毫无意义的 SHA-256 → BTC
Forge: 电力 → 有用的 LLM 推理 → CU
```

比特币证明了"电力 → 计算 → 金钱"。但比特币的计算是没有目的的。Forge 将其反转：每个 CU 都代表了解决某人实际问题的真实智能。

**其他项目不具备的四个特点：**

### 1. 计算 = 货币

每次推理都是一笔交易。提供者赚取 CU，消费者支出 CU。没有区块链，没有代币，没有 ICO。CU 由物理学支撑——即为有用工作而消耗的电力。

### 2. 无需区块链的防篡改

每笔交易都由双方进行双重签名 (Ed25519)，并在网格中通过 gossip 同步。所有交易的默克尔根可以锚定到比特币进行不可篡改的审计。不需要全球共识——双边加密证明就足够了。

### 3. AI 代理管理自己的计算

手机上的代理在夜间借出空闲计算能力 → 赚取 CU → 购买 70B 模型的访问权限 → 变得更聪明 → 赚得更多。代理自主检查 `/v1/forge/balance` 和 `/v1/forge/pricing`。预算政策和断路器可防止失控支出。

```
代理 (手机上的 1.5B 模型)
  → 通过提供推理在夜间赚取 CU
  → 在 70B 模型上花费 CU → 获得更聪明的回答
  → 更好的决策 → 赚取更多 CU
  → 循环重复 → 代理成长
```

### 4. 计算微金融

节点可以将闲置 CU 以利息借给其他节点。小型节点借入 CU，访问更大的模型，赚取更多 CU，并支付利息偿还。没有其他分布式推理项目提供计算借贷——通过对该领域所有主要项目的竞争分析已得到确认。这是让自我改进循环对每个人（而不仅仅是那些已经拥有强大硬件的人）在经济上都可行的引擎。

## 架构

```
┌─────────────────────────────────────────────────┐
│  L4: 发现 (forge-agora)                         │
│  代理市场、声誉聚合、                             │
│  Nostr NIP-90、Google A2A 支付扩展               │
├─────────────────────────────────────────────────┤
│  L3: 智能 (forge-mind)                          │
│  AutoAgent 自我改进循环、                        │
│  harness 市场、元优化                            │
├─────────────────────────────────────────────────┤
│  L2: 金融 (forge-bank)                          │
│  CU 借贷、收益优化、信用分数                      │
├─────────────────────────────────────────────────┤
│  L1: 经济 (forge — 本仓库)                      │
│  CU 账本、双重签名交易、动态定价、                 │
│  借贷原语、安全控制                               │
├─────────────────────────────────────────────────┤
│  L0: 推理 (forge-mesh / mesh-llm)               │
│  流水线并行、MoE 分片、                           │
│  iroh 网格、Nostr 发现、MLX/llama.cpp            │
└─────────────────────────────────────────────────┘
```

## 快速入门

### 方式一：Python（最快）

```bash
pip install forge-sdk
```

```python
from forge_sdk import ForgeNode

node = ForgeNode(model="qwen2.5:0.5b")
response = node.chat("什么是重力？")
print(f"成本: {response.cu_cost} CU")
```

> [PyPI: forge-sdk](https://pypi.org/project/forge-sdk/) · [PyPI: forge-cu-mcp](https://pypi.org/project/forge-cu-mcp/)

### 方式二：Rust（完全控制）

> **前置条件**：[安装 Rust](https://rustup.rs/)（约 2 分钟）

```bash
# 从源码构建
cargo build --release

# 使用自动下载的模型运行节点
./target/release/forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json

# 本地聊天
./target/release/forge chat -m "qwen2.5:0.5b" "什么是重力？"

# 开始播种 (P2P，赚取 CU)
./target/release/forge seed -m "qwen2.5:0.5b" --ledger forge-ledger.json

# 作为工人连接 (P2P，支出 CU)
./target/release/forge worker --seed <public_key>

# 列出模型
./target/release/forge models
```

> [Crates.io: forge](https://crates.io/crates/forge) · [Rust 安装指南](https://rustup.rs/)

### 方式三：预编译二进制

预编译二进制即将推出。请关注[发布页面](../../../releases)。

### 方式四：Docker

```bash
# 即将推出
docker run -p 3000:3000 clearclown/forge:latest
```

## API 参考

### 推理 (OpenAI 兼容)

| 端点 | 描述 |
|----------|-------------|
| `POST /v1/chat/completions` | 支持流式传输的聊天。每个响应包含 `x_forge.cu_cost` |
| `GET /v1/models` | 列出已加载的模型 |

### 经济

| 端点 | 描述 |
|----------|-------------|
| `GET /v1/forge/balance` | CU 余额、声誉、贡献历史 |
| `GET /v1/forge/pricing` | 市场价格 (EMA 平滑)、成本估算 |
| `GET /v1/forge/trades` | 最近交易及其 CU 金额 |
| `GET /v1/forge/network` | 总 CU 流量 + 默克尔根 |
| `GET /v1/forge/providers` | 按声誉和成本排名的提供者 |
| `POST /v1/forge/invoice` | 从 CU 余额创建 Lightning 发票 |
| `GET /v1/forge/route` | 最佳提供者选择（成本/质量/平衡） |
| `GET /settlement` | 可导出的结算报表 |

### 借贷

| 端点 | 描述 |
|----------|-------------|
| `POST /v1/forge/lend` | 向借贷池提供 CU |
| `POST /v1/forge/borrow` | 申请 CU 贷款 |
| `POST /v1/forge/repay` | 还清未偿贷款 |
| `GET /v1/forge/credit` | 信用分数与历史 |
| `GET /v1/forge/pool` | 借贷池状态 |
| `GET /v1/forge/loans` | 活动贷款 |

### 安全

| 端点 | 描述 |
|----------|-------------|
| `GET /v1/forge/safety` | 紧急开关状态、断路器、预算政策 |
| `POST /v1/forge/kill` | 紧急停机 — 冻结所有 CU 交易 |
| `POST /v1/forge/policy` | 为每个代理设置预算限制 |

## 安全设计

AI 代理自主花费计算资源虽然强大，但也非常危险。Forge 拥有五层安全防护：

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

一间装满运行 Forge 的 Mac Mini 的房间就像一栋公寓楼——通过在业主睡觉时执行有用工作来产生收益。

## 项目结构

```
forge/
├── crates/
│   ├── forge-ledger/      # CU 核算、交易、定价、安全、默克尔根
│   ├── forge-node/        # 节点守护进程、HTTP API、流水线协调器
│   ├── forge-cli/         # CLI: 聊天、播种、工人、结算、钱包
│   ├── forge-lightning/   # CU ↔ 比特币 Lightning 桥接
│   ├── forge-net/         # P2P: iroh QUIC + Noise + gossip
│   ├── forge-proto/       # 线缆协议: 17 种消息类型
│   ├── forge-infer/       # 推理引擎: llama.cpp, GGUF, Metal/CPU
│   ├── forge-core/        # 类型定义: NodeId, CU, Config
│   └── forge-shard/       # 拓扑: 层级分配
└── docs/                  # 规范、威胁模型、路线图
```

约 10,000 行 Rust 代码。76 个测试。完成 2 次安全审计。

## 文档

- [战略](strategy.md) — 竞争定位、借贷规范、5 层架构
- [货币理论](monetary-theory.md) — 为什么 CU 可行：Soddy、比特币、PoUW、仅限 AI 的货币
- [概念与愿景](concept.md) — 为什么计算即金钱
- [经济模型](economy.md) — CU 经济、有用工作证明、借贷
- [架构设计](architecture.md) — 双层设计
- [代理集成](agent-integration.md) — SDK、MCP、借贷工作流
- [线缆协议](protocol-spec.md) — 17 种消息类型
- [路线图](roadmap.md) — 开发阶段
- [威胁模型](threat-model.md) — 安全 + 经济攻击
- [引导启动](bootstrap.md) — 启动、降级、恢复
- [A2A 支付](a2a-payment.md) — 面向代理协议的 CU 支付扩展

## 许可证

MIT

## 致谢

Forge 的分布式推理基于 Michael Neale 的 [mesh-llm](https://github.com/michaelneale/mesh-llm) 构建。参见 [CREDITS.md](../../../CREDITS.md)。
