# Release Readiness Review

Date: 2026-05-31

Project: FerrisOxide Signal

Stage: Release Gate

Owner Role: Release Engineer / GitHub Maintainer Specialist

## Current Status

This review records the initial public-repository publication gate. Since publication, later PRs have completed the validated MVP, transform milestones through M14, the M15-M20 MVP-exit implementation, M21-M24 runtime-path follow-up, and M25-M36 comprehensive filter/signal-conditioning suite. The release notes below preserve historical slices; PR #175 records the M15-M36 mainline merge while release tags, crate publication, and public announcement remain separate approval gates.

## M36 Release Readiness Update

- Scope: M25-M36 comprehensive-suite closure through `docs/m36-comprehensive-suite-closure-pipeline-report.md`.
- PR: `https://github.com/kota-wilson/ferrisoxide/pull/175`.
- Merge commit: `f833a02f7bd59eec15119f88984dad10bdcc3725`.
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed; completed 2026-06-02T10:40:12Z.
- Validation: `cargo fmt --check`; focused catalog tests; direct catalog CLI inspection; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`; trailing-whitespace scans; Markdown link scans; stale-reference scans.
- Release decision: Pass for mainline merge through PR #175; Not Applicable for tag publication, crate publication, or public announcement because those release actions were not requested.
- Messaging boundary: may say the desktop sampled-waveform conditioning suite is complete through M36 on `main`, but must not claim live DAQ, runtime loader, HAL/RTOS support, target hardware execution, hardware calibration, hardware qualification, safety certification, regulatory compliance, or airworthiness evidence.
- Gated follow-ups: exact elliptic/Cauer design, efficient polyphase resampling, Hilbert envelope, optimized FFT/performance work, phase-difference estimation, gain/phase matching, advanced acoustic/domain packs, advanced sensor calibration packs, `split_by_event`, package/runtime expansion, and external release operations.

## Scope

Publish the initial public GitHub repository for the MVP Rust waveform analysis tool.

## Evidence Reviewed

| Area | Evidence | Result |
|---|---|---|
| License | `LICENSE`, `decisions/ADR-002-license-assumption.md` | Pass |
| Dependency review | `docs/dependency-review.md`, `Cargo.lock` | Pass |
| Build and tests | `docs/validation-log.md` | Pass |
| User documentation | `README.md`, `docs/usage-mvp.md` | Pass |
| Contributor readiness | `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `.github/` | Pass |
| Scope guardrails | `README.md`, `AGENTS.md`, `project-charter.md` | Pass |

## Release Notes

- Initial Rust workspace with `ferrisoxide-core` and `ferrisoxide-cli`.
- CSV waveform loading with named time/channel mapping.
- TOML config for input, filters, and min/max voltage criteria.
- Moving-average and first-order low-pass filters.
- Text and JSON report output.
- CI, contribution, issue, PR, security, and license files.

## Gate Decision

- Gate: Release Gate.
- Decision: Pass; public repository published.
- Reason: License and publication were approved, dependency review passed, validation is current, open-source metadata exists, and the initial GitHub Actions run passed.
- Residual risk: The MVP should not be presented as production-grade signal-processing or certified validation software.
- Next owner: Release Engineer.

## Publication Result

- Repository: `https://github.com/kota-wilson/ferrisoxide-signal`
- Visibility: Public.
- Default branch: `main`.
- Initial commit: `dab0866`.
- Initial CI run: `26699230596`, passed.
- Follow-up CI maintenance: `actions/checkout` upgraded from v4 to v5 to use the Node 24 runtime.

## M4 Release Update

- PR: `https://github.com/kota-wilson/ferrisoxide-signal/pull/36`
- Merge commit: `a0d381556ff5f5d044f230217b335b73b3b57608`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 28 seconds.
- Issues closed by PR: #27, #28, #29, #30, #31, #32, #33, #34.
- Milestone: `M4: Signal Accuracy and Validation`, closed with 8 closed issues and 0 open issues.

