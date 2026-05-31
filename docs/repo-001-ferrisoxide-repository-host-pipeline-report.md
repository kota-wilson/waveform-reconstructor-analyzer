# REPO-001 FerrisOxide Repository Host Pipeline Report

Date: 2026-05-31

Repository: `kota-wilson/ferrisoxide`

Branch: `chore/repo-name-ferrisoxide`

Issue: #111, `REPO-001 Correct main repository name to ferrisoxide`

Requirement: WRA-RQ-065

Owner Roles: Product Architect / GitHub Maintainer Specialist / Core Software Engineer / Verification and Validation Engineer

## Objective

Correct the main repository identity from `ferrisoxide-signal` to `ferrisoxide` while keeping `ferrisoxide-signal` as the signal-analysis crate and CLI binary.

## Scope Boundaries

In scope:

- GitHub repository host verification for `kota-wilson/ferrisoxide`.
- Local `origin` remote verification for `https://github.com/kota-wilson/ferrisoxide.git`.
- Workspace metadata repository URL update.
- Current README, environment, project memory, architecture, requirements, traceability, and risk documentation updates.
- ADR amendment that separates repository host identity from crate and CLI identity.

Out of scope:

- Renaming the `ferrisoxide-signal` crate or CLI binary.
- Rewriting historical PR links, issue history, validation logs, or audit evidence.
- Creating a GitHub organization.
- crates.io publishing or reservation.
- Domain registration, logo, trademark, Rust-affiliation, or legal-suitability claims.
- Any GUI, DAQ, embedded runtime, controller, or certification scope expansion.

## Stage Log

| Stage | Gate | Decision | Artifact / Evidence | Residual Risk | Next Owner |
|---|---|---|---|---|---|
| Intake | Intake Gate | Pass | Maintainer request clarified that `ferrisoxide` is the main repository and `ferrisoxide-signal` is a crate. | None for issue selection. | Project Coordinator |
| Project Creation | Project Creation Gate | Not Applicable | Existing FerrisOxide repository and project package already exist. | No new project package needed. | Project Coordinator |
| Project Orchestration | Orchestration Gate | Pass | Issue #111 created to make the repository-name correction auditable before continuing M8. | M8 issues remain paused until this naming correction merges. | Project Orchestrator |
| Research | Research Gate | Pass | Reviewed current repository metadata, README, ADR-006, brand architecture, project state, requirements, traceability, risk register, and remote URL. | Historical links still mention old host as audit evidence. | Product Architect |
| Requirements | Requirements Gate | Pass | WRA-RQ-065 added to `requirements.md` and `traceability-matrix.md`. | Requirement remains local until PR/CI/merge. | Verification and Validation Engineer |
| Architecture | Architecture Gate | Pass | ADR-007 records `ferrisoxide` as repository host and preserves `ferrisoxide-signal` as crate/CLI identity. | Future organization/crates.io/domain names still need separate gates. | Software Architect |
| Abstraction Review | Granularity Gate | Pass | File-level scope is explicit: metadata/docs/project-memory updates only; no crate or CLI rename. | Future brand changes need their own issue. | Abstraction Review Engineer |
| Approval | Human Approval Gate | Pass | User explicitly approved the repository-name correction and PR pipeline. | No new dependency or architecture-expansion approval required. | Project Coordinator |
| Dependency | Dependency Gate | Not Applicable | No dependency, lockfile, package manager, SDK, HAL, or target-toolchain changes. | None. | Security Engineer |
| Implementation | Implementation Gate | Pass locally | `Cargo.toml`, README, ADRs, brand docs, project state, requirements, traceability, risk, environment, charter, and orchestration files updated. | Historical evidence intentionally retains old URLs where appropriate. | Core Software Engineer |
| Testing | Testing Gate | Pass locally | Repo view, remote URL, Cargo metadata, current-doc scan, fmt, workspace tests, clippy, and diff check passed. | GitHub CI pending until PR. | Test Automation Engineer |
| V&V | V&V Gate | Pass locally | Acceptance criteria were verified against issue #111 and WRA-RQ-065; `ferrisoxide-signal` remains limited to crate/CLI/signal-slice references. | Protected-branch CI still required. | Verification and Validation Engineer |
| QA | QA Gate | Pass locally | Current docs clarify repository vs crate identity and avoid CLI example churn. | Readers may still encounter old host in historical reports. | QA Engineer |
| Security | Security Gate | Pass | No credentials, secrets, permissions, dependencies, network code, or signing behavior changed. | Repository rename relies on GitHub redirect behavior for historical links. | Security Engineer |
| Performance | Performance Gate | Not Applicable | Metadata and documentation change only; no runtime hot path. | None. | Performance Engineer |
| Documentation | Documentation Gate | Pass locally | README, brand architecture, ADR-006, ADR-007, requirements, traceability, risk, project state, environment, charter, and orchestration docs updated. | Historical docs may still mention the old host as time-bound evidence. | Documentation Engineer |
| Code Review | Code Review Gate | Pass locally | Local review confirmed no accidental crate/CLI rename and no broad historical evidence rewrite. | External review occurs through protected PR. | Code Reviewer |
| Evaluation | Evaluation Gate | Pass locally | Issue #111 acceptance criteria are mapped and local validation passed. | Release readiness depends on PR and required CI. | Evaluation Engineer |
| Release | Release Gate | Blocked until PR | Branch must pass local validation, then PR must pass required `rust` CI. | GitHub checks may find environment-specific issues. | GitHub Maintainer Specialist |
| Community | Community Gate | Blocked until PR | PR body should include `Fixes #111`; issue closes after protected merge. | M8 work resumes after merge. | Community Engineering Lead |
| Retrospective | Retrospective Gate | Pass locally | Lesson: repository-level names must not be inferred from crate names once a product family exists. | Future naming work should start from repository/crate/binary matrix. | Project Coordinator |

