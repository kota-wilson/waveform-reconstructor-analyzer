# Transform Runtime Profile Compatibility

Date: 2026-06-01

Status: M10-004 / issue #135 compatibility artifact updated by M14. M13 implements validator code for metadata/profile checks; M14 adds high-pass baseline correction as desktop-only support and does not expose transforms to deployment packages or embedded runtimes.

## Purpose

FerrisOxide needs explicit runtime profile compatibility so a transform implemented for desktop analysis is not automatically treated as supported by Raspberry Pi 5 bare-metal ARM64, Pico 2, deployment packages, or certification workflows.

This document defines the rules validation code applies before a transform is exposed to a runtime profile. M13 implements the first code-level validator in `crates/ferrisoxide-core/src/runtime_profile.rs`.

## Runtime Profiles

Runtime profile names come from `docs/transform-capability-model.md`.

| Runtime Profile | Meaning |
|---|---|
| `desktop` | `std` desktop authoring and offline analysis. |
| `pi5_no_std_candidate` | Candidate for Raspberry Pi 5 bare-metal ARM64 only after no_std, bounded-buffer, and parity evidence exist. |
| `pico2_candidate` | Candidate for the optional Pico 2 micro-runtime subset only after fixed-buffer and compact-rule evidence exist. |
| `future_gated` | Not executable in current product paths; requires later gate approval. |

## Validation Timing

Runtime profile compatibility should be checked before any of these actions:

- accepting a transform in a runtime-targeted config
- exporting a transform into a rule package
- exporting a transform into a deployment package
- claiming embedded-compatible support
- claiming Pico 2 compatibility
- claiming runtime, hardware, or certification readiness

Desktop analysis may continue using current implemented transforms through existing config compatibility rules.

## Compatibility Rules

| Rule ID | Rule | Validation Behavior |
|---|---|---|
| TRP-001 | Transform must name the requested runtime profile in `runtime_profiles`. | Reject when requested runtime profile is absent. |
| TRP-002 | `capability_status` must be `implemented` for executable exposure. | Reject `planned`, `research`, `dependency_gated`, `hardware_gated`, and `certification_gated`. |
| TRP-003 | `future_gated` is never executable. | Reject any attempt to run or export a future-gated transform. |
| TRP-004 | `offline_only = true` is incompatible with streaming or embedded runtime exposure. | Reject for `pi5_no_std_candidate` and `pico2_candidate` unless a later profile explicitly supports offline full-record operation. |
| TRP-005 | `sample_rate_required = true` requires valid timing metadata before execution. | Reject missing, invalid, duplicate, or non-increasing timing evidence when the transform requires timing. |
| TRP-006 | Pi 5 exposure requires no_std-compatible implementation evidence. | Reject unless the transform has no_std-safe implementation, bounded resource assumptions, and parity or target-check evidence. |
| TRP-007 | Pico 2 exposure requires compact fixed-buffer evidence. | Reject unless memory, parameter, and output constraints fit the approved micro-runtime subset. |
| TRP-008 | Dependency-gated transforms require dependency approval before implementation or exposure. | Reject until dependency review passes. |
| TRP-009 | Hardware-gated transforms require hardware/environment approval before live runtime claims. | Reject until hardware and environment gates pass. |
| TRP-010 | Certification-gated transforms require separate certification evidence planning. | Reject certification or regulatory claims until approved evidence exists. |

## Implemented Error Shape

M13 validation code returns structured errors instead of free-form strings.

Implemented fields:

| Field | Meaning |
|---|---|
| `kind` | Stable error kind, such as `unsupported_transform_runtime_profile`. |
| `field` | Config or package field that requested the unsupported profile. |
| `transform_name` | Stable transform name. |
| `requested_profile` | Runtime profile requested by the caller. |
| `supported_profiles` | Runtime profiles declared by the transform. |
| `reason` | Human-readable explanation. |

Implemented error kinds:

- `unsupported_transform_runtime_profile`
- `future_gated_transform`
- `planned_transform_not_implemented`
- `offline_transform_not_streaming_supported`
- `missing_sample_timing`
- `invalid_sample_timing`
- `missing_no_std_evidence`
- `missing_micro_runtime_evidence`
- `dependency_gate_required`
- `hardware_gate_required`
- `certification_gate_required`

