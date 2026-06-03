# Dependency Review

Date: 2026-05-31

Project: FerrisOxide Signal

Stage: Dependency Gate

Owner Role: Security Engineer / Software Architect

## Approval

The user approved proceeding through the dependency, license, and publication gates after the dependency-free MVP validation.

Current status: Approved dependencies were added and pinned in `Cargo.lock`; the public repository publication gate later passed. Any new dependency still requires a fresh dependency review.

## Proposed Dependencies

| Crate | Scope | Purpose | License Expectation | Alternative Considered | Decision |
|---|---|---|---|---|---|
| `csv` | `ferrisoxide-core` | Robust header-based CSV parsing and records handling. | MIT / Unlicense family in the Rust ecosystem. | Continue hand-written parser. | Approved because CSV dialect risk is already tracked. |
| `serde` | `ferrisoxide-core` | Derive stable config and report data structures. | MIT / Apache-2.0 family. | Manual parsing/serialization. | Approved because structured data is central to config and reports. |
| `serde_json` | `ferrisoxide-core` | JSON report rendering for automation. | MIT / Apache-2.0 family. | Manual JSON strings. | Approved because manual JSON is error-prone. |
| `toml` | `ferrisoxide-cli` | Parse user-facing analysis config files. | MIT / Apache-2.0 family. | Keep CLI-only criteria. | Approved because the project already defines TOML config examples. |
| `plotters` | `ferrisoxide-plot` | Render optional desktop waveform plots to SVG, including 2D and 3D line charts. | MIT, per Plotters package metadata. | Hand-written SVG rendering. | Approved by user for M5 plotting; constrained with `default-features = false` and `svg_backend` / `line_series` only. |

## M5 Plotting Dependency Review

Plotters was reviewed for the approved plotting slice after the user approved adding the dependency.

| Item | Evidence | Result |
|---|---|---|
| Crate scope | `plotters = { version = "0.3.7", default-features = false, features = ["svg_backend", "line_series"] }` | Pass |
| Backend scope | SVG backend only; no bitmap, GUI, GIF, or interactive backend feature selected. | Pass |
| License | Plotters Cargo metadata lists `license = "MIT"`. | Pass |
| Transitive build surface | `cargo tree -p ferrisoxide-plot` shows native active tree: `plotters`, `plotters-backend`, `plotters-svg`, `num-traits`, and `autocfg`, plus existing `ferrisoxide-core` dependencies. | Pass |
| Architecture boundary | Dependency lives in `ferrisoxide-plot`; `ferrisoxide-core` and `ferrisoxide-signal` remain plotting-free. | Pass |

Reviewed sources:

- Plotters SVG backend docs: `https://docs.rs/plotters/latest/plotters/backend/struct.SVGBackend.html`
- Plotters feature-control docs: `https://docs.rs/plotters/`
- Plotters package metadata: `https://raw.githubusercontent.com/plotters-rs/plotters/master/plotters/Cargo.toml`

## M3 RTOS Follow-Up Dependency Review

The M3 RTOS adapter/prototype branch adds no new third-party crates.

| Item | Evidence | Result |
|---|---|---|
| Adapter crate | `crates/ferrisoxide-embedded/Cargo.toml` depends only on local `ferrisoxide-signal`. | Pass |
| QEMU proof crate | `embedded/arm64/qemu/Cargo.toml` depends only on local `ferrisoxide-embedded` and `ferrisoxide-signal`. | Pass |
| Zephyr prototype | `embedded/arm64/zephyr/` is not wired into Cargo and adds no SDK dependency. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-embedded` shows only `ferrisoxide-embedded` -> `ferrisoxide-signal`. | Pass |
| Toolchain scope | No ARM64 target, QEMU binary, Zephyr SDK, west workspace, CMake, HAL, or unsafe FFI is added. | Pass |

## M6 Measurement Engine Dependency Review

The M6 measurement-engine extraction adds no new third-party crates.

| Item | Evidence | Result |
|---|---|---|
| Measurement crate | `crates/ferrisoxide-measurements/Cargo.toml` has no dependency entries. | Pass |
| Core dependency | `crates/ferrisoxide-core/Cargo.toml` depends on local `ferrisoxide-measurements`. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-measurements` shows only `ferrisoxide-measurements`. | Pass |

## M6 Completion Dependency Review

