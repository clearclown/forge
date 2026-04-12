"""
Phase 8 SDK tests — one test per new method, all using mock to avoid a live server.
"""

import pytest
from unittest.mock import MagicMock, patch
from forge_sdk import ForgeClient


@pytest.fixture
def client():
    return ForgeClient(base_url="http://127.0.0.1:3000", api_token="test")


# ---------------------------------------------------------------------------
# L2 Bank
# ---------------------------------------------------------------------------


def test_bank_portfolio_calls_get(client):
    with patch.object(client, "_get", return_value={"cash_cu": 1000}) as m:
        result = client.bank_portfolio()
        m.assert_called_once_with("/v1/forge/bank/portfolio")
        assert result == {"cash_cu": 1000}


def test_bank_tick_calls_post_empty_body(client):
    with patch.object(client, "_post", return_value=[]) as m:
        result = client.bank_tick()
        m.assert_called_once_with("/v1/forge/bank/tick", {})
        assert result == []


def test_bank_set_strategy_no_fraction(client):
    with patch.object(client, "_post", return_value={"ok": True, "strategy": "balanced"}) as m:
        result = client.bank_set_strategy("balanced")
        m.assert_called_once_with("/v1/forge/bank/strategy", {"strategy": "balanced"})
        assert result["ok"] is True


def test_bank_set_strategy_with_fraction(client):
    with patch.object(client, "_post", return_value={"ok": True, "strategy": "conservative"}) as m:
        result = client.bank_set_strategy("conservative", base_commit_fraction=0.3)
        m.assert_called_once_with(
            "/v1/forge/bank/strategy",
            {"strategy": "conservative", "base_commit_fraction": 0.3},
        )
        assert result["ok"] is True


def test_bank_set_risk(client):
    with patch.object(client, "_post", return_value={"ok": True, "tolerance": "aggressive"}) as m:
        result = client.bank_set_risk("aggressive")
        m.assert_called_once_with("/v1/forge/bank/risk", {"tolerance": "aggressive"})
        assert result["tolerance"] == "aggressive"


def test_bank_list_futures_calls_get(client):
    with patch.object(client, "_get", return_value=[]) as m:
        result = client.bank_list_futures()
        m.assert_called_once_with("/v1/forge/bank/futures")
        assert result == []


def test_bank_create_future_required_fields(client):
    expected = {"contract_id": "abc123"}
    with patch.object(client, "_post", return_value=expected) as m:
        result = client.bank_create_future(
            counterparty_hex="a" * 64,
            notional_cu=5000,
            strike_price_msats=100,
            expires_at_ms=9999999999999,
        )
        m.assert_called_once_with(
            "/v1/forge/bank/futures",
            {
                "counterparty_hex": "a" * 64,
                "notional_cu": 5000,
                "strike_price_msats": 100,
                "expires_at_ms": 9999999999999,
            },
        )
        assert result == expected


def test_bank_create_future_with_margin(client):
    with patch.object(client, "_post", return_value={"contract_id": "x"}) as m:
        client.bank_create_future(
            counterparty_hex="b" * 64,
            notional_cu=1000,
            strike_price_msats=50,
            expires_at_ms=1234567890000,
            margin_fraction=0.05,
        )
        call_args = m.call_args[0][1]
        assert call_args["margin_fraction"] == 0.05


def test_bank_risk_assessment_calls_get(client):
    with patch.object(client, "_get", return_value={"portfolio_value_cu": 10000}) as m:
        result = client.bank_risk_assessment()
        m.assert_called_once_with("/v1/forge/bank/risk-assessment")
        assert "portfolio_value_cu" in result


def test_bank_optimize_calls_post(client):
    with patch.object(client, "_post", return_value={"applied": False, "decisions": [], "rationale": "ok"}) as m:
        result = client.bank_optimize(max_var_99_cu=500)
        m.assert_called_once_with("/v1/forge/bank/optimize", {"max_var_99_cu": 500})
        assert "applied" in result


# ---------------------------------------------------------------------------
# L4 Agora
# ---------------------------------------------------------------------------


def test_agora_register_required_fields(client):
    with patch.object(client, "_post", return_value={"ok": True}) as m:
        result = client.agora_register(
            agent_hex="c" * 64,
            models_served=["qwen3-8b"],
            cu_per_token=3,
            tier="medium",
        )
        m.assert_called_once_with(
            "/v1/forge/agora/register",
            {
                "agent_hex": "c" * 64,
                "models_served": ["qwen3-8b"],
                "cu_per_token": 3,
                "tier": "medium",
            },
        )
        assert result["ok"] is True


def test_agora_register_with_last_seen_ms(client):
    with patch.object(client, "_post", return_value={"ok": True}) as m:
        client.agora_register(
            agent_hex="d" * 64,
            models_served=["llama3"],
            cu_per_token=2,
            tier="small",
            last_seen_ms=1700000000000,
        )
        call_body = m.call_args[0][1]
        assert call_body["last_seen_ms"] == 1700000000000


def test_agora_list_agents_calls_get(client):
    with patch.object(client, "_get", return_value=[]) as m:
        result = client.agora_list_agents()
        m.assert_called_once_with("/v1/forge/agora/agents")
        assert result == []


