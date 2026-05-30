# language: en
# Maps to:
#   - docs/specs/tasks/task-10.2-release-docs-packaging.md

Feature: release
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want release behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-10.2-release-docs-packaging.md
  Scenario: SCEN-10.2.1 - release checklist 包含 compatibility gate 证据
    Given a promptfoo 0.121.13 compatibility fixture for release
    When promptfoo-rs executes task 10.2 behavior
    Then TEST-10.2.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-10.2-release-docs-packaging.md
  Scenario: SCEN-10.2.2 - README、架构文档、兼容矩阵、贡献指南齐全
    Given a promptfoo 0.121.13 compatibility fixture for release
    When promptfoo-rs executes task 10.2 behavior
    Then TEST-10.2.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-10.2-release-docs-packaging.md
  Scenario: SCEN-10.2.3 - 稳定版发布失败时只能发 prerelease 或 nightly
    Given a promptfoo 0.121.13 compatibility fixture for release
    When promptfoo-rs executes task 10.2 behavior
    Then TEST-10.2.3 records the expected observable result
