# End-User Design Review

Date: 2026-05-31

Branch: `feature/end-user-design-review`

Review type: Domain end-user design review

Pull request: #23, `https://github.com/kota-wilson/waveform-reconstructor-analyzer/pull/23`

Review status: Advisory design review; not a hardware validation, RTOS production-readiness determination, flight certification determination, legal opinion, or regulatory compliance claim.

## Scope

This review evaluates the Waveform Reconstructor and Analyzer design from four end-user engineering perspectives:

- Electrical Signal Integrity Engineer.
- Environmental Test Engineer.
- Embedded RTOS Engineer.
- Flight Certification Assurance Engineer.

The review covers the current `main` branch after the M1-001, v0.2.0, and M3-RTOS-001 merges, plus this documentation review branch:

| Artifact | Evidence Reviewed |
|---|---|
| Product README | `README.md` |
| Architecture | `docs/architecture.md` |
| Usage | `docs/usage-mvp.md` |
| Requirements | `requirements.md` |
| Traceability | `traceability-matrix.md` |
| Risk register | `risk-register.md` |
| Merged implementation context | PR #16 criteria engine, PR #21 `no_std` signal primitives, and PR #22 CSV edge cases; all passed the required `rust` CI before merge. |

Certification context was treated cautiously. Official authority references consulted for context include FAA software/airborne electronic hardware guidance pages, FAA AC 20-115D material, and EASA AMC 20-115D material:

- https://www.faa.gov/aircraft/air_cert/design_approvals/air_software/
- https://www.faa.gov/regulations_policies/advisory_circulars/index.cfm/go/document.information/documentID/1038941
- https://www.easa.europa.eu/en/document-library/acceptable-means-of-compliance-and-guidance-materials/amc-20-115d

## Executive Summary

The current design is strong for an MVP CLI/core library: it separates parsing, models, filters, criteria, reports, and CLI orchestration; preserves raw data; uses structured errors; and avoids GUI, DAQ, hardware control, and certification claims.

The largest end-user gaps are not basic software structure. They are engineering evidence gaps:

- Electrical users need explicit units, tolerances, sample-rate assumptions, threshold semantics, filter latency/phase notes, and event terminology.
- Environmental test users need realistic fixture coverage, operator-oriented report evidence, and configuration error examples.
- Embedded/RTOS users now have the `wra-signal` `no_std` foundation, but still need target-build evidence before any RTOS claim.
- Certification-adjacent users need stronger claim boundaries, traceability discipline, configuration management evidence, and a clear statement that this tool is not qualified or certified.

## Findings

