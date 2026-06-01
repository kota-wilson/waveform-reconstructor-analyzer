# RTOS Deployment Package Format

Status: implemented schema and example package for M9-007 / issue #83.

Crate: `crates/ferrisoxide-deployment`

Example package: `examples/deployment-package/heated-actuator/`

## Purpose

The RTOS/controller deployment package format defines the artifact set that bridges desktop authoring and future controller-runtime consumption.

The format is meant to answer:

```text
Which production control config was approved?
Which test verification config was approved with it?
Which channel map links the signals?
Which manifest describes the artifact set?
Which checksum index detects accidental artifact drift?
Which qualification report and SVG evidence were reviewed?
When was the package generated?
```

It is not a runtime loader, not a binary firmware package, not a signing system, not an authentication mechanism, and not certification evidence.

## Package Layout

Required artifact set:

```text
deployment-package/
  production-control-config.toml
  test-verification-config.toml
  channel-map.toml
  manifest.json
  checksum.txt
  qualification-report.json
  qualification-evidence.svg
  generated-at.txt
```

## Artifact Roles

| Artifact | Role | Purpose |
|---|---|---|
| `production-control-config.toml` | `production_control_config` | Defines normal controller behavior. |
| `test-verification-config.toml` | `test_verification_config` | Defines qualification and production-test criteria. |
| `channel-map.toml` | `channel_map` | Maps fixture, DAQ, or runtime input names to logical channels and control inputs. |
| `manifest.json` | `package_manifest` | Lists package metadata, target profile, required artifacts, and integrity metadata. |
| `checksum.txt` | `checksum_index` | Detects accidental artifact drift. |
| `qualification-report.json` | `qualification_report` | Records software evidence for the package. |
| `qualification-evidence.svg` | `qualification_evidence_svg` | Provides human-reviewable visual evidence. |
| `generated-at.txt` | `generated_at` | Records the generation timestamp. |

Production control and test verification configs must remain separate files. They are linked by the manifest and evidence records instead of being collapsed into one mixed-purpose config.

## Manifest Shape

The manifest is JSON in the current example because it is convenient for automation and exact tests.

Representative shape:

```json
{
  "manifest_version": "0.1.0",
  "package": {
    "name": "heated-actuator-controller-deployment",
    "version": "0.1.0",
    "format_version": "0.1.0"
  },
  "target": {
    "kind": "controller_runtime",
    "identifier": "raspberry-pi-5-bare-metal"
  },
  "generated_at": "2026-06-01T00:00:00Z",
  "artifacts": [],
  "integrity": {
    "checksum_file": "checksum.txt",
    "algorithm": "fnv1a64",
    "scope": "artifact drift detection only",
    "security_note": "non-cryptographic integrity index; not signing, authentication, certification, or tamper-proofing"
  }
}
```

The crate validates required metadata and required artifact roles. Future runtime loaders can choose a constrained subset, but they should reject unsupported features before deployment instead of silently approximating desktop behavior.

## Operating Mode Profiles

The manifest also defines `mode_profiles` so production behavior, test verification, and signal validation are not conflated.

Required purposes:

- `production_control`
- `test_verification`
- `signal_validation`

Representative shape:

```json
{
  "id": "test-verification",
  "purpose": "test_verification",
  "uses_artifacts": [
    "test_verification_config",
    "channel_map"
  ]
}
```

Production control modes must select a production `control_mode` and consume `production_control_config`. Test verification and signal validation modes must not select production control behavior and must not consume `production_control_config`.

See `docs/controller-operating-modes.md` for the full mode policy.

## Validation Rules

`DeploymentPackageManifest::validate()` checks:

- `manifest_version` is present and matches the current deployment format version.
- package name, package version, package format version, target identifier, and generation timestamp are present.
- integrity fields are present.
- every required artifact role is listed.
- `production_control`, `test_verification`, and `signal_validation` mode profiles are listed.
- mode profiles use only allowed artifact combinations for their purpose.
- artifact paths and media types are non-empty.
- artifact paths are unique.
- production control and test verification configs are separate artifacts.
- `integrity.checksum_file` appears in the artifact list.

The validator intentionally does not read the filesystem, calculate checksums, sign packages, load RTOS configs, or inspect hardware targets.

## Checksum And Integrity Wording

`checksum.txt` is a drift-detection aid. It is not cryptographic signing and is not evidence of package authenticity, release approval, hardware qualification, or certification.

Required wording in package examples:

```text
not signing, authentication, certification, or tamper-proofing
```

This wording matters because deployment packages can otherwise look more authoritative than they are. FerrisOxide must not imply hardware qualification, safety certification, flight certification, or production readiness without separate gates and evidence.

## Current Example

The heated actuator example package links:

- production control config: `examples/deployment-package/heated-actuator/production-control-config.toml`
- test verification config: `examples/deployment-package/heated-actuator/test-verification-config.toml`
- channel map: `examples/deployment-package/heated-actuator/channel-map.toml`
- manifest: `examples/deployment-package/heated-actuator/manifest.json`
- checksum index: `examples/deployment-package/heated-actuator/checksum.txt`
- qualification report: `examples/deployment-package/heated-actuator/qualification-report.json`
- qualification evidence SVG: `examples/deployment-package/heated-actuator/qualification-evidence.svg`
- generated timestamp: `examples/deployment-package/heated-actuator/generated-at.txt`

Run the focused validator tests:

```bash
cargo test -p ferrisoxide-deployment
```

## Current Limits

This issue does not add:

- package export CLI for controller deployment packages,
- binary `rules.bin` or compact runtime serialization,
- RTOS runtime loader,
- HAL or SDK integration,
- cryptographic signing,
- hardware target execution,
- hardware qualification evidence,
- flight certification evidence.

Those remain separate future issues and approval gates.

## Hand-Off Note

Role: Embedded RTOS Engineer / Security Engineer / Documentation Engineer
Goal: Define and validate the RTOS/controller deployment package format.
Files changed: `crates/ferrisoxide-deployment/`, `examples/deployment-package/heated-actuator/`, and `docs/rtos-deployment-package-format.md`.
Checks run: See `docs/validation-log.md`.
Status: Implemented locally; PR, protected CI, merge, and issue #83 closure pending.
Known gaps: No runtime loader, package export command for controller packages, binary runtime package, signing, target hardware execution, or certification evidence.
Next recommended step: Open PR with `Fixes #83`, wait for required CI, and merge only after checks pass.
