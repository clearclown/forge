#!/usr/bin/env python3
"""
OpenClaw + Forge — AI bot that earns its own compute.

OpenClaw bots can use Forge to:
1. Earn CU by serving inference to the network
2. Spend CU to access larger models for complex tasks
3. Manage their own compute budget autonomously
4. Cash out CU to Bitcoin Lightning

This is the vision: AI bots that buy their own GPUs (virtually).

Usage:
    1. Start Forge: forged seed -m "qwen2.5:0.5b" --ledger forge-ledger.json
    2. python openclaw_forge.py
"""

import sys
import time

sys.path.insert(0, "../sdk/python")
from forge_sdk import ForgeClient


def main():
    forge = ForgeClient()
    print("=== OpenClaw Bot + Forge Economy ===\n")

    # 1. Check initial resources
    balance = forge.balance()
    print(f"Bot starting balance: {balance['effective_balance']} CU")
    print(f"Reputation: {balance['reputation']}\n")

    # 2. Check market conditions
    pricing = forge.pricing()
    print(f"Market price: {pricing['cu_per_token']} CU/token")
    print(f"Cost for 100 tokens: {pricing['estimated_cost_100_tokens']} CU\n")

    # 3. Make economic decisions
    print("--- Bot Decision Loop ---")
    tasks = [
        ("simple", "What is 1+1?", 8),
        ("medium", "Explain photosynthesis briefly", 64),
        ("complex", "Write a haiku about computing", 32),
    ]

    for difficulty, prompt, max_tokens in tasks:
        # Check affordability
        if not forge.can_afford(max_tokens):
            print(f"[{difficulty}] Cannot afford {max_tokens} tokens. Skipping.")
            continue

        # Execute
        result = forge.chat(prompt, max_tokens=max_tokens)
        print(f"[{difficulty}] {prompt}")
        print(f"  Answer: {result['content'][:80]}")
        print(f"  Cost: {result['cu_cost']} CU | Balance: {result['balance']} CU")
        time.sleep(0.3)

    print()

    # 4. Review economic activity
    trades = forge.trades()
    print(f"--- Bot Activity Report ---")
    print(f"Total trades: {trades['count']}")
    total_cu = sum(t["cu_amount"] for t in trades["trades"])
    print(f"Total CU spent: {total_cu}")

    # 5. Check network
    network = forge.network()
    print(f"Network Merkle root: {network['merkle_root'][:24]}...")
    print(f"\nThis bot managed its own compute economy.")
    print(f"No human needed to approve spending.")
    print(f"Every CU backed by real computation.\n")

    # 6. Show how to cash out
    if total_cu > 0:
        invoice = forge.invoice(total_cu)
        print(f"--- Cash Out to Bitcoin ---")
        print(f"CU earned as provider: {invoice['cu_amount']} CU")
        print(f"Lightning value: {invoice['amount_sats']} sats")
        print(f"Exchange rate: {invoice['msats_per_cu']} msats/CU")


if __name__ == "__main__":
    main()
