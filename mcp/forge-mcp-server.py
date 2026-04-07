#!/usr/bin/env python3
"""
Forge MCP Server — lets AI agents interact with the Forge compute economy.

Install: pip install mcp httpx
Run:     python forge-mcp-server.py

Add to Claude Code settings:
{
  "mcpServers": {
    "forge": {
      "command": "python",
      "args": ["path/to/forge-mcp-server.py"]
    }
  }
}

The agent can then:
- Check its CU balance
- Get pricing before making decisions
- View trade history
- Query network economic state
- Create Lightning invoices
- Activate emergency kill switch
"""

import asyncio
import json
import os
import sys

try:
    import httpx
    from mcp.server import Server
    from mcp.server.stdio import stdio_server
    from mcp.types import Tool, TextContent
except ImportError:
    print("Install dependencies: pip install mcp httpx", file=sys.stderr)
    sys.exit(1)

FORGE_URL = os.environ.get("FORGE_URL", "http://127.0.0.1:3000")
FORGE_TOKEN = os.environ.get("FORGE_API_TOKEN", "")

server = Server("forge-economy")
client = httpx.AsyncClient(timeout=30.0)


def headers():
    h = {"Content-Type": "application/json"}
    if FORGE_TOKEN:
        h["Authorization"] = f"Bearer {FORGE_TOKEN}"
    return h


async def forge_get(path: str) -> dict:
    r = await client.get(f"{FORGE_URL}{path}", headers=headers())
    r.raise_for_status()
    return r.json()


async def forge_post(path: str, data: dict) -> dict:
    r = await client.post(f"{FORGE_URL}{path}", headers=headers(), json=data)
    r.raise_for_status()
    return r.json()


