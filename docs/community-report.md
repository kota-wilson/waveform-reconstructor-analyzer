# Community Report

Date: 2026-05-31

Owner Role: Community Engineering Lead

## Publication

- Repository: `https://github.com/kota-wilson/ferrisoxide-signal`
- Visibility: Public.
- Default branch: `main`.
- CI status: Passing.

## Maintainer Notes

- The repository is an MVP, not production-certified validation software.
- Good first public issues should focus on malformed CSV fixtures, config schema documentation, and filter validation examples.
- No external contributors or issue traffic yet.

## M4 Community Update

- PR #36 merged and closed M4 issues #27-#34.
- Milestone `M4: Signal Accuracy and Validation` is closed.
- Follow-up community messaging should continue to frame validation fixtures as software validation aids, not hardware qualification or certification evidence.

## M5 Community Update

- Issue #38 records optional SVG plotting acceptance criteria.
- PR #39 merged and closed issue #38.
- Milestone `M5: Plotting and Visualization` is closed with 1 closed issue and 0 open issues.
- Follow-up community messaging should describe the feature as desktop SVG output only and avoid GUI, DAQ, embedded plotting, surface-plotting, or certification claims.

## M3 RTOS Follow-Up Community Update

- Issues #17, #18, and #19 record the remaining M3 RTOS acceptance criteria.
- PR #41 merged and closed issues #17, #18, and #19.
- Milestone `M3: RTOS / embedded no_std foundation` is closed with 4 closed issues and 0 open issues.
- Follow-up community messaging should avoid claiming ARM64 target execution, Zephyr support, RTOS production readiness, hardware validation, or certification evidence.

## M6 Measurement Engine Community Update

- Issues #43-#47 record the v0.4.0 measurement and evidence-engine roadmap.
- PR #48 merged and closed issue #43, the first implementation slice for reusable measurement extraction.
- Issues #44-#47 were later completed; milestone `v0.4.0: Measurement & Evidence Engine` is now closed.
- Follow-up community messaging should describe M6-001 as internal measurement reuse and compatibility preservation, not a new report schema, annotated SVG feature, batch-analysis feature, plugin runtime, hardware validation, or certification claim.

## M6-003 Report Measurement Schema Community Update

- Issue #45 records report measurement schema acceptance criteria.
- Branch `feature/m6-report-measurement-schema` implements stable measurement records, result links, exact golden JSON updates, and schema migration docs.
- PR #50 merged and closed issue #45.
- Issues #44, #46, and #47 were later completed by PR #52.
- Follow-up community messaging should describe M6-003 as a report evidence schema migration, not annotated SVG, batch analysis, plugin runtime, GUI, DAQ, hardware validation, or certification evidence.

## M6 Completion Community Update

- PR #52 from branch `feature/m6-complete-evidence-work` implements issues #44, #46, and #47.
- PR #52 merged and closed #44, #46, and #47.
- Milestone `v0.4.0: Measurement & Evidence Engine` is closed with 5 closed issues and 0 open issues.
- Repository issue list is empty after M6 completion.
- Follow-up community messaging should describe this as SVG evidence overlays, DSL direction documentation, and software known-answer validation, not GUI, DAQ, plugin runtime, hardware validation, or certification evidence.

## M15-M20 MVP Exit Community Update

- M15-M20 are complete as the MVP-exit pass and were later included in PR #175.
- No GitHub milestones/issues, release tag, or public announcement is created by this update.
- User-facing messaging may say FerrisOxide has passed local MVP-exit readiness for the desktop software workflow after this branch is reviewed, but must not claim live DAQ, target hardware execution, production RTOS readiness, hardware qualification, safety certification, regulatory compliance, or airworthiness evidence.
- Post-MVP work should be introduced from `docs/post-mvp-roadmap.md` one gated theme at a time.

## M21-M24 Runtime Path Community Update

- M21-M24 are complete as the runtime-path follow-up and were later included in PR #175.
- User-facing messaging may say rule-package export now supports `offset`, `gain`, and `invert` as software transforms, but must not claim calibrated sensor accuracy, runtime-loader implementation, target hardware execution, production RTOS readiness, hardware qualification, safety certification, regulatory compliance, or airworthiness evidence.
- Runtime-loader implementation should not be announced until a fresh implementation gate is approved and completed.

## M25-M36 Comprehensive Suite Community Update

- M25-M36 are merged to `main` through PR #175 as the comprehensive filter and signal-conditioning suite.
- User-facing messaging may say FerrisOxide covers the ordinary desktop sampled-waveform conditioning workflow through cataloged filters, feature records, simulation filters, sensor/software domain transforms, examples, corpus docs, package/runtime guardrails, and validation evidence.
- Messaging must still avoid claiming live DAQ, runtime-loader implementation, target hardware execution, production RTOS readiness, hardware calibration, hardware qualification, safety certification, regulatory compliance, or airworthiness evidence.
- Advanced dependency/design-gated work should be introduced as separate follow-up scope, not as already-supported behavior.

## Gate Decision

- Gate: Community Gate.
- Decision: Pass for initial publication, M5 issue/milestone closure, M3 issue/milestone closure, M6 issue planning, M6-001 issue closure, M6-003 issue closure, M6 completion, M15-M20 MVP exit, M21-M24 runtime path, and M25-M36 comprehensive-suite merge through PR #175.
- Reason: Public repository exists with templates, contribution docs, CI, clear scope, historical issue/PR closure evidence, and MVP-exit/runtime/comprehensive-suite messaging that separates desktop software readiness and narrow package semantics from hardware/runtime/certification scope.
- Residual risk: Community onboarding quality, plotting usability, downstream measurement-schema usability, batch workflow usability, comprehensive-suite usability, and embedded adapter usability are untested until external issue/PR feedback.
- Next owner: Project Coordinator.

## Hand-Off Note

Role: Community Engineering Lead
Goal: Confirm public community surface, MVP-exit messaging, runtime-path messaging, and M25-M36 mainline messaging boundaries exist.
Files changed: `docs/community-report.md`
Checks run: Repository, PR #175, and CI inspection.
Status: Pass for M25-M36 mainline merge through PR #175.
Known gaps: M15-M36 changes have not received broad external community feedback.
Next recommended step: Choose one gated follow-up or a separate release-publication plan.