def test_agora_reputation_calls_get_with_hex(client):
    hex_id = "e" * 64
    with patch.object(client, "_get", return_value={"overall": 0.75}) as m:
        result = client.agora_reputation(hex_id)
        m.assert_called_once_with(f"/v1/forge/agora/reputation/{hex_id}")
        assert result["overall"] == 0.75


def test_agora_find_required_only(client):
    with patch.object(client, "_post", return_value=[]) as m:
        result = client.agora_find(model_patterns=["*"])
        m.assert_called_once_with("/v1/forge/agora/find", {"model_patterns": ["*"]})
        assert result == []


def test_agora_find_all_optional_fields(client):
    with patch.object(client, "_post", return_value=[]) as m:
        client.agora_find(
            model_patterns=["qwen3-*"],
            max_cu_per_token=10,
            tier="large",
            min_reputation=0.5,
        )
        body = m.call_args[0][1]
        assert body["max_cu_per_token"] == 10
        assert body["tier"] == "large"
        assert body["min_reputation"] == 0.5


def test_agora_stats_calls_get(client):
    with patch.object(client, "_get", return_value={"agent_count": 0, "trade_count": 0}) as m:
        result = client.agora_stats()
        m.assert_called_once_with("/v1/forge/agora/stats")
        assert "agent_count" in result


def test_agora_snapshot_calls_get(client):
    with patch.object(client, "_get", return_value={"profiles": [], "trades": []}) as m:
        result = client.agora_snapshot()
        m.assert_called_once_with("/v1/forge/agora/snapshot")
        assert "profiles" in result


def test_agora_restore_calls_post_with_snapshot(client):
    snap = {"profiles": [], "trades": []}
    with patch.object(client, "_post", return_value={"ok": True}) as m:
        result = client.agora_restore(snap)
        m.assert_called_once_with("/v1/forge/agora/restore", snap)
        assert result["ok"] is True


# ---------------------------------------------------------------------------
# L3 Mind
# ---------------------------------------------------------------------------


def test_mind_init_minimal(client):
    with patch.object(client, "_post", return_value={"ok": True, "harness_version": 1}) as m:
        result = client.mind_init(system_prompt="Be helpful.")
        m.assert_called_once_with(
            "/v1/forge/mind/init",
            {"system_prompt": "Be helpful.", "optimizer": "echo"},
        )
        assert result["ok"] is True


def test_mind_init_cu_paid(client):
    with patch.object(client, "_post", return_value={"ok": True, "harness_version": 1}) as m:
        client.mind_init(
            system_prompt="Optimise me.",
            optimizer="cu_paid",
            api_url="https://api.anthropic.com",
            api_key="sk-test",
            model="claude-sonnet-4-6",
        )
        body = m.call_args[0][1]
        assert body["optimizer"] == "cu_paid"
        assert body["api_url"] == "https://api.anthropic.com"
        assert body["api_key"] == "sk-test"
        assert body["model"] == "claude-sonnet-4-6"


def test_mind_state_calls_get(client):
    with patch.object(client, "_get", return_value={"harness_version": 1}) as m:
        result = client.mind_state()
        m.assert_called_once_with("/v1/forge/mind/state")
        assert "harness_version" in result


def test_mind_improve_default_cycles(client):
    with patch.object(client, "_post", return_value={"cycles_executed": 1, "cycles": []}) as m:
        result = client.mind_improve()
        m.assert_called_once_with("/v1/forge/mind/improve", {"n_cycles": 1})
        assert result["cycles_executed"] == 1


def test_mind_improve_custom_cycles(client):
    with patch.object(client, "_post", return_value={"cycles_executed": 5, "cycles": []}) as m:
        result = client.mind_improve(n_cycles=5)
        m.assert_called_once_with("/v1/forge/mind/improve", {"n_cycles": 5})
        assert result["cycles_executed"] == 5


def test_mind_budget_all_fields(client):
    expected = {"ok": True, "max_cu_per_cycle": 100, "max_cu_per_day": 1000, "max_cycles_per_day": 10}
    with patch.object(client, "_post", return_value=expected) as m:
        result = client.mind_budget(
            max_cu_per_cycle=100,
            max_cu_per_day=1000,
            max_cycles_per_day=10,
        )
        m.assert_called_once_with(
            "/v1/forge/mind/budget",
            {"max_cu_per_cycle": 100, "max_cu_per_day": 1000, "max_cycles_per_day": 10},
        )
        assert result["ok"] is True


def test_mind_budget_omits_none_fields(client):
    with patch.object(client, "_post", return_value={"ok": True, "max_cu_per_cycle": 200, "max_cu_per_day": 500, "max_cycles_per_day": 5}) as m:
        client.mind_budget(max_cu_per_cycle=200)
        body = m.call_args[0][1]
        assert "max_cu_per_day" not in body
        assert "max_cycles_per_day" not in body
        assert body["max_cu_per_cycle"] == 200


def test_mind_stats_calls_get(client):
    with patch.object(client, "_get", return_value={"cycle_count": 0, "kept": 0}) as m:
        result = client.mind_stats()
        m.assert_called_once_with("/v1/forge/mind/stats")
        assert "cycle_count" in result
