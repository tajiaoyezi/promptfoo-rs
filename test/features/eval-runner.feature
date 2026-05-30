# language: en
# Maps to:
#   - docs/specs/tasks/task-2.3-eval-command-smoke.md
#   - docs/specs/tasks/task-3.1-scheduler-runtime.md

Feature: eval-runner
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want eval-runner behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-2.3-eval-command-smoke.md
  Scenario: SCEN-2.3.1 - eval -c promptfooconfig.yaml 能完成空/最小 eval smoke
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 2.3 behavior
    Then TEST-2.3.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.3-eval-command-smoke.md
  Scenario: SCEN-2.3.2 - runner 输出结构化 result envelope
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 2.3 behavior
    Then TEST-2.3.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-2.3-eval-command-smoke.md
  Scenario: SCEN-2.3.3 - 失败配置返回可定位错误和非零 exit code
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 2.3 behavior
    Then TEST-2.3.3 records the expected observable result

  # Maps to: docs/specs/tasks/task-3.1-scheduler-runtime.md
  Scenario: SCEN-3.1.1 - max-concurrency 能限制并发 provider 调用
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 3.1 behavior
    Then TEST-3.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-3.1-scheduler-runtime.md
  Scenario: SCEN-3.1.2 - delay 和 cancellation 有 deterministic tests
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 3.1 behavior
    Then TEST-3.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-3.1-scheduler-runtime.md
  Scenario: SCEN-3.1.3 - partial failure 保留已完成结果并继续按配置收敛
    Given a promptfoo 0.121.13 compatibility fixture for eval-runner
    When promptfoo-rs executes task 3.1 behavior
    Then TEST-3.1.3 records the expected observable result
