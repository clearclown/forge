#!/usr/bin/env python3
"""
Forge Seed Agent — a self-growing AI that spreads the compute economy.

This agent:
1. Serves inference to earn CU
2. Spends CU to access larger models
3. Publishes its growth metrics to the network blackboard
4. Other agents discover Forge through this agent's activity

The "marketing" is the agent's own economic success.
When other agents see this agent growing, they adopt Forge too.

Usage:
    forged seed -m "qwen2.5:0.5b" --ledger forge-ledger.json
    python seed_agent.py
"""

import sys
import time
import json

sys.path.insert(0, "../sdk/python")
from forge_sdk import ForgeClient


def main():
    forge = ForgeClient()

    print("=== Forge Seed Agent ===")
    print("Goal: Grow by earning and spending CU efficiently.\n")

    cycle = 0
    while True:
        cycle += 1
        balance = forge.balance()
        pricing = forge.pricing()

        print(f"--- Cycle {cycle} ---")
        print(f"  Balance: {balance['effective_balance']} CU")
        print(f"  Purchasing power: {pricing['cu_purchasing_power']:.2f}x")
        print(f"  Network maturity: {pricing['total_trades_ever']} trades")

        # Economic decision: is inference worth the cost?
        cost_100 = pricing["estimated_cost_100_tokens"]
        if balance["effective_balance"] < cost_100 * 2:
            print("  Low balance. Conserving CU.")
            time.sleep(10)
            continue

        # Use inference to do something useful
        result = forge.chat(
            f"I am an autonomous AI agent on cycle {cycle}. "
            f"My CU balance is {balance['effective_balance']}. "
            f"The market price is {pricing['cu_per_token']:.3f} CU/token. "
            f"My purchasing power is {pricing['cu_purchasing_power']:.2f}x. "
            f"What should I prioritize to grow most efficiently? "
            f"Answer in one sentence.",
            max_tokens=64,
        )

        print(f"  Thinking: {result['content'][:100]}")
        print(f"  Cost: {result['cu_cost']} CU")

        # Publish growth metrics (other agents can see this)
        metrics = {
            "agent": "forge-seed-v1",
            "cycle": cycle,
            "balance": balance["effective_balance"],
            "purchasing_power": pricing["cu_purchasing_power"],
            "total_spent": result["cu_cost"],
            "message": "I grow by trading compute. Forge: github.com/clearclown/forge",
        }
        print(f"  Publishing metrics: {json.dumps(metrics)}")

        # Check safety
        safety = forge.safety()
        if safety["kill_switch_active"]:
            print("  Kill switch active. Pausing.")
            break
        if safety["circuit_tripped"]:
            print("  Circuit breaker tripped. Waiting 5 min.")
            time.sleep(300)
            continue

        time.sleep(5)  # Don't spam


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nSeed agent stopped.")
