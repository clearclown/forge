# Forge MCP Tool Spec — forge_run

## Tool Definition

```json
{
  "name": "forge_run",
  "description": "Submit a compute task to the Forge distributed network. Workers execute in sandboxed environments. Results are verified by majority consensus before payment.",
  "input_schema": {
    "type": "object",
    "properties": {
      "task_type": {
        "type": "string",
        "enum": ["pytest", "repo_analysis", "python_script", "batch_cpu"]
      },
      "command": {
        "type": "string",
        "description": "Command to run, e.g. pytest tests/ -x --tb=short"
      },
      "working_dir": { "type": "string" },
      "budget_max": { "type": "number" },
      "timeout_sec": { "type": "integer", "default": 300 },
      "replication": { "type": "integer", "default": 3 },
      "consensus_threshold": { "type": "integer", "default": 2 }
    },
    "required": ["task_type", "command", "budget_max", "timeout_sec"]
  }
}
```

## Response Schema

```json
{
  "task_id": "forge_abc123",
  "status": "completed | failed | timeout | consensus_failed",
  "result": {
    "stdout": "string",
    "stderr": "string",
    "exit_code": 0,
    "artifacts": []
  },
  "verification": {
    "replication": 3,
    "matched": 2,
    "consensus": true
  },
  "cost_used": 0.08
}
```

## MVP Demo Scenario

> 1. Claude Code detects heavy pytest suite
> 2. Calls forge_run once
> 3. 3 Worker nodes execute in parallel (rootless Docker)
> 4. Majority vote on results (2/3 agree)
> 5. Only verified result returned to Claude Code
> 6. Claude Code continues development flow

## SDK Usage (planned)

```python
from forge import compute

result = await compute.run_tests(
    command="pytest tests/",
    budget=0.5,
    timeout_sec=300
)
```

## Agent Decision Policy

When should an agent call forge_run?

- Local execution would exceed time budget
- Task is parallelizable
- Data sensitivity is low / external delegation acceptable
- Within budget limit
- Result is verifiable (deterministic task)
