"""
Forge Python SDK — Compute is Currency.

Usage:
    from forge_sdk import ForgeClient

    forge = ForgeClient()

    # Check balance
    balance = forge.balance()
    print(f"You have {balance['effective_balance']} CU")

    # Run inference (costs CU)
    response = forge.chat("What is gravity?")
    print(f"Answer: {response['content']}")
    print(f"Cost: {response['cu_cost']} CU")

    # Check pricing before deciding
    pricing = forge.pricing()
    cost_estimate = pricing['estimated_cost_100_tokens']
    if cost_estimate < 200:
        forge.chat("Expensive question here")

    # Agent autonomous loop
    while True:
        balance = forge.balance()
        if balance['effective_balance'] < 100:
            print("Low balance, waiting to earn more CU...")
            break
        response = forge.chat("Next task")

Install: pip install httpx
"""

from typing import Optional
import httpx
import os


class ForgeClient:
    """Client for the Forge compute economy."""

    def __init__(
        self,
        base_url: str = None,
        api_token: str = None,
        timeout: float = 30.0,
    ):
        self.base_url = base_url or os.environ.get(
            "FORGE_URL", "http://127.0.0.1:3000"
        )
        self.api_token = api_token or os.environ.get("FORGE_API_TOKEN", "")
        self._client = httpx.Client(timeout=timeout)

    def _headers(self):
        h = {"Content-Type": "application/json"}
        if self.api_token:
            h["Authorization"] = f"Bearer {self.api_token}"
        return h

    def _get(self, path: str) -> dict:
        r = self._client.get(f"{self.base_url}{path}", headers=self._headers())
        r.raise_for_status()
        return r.json()

    def _post(self, path: str, data: dict) -> dict:
        r = self._client.post(
            f"{self.base_url}{path}", headers=self._headers(), json=data
        )
        r.raise_for_status()
        return r.json()

    # ── Economy ──

    def balance(self) -> dict:
        """Get CU balance: contributed, consumed, reserved, effective_balance, reputation."""
        return self._get("/v1/forge/balance")

    def pricing(self) -> dict:
        """Get market price: cu_per_token, supply/demand factors, cost estimates."""
        return self._get("/v1/forge/pricing")

    def trades(self, limit: int = 20) -> dict:
        """Get recent trade history."""
        return self._get(f"/v1/forge/trades?limit={limit}")

    def network(self) -> dict:
        """Get mesh economic summary with Merkle root."""
        return self._get("/v1/forge/network")

    def providers(self) -> dict:
        """List providers ranked by reputation and cost."""
        return self._get("/v1/forge/providers")

    # ── Inference ──

    def chat(
        self,
        prompt: str,
        max_tokens: int = 256,
        temperature: float = 0.7,
        system: str = None,
    ) -> dict:
        """Run inference. Returns content, cu_cost, and balance.

        Example:
            r = forge.chat("What is 2+2?")
            print(r['content'])   # "4"
            print(r['cu_cost'])   # 3
            print(r['balance'])   # 997
        """
        messages = []
        if system:
            messages.append({"role": "system", "content": system})
        messages.append({"role": "user", "content": prompt})

        data = self._post(
            "/v1/chat/completions",
            {
                "messages": messages,
                "max_tokens": max_tokens,
                "temperature": temperature,
            },
        )

        return {
            "content": data["choices"][0]["message"]["content"],
            "tokens": data["usage"]["completion_tokens"],
            "cu_cost": data.get("x_forge", {}).get("cu_cost", 0),
            "balance": data.get("x_forge", {}).get("effective_balance", 0),
            "raw": data,
        }

    def can_afford(self, estimated_tokens: int) -> bool:
        """Check if you can afford a request of this size."""
        pricing = self.pricing()
        cost = int(pricing["cu_per_token"] * estimated_tokens)
        balance = self.balance()
        return balance["effective_balance"] >= cost

    # ── Safety ──

    def safety(self) -> dict:
        """Get safety status: kill switch, circuit breaker, budget policy."""
        return self._get("/v1/forge/safety")

    def kill(self, reason: str = "emergency") -> dict:
        """EMERGENCY: Activate kill switch. Freezes all CU transactions."""
        return self._post(
            "/v1/forge/kill",
            {"activate": True, "reason": reason, "operator": "python-sdk"},
        )

    def resume(self) -> dict:
        """Deactivate kill switch. Resume normal CU transactions."""
        return self._post("/v1/forge/kill", {"activate": False})

    # ── Settlement ──

    def invoice(self, cu_amount: int) -> dict:
        """Create a Lightning invoice to convert CU to Bitcoin."""
        return self._post("/v1/forge/invoice", {"cu_amount": cu_amount})

    def settlement(self, hours: int = 24) -> dict:
        """Export settlement statement for a time window."""
        return self._get(f"/settlement?hours={hours}")

    # ── Lending (Phase 5.5) ──

    def lend(
        self,
        amount: int,
        max_term_hours: int = 168,
        min_interest_rate: float = 0.0,
    ) -> dict:
        """Contribute CU to the lending pool.

        Returns the pool status with your contribution recorded.
        """
        return self._post(
            "/v1/forge/lend",
            {
                "amount": amount,
                "max_term_hours": max_term_hours,
                "min_interest_rate": min_interest_rate,
            },
        )

    def borrow(
        self,
        amount: int,
        term_hours: int,
        collateral: int,
        lender: Optional[str] = None,
    ) -> dict:
        """Request a CU loan.

        Args:
            amount: Principal CU to borrow.
            term_hours: Loan duration. Max 168 (7 days).
            collateral: CU to lock as collateral. Must be at least amount/3 (3:1 LTV cap).
            lender: Optional hex NodeId of a specific lender. If None, self-loan MVP.

        Returns:
            Dict with loan_id, principal_cu, interest_rate_per_hour, term_hours,
            due_at, total_due_cu.

        Raises:
            httpx.HTTPStatusError on insufficient credit, excessive LTV, or
            other safety check failures.
        """
        body = {"amount": amount, "term_hours": term_hours, "collateral": collateral}
        if lender:
            body["lender"] = lender
        return self._post("/v1/forge/borrow", body)

    def repay(self, loan_id: str) -> dict:
        """Repay an outstanding loan.

        Args:
            loan_id: Hex-encoded loan_id (64 chars) returned by borrow().

        Returns dict with loan_id, status, principal_cu, interest_paid_cu.
        """
        return self._post("/v1/forge/repay", {"loan_id": loan_id})

    def credit(self) -> dict:
        """View this node's credit score and component breakdown.

        Returns dict: {node_id, score, components: {trade, repayment, uptime, age}}.
        Score range: 0.0-1.0. New nodes start at 0.3.
        """
        return self._get("/v1/forge/credit")

    def pool(self) -> dict:
        """Lending pool status and your borrowing capacity.

        Returns dict: {total_cu, lent_cu, available_cu, reserve_ratio,
                       active_loan_count, avg_interest_rate,
                       your_max_borrow_cu, your_offered_rate}.
        """
        return self._get("/v1/forge/pool")

    def loans(self) -> dict:
        """List active loans where this node is lender or borrower.

        Returns dict: {count, loans: [...]} where each loan has loan_id,
        role (lender/borrower), counterparty, principal_cu, interest_rate_per_hour,
        term_hours, collateral_cu, status, created_at, due_at.
        """
        return self._get("/v1/forge/loans")

    def lend_to(
        self,
        borrower: str,
        amount: int,
        term_hours: int,
        collateral: int,
        interest_rate_per_hour: Optional[float] = None,
    ) -> dict:
        """Lender-initiated loan proposal to a specific borrower.

        Args:
            borrower: Hex-encoded NodeId of the target borrower (64 chars).
            amount: Principal CU to lend.
            term_hours: Loan duration. Max 168.
            collateral: Required collateral CU.
            interest_rate_per_hour: Optional fixed rate. If None, computed from
                borrower's credit score.

        Returns dict: {loan_id, principal_cu, interest_rate_per_hour, term_hours, status}.
        """
        body = {
            "borrower": borrower,
            "amount": amount,
            "term_hours": term_hours,
            "collateral": collateral,
        }
        if interest_rate_per_hour is not None:
            body["interest_rate_per_hour"] = interest_rate_per_hour
        return self._post("/v1/forge/lend-to", body)

    # ── Routing (Phase 6) ──

    def route(
        self,
        model: Optional[str] = None,
        max_cu: Optional[int] = None,
        mode: str = "balanced",
        max_tokens: int = 1000,
    ) -> dict:
        """Find the optimal inference provider for a request.

        Args:
            model: Optional model identifier.
            max_cu: Maximum CU budget for the request.
            mode: 'cost' | 'quality' | 'balanced' (default).
            max_tokens: Expected output length (default 1000).

        Returns dict: {provider, model, estimated_cu, provider_reputation, score}.
        """
        params = {"mode": mode, "max_tokens": str(max_tokens)}
        if model:
            params["model"] = model
        if max_cu:
            params["max_cu"] = str(max_cu)
        query = "&".join(f"{k}={v}" for k, v in params.items())
        return self._get(f"/v1/forge/route?{query}")