The M6 completion branch adds no new third-party crates. Annotated SVG overlays reuse the existing `ferrisoxide-plot` Plotters SVG dependency, criteria DSL work is documentation-only, and measurement validation fixtures use existing workspace test and JSON report paths.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | No new dependency entries in workspace Cargo manifests. | Pass |
| Plotting backend | Evidence overlays reuse existing SVG line/text/marker rendering in `ferrisoxide-plot`. | Pass |
| Runtime surface | No GUI, bitmap, web, DAQ, plugin, RTOS SDK, HAL, or FFI dependency added. | Pass |
| Scope boundary | No parser, plotting, report, file I/O, DAQ, RTOS SDK, HAL, or plugin dependency is added. | Pass |

## M8-001 Rule Schema Dependency Review

The M8-001 rule schema crate adds no new third-party crates. It reuses approved workspace `serde` for schema serialization derives and approved workspace `serde_json` only as a dev-dependency for round-trip tests.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-rule-schema/Cargo.toml` uses only `serde.workspace = true` and `serde_json.workspace = true` under dev-dependencies. | Pass |
| Scope boundary | The crate does not depend on `ferrisoxide-core`, `ferrisoxide-cli`, `ferrisoxide-plot`, CSV parsing, plotting, reports, DAQ, controller I/O, HALs, SDKs, or RTOS tooling. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-rule-schema` is recorded in `docs/m8-001-rule-schema-crate-pipeline-report.md`. | Pass |
| Future work boundary | Validation, export commands, checksums/manifests, shared execution, no_std compatibility, and parity tests remain separate M8 issues. | Pass |

## M8-002 Rule Package Format Dependency Review

The M8-002 package-format branch adds no new third-party crates. It reuses approved workspace `toml` as a dev-dependency in `ferrisoxide-rule-schema` so the checked-in `examples/rule-package/rules.toml` file is parse-tested against the schema.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-rule-schema/Cargo.toml` adds `toml.workspace = true` under dev-dependencies only. | Pass |
| Scope boundary | TOML parsing is used only by schema tests for documentation examples; no runtime parser, export command, file I/O, checksum, binary format, HAL, SDK, DAQ, GUI, or RTOS dependency is added. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-rule-schema` is recorded in `docs/m8-002-rule-package-format-pipeline-report.md`. | Pass |
| Future work boundary | Validator semantics remain #68; export remains #69; checksum/manifest behavior remains #70. | Pass |

## M8-003 Rule Package Validator Dependency Review

