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

## Risk Assessment

- Supply-chain risk: Medium; dependencies are common Rust ecosystem crates, but exact transitive dependencies must remain visible in `Cargo.lock`.
- License risk: Low/Medium; confirm resolved crate license metadata during release readiness review.
- Maintenance risk: Low/Medium; these crates are widely used and reduce custom parser surface.
- Security risk: Medium; malformed input parsing expands attack surface and needs tests.
- Plotting risk: Low/Medium; SVG output is local-file only, but future plotting backends could expand native or GUI dependencies if not gated.
- Embedded toolchain risk: Medium; future RTOS SDKs, HALs, FFI, or target CI require fresh review before adoption.
- Measurement extraction risk: Medium; evidence values and tie behavior must remain guarded by exact golden reports.

## Gate Decision

- Gate: Dependency Gate.
- Decision: Pass.
- Reason: User approved adding dependencies; the selected crates directly support tracked requirements and avoid hand-rolled structured parsing. M5 Plotters usage is constrained to an isolated plotting crate and SVG line rendering. M3 RTOS follow-up, M6 measurement-engine work, and M6 completion work add no third-party dependencies.
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
