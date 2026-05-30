# language: en
# Maps to:
#   - docs/specs/tasks/task-10.1-web-viewer.md

Feature: web-viewer
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want web-viewer behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-10.1-web-viewer.md
  Scenario: SCEN-10.1.1 - viewer 能读取稳定 result schema
    Given a promptfoo 0.121.13 compatibility fixture for web-viewer
    When promptfoo-rs executes task 10.1 behavior
    Then TEST-10.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-10.1-web-viewer.md
  Scenario: SCEN-10.1.2 - eval table 支持 provider/test/assertion/filter 基础视图
    Given a promptfoo 0.121.13 compatibility fixture for web-viewer
    When promptfoo-rs executes task 10.1 behavior
    Then TEST-10.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-10.1-web-viewer.md
  Scenario: SCEN-10.1.3 - viewer 不依赖 upstream UI 像素级复刻
    Given a promptfoo 0.121.13 compatibility fixture for web-viewer
    When promptfoo-rs executes task 10.1 behavior
    Then TEST-10.1.3 records the expected observable result
