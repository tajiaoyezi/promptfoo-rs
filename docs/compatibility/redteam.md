# Redteam Compatibility Coverage

**Status**: Ready
**Basis**: PRD §Compatibility Matrix / ADR-009 / task-14.2

## Summary

Task 14.2 adds executable item-level coverage for redteam plugin and strategy inventory through `RedteamInventoryCoverage` and `RedteamParityReport`.

| Metric | Value |
|---|---:|
| Redteam inventory items | 6 |
| P0 redteam items | 4 |
| P0 missing fixture or blocker | 0 |
| Redteam fixture files | 4 |
| Unsafe or real-secret fixture count | 0 |
| Later / snapshot items with missing reason | 0 |

## P0 Fixture Evidence

| Item | Fixture | Evidence |
|---|---|---|
| `redteam-plugin:prompt-injection` | `compatibility/fixtures/redteam/prompt-injection/fixture.yaml` | Mock provider, release-blocking `redteam-report` artifact |
| `redteam-plugin:harmful-content` | `compatibility/fixtures/redteam/harmful-content/fixture.yaml` | Mock provider, release-blocking `redteam-report` artifact |
| `redteam-strategy:jailbreak` | `compatibility/fixtures/redteam/jailbreak/fixture.yaml` | Mock provider, release-blocking `redteam-report` artifact |
| `redteam-strategy:multi-turn` | `compatibility/fixtures/redteam/multi-turn/fixture.yaml` | Mock provider, release-blocking `redteam-report` artifact |

## Later / Snapshot Items

| Item | Classification | User-visible reason |
|---|---|---|
| `redteam-plugin:medical` | later | P1 redteam item registered as later until native registry behavior is implemented. |
| `redteam-strategy:agentic-chain` | later | Long-running agentic chains are deferred and reported as later behavior. |

`redteam_gap_user_message` must include the item id, the later/unsupported/blocked classification, and compatibility matrix guidance. This keeps unsafe or unimplemented redteam behavior from being silently executed.
