# language: en
# Maps to:
#   - docs/specs/tasks/task-2.1-workspace-cli-skeleton.md

Feature: cli
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want cli behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
  Scenario: SCEN-2.1.1 - cargo workspace 能承载 core/cli 代码并通过 cargo check
    Given a promptfoo 0.121.13 compatibility fixture for cli
    When promptfoo-rs executes task 2.1 behavior
    Then TEST-2.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
  Scenario: SCEN-2.1.2 - CLI 暴露 eval/view/cache/redteam/mcp/code-scans/scan-model/import/export skeleton
    Given a promptfoo 0.121.13 compatibility fixture for cli
    When promptfoo-rs executes task 2.1 behavior
    Then TEST-2.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
  Scenario: SCEN-2.1.3 - 未知命令和无效 flag 按稳定 stderr/exit code 返回
    Given a promptfoo 0.121.13 compatibility fixture for cli
    When promptfoo-rs executes task 2.1 behavior
    Then TEST-2.1.3 records the expected observable result
