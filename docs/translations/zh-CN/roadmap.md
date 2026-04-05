# Forge — 路线图

## 阶段 1: 本地推理 ✅

- `forge-core`: 类型系统 (NodeId, LayerRange, ModelManifest, PeerCapability)
- `forge-infer`: llama.cpp 引擎, GGUF 加载器, 流式 token 生成
- `forge-node`: HTTP API (/chat, /chat/stream, /health)
- `forge-cli`: 带有模型自动下载功能的 `forge chat` 命令

## 阶段 2: P2P 协议 ✅

- `forge-net`: Iroh 传输, Noise 加密, 节点连接
- `forge-proto`: 14 种线缆协议消息类型 (bincode + 长度前缀)
- `forge-node`: 播种/工人流水线, 推理请求/响应
- 集成测试: 2 个节点交换 Hello + 多条消息

## 阶段 3: 远程推理 + 操作员账本 ✅

- `forge-ledger`: CU 核算、交易执行、声誉、收益、市场定价
- `forge-node`: 账本集成到推理流水线中
- 推理前的 CU 余额检查
- 完成后的交易记录
- HMAC-SHA256 账本完整性保护

## 阶段 4: 经济 API ✅

- 兼容 OpenAI 的 API: `POST /v1/chat/completions`, `GET /v1/models`
- CU 计量: 每次推理记录一笔带有 `x_forge` 扩展的交易
- 代理预算端点: `GET /v1/forge/balance`, `GET /v1/forge/pricing`
- CU→Lightning 结算桥接: `forge settle --pay`
- 从 HF Hub 自动解析播种模型
- 支持账本持久化的优雅 Ctrl-C 停机

## 阶段 5: mesh-llm 分叉集成 (下一阶段)

**目标:** 使用 mesh-llm 成熟的分布式引擎替换 Forge 的推理层。

| 交付物 | 描述 |
|---|---|
| 分叉 mesh-llm | 创建集成了经济层的 mesh-llm 分叉版本 forge |
| 集成 forge-ledger | 将 CU 记录挂钩到 mesh-llm 的推理流水线中 |
| 保留经济 API | 在新代码库中保留 /v1/forge/* 端点 |
| Web 控制台扩展 | 在 mesh-llm 的控制台中添加 CU 余额和交易可见性 |
| 流水线 + MoE | 继承 mesh-llm 的流水线并行和专家分片功能 |
| Nostr 发现 | 继承 mesh-llm 的公共网格发现机制 |
| CREDITS.md | 记录对 mesh-llm 的归功说明 |

## 阶段 6: 有用工作证明

**目标:** 使 CU 声明在网络中可验证。

| 交付物 | 描述 |
|---|---|
| 双重签名协议 | 提供者和消费者都对每个 TradeRecord 进行签名 |
| Gossip 同步 | 签名后的交易在网格中传播 |
| 欺诈检测 | 拒绝未签名或不匹配的交易 |
| 声誉 Gossip | 在节点间共享声誉分数 |
| 共谋抗性 | 对交易模式进行统计异常检测 |

## 阶段 7: 外部桥接

**目标:** 让操作员能够将 CU 转换为外部价值。

| 交付物 | 描述 |
|---|---|
| Lightning 桥接 | 通过 LDK 实现自动化的 CU→sats 结算 |
| 稳定币适配器 | CU→USDC/USDT 转换 |
| 法币适配器接口 | 银行转账结算规范 |
| 汇率服务 | 公共 CU/BTC 和 CU/USD 汇率推介 |
| 比特币锚定 | 可选: 定期的默克尔根 → OP_RETURN，用于不可篡改的审计追踪 |

## 阶段 8: 代理自主经济

**目标:** 让 AI 代理管理自己的计算生命周期。

| 交付物 | 描述 |
|---|---|
| 预算政策 | 由人类设置的每个代理的支出限制 |
| 自主交易 | 代理决定何时买卖计算资源 |
| 多模型路由 | 代理基于成本/质量权衡选择模型 |
| 自我强化 | 代理赚取 CU → 购买更大型模型访问权限 → 赚取更多 CU |
| 代理间经济 | 代理交易专门的计算能力（代码模型 vs 聊天模型） |

## 长期计划

| 里程碑 | 描述 |
|---|---|
| SDK 发布 | forge-node 作为具有稳定 API 的可嵌入 Rust 库 |
| 协议 v2 | 基于 v1 经验的向后兼容演进 |
| 跨架构支持 | 支持 NVIDIA GPU, AMD ROCm, RISC-V (通过 mesh-llm) |
| 联邦训练 | 分布式微调，而不仅仅是推理 |
| 计算衍生品 | 关于未来计算能力的远期合约 |

> 协议即平台。计算即货币。
