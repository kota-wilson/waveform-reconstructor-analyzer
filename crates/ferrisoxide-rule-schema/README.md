# ferrisoxide-rule-schema

`ferrisoxide-rule-schema` owns the versioned portable FerrisOxide Rule Package model.

It is a schema and validation crate only. It deliberately does not parse CSV, evaluate rules, render reports, plot SVGs, export deployment packages, compute checksum algorithms, talk to DAQ/controller I/O, bind hardware HALs, or claim hardware qualification/certification suitability.

## Current Scope

The initial schema covers:

- package metadata and schema version,
- target profile,
- channel definitions, source names, thresholds, and engineering units,
- sample-rate assumptions,
- filter definitions,
- measurement-backed criteria definitions,
- timing limits through unit-bearing criterion requirements,
- explicit requirement units,
- structured validation errors before export or execution.

Future issues add export commands, checksums/manifests, shared execution, no_std boundaries, and parity tests.

The initial reviewable package format is documented in `../../docs/rule-package-format.md`, with parse-tested examples in `../../examples/rule-package/`.

## Hand-Off Note

Role: Software Architect / Core Software Engineer
Goal: Define the first portable rule package schema crate.
Files changed: `crates/ferrisoxide-rule-schema/`
Checks run: Workspace validation recorded in `docs/validation-log.md`.
Status: Initial schema only.
Known gaps: No validator, export command, manifest/checksum, binary package, rule engine, or embedded runtime integration yet.
Next recommended step: Implement package validation in M8-003.
