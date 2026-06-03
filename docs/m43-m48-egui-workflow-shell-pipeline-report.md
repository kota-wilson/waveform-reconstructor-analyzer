# M43-M48 Native egui Workflow Shell Pipeline Report

Date: 2026-06-03

Stage: GUI gate, implementation, and local validation

Owner Role: Core Software Engineer / Product Architect

## Summary

M43-M48 implement the first native GUI shell for the existing desktop workflow. M49-M50 refine the Source page with CSV file selection, explicit header loading, and header-driven unit selectors. M51 refines the Plot page with channel checkboxes derived from Source channel state. M52 adds scalable render-only plotting for large loaded CSV series with resolution modes, min/max viewport decimation, cached render points, and multiresolution plot pyramids. M53 refines the Config page with Source-derived channel sections, dropdown-only action/criteria choices, numeric-only value fields, generated/opened TOML preview, `Open TOML`, `Save As`, and `Save` behavior. WRA-RQ-139 refines the Run page with native output-directory folder selection. The implementation extracts CLI workflow behavior into `ferrisoxide-workflow`, adds `ferrisoxide-gui` with an optional native egui feature, records the dependency gate, and adds a macOS CI job for native GUI checks.

The work is macOS-first and software-only. It does not add live DAQ, hardware acquisition, runtime-loader execution, packaging, release publication, or certification evidence.

GitHub tracking:

- Milestone #15: `M43-M48: Native egui Workflow Shell`
- Issues #179 through #184: M43 through M48 tracking issues.
- Issue #186: M49 CSV picker and header loading.
- Issue #185: M50 time and per-channel unit selectors.
- Issue #187: M51 plot channel selectors.
- Issue #188: M52 scalable GUI plot rendering.
- Issue #189: M53 channel-based Config builder.

## Stage Decisions

| Stage | Decision | Evidence |
|---|---|---|
| Intake | Pass | User approved gating GUI milestones and implementing the egui plan. |
| Issue Planning | Pass | GitHub milestone #15 and issues #179 through #189 track M43-M53 GUI work. |
| Requirements | Pass | WRA-RQ-128 through WRA-RQ-139 define the workflow-library, GUI shell, Source-page UX, Config-page builder, Plot-page selector, scalable rendering refinement, and Run-page output directory picker scope. |
| Architecture | Pass | `ferrisoxide-workflow` shares CLI behavior with the GUI; `ferrisoxide-cli` delegates to the shared crate. |
| Dependency | Pass | `docs/dependency-review.md` records exact `eframe = "=0.28.1"`, `egui_plot = "=0.28.1"`, and `rfd = "=0.14.1"` pins. |
| Implementation | Pass locally | `crates/ferrisoxide-workflow` and `crates/ferrisoxide-gui` implement the approved workflow shell. |
| Testing | Pass | Focused workflow and GUI checks passed locally; full workspace validation is recorded in `docs/validation-log.md`; PR #190 `rust` and `gui-macos` checks passed before merge. |
| Release | Not Applicable | No release, installer, tag, package, publication, or public announcement is included. |

## Implementation Evidence

- Added `ferrisoxide-workflow` as the shared API layer for source inspection, CSV header loading, config scaffolding, analysis, evaluation bundles, and CSV plot-series loading.
- Replaced the CLI binary implementation with a thin wrapper around `ferrisoxide_workflow::run`.
- Added `ferrisoxide-gui` with default-feature state tests and a `native` feature for the egui app.
- Added native panels for source inspection, config editing/scaffolding, run controls, result/artifact review, and interactive CSV plotting.
- Added a native Source-page CSV file selector through optional `rfd`, an explicit `Load Channels` action, Time Column and Time Unit dropdowns, and per-channel unit selectors.
- Added Plot-page channel checkboxes beside `Load Series`; selected plot channels filter loaded series without changing Source-channel assignment.
- Added Plot-page Fast/Balanced/Detailed/Full resolution control, viewport-aware min/max bucket decimation, cached rendered point state, and multiresolution plot pyramids while preserving the raw loaded series for analysis/export.
- Added Config-page channel sections derived from Source channels, dropdown-driven filter/action and criterion rows, numeric controls for numeric fields, generated/opened TOML preview, and native `Open TOML` / `Save As` / `Save` behavior.
- Added Run-page native output-directory folder selection.
- Added core channel-scoped filter wrapping for supported generated same-time-axis filter rows so selected-channel filter configs apply only to the selected channel.
- Added `.github/workflows/ci.yml` `gui-macos` job for native GUI feature checks.

## Validation

Commands run locally:

- `cargo test -p ferrisoxide-workflow workflow_api`: Pass; 4 focused shared workflow API tests passed.
- `cargo test -p ferrisoxide-workflow workflow_api_loads_csv_headers`: Pass.
- `cargo test -p ferrisoxide-core channel_scoped -- --nocapture`: Pass.
- `cargo test -p ferrisoxide-gui`: Pass; 16 GUI state tests passed after M53 and the Run-page picker refinement.
- `cargo check -p ferrisoxide-gui --features native`: Pass.
- `cargo tree -p ferrisoxide-gui --features native -i egui`: Pass; single `egui v0.28.1` resolved through `eframe`, `egui-winit`, `egui_glow`, and `egui_plot`.
- `cargo tree -p ferrisoxide-gui --features native --depth 2`: Pass; native GUI dependency surface recorded for review.

Final workspace validation and PR #190 merge evidence are recorded in the newest `docs/validation-log.md` sections.

## Scope Notes

- GUI dependencies are optional and isolated to `ferrisoxide-gui --features native`.
- The existing Ubuntu workspace CI path remains unchanged.
- The GUI uses native dialogs only for selecting local CSV input, TOML config open/save paths, and evaluation output directories; packaging and release-product scope remain gated.
- The app does not claim complete GUI product maturity, release packaging, hardware validation, RTOS readiness, live acquisition, or certification evidence.

## Hand-Off Note

Role: Core Software Engineer / Product Architect
Goal: Implement the M43-M53 native egui workflow shell plus Source/Config/Run/Plot-page UX and scalable rendering refinements.
Files changed: `Cargo.toml`, `.github/workflows/ci.yml`, `crates/ferrisoxide-workflow/`, `crates/ferrisoxide-gui/`, roadmap/state docs.
Checks run: See validation section and `docs/validation-log.md`.
Status: Complete; merged in PR #190.
Known gaps: No installer, release artifact, persisted per-channel unit config schema beyond generated config text, simulated/live plot-channel support beyond future-compatible state derivation, live DAQ, runtime-loader, hardware acquisition, or certification evidence. M52 is render-only GUI optimization and does not make benchmarked end-to-end performance claims.
Next recommended step: Select any future GUI, packaging, live DAQ, runtime, hardware, release, or certification follow-up only after an explicit gate.