The M8-003 validator branch adds no new third-party crates. It promotes approved workspace `serde_json` and `toml` to regular `ferrisoxide-rule-schema` dependencies so public parse helpers can classify invalid JSON/TOML packages before export or execution.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-rule-schema/Cargo.toml` depends on approved `serde`, `serde_json`, and `toml`. | Pass |
| Scope boundary | The validator parses strings and validates in-memory packages only; it does not add file I/O, export commands, checksum algorithms, binary serialization, rule execution, CLI behavior, DAQ, HAL, SDK, GUI, or RTOS integration. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-rule-schema` is recorded in `docs/m8-003-rule-package-validator-pipeline-report.md`. | Pass |
| Future work boundary | Export (#69), manifest/checksum generation (#70), and shared rule execution (#73) were implemented in later M8 PRs; no_std compatibility and parity tests remain separate issues/gates. | Pass |

## M8-004 Rule Package Export Dependency Review

The M8-004 export branch adds no new third-party crates. It adds a local dependency from `ferrisoxide-cli` to `ferrisoxide-rule-schema` and reuses approved workspace `serde_json` to render `rules.json`.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-cli/Cargo.toml` adds local `ferrisoxide-rule-schema` and approved `serde_json.workspace = true`. | Pass |
| Scope boundary | The command writes desktop package artifacts only; it does not add GUI, DAQ, controller SDK, HAL, RTOS production integration, signing, checksum algorithms, binary serialization, hardware qualification, or certification claims. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-cli` is recorded in `docs/m8-004-rule-package-export-pipeline-report.md`. | Pass |
| Future work boundary | Manifest/checksum generation (#70) and shared rule execution (#73) were implemented in later M8 PRs; no_std compatibility remains #72. | Pass |

## M8-005 Rule Package Manifest And Checksum Dependency Review

The M8-005 manifest/checksum branch adds no new third-party crates. It uses a small deterministic `fnv1a64` checksum helper in `ferrisoxide-rule-schema` for artifact drift evidence only.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | No new workspace or crate dependency entries are added. | Pass |
| Algorithm scope | `fnv1a64` is deterministic and dependency-free, but explicitly non-cryptographic. | Pass |
| Security boundary | Manifest/checksum docs and artifact metadata state that checksum evidence is not signing, tamper resistance, security certification, hardware qualification, or flight certification evidence. | Pass |
| Mismatch behavior | `validate_artifact_checksum()` returns structured `ChecksumMismatch` errors for changed artifact contents. | Pass |
| Future work boundary | Binary package serialization, cryptographic signing, runtime loaders, no_std compatibility, and parity tests remain separate issues/gates; shared rule execution is covered by M8-006. | Pass |

## M8-006 Shared Rule Engine Dependency Review

The M8-006 shared rule engine branch adds no new third-party crates. It adds local workspace crate `ferrisoxide-rule-engine`, keeps it dependent only on local `ferrisoxide-measurements`, and wires `ferrisoxide-core` plus embedded-compatible host tests through that shared semantics crate.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-rule-engine/Cargo.toml` depends only on local `ferrisoxide-measurements`; `ferrisoxide-core` adds local `ferrisoxide-rule-engine`; `ferrisoxide-embedded` adds it as a dev-dependency for host tests only. | Pass |
| Scope boundary | The shared engine operates over caller-provided slices and does not add CSV parsing, TOML parsing, plotting, report rendering, file I/O, DAQ/controller I/O, HALs, SDKs, RTOS integration, signing, binary serialization, or certification claims. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-rule-engine` is recorded in `docs/m8-006-shared-rule-engine-pipeline-report.md`. | Pass |
| Future work boundary | no_std rule-engine compatibility remains #72; exact desktop-vs-embedded parity fixtures remain #74. | Pass |

## M8-007 no_std Rule Boundary Dependency Review

The M8-007 no_std boundary branch adds no new third-party crates. It makes `ferrisoxide-rule-engine` a `#![no_std]` crate, keeps its runtime dependency limited to local `ferrisoxide-measurements`, and separates the desktop-style owned evidence API from a borrowed summary API for constrained embedded-compatible callers.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | No new workspace or crate dependency entries are added. | Pass |
| Rule-engine target boundary | `cargo check -p ferrisoxide-rule-engine --target aarch64-unknown-none` passes. | Pass |
| Embedded target boundary | `cargo check -p ferrisoxide-embedded --target aarch64-unknown-none` passes. | Pass |
| Dependency tree | `cargo tree -p ferrisoxide-rule-engine --target aarch64-unknown-none` and `cargo tree -p ferrisoxide-embedded --target aarch64-unknown-none` show only local embedded-compatible crates in the constrained path. | Pass |
| Scope boundary | The no_std engine keeps CSV parsing, TOML parsing, file I/O, plotting, report rendering, DAQ/controller I/O, hardware HALs, target SDKs, RTOS integration, signing, binary package loading, hardware qualification, and certification claims out of scope. | Pass |
| Heap boundary | The full evidence API uses `alloc` for owned desktop result records; `evaluate_borrowed_rule` returns borrowed/static result and error data for basic no-heap pass/fail evaluation where practical. | Pass |

## M8-008 Desktop-vs-Embedded Parity Dependency Review

The M8-008 parity branch adds no new third-party crates. It adds a local dev-dependency from `ferrisoxide-core` tests to `ferrisoxide-rule-schema` so the exact parity fixture can parse the same rule package used by both desktop and embedded-compatible paths.

| Check | Evidence | Result |
|---|---|---|
| Dependency files | `crates/ferrisoxide-core/Cargo.toml` adds only local `ferrisoxide-rule-schema` under dev-dependencies. | Pass |
| Fixture scope | `tests/parity/` contains static CSV, TOML, and expected JSON files only. | Pass |
| Runtime surface | No new runtime dependency, parser dependency, plotting backend, report dependency, HAL, SDK, RTOS integration, DAQ integration, binary package loader, signing, hardware qualification, or certification claim is added. | Pass |
| Validation | `cargo test -p ferrisoxide-core --test rule_parity` exercises the local dev-dependency only in tests. | Pass |

## M43-M48 Native egui GUI Dependency Review

Date: 2026-06-03

The M43-M48 GUI gate adds optional native GUI dependencies for `ferrisoxide-gui --features native`. The default workspace path and CLI remain usable without enabling the GUI feature.

The original planning default named the current `eframe` and `egui_plot` latest versions. Implementation review found that current `eframe 0.34.3` requires Rust 1.92, while this workspace still declares `rust-version = "1.76"`. The accepted implementation therefore uses exact-pinned `0.28.1` egui-family crates rather than silently raising the workspace MSRV.

| Crate | Scope | Purpose | License Expectation | Alternative Considered | Decision |
|---|---|---|---|---|---|
| `eframe = "=0.28.1"` | Optional `ferrisoxide-gui` `native` feature only | Native egui application shell for the M37-M42 desktop workflow. | MIT OR Apache-2.0 family in egui metadata. | Use latest `0.34.3` and raise workspace MSRV; defer GUI; build a custom terminal UI. | Approved because it preserves `rust-version = "1.76"` and keeps native GUI dependencies out of default features. |
| `egui_plot = "=0.28.1"` | Optional `ferrisoxide-gui` `native` feature only | Interactive CSV waveform/result plotting inside the GUI shell. | MIT OR Apache-2.0 family in egui metadata. | Keep SVG-only plotting; custom egui drawing. | Approved because it resolves to the same `egui v0.28.1` as `eframe` and avoids custom plot logic. |

Reviewed sources:

- `eframe` package docs: `https://docs.rs/crate/eframe/latest`
- `egui_plot` package/dependency docs: `https://deps.rs/crate/egui_plot/0.35.0`
- `eframe 0.34.3` source metadata: `https://docs.rs/crate/eframe/0.34.3/source/Cargo.toml`

Resolved dependency evidence:

| Check | Evidence | Result |
|---|---|---|
| Feature isolation | `crates/ferrisoxide-gui/Cargo.toml` keeps `default = []` and enables `eframe` / `egui_plot` only through `native`. | Pass |
| Single egui version | `cargo tree -p ferrisoxide-gui --features native -i egui` resolves one `egui v0.28.1` shared by `eframe`, `egui-winit`, `egui_glow`, and `egui_plot`. | Pass |
| Native dependency surface | `cargo tree -p ferrisoxide-gui --features native --depth 2` shows the native stack under `eframe v0.28.1`, including `egui`, `egui-winit`, `egui_glow`, `glow`, `glutin`, `winit`, `image`, and macOS Objective-C/windowing crates. | Pass |
| Architecture boundary | `ferrisoxide-gui` depends on local `ferrisoxide-workflow`; core signal, rule-engine, embedded, DAQ abstraction, simulator, and CLI crates do not depend on egui. | Pass |
| CI boundary | Existing Ubuntu `rust` job remains unchanged; new `gui-macos` job checks the native feature explicitly. | Pass locally; protected CI pending PR run |

Gate Decision:

- Gate: Dependency Gate.
- Decision: Pass for optional native GUI dependency use.
- Reason: The user explicitly approved the GUI milestone gate and egui plan. Exact `0.28.1` pins preserve the workspace MSRV, resolve one egui version, and isolate native GUI dependencies to one optional crate/feature.
- Residual risk: The native GUI transitive dependency surface is substantially larger than the CLI path, and license/advisory automation is still not present.
- Next owner: Security Engineer / Core Software Engineer.

## M49-M50 And Run/Config GUI File Dialog Dependency Review

Date: 2026-06-03

The M49-M50 Source-page UX refinement added a native CSV file selector to `ferrisoxide-gui --features native`. Later Config-page and Run-page refinements reuse the same optional `rfd` dependency for TOML open/save dialogs and output-directory folder selection. The dependency remains optional and is not enabled for the default workspace, CLI, core signal-processing crates, embedded-compatible crates, DAQ abstractions, simulator, controller I/O, or rule-package paths.

User gate:

- Gate: Human Approval / Dependency Gate.
- Decision: Pass.
- Evidence: The user requested native selectors for Source CSV input, Config TOML open/save paths, and Run output directories, and pre-approved human gates for this implementation.
- Scope limit: File dialogs are limited to local CSV selection, TOML config open/save selection, and output-directory selection in the optional native GUI. The change does not add packaging, signing, notarization, live DAQ, hardware channel discovery, SDKs, drivers, runtime loaders, release publication, or certification evidence.

| Crate | Scope | Purpose | License Expectation | Alternative Considered | Decision |
|---|---|---|---|---|---|
| `rfd = "=0.14.1"` | Optional `ferrisoxide-gui` `native` feature only | Native file/folder dialogs for CSV input selection, TOML config open/save paths, and evaluation output directory selection. | MIT, per `rfd 0.14.1` package metadata. | Keep manual text entry; build custom platform-specific pickers; use latest `rfd 0.17.2` without MSRV review. | Approved because it supplies the requested native selectors, stays behind `native`, and the exact `0.14.1` pin avoids silently adopting the latest transitive surface. |

Reviewed sources:

- `rfd` latest docs and API example: `https://docs.rs/rfd/latest/rfd/`
- `rfd 0.14.1` source metadata: `https://docs.rs/crate/rfd/0.14.1/source/Cargo.toml.orig`

Resolved dependency evidence:

| Check | Evidence | Result |
|---|---|---|
| Feature isolation | `crates/ferrisoxide-gui/Cargo.toml` keeps `default = []` and enables `rfd` only through `native`. | Pass |
| Pin and lockfile | Workspace manifest pins `rfd = "=0.14.1"` and `Cargo.lock` records `rfd v0.14.1`. | Pass |
| Native dependency surface | `cargo check -p ferrisoxide-gui --features native` fetched and compiled `rfd v0.14.1` plus macOS Objective-C support crates `block`, `malloc_buf`, `objc`, `objc-foundation`, and `objc_id`. | Pass |
| Architecture boundary | `rfd` is used only from `crates/ferrisoxide-gui/src/native.rs`; the workflow API owns CSV header loading and all analysis behavior remains shared with the CLI. | Pass |

Gate Decision:

- Gate: Dependency Gate.
- Decision: Pass for optional native GUI file/folder selection.
- Reason: The user explicitly requested picker controls and pre-approved the gate; `rfd` is constrained to the optional native GUI feature and does not affect default CLI/workspace behavior.
- Residual risk: File dialogs increase native GUI platform surface, and Linux/Windows runtime behavior has not been visually validated in this macOS-first slice.
- Next owner: Security Engineer / Core Software Engineer.

## Risk Assessment

- Supply-chain risk: Medium; dependencies are common Rust ecosystem crates, but exact transitive dependencies must remain visible in `Cargo.lock`.
- License risk: Low/Medium; confirm resolved crate license metadata during release readiness review.
- Maintenance risk: Low/Medium; these crates are widely used and reduce custom parser surface.
- Security risk: Medium; malformed input parsing expands attack surface and needs tests.
- Plotting risk: Medium; SVG output is local-file only, and the M43-M50 native GUI dependency is optional but expands the desktop dependency surface when enabled.
- Embedded toolchain risk: Medium; future RTOS SDKs, HALs, FFI, or target CI require fresh review before adoption.
- Measurement extraction risk: Medium; evidence values and tie behavior must remain guarded by exact golden reports.
- Rule package drift risk: Medium; the schema crate, parse-tested examples, validator, export command, manifest/checksum evidence, shared rule engine, no_std boundary, and exact parity fixtures reduce duplicated shapes and semantics, but runtime package loading and hardware-target execution remain future work before runtime claims.

## Gate Decision

- Gate: Dependency Gate.
- Decision: Pass.
- Reason: User approved adding dependencies; the selected crates directly support tracked requirements and avoid hand-rolled structured parsing. M5 Plotters usage is constrained to an isolated plotting crate and SVG line rendering. M43-M50 native GUI dependencies are optional and isolated to the `ferrisoxide-gui` native feature. M3 RTOS follow-up, M6 measurement-engine work, M6 completion work, M8-001 rule-schema work, M8-002 package-format work, M8-003 validator work, M8-004 export work, M8-005 manifest/checksum work, M8-006 shared-rule work, M8-007 no_std boundary work, and M8-008 parity fixture work add no new third-party dependencies.
- Residual risk: Dependency license and advisory scanning is not automated yet.
- Next owner: Core Software Engineer.

## Hand-Off Note

Role: Security Engineer / Software Architect
Goal: Approve minimal dependencies for config, CSV, and report upgrades.
Files changed: `docs/dependency-review.md`
Checks run: Not applicable; implementation follows.
Status: Pass.
Known gaps: License metadata should be rechecked before release publication.
Next recommended step: Add dependencies to Cargo manifests, implement config parsing and JSON reports, then run Cargo validation.
