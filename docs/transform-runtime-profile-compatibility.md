# Transform Runtime Profile Compatibility

Date: 2026-06-01

Status: M10-004 / issue #135 compatibility artifact. This document defines validation rules before implementation; it does not expose transforms to deployment packages or embedded runtimes.

## Purpose

FerrisOxide needs explicit runtime profile compatibility so a transform implemented for desktop analysis is not automatically treated as supported by Raspberry Pi 5 bare-metal ARM64, Pico 2, deployment packages, or certification workflows.

This document defines the rules future validation code should apply before a transform is exposed to a runtime profile.

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

## Planned Error Shape

Future validation code should return structured errors instead of free-form strings.

Suggested fields:

| Field | Meaning |
|---|---|
| `kind` | Stable error kind, such as `unsupported_transform_runtime_profile`. |
| `field` | Config or package field that requested the unsupported profile. |
| `transform_name` | Stable transform name. |
| `requested_profile` | Runtime profile requested by the caller. |
| `supported_profiles` | Runtime profiles declared by the transform. |
| `reason` | Human-readable explanation. |

Suggested error kinds:

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

## Current Transform Compatibility Matrix

Current transform mappings come from `docs/current-transform-metadata-mapping.md`.

| Transform | Desktop | Pi 5 no_std candidate | Pico 2 candidate | Reason |
|---|---|---|---|---|
| `moving_average` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; no no_std transform metadata or parity evidence yet. |
| `low_pass` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; timing assumptions and no_std parity evidence are not established. |
| `adc_quantize` | Allow | Reject for runtime exposure today | Reject for runtime exposure today | Implemented in desktop `ferrisoxide-core`; no compact runtime exposure evidence yet. |

This matrix does not remove existing desktop analysis support. It only prevents runtime/deployment overclaims.

## Future M11/M12 Compatibility Direction

| Planned Area | Expected First Profile | Embedded Exposure Direction |
|---|---|---|
| Pointwise transforms | `desktop` | Pi 5/Pico exposure only after no_std or fixed-buffer implementation and parity evidence. |
| Baseline transforms | `desktop` | Many baseline transforms are likely desktop-only until resource and timing behavior are proven. |
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
Goal: Complete M10-004 / issue #135 by defining transform runtime profile compatibility rules.
Files changed: `docs/transform-runtime-profile-compatibility.md`
Checks run: Documentation and compatibility review.
Status: Complete through PR #138; issue #135 and milestone #10 are closed.
Known gaps: Runtime validation code, deployment exposure, and embedded/no_std transform exposure remain future gated work.
Next recommended step: Use these rules before exposing any transform metadata to rule packages, deployment packages, Pi 5, or Pico 2 runtime paths.
