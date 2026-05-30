# language: en
# Maps to:
#   - docs/specs/tasks/task-5.1-result-store-schema.md
#   - docs/specs/tasks/task-5.2-output-ci-contracts.md

Feature: output-writers
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want output-writers behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-5.1-result-store-schema.md
  Scenario: SCEN-5.1.1 - JSONL append schema 覆盖 result、error、metadata、latency shape
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.1 behavior
    Then TEST-5.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-5.1-result-store-schema.md
  Scenario: SCEN-5.1.2 - SQLite/libSQL schema 支持按 eval、case、provider、assertion 查询
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.1 behavior
    Then TEST-5.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-5.1-result-store-schema.md
  Scenario: SCEN-5.1.3 - 10k case 写入不需要完整结果集常驻内存
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.1 behavior
    Then TEST-5.1.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-5.2-output-ci-contracts.md
  Scenario: SCEN-5.2.1 - JSON/JUnit/CSV 至少可用于 CI 消费
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.2 behavior
    Then TEST-5.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-5.2-output-ci-contracts.md
  Scenario: SCEN-5.2.2 - SARIF 和 HTML 有稳定 data contract snapshot
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.2 behavior
    Then TEST-5.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-5.2-output-ci-contracts.md
  Scenario: SCEN-5.2.3 - stdout/stderr/exit code 与 P0 CLI fixtures 对齐
    Given a promptfoo 0.121.13 compatibility fixture for output-writers
    When promptfoo-rs executes task 5.2 behavior
    Then TEST-5.2.3 records the expected observable result