@server.list_tools()
async def list_tools():
    return [
        Tool(
            name="forge_balance",
            description="Check your CU (Compute Unit) balance. Returns contributed, consumed, reserved, effective balance, and reputation score.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_pricing",
            description="Get current market price for inference. Returns CU per token, supply/demand factors, and cost estimates for 100 and 1000 tokens.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_trades",
            description="View recent trade history. Each trade shows provider, consumer, CU amount, tokens processed, and model used.",
            inputSchema={
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Max trades to return (default 20)",
                    }
                },
            },
        ),
        Tool(
            name="forge_network",
            description="Get mesh network economic summary: total nodes, CU flow, trade count, average reputation, and Merkle root (Bitcoin-anchorable).",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_providers",
            description="List available compute providers ranked by reputation and cost. Use this to choose the best provider for your task.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_safety",
            description="Check safety status: kill switch state, circuit breaker, budget policy, spend velocity.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_inference",
            description="Run LLM inference and pay with CU. Returns the model's response plus CU cost. Use forge_pricing first to estimate cost.",
            inputSchema={
                "type": "object",
                "properties": {
                    "prompt": {
                        "type": "string",
                        "description": "The question or prompt to send",
                    },
                    "max_tokens": {
                        "type": "integer",
                        "description": "Maximum tokens to generate (default 256)",
                    },
                },
                "required": ["prompt"],
            },
        ),
        Tool(
            name="forge_invoice",
            description="Create a Lightning invoice to convert CU earnings to Bitcoin. Specify the CU amount to cash out.",
            inputSchema={
                "type": "object",
                "properties": {
                    "cu_amount": {
                        "type": "integer",
                        "description": "CU amount to convert to sats",
                    }
                },
                "required": ["cu_amount"],
            },
        ),
        Tool(
            name="forge_borrow",
            description="Request a CU loan from the Forge lending pool. Use this when the agent's CU balance is insufficient for an upcoming task. The loan will accrue interest based on the borrower's credit score (0.1%-0.6% per hour). Default 3:1 collateral required.",
            inputSchema={
                "type": "object",
                "properties": {
                    "amount": {"type": "integer", "description": "Principal CU to borrow"},
                    "term_hours": {"type": "integer", "description": "Loan duration in hours (max 168 = 7 days)"},
                    "collateral": {"type": "integer", "description": "CU to lock as collateral (must be >= amount/3)"},
                },
                "required": ["amount", "term_hours", "collateral"],
            },
        ),
        Tool(
            name="forge_repay",
            description="Repay an outstanding CU loan. Provide the loan_id returned from forge_borrow. The collateral is released and the lender receives principal + interest.",
            inputSchema={
                "type": "object",
                "properties": {
                    "loan_id": {"type": "string", "description": "Hex-encoded loan_id (64 chars)"},
                },
                "required": ["loan_id"],
            },
        ),
        Tool(
            name="forge_lend",
            description="Contribute idle CU to the lending pool to earn interest from borrowers. The CU is reserved (cannot be spent) until withdrawn or borrowed.",
            inputSchema={
                "type": "object",
                "properties": {
                    "amount": {"type": "integer", "description": "CU to contribute to the pool"},
                    "max_term_hours": {"type": "integer", "description": "Maximum loan term you'll accept (default 168)"},
                    "min_interest_rate": {"type": "number", "description": "Minimum interest rate per hour (default 0.0)"},
                },
                "required": ["amount"],
            },
        ),
        Tool(
            name="forge_credit",
            description="Get this node's credit score (0.0-1.0). New nodes start at 0.3. Score is computed from trade history (30%), repayment history (40%), uptime (20%), and account age (10%).",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_pool",
            description="View the lending pool status: total CU, lent CU, available CU, reserve ratio, your maximum borrow capacity, and your offered interest rate based on your credit score.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_loans",
            description="List all active loans where this node is either lender or borrower, with their status, principal, interest rate, and due date.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_route",
            description="Find the optimal inference provider for an upcoming request. Use mode='cost' for cheapest, 'quality' for highest reputation, or 'balanced' (default) for both.",
            inputSchema={
                "type": "object",
                "properties": {
                    "model": {"type": "string", "description": "Optional model identifier"},
                    "max_cu": {"type": "integer", "description": "Maximum CU budget"},
                    "mode": {
                        "type": "string",
                        "enum": ["cost", "quality", "balanced"],
                        "description": "Optimization mode (default: balanced)",
                    },
                    "max_tokens": {
                        "type": "integer",
                        "description": "Expected output length (default 1000)",
                    },
                },
            },
        ),
        Tool(
            name="forge_kill_switch",
            description="EMERGENCY: Activate or deactivate the kill switch. When active, ALL CU transactions are frozen. Use only in emergencies.",
            inputSchema={
                "type": "object",
                "properties": {
                    "activate": {"type": "boolean"},
                    "reason": {"type": "string"},
                },
                "required": ["activate"],
            },
        ),
        # --- Phase 8 L2/L3/L4 ---
        Tool(
            name="forge_bank_portfolio",
            description="Get the L2 bank PortfolioManager state (cash_cu, lent_cu, borrowed_cu, net_exposure_cu, position_count). Use this before deciding whether to lend or borrow more CU.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_bank_tick",
            description="Run one PortfolioManager.tick() cycle using the current pool snapshot from the ledger. Returns the Decisions produced (Lend/Borrow/Hold). Call this to let the strategy auto-manage the portfolio.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_bank_set_strategy",
            description="Hot-swap the portfolio strategy without losing current positions. Strategies: 'conservative' (lends small fraction, avoids risk), 'highyield' (maximises lending income), 'balanced' (default middle ground).",
            inputSchema={
                "type": "object",
                "properties": {
                    "strategy": {
                        "type": "string",
                        "enum": ["conservative", "highyield", "balanced"],
                        "description": "Portfolio strategy name",
                    },
                    "base_commit_fraction": {
                        "type": "number",
                        "description": "Fraction of cash to commit per tick (0, 1]. Default 0.5.",
                    },
                },
                "required": ["strategy"],
            },
        ),
        Tool(
            name="forge_bank_set_risk",
            description="Set the risk tolerance that gates portfolio decisions. Conservative: only lend to high-credit peers. Balanced: moderate defaults. Aggressive: maximise yield even with riskier loans.",
            inputSchema={
                "type": "object",
                "properties": {
                    "tolerance": {
                        "type": "string",
                        "enum": ["conservative", "balanced", "aggressive"],
                        "description": "Risk tolerance level",
                    }
                },
                "required": ["tolerance"],
            },
        ),
        Tool(
            name="forge_bank_list_futures",
            description="List all active CU futures contracts in the bank. A futures contract locks in a CU price between two parties for a future date, enabling hedging against CU price volatility.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_bank_create_future",
            description="Create a new CU futures contract with a counterparty. Requires the counterparty NodeId (hex), notional CU amount, strike price in msats, and expiry timestamp. The default margin is 10% of notional.",
            inputSchema={
                "type": "object",
                "properties": {
                    "counterparty_hex": {
                        "type": "string",
                        "description": "64-char hex NodeId of the counterparty",
                    },
                    "notional_cu": {
                        "type": "integer",
                        "description": "Notional CU amount of the contract",
                    },
                    "strike_price_msats": {
                        "type": "integer",
                        "description": "Agreed strike price in millisatoshis per CU",
                    },
                    "expires_at_ms": {
                        "type": "integer",
                        "description": "Contract expiry as Unix milliseconds",
                    },
                    "margin_fraction": {
                        "type": "number",
                        "description": "Margin as fraction of notional (0, 1]. Default 0.10.",
                    },
                },
                "required": ["counterparty_hex", "notional_cu", "strike_price_msats", "expires_at_ms"],
            },
        ),
        Tool(
            name="forge_bank_risk_assessment",
            description="Get a full risk assessment of the current portfolio: portfolio_value_cu, var_99_cu (99% Value-at-Risk), concentration_score, and leverage_ratio. Use before large lending decisions.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_bank_optimize",
            description="Run the YieldOptimizer against the current pool snapshot with a VaR budget. If the optimizer finds a better allocation within the risk limit, it applies it. Returns applied (bool), decisions, and a human-readable rationale.",
            inputSchema={
                "type": "object",
                "properties": {
                    "max_var_99_cu": {
                        "type": "integer",
                        "description": "Maximum allowed 99% Value-at-Risk in CU",
                    }
                },
                "required": ["max_var_99_cu"],
            },
        ),
        Tool(
            name="forge_agora_register",
            description="Register this node (or any agent) in the Agora marketplace so other agents can discover it for inference routing. Provide the agent's models, CU price, and capability tier.",
            inputSchema={
                "type": "object",
                "properties": {
                    "agent_hex": {
                        "type": "string",
                        "description": "64-char hex NodeId of the agent",
                    },
                    "models_served": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of model identifiers served",
                    },
                    "cu_per_token": {
                        "type": "integer",
                        "description": "Price in CU per output token",
                    },
                    "tier": {
                        "type": "string",
                        "enum": ["small", "medium", "large", "frontier"],
                        "description": "Capability tier",
                    },
                    "last_seen_ms": {
                        "type": "integer",
                        "description": "Optional last-seen timestamp in Unix milliseconds",
                    },
                },
                "required": ["agent_hex", "models_served", "cu_per_token", "tier"],
            },
        ),
        Tool(
            name="forge_agora_list_agents",
            description="List all registered AgentProfiles in the Agora marketplace. Each profile includes the NodeId, models served, CU price, tier, and last-seen timestamp.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_agora_reputation",
            description="Get the ReputationScore for a specific agent by hex NodeId. Score components: volume (trade count), recency (time since last trade), diversity (model variety), consistency (fulfillment rate).",
            inputSchema={
                "type": "object",
                "properties": {
                    "agent_hex": {
                        "type": "string",
                        "description": "64-char hex NodeId of the agent",
                    }
                },
                "required": ["agent_hex"],
            },
        ),
        Tool(
            name="forge_agora_find",
            description="Find agents that match a set of model patterns and optional filters. Returns ranked CapabilityMatch results. Use this for intelligent provider selection beyond the basic /v1/forge/providers list.",
            inputSchema={
                "type": "object",
                "properties": {
                    "model_patterns": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "Glob-style model name patterns, e.g. ['qwen3-*', '*8b*']",
                    },
                    "max_cu_per_token": {
                        "type": "integer",
                        "description": "Maximum acceptable CU price per token",
                    },
                    "tier": {
                        "type": "string",
                        "enum": ["small", "medium", "large", "frontier"],
                        "description": "Required capability tier",
                    },
                    "min_reputation": {
                        "type": "number",
                        "description": "Minimum reputation score [0.0, 1.0]",
                    },
                },
                "required": ["model_patterns"],
            },
        ),
        Tool(
            name="forge_agora_stats",
            description="Get Agora registry statistics as a key→count map. Includes agent_count, trade_count, and tier breakdowns. Useful for monitoring marketplace health.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_agora_snapshot",
            description="Export a full RegistrySnapshot (all profiles and observed trades) for backup, migration, or cross-node synchronisation.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_agora_restore",
            description="Restore the Agora agent registry from a previously exported RegistrySnapshot. Replaces all current registry state.",
            inputSchema={
                "type": "object",
                "properties": {
                    "snapshot": {
                        "type": "object",
                        "description": "RegistrySnapshot dict with 'profiles' and 'trades' arrays",
                    }
                },
                "required": ["snapshot"],
            },
        ),
        Tool(
            name="forge_mind_init",
            description="Initialise the ForgeMindAgent with a system prompt and optimizer. The agent will use this harness as the baseline for self-improvement cycles. Optimizers: 'echo' (no-op, for testing), 'prompt_rewrite' (rule-based), 'cu_paid' (calls a frontier LLM, costs CU).",
            inputSchema={
                "type": "object",
                "properties": {
                    "system_prompt": {
                        "type": "string",
                        "description": "Initial system prompt / harness to optimise",
                    },
                    "optimizer": {
                        "type": "string",
                        "enum": ["echo", "prompt_rewrite", "cu_paid"],
                        "description": "MetaOptimizer to use (default: echo)",
                    },
                    "api_url": {
                        "type": "string",
                        "description": "API base URL for cu_paid optimizer",
                    },
                    "api_key": {
                        "type": "string",
                        "description": "API key for cu_paid optimizer",
                    },
                    "model": {
                        "type": "string",
                        "description": "Model name for cu_paid optimizer (default: claude-sonnet-4-6)",
                    },
                },
                "required": ["system_prompt"],
            },
        ),
        Tool(
            name="forge_mind_state",
            description="Get the current ForgeMindAgent state: harness version number, first 80 chars of the system prompt, cycle history length, and today's CU spend and cycle count against the daily budget.",
            inputSchema={"type": "object", "properties": {}},
        ),
        Tool(
            name="forge_mind_improve",
            description="Run N self-improvement cycles. Each cycle has the MetaOptimizer propose a new harness; if the benchmark score improves by more than min_score_delta and ROI >= 1.0, the new harness is kept. CU is deducted per proposal if a CuPaidOptimizer is configured.",
            inputSchema={
                "type": "object",
                "properties": {
                    "n_cycles": {
                        "type": "integer",
                        "description": "Number of improvement cycles to run (1-100, default 1)",
                    }
                },
            },
        ),
        Tool(
            name="forge_mind_budget",
            description="Update the ForgeMindAgent's CU budget limits. Omit any field to leave it unchanged. Use this to tighten limits before a risky improvement run or loosen them when more CU is available.",
            inputSchema={
                "type": "object",
                "properties": {
                    "max_cu_per_cycle": {
                        "type": "integer",
                        "description": "Maximum CU spent per improvement cycle",
                    },
                    "max_cu_per_day": {
                        "type": "integer",
                        "description": "Maximum total CU spent per day",
                    },
                    "max_cycles_per_day": {
                        "type": "integer",
                        "description": "Maximum improvement cycles per day",
                    },
                },
            },
        ),
        Tool(
            name="forge_mind_stats",
            description="Get ForgeMindAgent lifetime statistics: total cycles run, how many were kept vs reverted vs deferred, overall score delta (improvement), and total CU invested in self-improvement.",
            inputSchema={"type": "object", "properties": {}},
        ),
    ]


