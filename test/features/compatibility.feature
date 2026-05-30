# language: en
# Maps to:
#   - docs/specs/tasks/task-1.1-baseline-lock.md
#   - docs/specs/tasks/task-1.2-compatibility-matrix.md
#   - docs/specs/tasks/task-6.1-upstream-harness-runner.md
#   - docs/specs/tasks/task-6.2-golden-diff-release-gate.md

Feature: compatibility
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want compatibility behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-1.1-baseline-lock.md
  Scenario: SCEN-1.1.1 - baseline lock 记录 promptfoo 0.121.13、commit 4860e99、npm artifact、container artifact 与采集命令
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.1 behavior
    Then TEST-1.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-1.1-baseline-lock.md
  Scenario: SCEN-1.1.2 - 缺失任一 artifact 校验时 release gate 标为 blocked
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.1 behavior
    Then TEST-1.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-1.1-baseline-lock.md
  Scenario: SCEN-1.1.3 - baseline 文件禁止引用 latest 或浮动 tag
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.1 behavior
    Then TEST-1.1.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-1.2-compatibility-matrix.md
  Scenario: SCEN-1.2.1 - 矩阵覆盖 CLI、config、provider、assertion、redteam、MCP、scan、output、Node API、cloud/share 边界
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.2 behavior
    Then TEST-1.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-1.2-compatibility-matrix.md
  Scenario: SCEN-1.2.2 - 每项能力都有 P0/P1/P2、native/bridge/unsupported/later、验证方式和 owner 字段
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.2 behavior
    Then TEST-1.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-1.2-compatibility-matrix.md
  Scenario: SCEN-1.2.3 - P2 known gap 不允许空 reason
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 1.2 behavior
    Then TEST-1.2.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.1-upstream-harness-runner.md
  Scenario: SCEN-6.1.1 - harness 固定 baseline artifact 并拒绝 latest
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.1 behavior
    Then TEST-6.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.1-upstream-harness-runner.md
  Scenario: SCEN-6.1.2 - 同一 fixture 能生成 upstream artifact 与 rs artifact
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.1 behavior
    Then TEST-6.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.1-upstream-harness-runner.md
  Scenario: SCEN-6.1.3 - 时间、路径、随机 ID、latency 归一化规则有 snapshot
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.1 behavior
    Then TEST-6.1.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.2-golden-diff-release-gate.md
  Scenario: SCEN-6.2.1 - diff 分类包含 matching、intentional-difference、unsupported、later、upstream-ambiguous、bug
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.2 behavior
    Then TEST-6.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.2-golden-diff-release-gate.md
  Scenario: SCEN-6.2.2 - P0 bug/未分类差异阻断 stable release
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.2 behavior
    Then TEST-6.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-6.2-golden-diff-release-gate.md
  Scenario: SCEN-6.2.3 - P1 snapshot 和 P2 登记完整性进入 gate summary
    Given a promptfoo 0.121.13 compatibility fixture for compatibility
    When promptfoo-rs executes task 6.2 behavior
    Then TEST-6.2.3 records the expected observable result
