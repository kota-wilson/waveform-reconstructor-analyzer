# ADR-001: Initial Rust Workspace Architecture

Date: 2026-05-30

## Status

Accepted for MVP scaffold.

## Context

The product needs a Rust core library, CLI MVP, and future GUI or language binding support.

## Decision

Use a Cargo workspace with:

- `crates/ferrisoxide-core` for reusable data, parser, filter, criteria, analysis, and report APIs.
- `crates/ferrisoxide-cli` for command-line orchestration.

## Consequences

- Future UI and bindings can reuse `ferrisoxide-core`.
- CLI remains thin.
- Workspace validation can use standard Cargo commands.

## Alternatives Considered

- Single binary crate: simpler but weaker for future bindings.
- Single library crate only: lacks MVP CLI entry point.
- Multi-language scaffold now: too broad for MVP.