Gate: Release Gate for M4.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release or certification artifact.

## M5 Release Update

- PR: `https://github.com/kota-wilson/ferrisoxide-signal/pull/39`
- Merge commit: `9bc3d53bf416fff7e280abbcc24840c34811918f`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 31 seconds.
- Issue: #38, `M5-001 Add optional SVG waveform plotting with third axis`
- Milestone: `M5: Plotting and Visualization`, closed with 1 closed issue and 0 open issues.
- Scope: Optional desktop SVG plotting only.
- Validation: `cargo fmt`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; 2D/3D CLI smoke plots; `cargo fmt --check`; `git diff --check`; `cargo metadata --format-version 1 --no-deps`; `cargo tree -p ferrisoxide-plot`.
- Dependency evidence: Plotters approved by user and isolated to `ferrisoxide-plot` with SVG backend and line-series features.

Gate: Release Gate for M5.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release, visual-quality certification, hardware validation, or certification artifact.

## M3 RTOS Adapter Release Update

- PR: `https://github.com/kota-wilson/ferrisoxide-signal/pull/41`
- Merge commit: `36e6d20523c14441e493f7fd48d4776e891f894a`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 27 seconds.
- Issues: #17 `M3-RTOS-002 Add ARM64 QEMU embedded demo`; #18 `M3-RTOS-003 Add RTOS adapter abstraction`; #19 `M3-RTOS-004 Add Zephyr feasibility prototype`.
- Milestone: `M3: RTOS / embedded no_std foundation`, closed with 4 closed issues and 0 open issues.
- Scope: no_std adapter boundary, host-checkable QEMU proof slice, and isolated Zephyr feasibility prototype.
- Validation: `cargo fmt`; `cargo test --workspace`; `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo tree -p ferrisoxide-embedded`; `cargo fmt --check`; `git diff --check`.
- Dependency evidence: no new third-party dependencies; `ferrisoxide-embedded` depends only on local `ferrisoxide-signal`.

Gate: Release Gate for M3 RTOS follow-up.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release, ARM64 boot claim, Zephyr SDK validation, hardware validation, RTOS production readiness claim, or certification artifact.

## M6 Measurement Engine Release Update

- PR: `https://github.com/kota-wilson/ferrisoxide-signal/pull/48`
- Merge commit: `559c96151f6f1d9a99d3d399a0e6bd046bfe5f51`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 27 seconds.
- Issue: #43, `M6-001 Extract measurement engine from criteria evaluation`
- Milestone: `v0.4.0: Measurement & Evidence Engine`
- Milestone status: later closed by PR #52 completion evidence with 5 closed issues and 0 open issues.
- Scope: reusable no_std measurement primitives and criteria integration only.
- Validation: `cargo fmt`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo tree -p ferrisoxide-measurements`; `git diff --check`.
- Dependency evidence: no new third-party dependencies; `ferrisoxide-measurements` is a local no-dependency crate.
- Deferred issues: #44, #45, #46, and #47 were later completed under milestone #6.

Gate: Release Gate for M6-001.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release, report schema migration, annotated SVG evidence, hardware validation, or certification artifact.

## M6-003 Report Measurement Schema Release Update

- PR: `https://github.com/kota-wilson/ferrisoxide-signal/pull/50`
- Merge commit: `f7e21695f501890669d591d0d7cbc9b731a541bb`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 31 seconds.
- Issue: #45, `M6-003 Add report measurement schema and golden JSON updates`
- Milestone: `v0.4.0: Measurement & Evidence Engine`
- Milestone status: later closed by PR #52 completion evidence with 5 closed issues and 0 open issues.
- Scope: report measurement records, result `measurement_id` links, exact golden JSON updates, and schema docs.
- Validation: local `cargo fmt`, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, and `git diff --check` passed; PR #50 required `rust` CI passed.
- Dependency evidence: no new third-party dependencies; report schema changes use existing Serde/JSON support.
- Deferred issues: #44, #46, and #47 were later completed by PR #52.

