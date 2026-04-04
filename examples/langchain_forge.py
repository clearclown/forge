#!/usr/bin/env python3
"""
LangChain + Forge — Use Forge as a LangChain LLM backend.

Every inference through LangChain now costs CU and earns CU for providers.
The Forge node is OpenAI-compatible, so LangChain works out of the box.

Usage:
    1. Start Forge: forged node -m "qwen2.5:0.5b" --ledger forge-ledger.json
    2. pip install langchain-openai
    3. python langchain_forge.py
"""

try:
    from langchain_openai import ChatOpenAI
    from langchain_core.messages import HumanMessage
except ImportError:
    print("Install: pip install langchain-openai")
    exit(1)

import httpx

FORGE_URL = "http://127.0.0.1:3000"


def main():
    # Check balance before starting
    balance = httpx.get(f"{FORGE_URL}/v1/forge/balance").json()
    print(f"Starting balance: {balance['effective_balance']} CU\n")

    # LangChain with Forge backend
    llm = ChatOpenAI(
        base_url=f"{FORGE_URL}/v1",
        api_key="not-needed",  # Forge uses CU, not API keys
        model="qwen2.5-0.5b-instruct-q4_k_m",
        max_tokens=64,
    )

    # Run inference (costs CU)
    response = llm.invoke([HumanMessage(content="Explain gravity in one sentence.")])
    print(f"Response: {response.content}\n")

    # Check balance after
    balance = httpx.get(f"{FORGE_URL}/v1/forge/balance").json()
    print(f"Balance after: {balance['effective_balance']} CU")

    trades = httpx.get(f"{FORGE_URL}/v1/forge/trades").json()
    print(f"Trades recorded: {trades['count']}")
    print("\nEvery LangChain call now has an economic footprint.")


if __name__ == "__main__":
    main()
