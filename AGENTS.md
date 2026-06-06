# FerrisOxide Repository Instructions

This project inherits the root `/Users/kota/Desktop/softwareai/AGENTS.md`, `STUDIO-CONSTITUTION.md`, and studio pipeline rules.

## Project Scope

- Product workspace: FerrisOxide.
- Current implemented product slice: FerrisOxide Signal.
- Type: Rust-centered open-source product repository.
- Domain: signal analysis for CSV waveform reconstruction, filter simulation, and pass/fail evaluation.
- Current phase: local MVP-exit passed for the desktop software workflow; post-MVP scope remains gated.

## Engineering Guardrails

- Keep all work inside `/Users/kota/Desktop/softwareai/projects/ferrisoxide`.
- Do not install global packages, edit shell configuration, or modify system runtimes.
- Use project-local Rust tooling and standard Cargo commands.
- Third-party crates are allowed only after dependency review and explicit approval; approved MVP crates are recorded in `docs/dependency-review.md`.
- Do not claim aerospace certification, hardware validation certification, or regulatory compliance.
- Preserve raw waveform input data; filtering and analysis outputs must be derived artifacts.
- State units, sample rate assumptions, tolerances, and criteria definitions in docs and tests.

## Architecture Diagram Rule

Every major crate must include an `architecture.md` file with a Mermaid flowchart.

The root `docs/architecture/ferrisoxide-overview.md` file contains the system-level FerrisOxide flowchart.

Crate diagrams must stay focused on that crate only:

- Inputs.
- Internal processing stages.
- Outputs.
- Public APIs.
- Error paths when important.

Do not duplicate the full system diagram inside every crate.

Update the relevant diagram whenever crate responsibilities, APIs, or data flow change.

## Current Stop Condition

This repository has a dependency-reviewed post-MVP local desktop slice with CSV loading, waveform modeling, derived transforms, TOML config parsing, waveform criteria, text/JSON/SVG reports, local batch analysis, rule-package review artifacts, and separate embedded-boundary crates. Stop before claiming production-grade signal-processing behavior, expanding into GUI/DAQ/certification/hardware/runtime work, publishing releases, or adding dependencies without a fresh gate decision.
