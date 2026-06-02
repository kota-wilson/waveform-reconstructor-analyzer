# Retrospective

Date: 2026-05-31

Owner Role: Project Coordinator

## Current Status

This started as the initial publication retrospective. Follow-up GitHub issues have since covered M1, v0.2.0, M3, M4, M5, ADC quantization, M6 measurement/evidence work, and later milestones through M14. M15-M20 are complete as the MVP-exit pass without GitHub issue creation, M21-M24 are complete as the first narrow runtime-path follow-up, and M25-M36 are merged to `main` through PR #175 as the comprehensive filter and signal-conditioning suite.

## What Worked

- The project stayed inside the workspace and used project-local Cargo tooling.
- Dependency approval happened before adding crates.
- The MVP reached a public GitHub repository with passing CI.
- Traceability and gate artifacts were updated as the project moved forward.
- M4 closed eight validation issues without adding dependencies or expanding into GUI, DAQ, RTOS, or certification scope.
- M5 kept plotting isolated in a dedicated crate and added only the approved SVG line-rendering dependency surface.
- M3 follow-up work added embedded adapter and prototype artifacts without installing SDKs, adding HALs, or claiming production RTOS readiness.
- M6 measurement extraction used existing golden JSON tests as a strong regression guard against subtle evidence drift.
- M6-003 used the measurement layer to separate reusable report evidence from criteria decisions without expanding into SVG, DSL, batch, GUI, DAQ, or certification scope.
- M6 completion reused report measurement IDs for SVG evidence, documented DSL direction before implementing syntax, and added known-answer measurement fixtures.
- M15-M20 focused on product readiness instead of adding broad DSP scope: config reference, artifact contract, local batch workflow, transform-package compatibility, validation index, and readiness gates.
- M21-M24 showed that post-MVP runtime work needs very small slices: package semantics for `offset`/`gain`/`invert`, borrowed-slice parity, fixture corpus, and a loader design gate before any loader implementation.
- The batch workflow stayed local and file-based, which avoided pulling in DAQ, scheduling, service, database, or hardware assumptions.
- M25-M36 worked because the registry came first: every new transform family had catalog metadata, package support, runtime support, evidence level, docs, examples, and validation before completeness claims.
- Dependency-light implementation kept momentum while exact elliptic/Cauer design, efficient polyphase resampling, Hilbert envelope, optimized FFT work, phase/gain matching, advanced acoustic packs, and calibration packs stayed gated instead of blocking ordinary engineering workflows.

## What To Improve

- Add automated license/advisory checks before the next dependency expansion.
- Add malformed CSV/config fixtures earlier.
- Create a config schema reference before widening the config format.
- Consider replacing hand-rolled CLI parsing after dependency review.
- Decide whether the next validation milestone should add external capture corpora, stronger filter-response checks, or schema hardening before adding new user interfaces.
- Reuse `measurement_id` in annotated SVG evidence instead of recalculating independent evidence markers.
- Add visual regression automation before expanding beyond simple 2D evidence labels.
- Add visual regression or rendered-output review before broadening plotting beyond SVG line charts.
- Add target execution and SDK validation only after a fresh environment/toolchain gate.
- Add automated docs link checking and config/report drift checks before broad post-MVP expansion.
- Refresh benchmarks under a controlled performance gate before making stronger throughput claims.
- Consider opening post-MVP issues one theme at a time to avoid turning runtime-loader work into broad embedded/platform scope.
- Add generated catalog/config-reference drift checks before future broad transform expansion.
- Add larger benchmark fixtures before claiming throughput on production-scale waveform sets.

## Gate Decision

- Gate: Retrospective Gate.
- Decision: Pass.
- Reason: Lessons and next actions are recorded, including the M4 post-merge outcome, M5 plotting scope control, M3 embedded prototype scope control, M6 measurement regression-guard lesson, M6-003 report-schema migration lesson, M6 completion overlay/fixture lesson, M15-M20 MVP-exit scope-control lesson, M21-M24 runtime-path scoping lesson, M25-M36 catalog-first comprehensive-suite lesson, and PR #175 mainline merge.
- Residual risk: Release publication, advanced DSP/domain follow-ups, and runtime-loader implementation still need prioritization, issue planning, and approval.
- Next owner: Community Engineering Lead.

## Hand-Off Note

Role: Project Coordinator
Goal: Capture lessons from project creation through public publication, MVP exit, M21-M24 runtime-path follow-up, and M25-M36 comprehensive-suite mainline merge.
Files changed: `docs/retrospective.md`
Checks run: Reviewed project artifacts and final CI status.
Status: Pass.
Known gaps: Annotated SVG evidence still lacks visual regression, automated config/report drift checks are not implemented, benchmark fixtures remain small, and embedded work still lacks target execution or SDK validation.
Next recommended step: Choose one gated advanced follow-up or a separate release-publication plan.