| ID | Role | Severity | Evidence | Recommendation | Owner | Suggested Next Artifact |
|---|---|---:|---|---|---|---|
| EDR-001 | Electrical Signal Integrity Engineer | High | `requirements.md` and criteria docs now use `transient event` terminology; architecture and README still use broad min/max language for some examples. | Preserve `transient event`, `spurious transition`, `contact bounce`, `dropout`, and `threshold crossing event` terminology in future requirements and docs. | Software Architect / Documentation Engineer | Terminology ADR or docs note. |
| EDR-002 | Electrical Signal Integrity Engineer | High | `Waveform` defaults to seconds and volts; no per-channel units, tolerances, calibration, scale factor, or uncertainty model exists on `main`. | Add a waveform metadata model that can record channel unit, threshold unit, tolerance, sample rate or timestamp policy, source file, and optional acquisition notes. | Software Architect | Issue #4 implementation plan or ADR. |
| EDR-003 | Electrical Signal Integrity Engineer | High | Filters exist, and `docs/architecture.md` says edge behavior, latency, and sample-rate assumptions must be documented before production-stable claims. | Add filter behavior docs and tests that state moving-average delay, low-pass assumptions, behavior with non-monotonic timestamps, and whether filtering changes criteria evidence timestamps. | Systems Engineer / Test Automation Engineer | Filter assumptions doc plus synthetic filter tests. |
| EDR-004 | Electrical Signal Integrity Engineer | Medium | Current reports include criterion evidence and golden JSON tests for v0.2.0 cases. | Keep every criterion report stable with measured value, required value, sample index, timestamp, channel, and failed criterion. Document report compatibility expectations before changing JSON fields. | Core Software Engineer / Test Automation Engineer | Report schema compatibility note. |
| EDR-005 | Environmental Test Engineer | High | Fixture CSVs now cover clean square wave, noisy square wave, switch bounce/transients, dropout, slow rise/fall, and multi-channel data. | Maintain a fixture matrix that maps each fixture to the criterion it verifies, expected outcome, and operator interpretation. | Test Automation Engineer | Fixture coverage matrix in `docs/` plus tests. |
| EDR-006 | Environmental Test Engineer | High | README usage examples run the CLI, but operator workflow does not yet show "what failed, where, and by how much" for environmental test review. | Add an environmental test use-case guide with config example, expected text report, expected JSON report, and interpretation notes for transient events and dropouts. | Documentation Engineer / Environmental Test Engineer | Issue #13 docs. |
| EDR-007 | Environmental Test Engineer | Medium | Config validation tests exist, but bad-config examples are not prominent in operator docs. | Add docs that show clear operator-facing errors for bad TOML, missing input sections, unknown criteria, and malformed thresholds. | Test Automation Engineer / Documentation Engineer | Invalid-config examples in usage docs. |
| EDR-008 | Environmental Test Engineer | Medium | No explicit test-run metadata exists for test article, run ID, environment condition, operator, or data source. | Keep metadata optional for MVP, but reserve schema fields for test run ID, source file, channel label, and condition notes before reports are used for review boards. | Software Architect | Metadata model issue or report schema ADR. |
| EDR-009 | Embedded RTOS Engineer | High | Current `main` includes `wra-signal` as a separate `#![no_std]` crate while desktop parsing/reporting remains outside the embedded path. | Preserve the separate `wra-signal` path before adding QEMU, RTOS, Embassy-style, or Zephyr adapters. Do not push CSV, file I/O, plotting, or reports into embedded crates. | Embedded RTOS Engineer / Core Software Engineer | M3 roadmap follow-up. |
| EDR-010 | Embedded RTOS Engineer | High | No ARM64 target-build evidence exists on `main`. | For M3-RTOS-002, require exact target, command, linker/runtime assumptions, memory layout assumptions, and CI or local log evidence before claiming ARM64 QEMU support. | Embedded RTOS Engineer / DevOps Engineer | M3-RTOS-002 test plan. |
| EDR-011 | Embedded RTOS Engineer | Medium | Future RTOS adapter ownership for timebase, scheduler, memory, and I/O is not defined on `main`. | Add an RTOS adapter interface design that states who owns timestamps, sample ingestion cadence, buffer lifecycle, error handling, and result export. | Software Architect / Embedded RTOS Engineer | M3-RTOS-003 ADR. |
| EDR-012 | Flight Certification Assurance Engineer | Critical | README says aerospace certification claims are out of scope, which is good; no certification evidence program exists. | Preserve explicit non-goals. Add a certification disclaimer to any future environmental/aerospace-facing docs: this is engineering analysis software, not certified tooling or qualified verification software. | Documentation Engineer / Project Coordinator | Certification claim-boundary note. |
| EDR-013 | Flight Certification Assurance Engineer | High | Current traceability is useful for open-source quality, but not sufficient for certification evidence. | If aviation use is ever considered, define a separate assurance plan for requirements baselines, independence, verification procedures, configuration management, tool qualification assessment, and change impact analysis. | Technical Director / V&V Engineer | Assurance plan proposal, not implementation. |
| EDR-014 | Flight Certification Assurance Engineer | High | CI proves Rust tests pass, but not lifecycle data, independence, or tool qualification. | Keep CI evidence framed as development verification only. Do not describe CI, golden tests, or reports as certification evidence unless reviewed under a formal assurance program. | Release Engineer / Flight Certification Assurance Engineer | Release-note language review. |
| EDR-015 | Flight Certification Assurance Engineer | Medium | Dependencies are approved for MVP, but there is no automated advisory/license/toolchain provenance check. | Add an issue for dependency/license/advisory scanning before any safety-adjacent or certification-adjacent positioning. | Security Engineer / DevOps Engineer | Supply-chain review issue. |

