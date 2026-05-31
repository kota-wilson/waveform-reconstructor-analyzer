# Waveform Reconstructor and Analyzer Instructions

This project inherits the root `/Users/kota/Desktop/softwareai/AGENTS.md`, `STUDIO-CONSTITUTION.md`, and studio pipeline rules.

## Project Scope

- Product: Waveform Reconstructor and Analyzer.
- Type: Rust-centered open-source product repository.
- Domain: signal analysis for CSV waveform reconstruction, filter simulation, and pass/fail evaluation.
- Current phase: validated MVP implementation slice.

## Engineering Guardrails

- Keep all work inside `/Users/kota/Desktop/softwareai/projects/waveform-reconstructor-analyzer`.
- Do not install global packages, edit shell configuration, or modify system runtimes.
- Use project-local Rust tooling and standard Cargo commands.
- Third-party crates are allowed only after dependency review and explicit approval; approved MVP crates are recorded in `docs/dependency-review.md`.
- Do not claim aerospace certification, hardware validation certification, or regulatory compliance.
- Preserve raw waveform input data; filtering and analysis outputs must be derived artifacts.
- State units, sample rate assumptions, tolerances, and criteria definitions in docs and tests.

## Current Stop Condition

This repository has a dependency-reviewed MVP slice with CSV loading, waveform modeling, derived transforms, TOML config parsing, waveform criteria, text/JSON reports, CLI analysis, and a separate `wra-signal` embedded foundation. Stop before claiming production-grade signal-processing behavior, expanding into GUI/DAQ/certification work, or adding more dependencies without a fresh gate decision.
