# Retrospective

Date: 2026-05-31

Owner Role: Project Coordinator

## Current Status

This is the initial publication retrospective. Follow-up GitHub issues have since been created for M1, v0.2.0, M3, M4, M5, ADC quantization, and M6 measurement/evidence work; remaining open issues are tracked in `project-state.md`.

## What Worked

- The project stayed inside the workspace and used project-local Cargo tooling.
- Dependency approval happened before adding crates.
- The MVP reached a public GitHub repository with passing CI.
- Traceability and gate artifacts were updated as the project moved forward.
- M4 closed eight validation issues without adding dependencies or expanding into GUI, DAQ, RTOS, or certification scope.
- M5 kept plotting isolated in a dedicated crate and added only the approved SVG line-rendering dependency surface.
- M3 follow-up work added embedded adapter and prototype artifacts without installing SDKs, adding HALs, or claiming production RTOS readiness.
- M6 measurement extraction used existing golden JSON tests as a strong regression guard against subtle evidence drift.

## What To Improve

- Add automated license/advisory checks before the next dependency expansion.
- Add malformed CSV/config fixtures earlier.
- Create a config schema reference before widening the config format.
- Consider replacing hand-rolled CLI parsing after dependency review.
- Decide whether the next validation milestone should add external capture corpora, stronger filter-response checks, or schema hardening before adding new user interfaces.
- Expand report/SVG evidence only after the measurement schema is documented and golden tests are updated intentionally.
- Add visual regression or rendered-output review before broadening plotting beyond SVG line charts.
- Add target execution and SDK validation only after a fresh environment/toolchain gate.

## Gate Decision

- Gate: Retrospective Gate.
- Decision: Pass.
- Reason: Lessons and next actions are recorded, including the M4 post-merge outcome, M5 plotting scope control, M3 embedded prototype scope control, and M6 measurement regression-guard lesson.
- Residual risk: Remaining follow-up issues still need prioritization and implementation.
- Next owner: Community Engineering Lead.

## Hand-Off Note

Role: Project Coordinator
Goal: Capture lessons from project creation through public publication.
Files changed: `docs/retrospective.md`
Checks run: Reviewed project artifacts and final CI status.
Status: Pass.
Known gaps: Measurement reports still lack a separate measurement schema, plotting still lacks visual regression, and embedded work still lacks target execution or SDK validation.
Next recommended step: Complete the M6 report/SVG evidence issues before adding new visualization backends, interactive plotting scope, RTOS SDKs, HALs, or target CI.
