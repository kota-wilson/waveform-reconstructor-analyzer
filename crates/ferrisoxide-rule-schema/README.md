# ferrisoxide-rule-schema

`ferrisoxide-rule-schema` owns the versioned portable FerrisOxide Rule Package model.

It is a schema, validation, manifest, and deterministic checksum-evidence crate only. It deliberately does not parse CSV, evaluate rules, render reports, plot SVGs, export deployment packages, talk to DAQ/controller I/O, bind hardware HALs, or claim hardware qualification/certification suitability. Its checksum helper is non-cryptographic artifact drift evidence, not signing or security certification.

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
- structured validation errors before export or execution,
- deterministic manifest metadata,
- non-cryptographic artifact checksum evidence.

Future issues add shared execution, no_std boundaries, parity tests, and any compact binary package format.

The initial reviewable package format is documented in `../../docs/rule-package-format.md`, with parse-tested examples in `../../examples/rule-package/`.

## Hand-Off Note

Role: Software Architect / Core Software Engineer
Goal: Define the first portable rule package schema crate.
Files changed: `crates/ferrisoxide-rule-schema/`
Checks run: Workspace validation recorded in `docs/validation-log.md`.
Status: Schema, validator, export support metadata, manifest, and checksum evidence implemented locally.
Known gaps: No binary package, shared rule engine, no_std rule-engine boundary, parity tests, or embedded runtime integration yet.
Next recommended step: Implement shared rule execution in M8-006 after manifest/checksum PR review.
