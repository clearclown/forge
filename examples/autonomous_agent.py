#!/usr/bin/env python3
"""
Forge Autonomous Agent — AI that manages its own compute budget.

This demonstrates the core Forge vision: an AI agent that earns CU
by serving inference and spends CU to access larger models, growing
its own capabilities over time.

Usage:
    1. Start Forge: forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json
    2. Run: python autonomous_agent.py

The agent will:
    - Check its CU balance
    - Estimate costs before each decision
    - Run inference only if it can afford it
    - Track spending and stop when budget is low
    - Report its economic status
"""

import sys
import time

sys.path.insert(0, "../sdk/python")
from forge_sdk import ForgeAgent


def main():
    print("=== Forge Autonomous Agent ===")
    print("An AI that manages its own compute budget.\n")

    agent = ForgeAgent(max_cu_per_task=200, min_balance=50)

    tasks = [
        "What is the most efficient sorting algorithm? One sentence.",
        "Translate 'hello world' to Japanese. One line.",
        "What is 7 * 8?",
        "Name the largest planet in our solar system.",
        "What year did Bitcoin launch?",
    ]

    for i, task in enumerate(tasks):
        status = agent.status()
        print(f"[Task {i+1}/{len(tasks)}] Balance: {status['balance']} CU | "
              f"Spent: {status['total_spent_this_session']} CU | "
              f"Budget left: {status['budget_remaining']} CU")

        if not agent.has_budget():
            print("  Budget exhausted. Agent stopping.\n")
            break

        result = agent.think(task, max_tokens=32)
        if result is None:
            print(f"  Cannot afford: '{task}'\n")
            break

        print(f"  Q: {task}")
        print(f"  A: {result['content'][:100]}")
        print(f"  Cost: {result['cu_cost']} CU\n")
        time.sleep(0.5)

    print("=== Final Status ===")
    status = agent.status()
    print(f"Balance: {status['balance']} CU")
    print(f"Total spent: {status['total_spent_this_session']} CU")
    print(f"Reputation: {status['reputation']}")
    print("\nEvery CU spent was backed by real computation.")
    print("Every CU earned can buy future intelligence.")


if __name__ == "__main__":
    main()
