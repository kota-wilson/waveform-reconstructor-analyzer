# Native egui Workflow Shell Roadmap

Date: 2026-06-03

Status: M43-M53 plus WRA-RQ-139 are complete and merged through PR #190 as a gated native egui workflow shell, with Source, Config, Run, and Plot page UX plus scalable plotting refinements. GitHub milestone #15 is closed with issues #179 through #189 closed by PR #190. Live/realtime DAQ, vendor SDKs, hardware acquisition, runtime loaders, installers, release publication, and certification evidence remain separately gated.

## Purpose

The M37-M42 desktop workflow made the CLI path coherent from source inspection through evaluation bundles. M43-M48 add a macOS-first native GUI shell over that same workflow without duplicating analysis behavior or adding live hardware scope.

The GUI is an authoring and review surface for existing software-only CSV and fixture-simulation workflows. It is not a live DAQ tool, hardware controller, runtime loader, certified test system, release artifact, or packaged desktop application.

## Milestones

| Milestone | Goal | Status |
|---|---|---|
| M43 | GUI gate, dependency review, GitHub tracking, and MSRV-compatible dependency pins. | Complete; merged in PR #190 |
| M44 | Shared workflow library extraction from the CLI implementation. | Complete; merged in PR #190 |
| M45 | Optional native `ferrisoxide-gui` crate and egui app shell. | Complete; merged in PR #190 |
| M46 | Source inspection and config scaffolding panels. | Complete; merged in PR #190 |
| M47 | Analysis/evaluation run controls and results/artifact review. | Complete; merged in PR #190 |
| M48 | Interactive CSV plot review, macOS GUI CI, docs, and validation closure. | Complete; merged in PR #190 |
| M49 | Source-page CSV file selector and explicit Load Channels header-loading action. | Complete; merged in PR #190 |
| M50 | Header-driven Time Column dropdown, Time Unit dropdown, and per-channel unit selectors. | Complete; merged in PR #190 |
| M51 | Plot-page channel checkboxes derived from Source channel state. | Complete; merged in PR #190 |
| M52 | Scalable Plot-page rendering with resolution control, min/max viewport decimation, cached render points, and multiresolution plot pyramids. | Complete; merged in PR #190 |
| M53 | Channel-based Config-page builder with dropdown-only action/criteria choices and numeric-only value fields. | Complete; merged in PR #190 |

## Implementation Shape

- `ferrisoxide-workflow` owns shared request/result APIs for inspection, CSV header loading, scaffolding, analysis, evaluation bundles, and CSV plot-series loading.
- `ferrisoxide-cli` is now a thin binary wrapper over `ferrisoxide-workflow::run`, preserving existing CLI behavior.
- `ferrisoxide-gui` has default-feature state tests and a `native` feature for `eframe`, `egui_plot`, and optional native CSV file dialogs through `rfd`.
- The GUI uses typed Rust calls into `ferrisoxide-workflow`; it does not shell out to the CLI.
- Native GUI dependencies are exact-pinned where needed to preserve the workspace `rust-version = "1.76"` claim.
- Complete user-facing button/control documentation lives in `docs/desktop-user-workflow.md`.

## Scope Boundaries

In scope:

- CSV and fixture-simulation source path inputs.
- CSV Source-page file selector and explicit `Load Channels` action.
- Header-populated Time Column dropdown and channel assignment rows.
- Time Unit dropdown and per-channel GUI unit dropdowns for loaded CSV headers.
- CSV source inspection and config scaffolding.
- Config-page channel sections derived from enabled Source channels.
- Dropdown-driven filter/action and criterion authoring with numeric controls for numeric values.
- Generated or opened config TOML preview with explicit `Open TOML` and native `Save As` behavior.
- Core-backed channel scoping for supported generated same-time-axis filter rows.
- CSV analysis preview and CSV/simulation evaluation bundle generation.
- Run-page output directory selection through a native folder picker.
- Bundle artifact listing and summary preview.
- Interactive CSV plot-series display through `egui_plot`.
- Plot-page channel checkboxes that filter the displayed CSV series without mutating Source-channel assignment.
- Plot-page Fast/Balanced/Detailed/Full resolution control, cached viewport render points, min/max bucket decimation, and multiresolution plot pyramids for large loaded CSV series.
- macOS native compile/clippy CI for the explicit GUI feature.

Out of scope:

- Persisted per-channel unit schema changes beyond the current GUI Source-page state.
- GUI packaging, signing, notarization, installers, or release publication.
- Live/realtime DAQ, vendor SDKs, drivers, hardware channel discovery, or hardware acquisition.
- HAL/RTOS adapters, target-board execution, runtime-loader implementation, or controller deployment.
- Hardware qualification, flight certification, regulatory compliance, or airworthiness evidence.

## Gate Decisions

| Gate | Decision | Evidence | Residual Risk |
|---|---|---|---|
| Human approval | Pass | User requested GUI milestone gate and implementation of the egui plan. | Future GUI expansion still needs scoped approval. |
| Dependency gate | Pass | `docs/dependency-review.md` records exact `eframe`/`egui_plot` pins, MSRV rationale, `rfd` file-dialog review, and Cargo evidence. | Native GUI transitive surface is much larger than the CLI path. |
| Architecture gate | Pass | CLI behavior moved behind `ferrisoxide-workflow`, and GUI calls shared APIs. | Broader workflow refactoring should remain incremental. |
| CI gate | Pass | `cargo test -p ferrisoxide-gui`, `cargo check -p ferrisoxide-gui --features native`, PR #190 `rust`, and PR #190 `gui-macos` checks passed. | Visual regression automation remains future-gated. |
| Issue tracking gate | Pass | GitHub milestone #15 is closed with issues #179 through #189 closed by PR #190. | Future GUI expansion needs a new gate and tracking package. |

## Hand-Off Note

Role: Product Architect / Core Software Engineer
Goal: Gate and implement the native egui workflow shell without expanding live DAQ, runtime, hardware, release, or certification scope.
Files changed: `Cargo.toml`, `.github/workflows/ci.yml`, `crates/ferrisoxide-workflow/`, `crates/ferrisoxide-gui/`, roadmap/state docs.
Checks run: See `docs/validation-log.md`.
Status: Complete; merged in PR #190.
Known gaps: No packaging, no installer, no visual regression harness, no persisted per-channel unit schema beyond generated config text, no simulated/live plot-channel support beyond future-compatible state derivation, no live DAQ, no runtime loader, no hardware validation, no certification evidence. M52 optimizes GUI rendering only; analysis/export data remains full-fidelity and separate benchmark claims still require dedicated performance evidence.
Next recommended step: Select any future GUI, packaging, live DAQ, runtime, hardware, release, or certification follow-up only after an explicit gate.
