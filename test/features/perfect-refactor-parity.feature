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
#   - docs/specs/tasks/task-19.1-viewer-config-source-reclassification.md
#   - docs/specs/tasks/task-19.2-core-config-source-fixture-burndown.md
#   - docs/specs/tasks/task-19.3-provider-request-response-fixture-burndown.md
#   - docs/specs/tasks/task-19.4-external-authority-blocker-waiver-gate.md
#   - docs/specs/tasks/task-20.1-source-provider-accounting-reconciliation.md
#   - docs/specs/tasks/task-20.2-perfect-refactor-claim-contract.md
#   - docs/specs/tasks/task-21.1-upstream-distribution-target-gate.md
#   - docs/specs/tasks/task-22.1-authority-unblock-packet-gate.md
#   - docs/specs/tasks/task-23.1-dynamic-github-latest-release-observation.md
#   - docs/specs/tasks/task-24.1-current-latest-upstream-authority-lock.md
#   - docs/specs/tasks/task-24.2-current-latest-source-inventory-reextract.md
#   - docs/specs/tasks/task-24.3-current-latest-full-function-golden-corpus.md
#   - docs/specs/tasks/task-24.4-current-latest-exhaustive-quality-gate.md
#   - docs/specs/tasks/task-25.1-current-latest-source-taxonomy-burndown.md
#   - docs/specs/tasks/task-26.1-current-latest-viewer-config-reclassification.md
#   - docs/specs/tasks/task-27.1-current-latest-core-config-burndown.md
#   - docs/specs/tasks/task-28.1-current-latest-provider-fixture-burndown.md
#   - docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
#   - docs/specs/tasks/task-30.1-current-latest-prompt-processing-burndown.md
#   - docs/specs/tasks/task-31.1-current-latest-cache-store-burndown.md
#   - docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md
#   - docs/specs/tasks/task-33.1-current-latest-eval-deletion-burndown.md
#   - docs/specs/tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md

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

  Scenario: SCEN-22.1.1 - perfect refactor unblock packet lists minimum external decisions
    Given source accounting external authority publication authority upstream distribution and perfect refactor claim artifacts are present
    When perfect refactor unblock packet evidence is generated
    Then TEST-22.1.1 proves perfect_refactor_claim_allowed remains false while blockers remain
    And TEST-22.1.2 proves provider blockers represented by external authority are not duplicated as source-only decisions
    And TEST-22.1.5 proves runtime smoke and docs expose the packet as a blocker handoff artifact

  Scenario: SCEN-23.1.1 - upstream latest release observation is dynamic
    Given GitHub latest release metadata can change independently from the frozen npm core package
    When upstream distribution target evidence is generated
    Then TEST-23.1.1 proves the observed release ref comes from latest release metadata instead of a hard-coded tag
    And TEST-23.1.2 proves release-channel classification still keeps current perfect refactor blocked when the dynamic latest release is not the npm core package

  Scenario: SCEN-24.1.1 - current latest target lock is immutable
    Given the user requires refactoring against the original promptfoo current latest complete functionality
    When current latest target evidence is generated
    Then TEST-24.1.1 proves npm latest GitHub HEAD and GitHub latest release channel are recorded with full SHAs
    And TEST-24.1.2 proves floating latest main master and HEAD strings are rejected as completion proof

  Scenario: SCEN-24.2.1 - current latest source inventory has no silent omissions
    Given a locked current latest target packet exists
    When current latest source inventory is re-extracted
    Then TEST-24.2.2 proves every command flag provider assertion redteam output config viewer Node API example and docs row has a stable source reference
    And TEST-24.2.4 proves unclassified rows keep perfect refactor blocked

  Scenario: SCEN-24.3.1 - current latest full function golden corpus is large and complete
    Given current latest source inventory rows have P0 P1 or P2 levels
    When current latest golden corpus verification runs
    Then TEST-24.3.1 proves every P0 row has executable upstream and rs artifacts
    And TEST-24.3.4 proves the corpus has at least 250 fixture cases or full inventory coverage when fewer rows exist

  Scenario: SCEN-24.4.1 - exhaustive quality gate limits bug claims to evidence
    Given current latest target inventory and golden corpus artifacts exist
    When current latest quality gate runs
    Then TEST-24.4.1 proves adapter golden corpus source coverage regression stress property runtime smoke and release blockers are aggregated
    And TEST-24.4.2 proves wording that claims no possible bugs is rejected
    And TEST-24.4.3 proves the perfect refactor claim remains false while any current target external publication or corpus evidence is missing
    And TEST-24.4.4 proves local readiness can pass while public perfect-refactor completion remains blocked by external or publication authority

  Scenario: SCEN-25.1.1 - current latest source taxonomy has no unknown rows
    Given Phase 24 quality evidence reports current-latest source inventory and matrix unclassified rows
    When the current-latest source taxonomy burndown runs against the locked target packet
    Then TEST-25.1.1 proves representative previously unknown source families become classified capability rows
    And TEST-25.1.2 proves P0 P1 and P2 levels keep their required evidence semantics
    And TEST-25.1.3 proves source inventory and matrix artifacts have no unclassified rows
    And TEST-25.1.4 proves remaining perfect-refactor blockers stay explicit instead of being hidden by taxonomy cleanup

  Scenario: SCEN-26.1.1 - current latest viewer config rows are not duplicate P0 config blockers
    Given Phase 25 current-latest artifacts classify every source row
    When the current-latest viewer config reclassification runs
    Then TEST-26.1.1 proves src/app config-named rows remain viewer evidence without duplicate config blockers
    And TEST-26.1.2 proves non-app config rows remain P0 config fixture or blocker rows
    And TEST-26.1.3 proves source matrix golden and quality artifacts keep complete row accounting
    And TEST-26.1.4 proves perfect-refactor completion remains false while real blockers remain

  Scenario: SCEN-27.1.1 - current latest core config blockers are specific decisions
    Given Phase 26 current-latest artifacts have 18 non-app config blockers
    When the current-latest core config burndown runs
    Then TEST-27.1.1 proves runtime and redteam config rows have fixture evidence
    And TEST-27.1.2 proves auxiliary code scan and MCP config rows are P1 snapshot evidence
    And TEST-27.1.3 proves cloud server telemetry and global config rows remain explicit external blockers
    And TEST-27.1.4 proves config golden blockers drop to external-only rows while perfect-refactor completion remains false

  Scenario: SCEN-28.1.1 - current latest provider blockers split fixture evidence from external authority
    Given Phase 27 current-latest artifacts have 38 provider blockers
    When the current-latest provider fixture burndown runs
    Then TEST-28.1.1 proves fixture-covered provider rows have P0 native fixture evidence
    And TEST-28.1.2 proves product credential account and private-service provider rows remain explicit external blockers
    And TEST-28.1.3 proves Rust and shell extractors emit equivalent provider evidence
    And TEST-28.1.4 proves provider golden blockers drop to external-only rows while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-29.1-current-latest-eval-runner-burndown.md
  Scenario: SCEN-29.1.1 - current latest eval runner blockers split fixtures snapshots and real blockers
    Given Phase 28 current-latest artifacts have 18 eval-runner blockers
    When the current-latest eval-runner burndown runs
    Then TEST-29.1.1 proves fixture-covered eval-runner rows have P0 native fixture evidence
    And TEST-29.1.2 proves optimizer event and synthesis rows are P1 snapshot evidence
    And TEST-29.1.3 proves adaptive rate-limit and provider-wrapper rows remain P0 blockers
    And TEST-29.1.4 proves Rust and shell extractors emit equivalent eval-runner evidence
    And TEST-29.1.5 proves eval-runner golden blockers drop to remaining blocker rows while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-30.1-current-latest-prompt-processing-burndown.md
  Scenario: SCEN-30.1.1 - current latest prompt processing blockers split fixtures snapshots and real blockers
    Given Phase 29 current-latest artifacts have 13 prompt-processing blockers
    When the current-latest prompt-processing burndown runs
    Then TEST-30.1.1 proves fixture-covered prompt-processing rows have P0 native fixture evidence
    And TEST-30.1.2 proves constants grading and Ragas prompt rows are P1 snapshot evidence
    And TEST-30.1.3 proves JSON Markdown Jinja JavaScript Python and executable processor rows remain P0 blockers
    And TEST-30.1.4 proves Rust and shell extractors emit equivalent prompt-processing evidence
    And TEST-30.1.5 proves prompt-processing golden blockers drop to remaining blocker rows while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-31.1-current-latest-cache-store-burndown.md
  Scenario: SCEN-31.1.1 - current latest cache store blockers split fixtures snapshots and real blockers
    Given Phase 30 current-latest artifacts have 9 cache-store blockers
    When the current-latest cache-store burndown runs
    Then TEST-31.1.1 proves fixture-covered cache-store rows have P0 native fixture evidence
    And TEST-31.1.2 proves database testing and signal helper rows are P1 snapshot evidence
    And TEST-31.1.3 proves eval deletion remains a P0 blocker
    And TEST-31.1.4 proves Rust and shell extractors emit equivalent cache-store evidence
    And TEST-31.1.5 proves cache-store golden blockers drop to remaining blocker rows while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-32.1-current-latest-local-prompt-processor-burndown.md
  Scenario: SCEN-32.1.1 - current latest local prompt processors split parser fixtures from script blockers
    Given Phase 31 current-latest artifacts have 6 prompt-processing blockers
    When the current-latest local prompt processor burndown runs
    Then TEST-32.1.1 proves JSON Markdown and Jinja processor rows have P0 native fixture evidence
    And TEST-32.1.2 proves JavaScript Python and executable processor rows remain P0 script-bridge blockers
    And TEST-32.1.3 proves Rust and shell extractors emit equivalent local prompt processor evidence
    And TEST-32.1.4 proves prompt-processing golden blockers drop to remaining script-backed blocker rows while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-33.1-current-latest-eval-deletion-burndown.md
  Scenario: SCEN-33.1.1 - current latest eval deletion removes only selected local eval records
    Given Phase 32 current-latest artifacts have 1 cache-store eval deletion blocker
    When the current-latest eval deletion burndown runs
    Then TEST-33.1.1 proves eval deletion removes selected SQLite result rows and assertion rows
    And TEST-33.1.2 proves missing eval deletion is a non-destructive no-op
    And TEST-33.1.3 proves the current-latest eval deletion row has P0 native fixture evidence and no cache-store blocker remains
    And TEST-33.1.4 proves Rust and shell extractors emit equivalent eval deletion evidence and total blockers drop to 40 while perfect-refactor completion remains false

  # Maps to: docs/specs/tasks/task-34.1-current-latest-eval-scheduler-rate-limit-burndown.md
  Scenario: SCEN-34.1.1 - current latest eval scheduler rate-limit rows have local deterministic evidence
    Given Phase 33 current-latest artifacts have 7 eval-runner scheduler rate-limit blockers
    When the current-latest eval scheduler rate-limit burndown runs
    Then TEST-34.1.1 proves provider rate-limit header parsing and key derivation are deterministic
    And TEST-34.1.2 proves provider rate-limit registry records headers and returns deterministic delay decisions
    And TEST-34.1.3 proves adaptive concurrency responds within configured bounds
    And TEST-34.1.4 proves provider call execution context and wrapper expose stable local metadata and header records
    And TEST-34.1.5 proves Rust and shell extractors emit equivalent eval scheduler evidence and total blockers drop to 33 while perfect-refactor completion remains false
