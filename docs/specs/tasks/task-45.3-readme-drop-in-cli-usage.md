# Task 45.3: readme-drop-in-cli-usage

**Status**: Done
**Priority**: P1
**Owner**: leafiellune
**Related Phase**: Phase 45 - promptfoo-drop-in-cli-entrypoints
**Dependencies**: task-45.1-rust-promptfoo-binary-alias, task-45.2-npm-promptfoo-bin-shims, task-10.2-release-docs-packaging

## 1. Background

After Phase 45 adds `promptfoo` entrypoints, the open-source docs must stop presenting `promptfoo-rs` as the only command and must align with upstream-style `promptfoo eval/view` usage. The docs also need to preserve the independent reimplementation and publication authority boundaries. 依据 upstream README, README/Quickstart docs added in commit `5d483b1`, PRD §Release constraints, docs/release.md, task 10.2, task 45.1, and task 45.2.

## 2. Goal

Update README, Quickstart, project overview, release docs, and compatibility wording so users see `promptfoo` as the drop-in local command, `promptfoo-rs` as an explicit alias, and no unsupported public release or perfect-refactor claims.

## 3. Scope

### In Scope

- `README.md`
- `README.en.md`
- `docs/QUICKSTART.md`
- `docs/QUICKSTART.en.md`
- `docs/PROJECT-OVERVIEW.md`
- `docs/release.md`
- `NOTICE` if command-name wording needs clarification.
- BDD traceability in `test/features/cli.feature`.

### Out Of Scope

- Real publication instructions that require credentials.
- Claiming `npm install -g promptfoo-rs` or `npm install -g promptfoo` works before publication evidence exists.
- Legal/brand approval beyond neutral independent-reimplementation wording.
- Changing compatibility matrix status without corresponding release-gate evidence.

## 4. Users / Actors

- New open-source user: wants to copy a command and run a local eval.
- Existing promptfoo user: wants to understand whether `promptfoo` means upstream or local binary.
- Contributor: needs docs that match tested entrypoints and release boundaries.

## 5. Behavior Contract

Docs must prefer `promptfoo` for installed/local drop-in command examples once tasks 45.1 and 45.2 are implemented. Docs must also mention `promptfoo-rs` remains available. Any npm/Homebrew/Cargo/GitHub Release instructions must remain conditional or dry-run-oriented until publication authority evidence exists. No docs may claim complete current-latest replacement, bug-free behavior, upstream endorsement, or public stable publication without release-gate support.

### 5.1 Required Reading

- README.md
- README.en.md
- docs/QUICKSTART.md
- docs/QUICKSTART.en.md
- docs/PROJECT-OVERVIEW.md
- docs/release.md
- NOTICE
- docs/specs/tasks/task-10.2-release-docs-packaging.md
- docs/specs/tasks/task-45.1-rust-promptfoo-binary-alias.md
- docs/specs/tasks/task-45.2-npm-promptfoo-bin-shims.md
- docs/decisions/adr-008-binary-first-multi-channel-release.md
- docs/decisions/adr-011-current-latest-full-refactor-target.md

### 5.2 Imports

- Documentation files listed in §3.
- Verification artifacts: `target/release-gates/current-latest-quality.json`, `target/release-gates/release-candidate.json`, `target/release-gates/publication-authority.json`.
- Shell/tooling commands: adapter §Commands Install / Lint / Typecheck / Unit Test / Integration tests / E2E tests / Coverage / Build / Runtime smoke; markdown local link checker.

### 5.3 函数签名

- Documentation contract: `assert_docs_prefer_promptfoo_command(paths: &[PathBuf]) -> DocCommandReport`
- Documentation contract: `assert_no_forbidden_release_claims(paths: &[PathBuf]) -> DocClaimReport`
- Link-check contract: `assert_local_markdown_links(paths: &[PathBuf]) -> Result<(), MissingLinkReport>`

## 6. Acceptance Criteria

- [x] **AC1** (upstream README): README and Quickstart show `promptfoo eval`, `promptfoo view`, and `promptfoo --help` as the preferred local command after build/install.
- [x] **AC2** (task 45.1 / 45.2): docs mention `promptfoo-rs` and `pf` only where supported by tested Rust/npm aliases.
- [x] **AC3** (ADR-008 / docs/release.md): install/publish sections distinguish local build/package smoke from real public registry publication.
- [x] **AC4** (ADR-011): docs contain no forbidden perfect-refactor, public stable, upstream endorsement, or bug-free claims.

