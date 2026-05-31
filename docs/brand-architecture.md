# FerrisOxide Brand Architecture

Date: 2026-05-31

Status: Adopted for in-repository identity through BRAND-002 / issue #98 and PR #99, then amended by REPO-001 / issue #111 to use `kota-wilson/ferrisoxide` as the main repository host. `ferrisoxide-signal` remains the signal-analysis crate and CLI identity. External organization, domain, crates.io publishing or reservation, trademark, legal-suitability, and logo work remain separate gates.

## Summary

FerrisOxide is the preferred umbrella brand for the broader platform direction and the correct main repository name.

The current signal-analysis product slice is FerrisOxide Signal. It is accurate for the current CLI/core scope and leaves room for the main repository to expand into signal analysis, qualification testing, controller simulation, rule authoring, embedded deployment, DAQ abstraction, and runtime targets.

The brand decision should separate:

- umbrella identity
- product names
- repository names
- crate names
- binaries
- package names
- domains
- documentation language

This keeps the source repository aligned with the product direction while keeping external availability, trademark, domain, crates.io, and migration checks explicit.

## Recommended Umbrella

Preferred umbrella:

```text
FerrisOxide
```

Rationale:

- Short.
- Memorable.
- Rust-adjacent.
- Broad enough for signal, control, DAQ, desktop, embedded, and runtime work.
- More durable than a narrow waveform-specific name.

Important caveat: source-level adoption does not verify legal or package-namespace suitability. Before organization creation, crate publication, domain registration, or broader marketing, perform GitHub organization, domain, crates.io, trademark, and Rust-affiliation review.

## Product Family

Recommended public product names:

| Product | Purpose |
|---|---|
| FerrisOxide Signal | Signal analysis, waveform validation, criteria evaluation, plotting, and reports. |
| FerrisOxide Runtime | Embedded and controller runtime execution of approved rules/configs. |
| FerrisOxide DAQ | DAQ abstraction, acquisition adapters, and fixture/test-double input. |
| FerrisOxide Control | Production control config, state-machine logic, controller I/O abstractions, and simulation. |
| FerrisOxide Desktop | Future desktop authoring, simulation, visualization, and deployment package workflow. |
| FerrisOxide Docs | Cross-repository specifications, validation evidence, architecture, and user guides. |

## Crate And Repository Direction

Current crate names use the FerrisOxide family identity:

```text
ferrisoxide-*
```

Current implemented crates:

| Crate | Role |
|---|---|
| `ferrisoxide-core` | Waveform model, CSV parsing, config, transforms, criteria, report model. |
| `ferrisoxide-cli` | Desktop command-line entry point and orchestration. |
| `ferrisoxide-embedded` | no_std adapter boundary for future embedded runtimes. |
| `ferrisoxide-measurements` | no_std measurement primitives used by criteria evidence. |
| `ferrisoxide-plot` | Desktop SVG plotting support. |
| `ferrisoxide-signal` | no_std signal primitives. |

Candidate future crates or repositories:

| Candidate | Role |
|---|---|
| `ferrisoxide` | Main multi-crate repository for the platform workspace. |
| `ferrisoxide-signal` | Signal-analysis crate, CLI binary, and product slice. |
| `ferrisoxide-runtime` | Shared embedded/controller runtime components. |
| `ferrisoxide-daq` | DAQ abstraction and adapters after dependency and environment gates. |
| `ferrisoxide-control` | Controller config, simulation, and I/O abstraction. |
| `ferrisoxide-docs` | Public specifications, roadmap, validation reports, and platform docs. |

Avoid using `ferrisoxide-platform` as the first rename unless the repository has already become a multi-product integration repository. It is broader than the current implementation and may overstate maturity.

## Recommended Migration Path

### Phase 0: Proposal Only

Complete through BRAND-001 / PR #96.

- Document FerrisOxide as a proposed umbrella brand.
- Keep the then-current repository URL.
- Keep then-current crate names.
- Keep the then-current CLI binary.
- Keep README title unchanged.
- Do not claim domain, trademark, or organization availability.

### Phase 1: Availability And Suitability Review

Required before external brand expansion:

- Check GitHub organization availability.
- Check repository-name availability.
- Check crates.io package-name availability.
- Check domain availability.
- Check trademark risk.
- Check whether Rust/Ferris references could imply affiliation with the Rust Project, Rust Foundation, or Rust language maintainers.
- Record findings in a new decision document.

### Phase 2: In-Repository Naming Transition

Current BRAND-002 scope:

- Rename Cargo workspace packages to `ferrisoxide-*`.
- Rename the CLI binary to `ferrisoxide-signal`.
- Update README, docs, validation fixtures, examples, and scripts.
- Add changelog entry.
- Keep old `WRA-*` requirement and risk IDs as stable audit identifiers.
- Do not publish crates or claim external namespaces.

### Phase 3: Repository Rename

After protected-branch PR merge and explicit maintainer approval:

- Rename repository host to the approved main repository target, `ferrisoxide`.
- Verify GitHub redirects.
- Update README badges, clone URLs, docs, examples, CI references, issue templates, and project memory.
- Keep compatibility language for the old name.

### Phase 4: External Package Migration

Only after external user impact and namespace checks are understood:

- Publish or reserve `ferrisoxide-*` crates only after crates.io, license, security, and compatibility review.
- Keep deprecated aliases or migration guidance where practical.
- Avoid breaking released CLI scripts without a release plan.

## Naming Recommendation

Best near-term public positioning:

```text
FerrisOxide Signal
```

Reason: it preserves the existing signal-analysis center of gravity while leaving room for runtime, DAQ, control, and desktop products.

Best eventual organization structure:

```text
FerrisOxide/
  ferrisoxide/
    crates/ferrisoxide-signal
    crates/ferrisoxide-runtime
    crates/ferrisoxide-daq
    crates/ferrisoxide-control
  ferrisoxide-docs
```

## Non-Goals

- No immediate GitHub organization creation.
- No logo or mascot work.
- No domain registration.
- No trademark claim.
- No Rust Project, Rust Foundation, or Rust language affiliation claim.

## External Gates After Source Adoption

Required before organization creation, crate publication, domain registration, or broader public marketing:

- Product naming approval.
- Maintainer approval.
- Trademark/legal suitability review.
- Dependency/package namespace review for crates.io.
- Documentation migration review.
- Release and community communication plan.
- Backward-compatibility review for users, scripts, links, and package names.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist
Goal: Capture FerrisOxide as the adopted in-repository identity and retain external namespace gates.
Files changed: `docs/brand-architecture.md`.
Checks run: `cargo metadata --format-version 1 --no-deps`; `cargo clean`; `cargo fmt --check`; `cargo test --workspace`; `cargo test --manifest-path embedded/arm64/qemu/Cargo.toml`; `cargo clippy --workspace --all-targets -- -D warnings`; CLI analyze smoke; CLI plot smoke; SVG overlay scan; benchmark smoke; `git diff --check`; identifier scan.
Status: In-repository adoption implemented through issue #98 / PR #99; main repository host corrected through REPO-001 / issue #111 to `kota-wilson/ferrisoxide`.
Known gaps: GitHub organization availability, domain availability, crates.io availability, trademark/legal suitability, Rust-affiliation risk, logo/assets, external package migration, and broad public communication are not verified.
Next recommended step: Complete REPO-001 release evidence, then return to M8 shared rule-engine work.