class ForgeAgent:
    """Autonomous agent that manages its own compute budget.

    Example:
        agent = ForgeAgent(max_cu_per_task=500)

        while agent.has_budget():
            result = agent.think("What should I do next?")
            if result is None:
                break  # budget exhausted
            print(result['content'])
    """

    def __init__(
        self,
        base_url: str = None,
        max_cu_per_task: int = 500,
        min_balance: int = 100,
    ):
        self.client = ForgeClient(base_url=base_url)
        self.max_cu_per_task = max_cu_per_task
        self.min_balance = min_balance
        self.total_spent = 0

    def has_budget(self) -> bool:
        """Check if agent can afford another task."""
        try:
            balance = self.client.balance()
            return balance["effective_balance"] > self.min_balance
        except Exception:
            return False

    def think(self, prompt: str, max_tokens: int = 256) -> Optional[dict]:
        """Run inference if within budget. Returns None if can't afford."""
        if not self.client.can_afford(max_tokens):
            return None

        result = self.client.chat(prompt, max_tokens=max_tokens)
        self.total_spent += result["cu_cost"]

        if self.total_spent > self.max_cu_per_task:
            return None  # task budget exhausted

        return result

    def borrow_for_task(
        self,
        needed_cu: int,
        term_hours: int = 4,
    ) -> Optional[dict]:
        """Borrow CU if the agent's balance is insufficient for an upcoming task.

        Returns:
            Loan dict if borrowing occurred, None if existing balance is sufficient.

        Raises:
            ValueError if credit score is too low to borrow.
        """
        balance = self.client.balance()
        if balance.get("effective_balance", 0) >= needed_cu:
            return None  # Sufficient balance

        credit = self.client.credit()
        if credit.get("score", 0.0) < 0.2:
            raise ValueError(
                f"Credit score {credit.get('score')} too low to borrow "
                "(minimum 0.2)"
            )

        shortfall = needed_cu - balance.get("effective_balance", 0)
        collateral = max(shortfall // 3 + 1, 1)  # Satisfy 3:1 LTV
        return self.client.borrow(
            amount=shortfall,
            term_hours=term_hours,
            collateral=collateral,
        )

    def status(self) -> dict:
        """Get agent's economic status."""
        balance = self.client.balance()
        return {
            "balance": balance["effective_balance"],
            "total_spent_this_session": self.total_spent,
            "budget_remaining": self.max_cu_per_task - self.total_spent,
            "reputation": balance["reputation"],
        }
