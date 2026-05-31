# ADR-006: FerrisOxide Signal Identity Adoption

Date: 2026-05-31

Status: Accepted and implemented for in-repository identifiers through BRAND-002 / issue #98 and PR #99. Amended by ADR-007 / REPO-001: the main repository host is `kota-wilson/ferrisoxide`, while `ferrisoxide-signal` remains the signal-analysis crate and CLI binary. GitHub organization creation, crates.io publishing or reservation, domain registration, trademark clearance, and logo work remain separate gates.

## Context

ADR-005 documented FerrisOxide as a proposal-stage umbrella brand and intentionally deferred public identifier changes. The maintainer later approved continuing with a thorough rename so the repository identity matches the product direction before broader external users depend on the old names.

The project is still pre-1.0 and is primarily maintained through GitHub PRs, Cargo workspace crates, CLI examples, validation fixtures, and documentation. That makes this the least disruptive point to align source-level identifiers.

## Decision

Adopt `FerrisOxide Signal` as the in-repository product identity.

Rename in-repository identifiers as follows:

| Old Identifier Class | New Identifier |
|---|---|
| Product name | `FerrisOxide Signal` |
| Repository target name | `ferrisoxide` |
| CLI binary | `ferrisoxide-signal` |
| Core crate | `ferrisoxide-core` |
| CLI crate | `ferrisoxide-cli` |
| Embedded adapter crate | `ferrisoxide-embedded` |
| Measurement crate | `ferrisoxide-measurements` |
| Plotting crate | `ferrisoxide-plot` |
| Signal primitive crate | `ferrisoxide-signal` |
| Future crate prefix | `ferrisoxide-*` |

Stable historical traceability identifiers such as `WRA-RQ-*`, `WRA-RISK-*`, and prior issue numbers are not renamed. They remain audit IDs, not product branding.

## Rationale

- The project roadmap now covers signal analysis, validation evidence, plotting, portable rule packages, embedded runtimes, controller simulation, and deployment-package planning.
- `FerrisOxide Signal` is broad enough for the current signal-validation center of gravity without claiming the whole future platform is implemented.
- A single visible identity across README, Cargo packages, CLI examples, and docs reduces user confusion.
- The project is early enough that a direct rename is lower risk than supporting long-lived dual branding.

## Compatibility Position

This rename does not add a legacy `wra` binary alias. The project has no formal stable release yet, so retaining both command names would create avoidable dual identity in examples, tests, and support material. The main repository is `ferrisoxide`; users should run the signal-analysis CLI binary as:

```text
ferrisoxide-signal
```

## External Boundaries

This decision does not claim:

- GitHub organization ownership for `FerrisOxide`.
- crates.io package reservation or publishing.
- domain availability.
- trademark clearance.
- logo or visual-identity approval.
- affiliation with the Rust Project, Rust Foundation, Rust language maintainers, or Rust trademark owners.
- certified aerospace, hardware qualification, or production-controller readiness.

## Verification

The rename is verified by:

- Cargo metadata showing `ferrisoxide-*` workspace packages and `ferrisoxide-signal` binary target.
- Workspace formatting, tests, clippy, QEMU-demo host test, CLI analyze smoke, CLI plot smoke, benchmark helper smoke, and whitespace checks.
- Documentation scan for unintended old public identifiers, with stable `WRA-*` audit IDs allowed.
- GitHub repository and local remote verification showing `kota-wilson/ferrisoxide`.
- GitHub PR review and protected-branch CI before merge.

## Consequences

Positive:

- Public-facing source identity now matches the approved product direction.
- Crate names, binary names, examples, and docs use one vocabulary.
- Future FerrisOxide product-family modules have a consistent naming pattern.

Negative:

- Existing local scripts using the old binary name must be updated.
- Historical reports and issue references may still include old names as audit evidence.
- External namespace and legal checks are still open risks.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist
Goal: Adopt FerrisOxide Signal as the signal-analysis identity while preserving audit traceability and clarifying that the main repository is FerrisOxide.
Files changed: `decisions/ADR-006-ferrisoxide-signal-identity-adoption.md`.
Checks run: `cargo metadata --format-version 1 --no-deps`; `cargo clean`; `cargo fmt --check`; `cargo test --workspace`; `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`; `cargo clippy --workspace --all-targets -- -D warnings`; CLI analyze smoke; CLI plot smoke; SVG overlay scan; benchmark smoke; `git diff --check`; identifier scan.
Status: Accepted and implemented through issue #98 / PR #99; repository-host scope amended by ADR-007 / REPO-001.
Known gaps: External organization, domain, crates.io, trademark, visual identity, and legal-suitability checks remain separate gates.
Next recommended step: Complete REPO-001 release evidence, then return to M8 shared rule-engine work.
