# language: en
# Maps to:
#   - docs/specs/tasks/task-11.1-current-upstream-target-policy.md
#   - docs/specs/tasks/task-11.2-item-level-capability-inventory.md
#   - docs/specs/tasks/task-11.3-compatibility-matrix-expansion.md
#   - docs/specs/tasks/task-12.1-p0-fixture-corpus.md
#   - docs/specs/tasks/task-12.2-executable-upstream-rs-runner.md
#   - docs/specs/tasks/task-12.3-golden-diff-ci-release-gate.md
#   - docs/specs/tasks/task-13.1-command-flag-parity.md
#   - docs/specs/tasks/task-13.2-eval-output-cache-parity.md
#   - docs/specs/tasks/task-14.1-provider-assertion-inventory-parity.md
#   - docs/specs/tasks/task-14.2-redteam-plugin-strategy-parity.md
#   - docs/specs/tasks/task-15.1-viewer-node-packaging-release.md
#   - docs/specs/tasks/task-15.2-performance-security-observability-gates.md
#   - docs/specs/tasks/task-16.1-cli-command-behavior-closure.md
#   - docs/specs/tasks/task-16.2-measured-release-gate-reports.md
#   - docs/specs/tasks/task-16.3-source-extracted-inventory-real-upstream-smoke.md
#   - docs/specs/tasks/task-17.1-frozen-source-inventory-extractor.md
#   - docs/specs/tasks/task-17.2-cli-global-eval-redteam-parity.md
#   - docs/specs/tasks/task-17.3-real-p0-golden-corpus-runner.md
#   - docs/specs/tasks/task-17.4-longtail-provider-assertion-redteam-classification.md
#   - docs/specs/tasks/task-17.5-release-installability-publication-readiness.md
#   - docs/specs/tasks/task-18.1-source-inventory-ledger-closure.md
#   - docs/specs/tasks/task-18.2-p0-provider-module-fixture-burndown.md
#   - docs/specs/tasks/task-18.3-current-upstream-rebaseline-gate.md
#   - docs/specs/tasks/task-18.4-publication-authority-release-gate.md