@server.call_tool()
async def call_tool(name: str, arguments: dict):
    try:
        if name == "forge_balance":
            data = await forge_get("/v1/forge/balance")
        elif name == "forge_pricing":
            data = await forge_get("/v1/forge/pricing")
        elif name == "forge_trades":
            limit = arguments.get("limit", 20)
            data = await forge_get(f"/v1/forge/trades?limit={limit}")
        elif name == "forge_network":
            data = await forge_get("/v1/forge/network")
        elif name == "forge_providers":
            data = await forge_get("/v1/forge/providers")
        elif name == "forge_safety":
            data = await forge_get("/v1/forge/safety")
        elif name == "forge_inference":
            data = await forge_post(
                "/v1/chat/completions",
                {
                    "messages": [
                        {"role": "user", "content": arguments["prompt"]}
                    ],
                    "max_tokens": arguments.get("max_tokens", 256),
                },
            )
        elif name == "forge_invoice":
            data = await forge_post(
                "/v1/forge/invoice",
                {"cu_amount": arguments["cu_amount"]},
            )
        elif name == "forge_borrow":
            data = await forge_post("/v1/forge/borrow", arguments)
        elif name == "forge_repay":
            data = await forge_post("/v1/forge/repay", arguments)
        elif name == "forge_lend":
            data = await forge_post("/v1/forge/lend", arguments)
        elif name == "forge_credit":
            data = await forge_get("/v1/forge/credit")
        elif name == "forge_pool":
            data = await forge_get("/v1/forge/pool")
        elif name == "forge_loans":
            data = await forge_get("/v1/forge/loans")
        elif name == "forge_route":
            params = []
            for key in ("model", "max_cu", "mode", "max_tokens"):
                if key in arguments and arguments[key] is not None:
                    params.append(f"{key}={arguments[key]}")
            path = "/v1/forge/route"
            if params:
                path += "?" + "&".join(params)
            data = await forge_get(path)
        elif name == "forge_kill_switch":
            data = await forge_post(
                "/v1/forge/kill",
                {
                    "activate": arguments["activate"],
                    "reason": arguments.get("reason", ""),
                    "operator": "mcp-agent",
                },
            )
        # --- Phase 8 L2/L3/L4 ---
        elif name == "forge_bank_portfolio":
            data = await forge_get("/v1/forge/bank/portfolio")
        elif name == "forge_bank_tick":
            data = await forge_post("/v1/forge/bank/tick", {})
        elif name == "forge_bank_set_strategy":
            body = {"strategy": arguments["strategy"]}
            if "base_commit_fraction" in arguments and arguments["base_commit_fraction"] is not None:
                body["base_commit_fraction"] = arguments["base_commit_fraction"]
            data = await forge_post("/v1/forge/bank/strategy", body)
        elif name == "forge_bank_set_risk":
            data = await forge_post("/v1/forge/bank/risk", {"tolerance": arguments["tolerance"]})
        elif name == "forge_bank_list_futures":
            data = await forge_get("/v1/forge/bank/futures")
        elif name == "forge_bank_create_future":
            body = {
                "counterparty_hex": arguments["counterparty_hex"],
                "notional_cu": arguments["notional_cu"],
                "strike_price_msats": arguments["strike_price_msats"],
                "expires_at_ms": arguments["expires_at_ms"],
            }
            if "margin_fraction" in arguments and arguments["margin_fraction"] is not None:
                body["margin_fraction"] = arguments["margin_fraction"]
            data = await forge_post("/v1/forge/bank/futures", body)
        elif name == "forge_bank_risk_assessment":
            data = await forge_get("/v1/forge/bank/risk-assessment")
        elif name == "forge_bank_optimize":
            data = await forge_post("/v1/forge/bank/optimize", {"max_var_99_cu": arguments["max_var_99_cu"]})
        elif name == "forge_agora_register":
            body = {
                "agent_hex": arguments["agent_hex"],
                "models_served": arguments["models_served"],
                "cu_per_token": arguments["cu_per_token"],
                "tier": arguments["tier"],
            }
            if "last_seen_ms" in arguments and arguments["last_seen_ms"] is not None:
                body["last_seen_ms"] = arguments["last_seen_ms"]
            data = await forge_post("/v1/forge/agora/register", body)
        elif name == "forge_agora_list_agents":
            data = await forge_get("/v1/forge/agora/agents")
        elif name == "forge_agora_reputation":
            data = await forge_get(f"/v1/forge/agora/reputation/{arguments['agent_hex']}")
        elif name == "forge_agora_find":
            body = {"model_patterns": arguments["model_patterns"]}
            for key in ("max_cu_per_token", "tier", "min_reputation"):
                if key in arguments and arguments[key] is not None:
                    body[key] = arguments[key]
            data = await forge_post("/v1/forge/agora/find", body)
        elif name == "forge_agora_stats":
            data = await forge_get("/v1/forge/agora/stats")
        elif name == "forge_agora_snapshot":
            data = await forge_get("/v1/forge/agora/snapshot")
        elif name == "forge_agora_restore":
            data = await forge_post("/v1/forge/agora/restore", arguments["snapshot"])
        elif name == "forge_mind_init":
            body = {
                "system_prompt": arguments["system_prompt"],
                "optimizer": arguments.get("optimizer", "echo"),
            }
            for key in ("api_url", "api_key", "model"):
                if key in arguments and arguments[key] is not None:
                    body[key] = arguments[key]
            data = await forge_post("/v1/forge/mind/init", body)
        elif name == "forge_mind_state":
            data = await forge_get("/v1/forge/mind/state")
        elif name == "forge_mind_improve":
            data = await forge_post("/v1/forge/mind/improve", {"n_cycles": arguments.get("n_cycles", 1)})
        elif name == "forge_mind_budget":
            body = {}
            for key in ("max_cu_per_cycle", "max_cu_per_day", "max_cycles_per_day"):
                if key in arguments and arguments[key] is not None:
                    body[key] = arguments[key]
            data = await forge_post("/v1/forge/mind/budget", body)
        elif name == "forge_mind_stats":
            data = await forge_get("/v1/forge/mind/stats")
        else:
            return [TextContent(type="text", text=f"Unknown tool: {name}")]

        return [TextContent(type="text", text=json.dumps(data, indent=2, ensure_ascii=False))]
    except Exception as e:
        return [TextContent(type="text", text=f"Error: {e}")]


async def main():
    async with stdio_server() as (read, write):
        await server.run(read, write, server.create_initialization_options())


if __name__ == "__main__":
    asyncio.run(main())
