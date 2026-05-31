# ADR-005: FerrisOxide Brand Architecture

Date: 2026-05-31

Status: Proposed and merged through PR #96

## Context

The repository began as Waveform Reconstructor and Analyzer, a Rust CLI/core library for CSV waveform loading, reconstruction, filters, pass/fail criteria, TOML config, and reports.

The roadmap now includes:

- signal analysis
- qualification testing
- controller simulation
- rule authoring
- deployment packages
- embedded/runtime targets
- DAQ abstractions
- production/test config separation

The original descriptive name remains accurate for the initial implementation, but it may become restrictive as the project becomes a broader engineering signal validation and controller deployment platform.

## Decision

Use FerrisOxide as the preferred umbrella brand candidate in planning documents.

Do not rename the repository, crates, binary, package names, domain, or GitHub organization as part of this decision.

Recommended future product mapping:

- FerrisOxide Signal: signal analysis, waveform validation, criteria evaluation, plotting, and reports.
- FerrisOxide Runtime: embedded and controller runtime execution.
- FerrisOxide DAQ: DAQ abstraction and acquisition adapters.
- FerrisOxide Control: controller simulation, production control config, and I/O abstractions.
- FerrisOxide Desktop: desktop authoring, visualization, simulation, and export workflows.

## Rationale

FerrisOxide is:

- short
- memorable
- Rust-adjacent
- broad enough for the long-term platform direction
- less restrictive than a waveform-specific name

The project should not adopt the name externally until availability, legal suitability, and migration impact are reviewed.

## Consequences

Positive:

- Creates a broader umbrella for future products.
- Allows the current repo to evolve without forcing all future concepts into a waveform-only name.
- Gives future crate/repository naming a coherent prefix.

Negative:

- May create confusion if introduced before a formal rename.
- The `Ferris` reference may be read as Rust-affiliated if documentation is careless.
- Availability and trademark suitability are unknown.
- Repo and crate migration could disrupt links, scripts, docs, and package consumers.

## Guardrails

- Keep current repository and crate names until a separate adoption gate.
- Do not claim GitHub organization, crates.io, domain, or trademark availability.
- Do not imply affiliation with the Rust Project, Rust Foundation, or Rust language maintainers.
- Prefer `FerrisOxide Signal` as the first product name if the current repository is renamed later.
- Keep `wra-*` crate names stable until a crate migration plan exists.

## Verification

For this ADR:

- Documentation review verifies the proposal is clear and does not perform a rename.
- Traceability records the decision and the deferred adoption gates.

For future adoption:

- GitHub organization and repository availability check.
- crates.io package-name availability check.
- domain availability check.
- trademark/legal suitability check.
- README, CI, docs, issue templates, and project memory migration check.
- user-impact and redirect review.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist
Goal: Decide how to represent FerrisOxide in planning without changing public identifiers.
Files changed: `decisions/ADR-005-ferrisoxide-brand-architecture.md`.
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Proposed and merged through PR #96.
Known gaps: External availability and legal suitability checks are not complete.
Next recommended step: Use `docs/brand-architecture.md` as the naming reference until a separate adoption issue is approved.
