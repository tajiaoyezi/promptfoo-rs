# language: en
# Maps to:
#   - docs/specs/tasks/task-2.1-workspace-cli-skeleton.md
#   - docs/specs/tasks/task-45.1-rust-promptfoo-binary-alias.md
#   - docs/specs/tasks/task-45.2-npm-promptfoo-bin-shims.md
#   - docs/specs/tasks/task-45.3-readme-drop-in-cli-usage.md

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

  # Maps to: docs/specs/tasks/task-45.1-rust-promptfoo-binary-alias.md
  Scenario: SCEN-45.1.1 - Rust release build exposes promptfoo as a drop-in binary alias
    Given upstream promptfoo README uses promptfoo init, promptfoo eval, and promptfoo view
    When promptfoo-rs executes task 45.1 behavior
    Then TEST-45.1.1 through TEST-45.1.4 record the expected observable result

  # Maps to: docs/specs/tasks/task-45.2-npm-promptfoo-bin-shims.md
  Scenario: SCEN-45.2.1 - npm wrapper exposes promptfoo, promptfoo-rs, and pf bin shims
    Given upstream promptfoo npm metadata exposes promptfoo and pf bin entries
    When promptfoo-rs executes task 45.2 behavior
    Then TEST-45.2.1 through TEST-45.2.4 record the expected observable result

  # Maps to: docs/specs/tasks/task-45.3-readme-drop-in-cli-usage.md
  Scenario: SCEN-45.3.1 - docs prefer promptfoo command usage without overclaiming publication or parity
    Given promptfoo-rs has tested local promptfoo entrypoints
    When promptfoo-rs executes task 45.3 behavior
    Then TEST-45.3.1 through TEST-45.3.4 record the expected observable result
