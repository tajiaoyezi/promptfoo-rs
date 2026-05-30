# language: en
# Maps to:
#   - docs/specs/tasks/task-7.1-redteam-command-flow.md
#   - docs/specs/tasks/task-7.2-redteam-registry-report.md

Feature: redteam-engine
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want redteam-engine behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-7.1-redteam-command-flow.md
  Scenario: SCEN-7.1.1 - redteam.yaml 能被加载并驱动 init/generate/eval/run/report skeleton
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.1 behavior
    Then TEST-7.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-7.1-redteam-command-flow.md
  Scenario: SCEN-7.1.2 - 核心流程可在 mock target 下生成风险结果
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.1 behavior
    Then TEST-7.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-7.1-redteam-command-flow.md
  Scenario: SCEN-7.1.3 - 失败路径输出可定位 report 错误
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.1 behavior
    Then TEST-7.1.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-7.2-redteam-registry-report.md
  Scenario: SCEN-7.2.1 - 核心 redteam plugin/strategy 矩阵登记 P0/P1/P2
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.2 behavior
    Then TEST-7.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-7.2-redteam-registry-report.md
  Scenario: SCEN-7.2.2 - 风险评分字段稳定并可 snapshot
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.2 behavior
    Then TEST-7.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-7.2-redteam-registry-report.md
  Scenario: SCEN-7.2.3 - report 输出能进入 compatibility harness
    Given a promptfoo 0.121.13 compatibility fixture for redteam-engine
    When promptfoo-rs executes task 7.2 behavior
    Then TEST-7.2.3 records the expected observable result