## Role Gate Decisions

| Gate | Owner | Decision | Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Domain Scope Gate | Project Orchestrator | Pass | Review scope, roles, reviewed artifacts, and non-goals listed above. | Future feature PRs may change some findings. | Electrical Signal Integrity Engineer |
| Signal Semantics Gate | Electrical Signal Integrity Engineer | Pass with findings | `requirements.md`, `docs/architecture.md`, README, merged criteria-engine context. | Metadata gaps remain until implemented. | Software Architect |
| Environmental Scenario Gate | Environmental Test Engineer | Pass with findings | README, fixtures, usage docs, merged criteria-engine context. | Operator interpretation docs remain incomplete. | Test Automation Engineer |
| Embedded Boundary Gate | Embedded RTOS Engineer | Pass with findings | Current workspace layout and merged `wra-signal` context. | No embedded target build evidence yet. | Embedded RTOS Engineer |
| Certification Claim Boundary Gate | Flight Certification Assurance Engineer | Pass with critical boundary note | README non-goals, traceability, validation logs, official FAA/EASA context links. | Any future aerospace-facing language needs fresh review. | Documentation Engineer |
| Granularity Gate | Abstraction Review Engineer | Pass | Findings name files, docs, owners, and suggested next artifacts. | Implementation tasks still need issue-level acceptance criteria. | Project Coordinator |
| Routing Gate | Project Coordinator | Pass | Findings mapped to owners and artifacts. | No new GitHub issues were created by this review. | Project Orchestrator |

## Recommended Backlog Routing

| Priority | Action | Suggested Target |
|---|---|---|
| P0 | Preserve certification non-goals and add claim-boundary language to future aerospace/environmental docs. | Documentation update before any aerospace-facing release language. |
| P1 | Route accepted review findings into small follow-up issues after this docs PR lands. | GitHub issues. |
| P1 | Preserve requirements terminology away from informal event wording. | Terminology ADR or docs note. |
| P1 | Add waveform metadata model for units, tolerance, source, and acquisition assumptions. | Issue #4. |
| P1 | Add fixture matrix and environmental test guide. | Issues #13 and #14. |
| P2 | Add filter assumption docs and timestamp/monotonic-time validation tests. | New issue or M1 follow-up. |
| P2 | Add dependency advisory/license scanning. | New issue before safety-adjacent positioning. |
| P2 | Add RTOS adapter ADR after `wra-signal` lands. | M3-RTOS-003. |

## Review Conclusion

The design is appropriate for a small CLI/core MVP and now has a merged v0.2.0 criteria engine plus a separate `wra-signal` embedded foundation. It should not be positioned as production-grade signal integrity software, embedded RTOS software, environmental test qualification software, or certification-supporting tooling yet.

The next design maturity step is to turn the findings above into small issues or PRs: terminology cleanup, metadata modeling, fixture/report evidence, filter assumption documentation, embedded target evidence, and certification claim-boundary docs.

## Hand-Off Note

Role: Project Orchestrator
Goal: Complete end-user design review using new domain specialty roles.
Files changed: `docs/end-user-design-review.md`, `requirements.md`, `project-state.md`
Checks run: Documentation review; terminology scan for informal event wording; `git diff --check`; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`.
Status: Review complete; accepted findings have been routed through later M1, v0.2.0, M3, and ADC quantization work, with remaining open items tracked in `project-state.md`.
Known gaps: Some advisory findings remain future work until selected for issue-specific implementation PRs.
Next recommended step: Route remaining open M1 and M3 issues through focused PRs.