## Current Validator API

The core validator is metadata-only. It validates existing `TransformStepMetadata` records and waveform timing metadata:

- `validate_transform_runtime_profile(transforms, requested_profile, timing)`
- `validate_waveform_metadata_runtime_profile(metadata, requested_profile)`
- `TransformRuntimeTimingEvidence::from_waveform_metadata(metadata)`

The validator currently accepts current transform metadata for `desktop` and rejects current waveform, event, and validation transform metadata for `pi5_no_std_candidate`, `pico2_candidate`, and `future_gated` exposure. That rejection is intentional: current transform implementations remain desktop analysis support unless a later issue supplies no_std, fixed-buffer, bounded-resource, and parity evidence.

## Current Transform Compatibility Matrix

Current transform mappings come from `docs/current-transform-metadata-mapping.md`.

| Transform | Desktop | Pi 5 no_std candidate | Pico 2 candidate | Reason |
|---|---|---|---|---|
| `moving_average` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; no no_std transform metadata or parity evidence yet. |
| `low_pass` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; timing assumptions and no_std parity evidence are not established. |
| `adc_quantize` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; no compact runtime exposure evidence yet. |
| `offset` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; no no_std/fixed-buffer parity evidence yet. |
| `gain` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; no no_std/fixed-buffer parity evidence yet. |
| `invert` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; no no_std/fixed-buffer parity evidence yet. |
| `clamp` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; nonlinear limiting is not exposed to runtime profiles yet. |
| `deadband` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; nonlinear threshold behavior is not exposed to runtime profiles yet. |
| `dc_remove` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; offline-only full-waveform mean removal is not a runtime transform. |
| `baseline_subtract` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; no no_std/fixed-buffer parity evidence yet. |
| `high_pass_baseline` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; no no_std/fixed-buffer timing, resource, or parity evidence yet. |
| `moving_median` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented for desktop analysis; nonlinear window behavior lacks embedded-compatible parity evidence. |

This matrix does not remove existing desktop analysis support. It only prevents runtime/deployment overclaims.

The legacy `export-rule-package` command still supports the earlier portable filter subset described in `docs/rule-package-format.md`. That legacy export support is not a blanket transform runtime-support claim. Any future transform-package or deployment-package exposure should call the runtime-profile validator before accepting or exporting transform metadata.

## Runtime Compatibility Direction

| Transform Area | Expected First Profile | Embedded Exposure Direction |
|---|---|---|
| Pointwise transforms | `desktop` | Pi 5/Pico exposure only after no_std or fixed-buffer implementation and parity evidence. |
| Baseline transforms | `desktop` | `dc_remove` remains offline-only; other baseline transforms need resource and timing review before runtime exposure. |
| Event transforms | `desktop` | Deterministic state/event transforms may become Pi 5 candidates after shared logic and parity tests. |
| Validation transforms | `desktop` | Embedded exposure requires shared rule/event semantics and exact parity where practical. |

## Non-Claims

Runtime profile compatibility does not claim:

- live DAQ support
- Raspberry Pi hardware boot evidence
- Pico 2 hardware support
- HAL or RTOS SDK support
- real-time timing guarantees
- production controller readiness
- hardware qualification
- flight certification or regulatory compliance

## Hand-Off Note

Role: Embedded RTOS Engineer / Software Architect
Goal: Complete M10-004 / issue #135 and support M13 runtime-profile validation.
Files changed: `docs/transform-runtime-profile-compatibility.md`, `crates/ferrisoxide-core/src/runtime_profile.rs`
Checks run: Documentation review; focused M13 runtime-profile tests; M14 high-pass baseline tests.
Status: M10 rules are complete through PR #138; M13 validator implementation is complete through PR #164; M14 high-pass baseline correction is complete through PR #173 and remains desktop-only.
Known gaps: Deployment exposure and embedded/no_std transform exposure remain future gated work.
Next recommended step: Use the validator before any future transform metadata is exposed to rule packages, deployment packages, Pi 5, or Pico 2 runtime paths.