Feature: perfect refactor parity
  In order to make promptfoo-rs a complete audited refactor of promptfoo
  As a promptfoo-rs maintainer
  I want every audited parity gap to map to executable S2V tasks and verification evidence

  Scenario: SCEN-11.1.1 - compatibility target policy separates frozen baseline from moving upstream
    Given the promptfoo-rs audit package records both baseline and current upstream evidence
    When the compatibility target policy is generated
    Then TEST-11.1.1 proves stable releases reference exactly one compatibility target

  Scenario: SCEN-11.2.1 - upstream item-level inventory covers commands providers assertions redteam outputs configs and APIs
    Given upstream promptfoo source docs and examples are available
    When the inventory extractor runs
    Then TEST-11.2.1 proves every discovered item has a stable inventory id and source reference

  Scenario: SCEN-11.3.1 - expanded compatibility matrix has no silent omissions
    Given the item-level inventory exists
    When the matrix expansion check runs
    Then TEST-11.3.1 proves every inventory item has P0 P1 or P2 status and verification owner

  Scenario: SCEN-12.1.1 - P0 fixture corpus contains at least 50 tracked fixtures
    Given the expanded P0 matrix rows exist
    When the fixture corpus count check runs
    Then TEST-12.1.1 proves at least 50 P0 fixtures are tracked with metadata

  Scenario: SCEN-12.2.1 - executable runner creates upstream rs normalized and diff artifacts
    Given a tracked compatibility fixture
    When the executable golden diff runner runs
    Then TEST-12.2.1 proves upstream promptfoo and promptfoo-rs artifacts are persisted and normalized

  Scenario: SCEN-12.3.1 - CI release gate blocks stable on P0 bug or unclassified diff
    Given persisted golden diff artifacts
    When the release gate evaluates P0 findings
    Then TEST-12.3.1 proves stable release is blocked for bug or unclassified diffs

  Scenario: SCEN-13.1.1 - CLI commands and flags are either compatible or explicitly classified
    Given the upstream command and flag inventory
    When promptfoo-rs exposes CLI help and command behavior
    Then TEST-13.1.1 proves every upstream command path maps to compatible unsupported later or blocked evidence

  Scenario: SCEN-13.2.1 - eval output cache resume and retry behavior is golden diffed
    Given P0 eval fixtures with output and cache settings
    When promptfoo-rs runs eval compatibility fixtures
    Then TEST-13.2.1 proves stdout stderr exit code output files and cache state match or are classified

  Scenario: SCEN-14.1.1 - provider and assertion inventory parity has no unclassified P0 item
    Given provider and assertion inventory rows
    When provider and assertion parity checks run
    Then TEST-14.1.1 proves every P0 item has native bridge or blocking evidence

  Scenario: SCEN-14.2.1 - redteam plugin and strategy inventory parity has no missing reason
    Given redteam plugin and strategy inventory rows
    When redteam parity checks run
    Then TEST-14.2.1 proves P0 rows have fixtures and P2 rows have reasons

  Scenario: SCEN-15.1.1 - viewer and node wrapper packages build and smoke test
    Given viewer and npm package metadata exists
    When release packaging verification runs
    Then TEST-15.1.1 proves viewer browser smoke and node wrapper smoke pass

  Scenario: SCEN-15.2.1 - release candidate gates cover performance security and runtime smoke
    Given a release candidate build
    When full release verification runs
    Then TEST-15.2.1 proves lint integration e2e coverage runtime smoke performance and security gates are enforced

  Scenario: SCEN-16.1.1 - CLI later commands become executable local compatibility behavior
    Given local result and cache artifacts exist
    When view cache import and export commands run
    Then TEST-16.1.1 proves those commands return stable local JSON behavior instead of later placeholders

  Scenario: SCEN-16.2.1 - release reports are measured or derived from this runtime smoke
    Given runtime smoke executes release candidate checks
    When performance security and release candidate reports are written
    Then TEST-16.2.1 proves stable decisions come from measured gate evidence rather than fixed JSON literals

  Scenario: SCEN-16.3.1 - real upstream smoke and source inventory evidence back the matrix
    Given frozen promptfoo upstream artifacts are reachable
    When source extraction and real upstream smoke run
    Then TEST-16.3.1 proves matrix and golden artifacts are based on upstream promptfoo 0.121.13 evidence

  Scenario: SCEN-17.1.1 - frozen upstream source inventory is extracted without silent omissions
    Given the frozen promptfoo 0.121.13 tag and npm package are the stable compatibility target
    When the source inventory extractor runs
    Then TEST-17.1.1 proves every extracted command provider assertion redteam output config viewer API and example item is recorded or release-blocking

  Scenario: SCEN-17.2.1 - CLI global eval and redteam surfaces match or classify upstream help
    Given upstream promptfoo help snapshots for top-level eval and redteam commands
    When promptfoo-rs parses help commands and representative invocations
    Then TEST-17.2.1 proves each user-visible command or flag is implemented unsupported later or blocked with stable evidence

  Scenario: SCEN-17.3.1 - real P0 corpus executes upstream and rs for at least 50 fixtures
    Given at least 50 P0 fixtures with mock or recorded providers
    When the real golden corpus runner executes the full release gate
    Then TEST-17.3.1 proves raw normalized diff and metadata artifacts exist for each upstream and rs run

  Scenario: SCEN-17.4.1 - long-tail provider assertion and redteam rows are classified
    Given source-extracted provider assertion and redteam inventory rows
    When long-tail parity classification runs
    Then TEST-17.4.1 proves no P0 row lacks fixture or blocker and no P2 later unsupported row lacks reason

  Scenario: SCEN-17.5.1 - release installability evidence separates dry-run readiness from public publication
    Given a release candidate that passed full compatibility gates
    When release installability verification runs
    Then TEST-17.5.1 proves local archives packages checksums and install smoke evidence are present while missing external credentials remain explicit blockers

  Scenario: SCEN-18.1.1 - source inventory ledger closes silent missing matrix rows
    Given source-extracted promptfoo items include command provider assertion redteam output config viewer and example rows
    When the source inventory ledger is generated
    Then TEST-18.1.1 proves every source item has accounting evidence and TEST-18.1.2 proves generated P0 rows remain release-blocking

  Scenario: SCEN-18.2.1 - P0 provider module blockers burn down through fixtures or explicit blockers
    Given the long-tail classification report lists P0 provider module blockers
    When provider module burndown verification runs
    Then TEST-18.2.1 proves each P0 provider module has fixture evidence or an explicit external blocker

  Scenario: SCEN-18.3.1 - current upstream rebaseline gate prevents ambiguous perfect claims
    Given frozen promptfoo baseline and observed current upstream HEAD differ
    When current upstream target policy is evaluated
    Then TEST-18.3.2 proves frozen mode cannot claim current-upstream perfect refactor

  Scenario: SCEN-18.4.1 - publication authority gate separates installability from published availability
    Given local release artifacts are installable but external credentials are absent
    When publication authority verification runs
    Then TEST-18.4.2 proves dry-run artifacts cannot set published=true without external evidence

  Scenario: SCEN-19.1.1 - viewer config source rows are reclassified without weakening core config
    Given source accounting ledger includes generated config rows from src/app and non-app runtime config files
    When viewer config source reclassification runs
    Then TEST-19.1.1 proves src/app config rows become P1 viewer evidence and TEST-19.1.2 proves non-app config rows remain P0 blockers

  Scenario: SCEN-19.2.1 - core config source blockers burn down through fixtures or explicit blockers
    Given non-app config rows remain after viewer config reclassification
    When core config source burndown verification runs
    Then TEST-19.2.1 proves runtime config rows have fixture evidence and TEST-19.2.3 proves unresolved config rows have specific blockers

  Scenario: SCEN-19.3.1 - provider module request response blockers burn down through dedicated fixtures
    Given remaining provider module blockers include mockable request response modules and external authority modules
    When provider request response fixture burndown runs
    Then TEST-19.3.1 proves non-external provider modules have dedicated fixtures or stay release-blocking with item-level reasons

  Scenario: SCEN-19.4.1 - external authority blockers remain explicit and unforgeable
    Given remaining blockers require credentials accounts private services legal brand or publication authority
    When external authority blocker gate runs
    Then TEST-19.4.1 proves every external blocker has a required decision and TEST-19.4.4 proves perfect refactor is not claimed while they remain

  Scenario: SCEN-20.1.1 - source provider accounting consumes provider burndown evidence
    Given source accounting includes provider rows and provider burndown classifies fixture covered versus external authority rows
    When source provider accounting reconciliation runs
    Then TEST-20.1.1 proves fixture covered provider rows leave remaining P0 blockers and TEST-20.1.3 proves only config external and provider external blockers remain

  Scenario: SCEN-20.2.1 - perfect refactor claim is separate from local stable release
    Given local stable release gates can pass while source current publication or external authority blockers remain
    When perfect refactor claim contract is generated
    Then TEST-20.2.1 proves perfect_refactor_claim_allowed is false and TEST-20.2.3 proves every blocker source is listed

  Scenario: SCEN-21.1.1 - upstream distribution target separates npm core from repository drift
    Given the latest npm promptfoo core package and GitHub repository/release observations can differ
    When upstream distribution target evidence is generated
    Then TEST-21.1.1 proves npm core package alignment is recorded and TEST-21.1.3 proves non-core GitHub release drift cannot imply current perfect refactor readiness
