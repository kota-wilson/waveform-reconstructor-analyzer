# Post-MVP Roadmap

Date: 2026-06-02

Status: Planning backlog after MVP exit, the M21-M24 runtime-path follow-up, and the complete M25-M36 comprehensive filter/signal-conditioning suite merged through PR #175. Release publication, dependency additions, runtime-loader implementation, live DAQ, HAL/RTOS, target hardware, and certification scope remain separately gated.

## Purpose

M20 moves FerrisOxide out of MVP for the local desktop software product surface. M21-M24 add the first narrow runtime-path follow-up for portable linear pointwise package semantics and a loader design gate. This roadmap keeps remaining work separate so post-MVP ideas do not blur the exit decision or imply broader runtime readiness.

The next selected planning theme is a comprehensive sampled-waveform filter and signal-conditioning suite. The detailed path lives in `docs/comprehensive-filter-signal-conditioning-roadmap.md`.

## Completed Local Runtime-Path Follow-Up

| Milestone | Scope | Status |
|---|---|---|
| M21 | Portable rule-package schema/export support for `offset`, `gain`, and `invert`. | Complete; merged in PR #175 |
| M22 | Shared borrowed-slice runtime-compatible semantics and desktop parity coverage for the linear pointwise subset. | Complete; merged in PR #175 |
| M23 | Positive TOML/JSON package fixtures and negative unsupported-transform fixtures. | Complete; merged in PR #175 |
| M24 | Runtime loader design gate for bounded Raspberry Pi 5 bare-metal package consumption. | Complete as design only; merged in PR #175 |

## Selected Next Planning Theme

| Theme | Scope | Status |
|---|---|---|
| Comprehensive filter and signal conditioning suite | M25-M36 roadmap for transform registry, data cleaning, pointwise/nonlinear conditioning, smoothing, frequency filters, resampling, envelope/energy/calculus, statistics, spectrum/time-frequency, fault injection, ADC/DAC simulation, multi-channel/sensor packs, and completeness closure. | M25-M36 complete and merged through PR #175; see `docs/comprehensive-filter-signal-conditioning-roadmap.md` |

## Candidate Milestone Themes

| Theme | Candidate Scope | Required Gate |
|---|---|---|
| Config and report automation | Generated config reference checks, README command refresh, automated Markdown link checking, schema compatibility tests. | Documentation / QA approval |
| Validation corpus expansion | More known-answer transform fixtures, benchmark refreshes, negative-case fixtures, visual regression for SVG output. | V&V / Performance approval |
| Portable transform export expansion | Explicit rule-schema representation and runtime-profile evidence for one rejected transform at a time beyond the M21 `offset`/`gain`/`invert` subset. | Architecture / Embedded RTOS / Compatibility approval |
| Batch workflow hardening | Batch manifest schema tests, report retention conventions, optional validation corpus runner, exact summary fixtures. | Core / Test Automation approval |
| Advanced DSP implementation | FIR/IIR design families, FFT/PSD workflows, resampling, spectrum analysis, time-frequency analysis, sensor-specific calibration packages, and fault-injection suites staged through M25-M36. | Research / Product / Dependency approval |
| Hardware and DAQ investigation | Vendor SDK review, fixture/live boundary design, acquisition security review, environment setup plan. | Human / Security / Environment / Hardware approval |
| Embedded runtime work | Implementation of the M24 runtime loader design, target compile checks, no_std package subsets, and target hardware demos. | Embedded RTOS / V&V / Human approval |
| Release operations | Tagged releases, changelog automation, release notes, crates.io or package publication planning. | Release / Security / Human approval |

## Explicitly Not Started

- No live DAQ SDK integration.
- No HAL or RTOS SDK integration.
- No target-board execution.
- No binary rule-package loader implementation.
- No signing, authentication, or tamper-proof package claims.
- No hosted service, scheduler, database workflow, plugin runtime, GUI, or web UI.
- No hardware qualification, flight certification, regulatory compliance, production safety certification, or airworthiness evidence.

## First Recommended Post-MVP Step

Choose one gated advanced follow-up or a separate release-publication plan. M25 created the transform registry and completeness contract, M26 added data-cleaning/timing-conditioning transforms, M27 added pointwise normalization/nonlinear transforms, M28 added smoothing/baseline conditioning transforms, M29 added standard frequency filters, M30 added resampling/timing-alignment transforms, M31 added envelope/energy/calculus filters and feature records, M32 added statistics/correlation filters and feature records, M33 added spectrum/window/time-frequency feature records, M34 added deterministic fault injection and ADC/DAC simulation, M35 added multi-channel, sensor, vibration, and control conditioning, and M36 closed catalog/docs/corpus/compatibility/readiness evidence before the PR #175 merge.

## Hand-Off Note

Role: Product Architect / Project Coordinator
Goal: Separate post-MVP backlog from MVP-exit readiness.
Files changed: `docs/post-mvp-roadmap.md`, linked readiness and roadmap docs.
Checks run: See `docs/validation-log.md`.
Status: Backlog separated; M21-M24 runtime-path follow-up and M25-M36 comprehensive suite work are complete and merged through PR #175.
Known gaps: Dependency-using advanced follow-ups still require dependency review. Exact elliptic/Cauer design, efficient polyphase resampling, Hilbert envelope, optimized FFT dependency/performance work, phase-difference estimation, gain/phase matching, advanced acoustic packs, advanced sensor calibration packs, and `split_by_event` multi-artifact segmentation remain dependency/design/future-gated.
Next recommended step: Choose one gated advanced follow-up or a separate release-publication plan.
