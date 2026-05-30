# language: en
# Maps to:
#   - docs/specs/tasks/task-8.1-mcp-runtime.md

Feature: mcp-runtime
  In order to migrate existing promptfoo workflows to promptfoo-rs
  As a promptfoo-rs maintainer
  I want mcp-runtime behavior to be specified, tested, and traceable to S2V task specs

  # Maps to: docs/specs/tasks/task-8.1-mcp-runtime.md
  Scenario: SCEN-8.1.1 - promptfoo mcp command skeleton 可运行
    Given a promptfoo 0.121.13 compatibility fixture for mcp-runtime
    When promptfoo-rs executes task 8.1 behavior
    Then TEST-8.1.1 records the expected observable result

  # Maps to: docs/specs/tasks/task-8.1-mcp-runtime.md
  Scenario: SCEN-8.1.2 - MCP provider request/response 有 protocol snapshot
    Given a promptfoo 0.121.13 compatibility fixture for mcp-runtime
    When promptfoo-rs executes task 8.1 behavior
    Then TEST-8.1.2 records the expected observable result

  # Maps to: docs/specs/tasks/task-8.1-mcp-runtime.md
  Scenario: SCEN-8.1.3 - MCP target materialization 错误路径稳定
    Given a promptfoo 0.121.13 compatibility fixture for mcp-runtime
    When promptfoo-rs executes task 8.1 behavior
    Then TEST-8.1.3 records the expected observable result
