# FerrisOxide Brand Architecture

Date: 2026-05-31

Status: Proposed. No repository, crate, binary, organization, domain, or package rename has been performed.

## Summary

FerrisOxide is the preferred umbrella brand candidate for the broader platform direction.

The current repository name, Waveform Reconstructor and Analyzer, describes the original MVP. It is still accurate for the current CLI/core scope, but it is becoming narrow as the roadmap expands into signal analysis, qualification testing, controller simulation, rule authoring, embedded deployment, DAQ abstraction, and runtime targets.

The brand decision should separate:

- umbrella identity
- product names
- repository names
- crate names
- binaries
- package names
- domains
- documentation language

This avoids a disruptive rename before external availability, trademark, domain, crates.io, and migration checks are complete.

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

Important caveat: availability and legal suitability have not been verified. Before adoption, perform GitHub organization, domain, crates.io, trademark, and Rust-affiliation review.

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

Current crate names such as `wra-core`, `wra-cli`, `wra-signal`, `wra-embedded`, and `wra-plot` should remain unchanged until a formal migration plan exists.

Candidate future crate prefix:

```text
ferrisoxide-*
```

Candidate future crates or repositories:

| Candidate | Role |
|---|---|
| `ferrisoxide-signal` | Main signal-analysis product and likely successor name for this repository if the repo remains focused on signal validation. |
| `ferrisoxide-runtime` | Shared embedded/controller runtime components. |
| `ferrisoxide-daq` | DAQ abstraction and adapters after dependency and environment gates. |
| `ferrisoxide-control` | Controller config, simulation, and I/O abstraction. |
| `ferrisoxide-docs` | Public specifications, roadmap, validation reports, and platform docs. |

Avoid using `ferrisoxide-platform` as the first rename unless the repository has already become a multi-product integration repository. It is broader than the current implementation and may overstate maturity.

## Recommended Migration Path

### Phase 0: Proposal Only

Current phase.

- Document FerrisOxide as a proposed umbrella brand.
- Keep the current repository URL.
- Keep all crate names.
- Keep the `wra` binary.
- Keep README title unchanged.
- Do not claim domain, trademark, or organization availability.

### Phase 1: Availability And Suitability Review

Required before adoption:

- Check GitHub organization availability.
- Check repository-name availability.
- Check crates.io package-name availability.
- Check domain availability.
- Check trademark risk.
- Check whether Rust/Ferris references could imply affiliation with the Rust Project, Rust Foundation, or Rust language maintainers.
- Record findings in a new decision document.

### Phase 2: Public Naming Transition

Only after approval:

- Add README language such as `FerrisOxide Signal, currently hosted as waveform-reconstructor-analyzer`.
- Keep old name visible for searchability.
- Add changelog entry.
- Add issue/PR template note for the transition.
- Keep CLI and crate names stable.

### Phase 3: Repository Rename

Only after explicit approval:

- Rename repository to the approved target, likely `ferrisoxide-signal`.
- Verify GitHub redirects.
- Update README badges, clone URLs, docs, examples, CI references, issue templates, and project memory.
- Keep compatibility language for the old name.
- Avoid crate renames in the same PR.

### Phase 4: Crate And Binary Migration

Only after external user impact is understood:

- Decide whether `wra-*` crate names should remain as internal implementation names.
- If renamed, publish or reserve `ferrisoxide-*` crates only after crates.io and compatibility review.
- Keep deprecated aliases or migration guidance where practical.
- Avoid breaking CLI scripts without a release plan.

## Naming Recommendation

Best near-term public positioning:

```text
FerrisOxide Signal
```

Reason: it preserves the existing signal-analysis center of gravity while leaving room for runtime, DAQ, control, and desktop products.

Best eventual organization structure:

```text
FerrisOxide/
  ferrisoxide-signal
  ferrisoxide-runtime
  ferrisoxide-daq
  ferrisoxide-control
  ferrisoxide-docs
```

## Non-Goals

- No immediate repository rename.
- No immediate GitHub organization creation.
- No immediate crate rename.
- No immediate binary rename.
- No logo or mascot work.
- No domain registration.
- No trademark claim.
- No Rust Project, Rust Foundation, or Rust language affiliation claim.

## Approval Gates Before Adoption

Required gates:

- Product naming approval.
- Maintainer approval.
- Trademark/legal suitability review.
- Dependency/package namespace review for crates.io.
- Documentation migration review.
- Release and community communication plan.
- Backward-compatibility review for users, scripts, links, and package names.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist
Goal: Capture FerrisOxide as a proposed umbrella brand without performing a rename.
Files changed: `docs/brand-architecture.md`.
Checks run: `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Proposal validated locally; PR release gate pending.
Known gaps: GitHub organization availability, domain availability, crates.io availability, trademark/legal suitability, Rust-affiliation risk, logo/assets, and migration timeline are not verified.
Next recommended step: Keep current repository identity until a formal naming adoption issue and approval gate are opened.