## 7. SDD / BDD / TDD Traceability

| Acceptance Criterion | BDD Scenario | TDD Test | Integration / E2E Test | Verification | Status |
|---|---|---|---|---|---|
| AC1 | SCEN-45.3.1 | TEST-45.3.1 | `tests/drop_in_cli_docs.rs` docs command checker | install, lint, typecheck, unit-test, build | Done |
| AC2 | SCEN-45.3.1 | TEST-45.3.2 | `tests/drop_in_cli_docs.rs` alias checker | install, lint, typecheck, unit-test, build | Done |
| AC3 | SCEN-45.3.1 | TEST-45.3.3 | `tests/drop_in_cli_docs.rs` publication-boundary checker | install, typecheck, unit-test, runtime-smoke, build | Done |
| AC4 | SCEN-45.3.1 | TEST-45.3.4 | `tests/drop_in_cli_docs.rs` forbidden-claim checker + current-latest quality wording policy | install, lint, typecheck, unit-test, coverage, build | Done |

## 8. Risks

- Docs can accidentally tell users to install from a channel that is not yet published.
- Using `promptfoo` in docs without independent-reimplementation wording can create brand confusion.
- If implementation tasks change alias names, docs must follow tested behavior rather than intent.

## 9. Verification Plan

- **Install**: adapter §Commands Install
- **Lint**: adapter §Commands Lint
- **Typecheck**: adapter §Commands Typecheck
- **Unit Test**: adapter §Commands Unit Test
- **Integration tests**: adapter §Commands Integration tests
- **E2E tests**: adapter §Commands E2E tests
- **Coverage**: adapter §Commands Coverage
- **Build**: adapter §Commands Build
- **Runtime smoke**: adapter §Commands Runtime smoke

## 10. Completion Notes

- **完成日期**：2026-06-04
- **改动文件**：
  - README.md
  - README.en.md
  - docs/QUICKSTART.md
  - docs/QUICKSTART.en.md
  - docs/PROJECT-OVERVIEW.md
  - docs/release.md
  - tests/drop_in_cli_docs.rs
  - docs/specs/tasks/task-45.3-readme-drop-in-cli-usage.md
  - docs/s2v-adapter.md
  - docs/specs/phases/phase-45-promptfoo-drop-in-cli-entrypoints.md
- **commit 列表**：
  - 318a0c0 test(docs): add task-45.3 drop-in CLI docs RED tests
  - 6a64389 docs(readme): document promptfoo drop-in CLI usage
  - 本 docs(spec) 回填提交见 git log：docs(spec): 回填 task-45.3 §10 Completion Notes + Status → Done
- **§9 Verification 结果**：
  - install: 通过；`s2v_verify_full` 执行 adapter Install，cargo fetch、viewer pnpm install、npm pnpm install 均成功。
  - lint: 通过；`bash scripts/release/lint.sh` 成功。
  - typecheck: 通过；`cargo check --workspace`、viewer typecheck、npm typecheck 均成功。
  - unit-test: 通过；`cargo test --workspace`、viewer test、npm test 均成功，包含 TEST-45.3.1 ~ TEST-45.3.4。
  - integration: 通过；`bash scripts/release/integration.sh` 成功。
  - e2e: 通过；`bash scripts/release/e2e.sh` 成功。
  - coverage: 通过；`bash scripts/release/coverage.sh` 成功，`s2v_coverage_threshold_guard` 通过。
  - build: 通过；adapter Build 成功，Rust release 与 viewer/npm build 继续通过。
  - runtime-smoke: 通过；`bash scripts/release/runtime-smoke.sh` 成功。
- **剩余风险 / 未做项**：真实 npm/GitHub Releases/Homebrew/Docker public publication 仍未执行；文档仅声明已测试的本地 `promptfoo` / `promptfoo-rs` / `pf` 入口和 `local build/package smoke`，不声明 public registry publication 或 perfect-refactor completion。
- **下游 task 影响**：Phase 45 可进入 phase smoke 收尾；Phase 43/44 的真实凭据、法律/品牌、外部 URL/digest 权限任务仍保持独立阻塞边界。
