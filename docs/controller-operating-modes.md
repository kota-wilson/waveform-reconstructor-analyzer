# Controller Operating Modes

Status: implemented manifest-level mode separation for M9-008 / issue #84.

Crate: `crates/ferrisoxide-deployment`

## Purpose

Controller-in-the-loop packages must distinguish three different runtime intents:

```text
production control
test verification
signal validation
```

Those modes may live in the same deployment package, but they must not be treated as the same behavior. A production controller mode can command outputs. A test verification mode evaluates qualification criteria. A signal-validation mode evaluates signal health or pass/fail criteria without selecting production control behavior.

## Mode Profiles

Deployment manifests use `mode_profiles` to describe allowed runtime purposes.

Example:

```json
{
  "id": "production-normal",
  "purpose": "production_control",
  "control_mode": "normal",
  "uses_artifacts": [
    "production_control_config"
  ]
}
```

The current required purposes are:

| Purpose | Meaning | Required artifacts | Forbidden artifacts |
|---|---|---|---|
| `production_control` | Runs approved production control behavior. | `production_control_config` | `test_verification_config`, `qualification_report`, `qualification_evidence_svg` |
| `test_verification` | Evaluates test/qualification criteria against observed or fixture signals. | `test_verification_config`, `channel_map` | `production_control_config`, `qualification_report`, `qualification_evidence_svg` |
| `signal_validation` | Evaluates signal criteria without commanding production behavior. | `test_verification_config`, `channel_map` | `production_control_config`, `qualification_report`, `qualification_evidence_svg` |

`production_control` requires `control_mode` because it must select a production control config mode such as `normal` or `safe`.

`test_verification` and `signal_validation` must not set `control_mode` because they must not silently select or execute production control behavior.

## Validation Rules

`DeploymentPackageManifest::validate()` rejects:

- missing `mode_profiles`,
- duplicate mode profile IDs,
- missing `production_control`, `test_verification`, or `signal_validation` purposes,
- empty mode IDs,
- mode profiles with no used artifacts,
- mode profiles referencing artifact roles not listed in the manifest,
- duplicate artifact roles inside a mode profile,
- `other` artifact roles inside a mode profile,
- production modes that omit `control_mode`,
- production modes that consume test verification or qualification evidence artifacts,
- test-verification modes that set `control_mode`,
- signal-validation modes that set `control_mode`,
- test-verification or signal-validation modes that consume production control artifacts.

Errors are structured as `DeploymentValidationError` values with field, kind, and message.

## Example Invalid Combinations

Invalid:

```json
{
  "id": "test-verification",
  "purpose": "test_verification",
  "control_mode": "normal",
  "uses_artifacts": [
    "production_control_config",
    "test_verification_config",
    "channel_map"
  ]
}
```

Reason: this mixes test verification with production control behavior. The validator returns `invalid_mode_artifact_combination` errors.

Invalid:

```json
{
  "id": "production-normal",
  "purpose": "production_control",
  "uses_artifacts": [
    "production_control_config"
  ]
}
```

Reason: production control must name the production control mode in `control_mode`.

## Current Limits

This issue does not add:

- a production runtime,
- a test-stand runtime,
- a signal-validation runtime,
- mode switching in firmware,
- HAL or SDK integration,
- live DAQ acquisition,
- real-time guarantees,
- hardware qualification evidence,
- certification evidence.

The current implementation is a manifest-level validation boundary for future runtime work.

## Hand-Off Note

Role: Software Architect / Verification and Validation Engineer
Goal: Document production, test, and signal-validation mode separation.
Files changed: `docs/controller-operating-modes.md`.
Checks run: See `docs/validation-log.md`.
Status: Implemented locally; PR, protected CI, merge, and issue #84 closure pending.
Known gaps: No runtime mode switcher, target loader, HAL/SDK adapter, live DAQ workflow, or certification evidence.
Next recommended step: Open PR with `Fixes #84`, wait for required CI, and merge only after checks pass.
