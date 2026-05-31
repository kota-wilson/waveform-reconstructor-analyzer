# ADR-007: FerrisOxide Repository Host Name

Date: 2026-05-31

Status: Accepted and implemented locally for REPO-001 / issue #111.

## Context

ADR-006 adopted FerrisOxide Signal for in-repository product identifiers and renamed the GitHub host to `kota-wilson/ferrisoxide-signal`. The maintainer clarified that this conflates the main repository with a crate: `ferrisoxide` is the main repository, while `ferrisoxide-signal` is a crate and CLI binary inside that repository.

The repository is still pre-1.0 and controlled through protected-branch PRs. This is a low-disruption point to correct the repository host before more downstream packages, controller-runtime modules, and external users depend on the wrong name.

## Decision

Use `ferrisoxide` as the main repository name.

Current naming contract:

| Identifier | Decision |
|---|---|
| GitHub repository | `kota-wilson/ferrisoxide` |
| Local project folder | `/Users/kota/Desktop/softwareai/projects/ferrisoxide` |
| Workspace repository metadata | `https://github.com/kota-wilson/ferrisoxide` |
| Product workspace | FerrisOxide |
| Current signal-analysis slice | FerrisOxide Signal |
| Signal primitive crate | `ferrisoxide-signal` |
| CLI binary | `ferrisoxide-signal` |
| Future crate prefix | `ferrisoxide-*` |

Historical PR links, issue text, validation logs, and pipeline reports may retain old repository URLs when they are audit evidence from the time they were produced. GitHub redirects are relied on for those historical links.

## Rationale

- `ferrisoxide` matches the umbrella workspace and avoids treating the signal crate as the whole repository.
- Keeping `ferrisoxide-signal` as the crate and CLI binary preserves accurate current product scope.
- The split leaves room for future crates such as runtime, DAQ, control, and deployment tooling without another repository-name correction.
- Updating current metadata and project memory reduces confusion for contributors while preserving historical audit records.

## Alternatives Considered

| Alternative | Reason Rejected |
|---|---|
| Keep `ferrisoxide-signal` as the repository host | Confuses the main repository with one crate and makes future platform modules look subordinate to the signal crate. |
| Rename every crate and CLI binary to `ferrisoxide` | Over-broad and less precise; the current shipped CLI is specifically the signal-analysis workflow. |
| Rewrite all historical PR and validation links | Weakens audit traceability and adds noisy churn; GitHub redirects make historical references usable. |

## Non-Goals

- No GitHub organization creation.
- No crates.io publication or reservation.
- No domain registration.
- No trademark or legal-suitability claim.
- No logo or visual identity work.
- No CLI binary or crate rename.
- No rewrite of historical PR evidence.

## Verification

The decision is verified by:

- `gh repo view kota-wilson/ferrisoxide --json nameWithOwner,url`.
- `git remote -v` showing `https://github.com/kota-wilson/ferrisoxide.git`.
- Cargo metadata showing workspace repository metadata set to `https://github.com/kota-wilson/ferrisoxide`.
- Documentation scan of current identity files showing `ferrisoxide` for the main repository and `ferrisoxide-signal` only for crate, CLI, or historical evidence references.
- Workspace formatting, tests, clippy, and diff checks before protected-branch merge.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist
Goal: Correct the main repository host to FerrisOxide while keeping FerrisOxide Signal as the current signal-analysis crate and CLI identity.
Files changed: `decisions/ADR-007-repository-host-ferrisoxide.md`.
Checks run: `gh repo view kota-wilson/ferrisoxide --json nameWithOwner,url`; `git remote -v`; `cargo metadata --format-version 1 --no-deps`; current-doc identity scan; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Accepted and validated locally; protected-branch PR, CI, and merge pending.
Known gaps: External organization, domain, crates.io, trademark, visual identity, and legal-suitability checks remain separate gates.
Next recommended step: Complete REPO-001 validation and PR, then resume M8 shared rule-engine issues.
