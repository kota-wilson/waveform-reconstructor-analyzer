# Dependency Review

Date: 2026-05-31

Project: Waveform Reconstructor and Analyzer

Stage: Dependency Gate

Owner Role: Security Engineer / Software Architect

## Approval

The user approved proceeding through the dependency, license, and publication gates after the dependency-free MVP validation.

Current status: Approved dependencies were added and pinned in `Cargo.lock`; the public repository publication gate later passed. Any new dependency still requires a fresh dependency review.

## Proposed Dependencies

| Crate | Scope | Purpose | License Expectation | Alternative Considered | Decision |
|---|---|---|---|---|---|
| `csv` | `wra-core` | Robust header-based CSV parsing and records handling. | MIT / Unlicense family in the Rust ecosystem. | Continue hand-written parser. | Approved because CSV dialect risk is already tracked. |
| `serde` | `wra-core` | Derive stable config and report data structures. | MIT / Apache-2.0 family. | Manual parsing/serialization. | Approved because structured data is central to config and reports. |
| `serde_json` | `wra-core` | JSON report rendering for automation. | MIT / Apache-2.0 family. | Manual JSON strings. | Approved because manual JSON is error-prone. |
| `toml` | `wra-cli` | Parse user-facing analysis config files. | MIT / Apache-2.0 family. | Keep CLI-only criteria. | Approved because the project already defines TOML config examples. |

## Risk Assessment

- Supply-chain risk: Medium; dependencies are common Rust ecosystem crates, but exact transitive dependencies must remain visible in `Cargo.lock`.
- License risk: Low/Medium; confirm resolved crate license metadata during release readiness review.
- Maintenance risk: Low/Medium; these crates are widely used and reduce custom parser surface.
- Security risk: Medium; malformed input parsing expands attack surface and needs tests.

## Gate Decision

- Gate: Dependency Gate.
- Decision: Pass.
- Reason: User approved adding dependencies; the selected crates directly support tracked requirements and avoid hand-rolled structured parsing.
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
