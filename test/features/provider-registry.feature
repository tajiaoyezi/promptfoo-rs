# language: en
# Maps to:
#   - docs/specs/tasks/task-4.1-p0-provider-registry.md

Feature: provider-registry
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want provider-registry behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-4.1-p0-provider-registry.md
  Scenario: SCEN-4.1.1 - 四类 P0 provider 均有 request/response snapshot
    Given a promptfoo 0.121.13 compatibility fixture for provider-registry
    When promptfoo-rs executes task 4.1 behavior
    Then TEST-4.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.1-p0-provider-registry.md
  Scenario: SCEN-4.1.2 - provider-scoped env/header/model/options 被归一化
    Given a promptfoo 0.121.13 compatibility fixture for provider-registry
    When promptfoo-rs executes task 4.1 behavior
    Then TEST-4.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-4.1-p0-provider-registry.md
  Scenario: SCEN-4.1.3 - 网络调用通过 mock server 验证，不依赖真实模型
    Given a promptfoo 0.121.13 compatibility fixture for provider-registry
    When promptfoo-rs executes task 4.1 behavior
    Then TEST-4.1.3 records the expected observable result
