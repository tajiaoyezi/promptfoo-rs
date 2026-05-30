# language: en
# Maps to:
#   - docs/specs/tasks/task-3.2-cache-resume-retry.md

Feature: cache-resume-store
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want cache-resume-store behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-3.2-cache-resume-retry.md
  Scenario: SCEN-3.2.1 - cache key fixture 覆盖 provider/config/test case 输入
    Given a promptfoo 0.121.13 compatibility fixture for cache-resume-store
    When promptfoo-rs executes task 3.2 behavior
    Then TEST-3.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-3.2-cache-resume-retry.md
  Scenario: SCEN-3.2.2 - resume 能从 partial JSONL/SQLite 状态继续
    Given a promptfoo 0.121.13 compatibility fixture for cache-resume-store
    When promptfoo-rs executes task 3.2 behavior
    Then TEST-3.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-3.2-cache-resume-retry.md
  Scenario: SCEN-3.2.3 - retry-errors 和 backoff 失败路径有可复现 tests
    Given a promptfoo 0.121.13 compatibility fixture for cache-resume-store
    When promptfoo-rs executes task 3.2 behavior
    Then TEST-3.2.3 records the expected observable result