## Acceptance Criteria Mapping

| Acceptance Criterion | Implementation |
|---|---|
| GitHub repository is `kota-wilson/ferrisoxide`. | Verified with `gh repo view kota-wilson/ferrisoxide --json nameWithOwner,url`. |
| Local remote points at the new URL. | Verified with `git remote -v`. |
| Current repository metadata uses the new repository URL. | `Cargo.toml` workspace package repository metadata points to `https://github.com/kota-wilson/ferrisoxide`. |
| Current docs use `ferrisoxide` for the main repository. | README, `AGENTS.md`, `docs/environment.md`, project state, charter, orchestration, brand docs, and ADRs updated. |
| Docs clarify `ferrisoxide-signal` is a crate/binary. | README, ADR-006, ADR-007, and brand architecture preserve crate/CLI identity. |
| Historical PR evidence may remain historically accurate. | ADR-007 and this report explicitly exclude rewriting historical PR and validation evidence. |
| Workspace checks pass. | Final validation log records fmt, tests, clippy, and diff check. |

## Validation Commands

| Command | Result | Notes |
|---|---|
| `gh repo view kota-wilson/ferrisoxide --json nameWithOwner,url` | Passed | Output resolved to `kota-wilson/ferrisoxide` and `https://github.com/kota-wilson/ferrisoxide`. |
| `git remote -v` | Passed | `origin` fetch/push URLs use `https://github.com/kota-wilson/ferrisoxide.git`. |
| `cargo metadata --format-version 1 --no-deps` | Passed | Workspace metadata loaded successfully and reported `https://github.com/kota-wilson/ferrisoxide` for workspace packages. |
| `cargo fmt --check` | Passed | Formatting is clean. |
| `cargo test --workspace` | Passed | 123 workspace tests passed across CLI, core, embedded, measurements, plot, rule schema, signal, and doctests. |
| `cargo clippy --workspace --all-targets -- -D warnings` | Passed | No clippy warnings. |
| Current-doc identity scan | Passed | Remaining `ferrisoxide-signal` old-host references in selected current files are historical release evidence or ADR context. |
| `git diff --check` | Passed | No whitespace errors. |

## Review Notes

- `ferrisoxide-signal` remains correct wherever it names the crate, CLI binary, or signal-analysis product slice.
- Historical PR links and validation logs are not rewritten because they are audit evidence from prior repository states and GitHub redirects should continue to resolve them.
- Future naming decisions should update a repository/crate/binary/product matrix before implementation.

## Hand-Off Note

Role: Product Architect / GitHub Maintainer Specialist / Verification and Validation Engineer
Goal: Correct the main repository name to FerrisOxide without renaming the signal crate or CLI binary.
Files changed: `Cargo.toml`, `README.md`, `AGENTS.md`, `docs/environment.md`, `project-charter.md`, `orchestration-plan.md`, `docs/brand-architecture.md`, `decisions/ADR-006-ferrisoxide-signal-identity-adoption.md`, `decisions/ADR-007-repository-host-ferrisoxide.md`, `docs/repo-001-ferrisoxide-repository-host-pipeline-report.md`, `docs/validation-log.md`, `requirements.md`, `traceability-matrix.md`, `risk-register.md`, `project-state.md`.
Checks run: `gh repo view kota-wilson/ferrisoxide --json nameWithOwner,url`; `git remote -v`; `cargo metadata --format-version 1 --no-deps`; current-doc identity scan; `cargo fmt --check`; `cargo test --workspace`; `cargo clippy --workspace --all-targets -- -D warnings`; `git diff --check`.
Status: Pass locally; PR, required CI, merge, and issue closure pending.
Known gaps: External organization, domain, crates.io, trademark, logo, legal-suitability, and crate publication checks remain separate gates.
Next recommended step: Run final validation, open a protected PR with `Fixes #111`, merge after required CI, then resume M8.