Gate: Release Gate for M6-003.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release, annotated SVG evidence, hardware validation, or certification artifact.

## M6 Completion Release Update

- PR: #52, `Complete M6 evidence and validation work`.
- Merge commit: `dd9c4bf39a5866f8a2cf903247db2ca0ded6a2b9`
- Merge method: rebase / fast-forward mainline.
- Required check: `rust`, passed in 27 seconds.
- Issues: #44 `M6-002 Add annotated SVG criteria evidence overlays`; #46 `M6-004 Document criteria DSL direction for engineering measurements`; #47 `M6-005 Add measurement-engine validation fixtures`.
- Milestone: `v0.4.0: Measurement & Evidence Engine`, closed with 5 closed issues and 0 open issues.
- Scope: 2D SVG evidence overlays, criteria DSL direction docs, and measurement-engine known-answer fixtures.
- Validation: local `cargo fmt`, `cargo fmt --check`, `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, annotated SVG CLI smoke, and `git diff --check` passed; PR #52 required `rust` CI passed.
- Dependency evidence: no new third-party dependencies.

Gate: Release Gate for M6 completion.
Decision: Pass.
Residual risk: This is mainline repository evidence, not a tagged product release, hardware validation, or certification artifact.

## M15-M20 MVP Exit Release Readiness Update

- Scope: Local MVP-exit branch readiness for M15 config reference, M16 artifact contract, M17 batch analysis workflow, M18 transform-package compatibility, M19 validation corpus index, and M20 readiness review.
- Release publication: Not performed.
- GitHub milestones/issues: Not created for M15-M20.
- External PR: Not opened by the original local implementation pass; later included in PR #175.
- Dependency evidence: No new third-party dependencies.
- Validation evidence: See `docs/validation-log.md`.
- Readiness evidence: `docs/mvp-exit-readiness-report.md` and `docs/m15-m20-mvp-exit-pipeline-report.md`.

Gate: Release Gate for local MVP exit.
Decision: Pass locally, no publication.
Residual risk: No release tag was published; release publication remains separately gated.

## M21-M24 Runtime Path Release Readiness Update

- Scope: Local runtime-path branch readiness for M21 linear pointwise package semantics, M22 shared borrowed-slice runtime semantics, M23 package compatibility corpus, and M24 runtime-loader design gate.
- Release publication: Not performed.
- GitHub milestones/issues: Not created for M21-M24.
- External PR: Not opened by the original local implementation pass; later included in PR #175.
- Dependency evidence: No new third-party dependencies.
- Validation evidence: See `docs/validation-log.md`.
- Readiness evidence: `docs/m21-m24-runtime-path-pipeline-report.md` and `docs/runtime-loader-design-gate.md`.

Gate: Release Gate for local M21-M24 runtime path.
Decision: Pass locally, no publication.
Residual risk: No release tag was published; release publication and runtime-loader implementation remain separately gated.

## Hand-Off Note

Role: Release Engineer / GitHub Maintainer Specialist
Goal: Track release readiness from initial publication through MVP exit, M21-M24 runtime-path follow-up, and M25-M36 comprehensive-suite mainline merge.
Files changed: `docs/release-readiness.md`, `.github/workflows/ci.yml`, and linked MVP-exit/runtime-path/comprehensive-suite artifacts.
Checks run: Uses validation evidence from `docs/validation-log.md`; GitHub Actions historical CI evidence is preserved for prior merged PRs, including PR #175.
Status: Pass for M25-M36 mainline merge through PR #175; no release tag published.
Known gaps: Release publication, crate publication, runtime-loader implementation, package/runtime expansion, hardware evidence, and certification evidence need separate approval gates.
Next recommended step: Choose a separately gated release publication plan or the next advanced follow-up theme.
