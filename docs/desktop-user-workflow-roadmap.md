# Desktop User Workflow Roadmap

Date: 2026-06-02

Status: Implementation artifact for M37 through M42 after the M25-M36 comprehensive filter and simulated signal-conditioning suite reached `main` through PR #175. PR #177 merged the CLI source inspection, CSV config scaffolding, workflow templates, CSV/simulation evaluation bundles, examples, docs, and tests to `main`. It adds no new dependencies, GUI, live DAQ SDKs, HAL/RTOS adapters, target hardware work, runtime loader implementation, release publication, or certification scope.

## Purpose

FerrisOxide now has enough analysis, transform, simulation, criteria, report, plotting, batch, and package evidence features that the next product risk is user flow fragmentation. A desktop engineer should be able to move through one understandable workflow:

```text
run FerrisOxide Signal
  -> choose a signal source
  -> identify and label channels
  -> apply transforms, filters, and feature/event calculations
  -> add pass/fail criteria for each relevant channel
  -> run evaluation
  -> review results and generated evidence
```

This document defines and records the milestone path for making that flow explicit and reviewable. The current desktop product remains CLI-first. A future GUI, vendor DAQ integration, live hardware acquisition, runtime loader, and target execution remain separately gated.

## Source Modes

| Source mode | Current support | Planned direction | Gate status |
|---|---|---|---|
| CSV data | Implemented through `analyze`, `plot`, `batch`, `inspect-source`, `scaffold-config`, and `evaluate-bundle` over local files and TOML configs. | Keep as the first-class desktop authoring path. | Implemented; merged in PR #177 for M38-M42. |
| Simulated signals | Implemented through `simulate`, fixture/test-double DAQ input, deterministic virtual controller simulation, M34 fault/ADC-DAC simulation transforms, `inspect-source --source simulation`, and `evaluate-bundle --source simulation`. | Treat simulation as a first-class source mode for desktop validation and fixture authoring. | Implemented; merged in PR #177 for M38-M42. |
| Realtime/live signals | Not implemented as live DAQ or hardware input. Current DAQ support is fixture/test-double only. | Define a source abstraction and UI language that can reserve a live/realtime mode without activating SDK, driver, hardware, or global setup work. | Pending separate human, dependency, security, environment, hardware, and V&V approval. |

## Workflow Contract

| Step | User intent | Current FerrisOxide support | Next gap to close |
|---|---|---|---|
| 1. Run program | Start the desktop analysis tool without private context. | `ferrisoxide-signal` CLI with `analyze`, `plot`, `simulate`, `batch`, `export-rule-package`, `transforms`, `inspect-source`, `scaffold-config`, `workflow-template`, and `evaluate-bundle`. | Implemented; merged in PR #177. |
| 2. Choose signal source | Use CSV data, simulated signals, or a future-gated realtime stream. | CSV and fixture simulation are implemented; live input is not. | Implemented for CSV and fixture simulation with gated realtime errors; merged in PR #177. |
| 3. Identify and label channels | Turn raw columns or fixture channels into engineering names, units, and roles. | `[input]` channel mapping, simulation channel maps, `inspect-source`, and `scaffold-config` exist. | Implemented for CSV scaffolds and simulation map inspection; merged in PR #177. |
| 4. Apply transforms and filters | Condition waveforms before evaluation while preserving raw lineage. | `[[filters]]`, `[[feature_transforms]]`, `[[event_transforms]]`, `[[event_validations]]`, `workflow-template`, and the transform catalog exist. | Implemented with recipes and examples; merged in PR #177. |
| 5. Add pass/fail criteria | Attach requirements to each relevant channel or event. | Legacy `[[criteria]]`, measurement-backed DSL criteria, event validations, scaffolded observed-bound starter criteria, and templates exist. | Implemented; engineering threshold approval remains user-owned; merged in PR #177. |
| 6. Run evaluation | Execute analysis over the selected source and config. | `analyze`, `simulate`, `batch`, and `evaluate-bundle` run deterministic local evaluations. | Implemented; merged in PR #177. |
| 7. Get results | Review pass/fail outcomes, values, plots, and artifacts. | Text, JSON, SVG evidence, batch summaries, rule packages, deployment package fixtures, qualification evidence reports, source summaries, bundle summaries, config copies, and failure triage notes exist. | Implemented for CSV and simulation bundles; merged in PR #177. |

## Milestone Path

### M37: Desktop User Workflow Contract

Goal:

- Define the desktop user journey as a first-class workflow.
- Map each step to implemented commands, planned gaps, and non-goals.
- Update README and project state so users see the path before implementation starts.

Required exit evidence:

- `docs/desktop-user-workflow-roadmap.md` exists and links from README.
- Requirements WRA-RQ-122 through WRA-RQ-127 record the M37-M42 path.
- Risk register records that this flow does not implement live DAQ or GUI support.
- No code, dependency, schema, runtime, hardware, or certification scope changes.
- Later implementation evidence: README and `docs/desktop-user-workflow.md` now link the implemented CLI workflow without adding GUI, live DAQ, runtime, hardware, release, or certification scope.

### M38: Signal Source Intake And Inspect

Goal:

- Make source choice explicit for CSV, simulation, and future-gated realtime modes.
- Add or design a source inspection workflow that reports time column candidates, channel columns, sample count, timing summary, inferred sample rate when safe, and source warnings.
- Reserve realtime/live input vocabulary without enabling vendor SDKs, drivers, live hardware, or global setup.

Required exit evidence:

- `inspect-source` supports CSV inspection.
- `inspect-source --source simulation` supports fixture simulation source inspection through a channel map.
- Unsupported realtime/live source names produce clear gated errors.
- Tests and docs prove no live DAQ SDK, HAL, RTOS, target hardware, unsafe FFI, or dependency addition was introduced.

### M39: Channel Labeling And Config Scaffold

Goal:

- Let users convert source inspection into a starter TOML config.
- Capture channel labels, units, source column names, logical roles, and derived output-channel names.
- Keep simulation channel maps and analysis input channel maps aligned enough for one workflow.

Required exit evidence:

- `scaffold-config` generates `[input]`, units, metadata, tolerances, transform placeholders, and per-channel observed-bound starter criteria for CSV sources.
- Missing time columns and missing selected channels are rejected through the shared CSV parser.
- Docs show CSV scaffolding and simulation channel-map inspection workflows; simulation channel maps remain the simulation labeling surface.

### M40: Transform And Criteria Authoring UX

Goal:

- Make transforms, filters, features, events, and criteria easier to author from the desktop workflow.
- Provide workflow recipes for common use cases: supply rail validation, switch bounce, response latency, noisy sensor cleanup, simulated ADC/fault injection, and multi-channel derived measurements.
- Keep raw-data preservation and derived-lineage evidence visible.

Required exit evidence:

- `workflow-template` renders TOML starters using current `[[filters]]`, `[[feature_transforms]]`, `[[event_transforms]]`, `[[event_validations]]`, and `[[criteria]]` surfaces.
- Transform catalog discovery is linked directly from README and `docs/desktop-user-workflow.md`.
- Criteria templates are channel-specific and produce clear pass/fail evidence.
- Docs warn when filtering, smoothing, resampling, or simulation can hide raw failures.

### M41: Evaluation Run Bundle

Goal:

- Produce one predictable output directory for a desktop workflow run.
- Preserve current `analyze`, `plot`, `simulate`, and `batch` behavior while defining a bundle convention.
- Make it easy to review inputs, configs, transformed lineage, reports, plots, and summary outcomes.

Required exit evidence:

- `evaluate-bundle` writes source summary, config copy, report text, report JSON, optional SVG, bundle summary, and failure triage notes for CSV inputs.
- `evaluate-bundle --source simulation` writes source summary, simulation workflow text/JSON, config copies, bundle summary, and failure triage notes for fixture simulation inputs.
- Existing `analyze`, `plot`, `simulate`, and `batch` behavior remains compatible.
- Focused tests cover deterministic CSV and simulation bundle artifact sets; overwrite refusal is inherited from the shared `write_output_file` path.

### M42: Desktop Workflow Polish And Validation Corpus

Goal:

- Prove the full user journey with end-to-end examples and validation assets.
- Make the workflow discoverable enough that a desktop engineer can use FerrisOxide without private implementation context.
- Keep GUI, live DAQ, hardware, runtime loader, release publication, and certification claims outside this milestone.

Required exit evidence:

- End-to-end docs cover CSV, simulation, transforms, criteria, evaluation, and results.
- `examples/m42-desktop-workflow-waveform.csv` and `examples/m42-desktop-workflow-config.toml` map one complete workflow from source through result bundle.
- README quick path and documentation map route users to the workflow.
- Link scan, whitespace scan, diff check, formatting, and relevant focused tests are part of final M42 validation.

## Gate Decisions

| Gate | Decision | Evidence | Next Owner |
|---|---|---|---|
| Desktop Workflow Roadmap Gate | Pass | This document defines M37-M42 from the user-requested desktop flow and maps current support versus gaps; implementation merged through PR #177. | Product Architect / Project Coordinator |
| Source Scope Gate | Pass | CSV and simulated sources are implemented through inspect/evaluate workflows; realtime/live DAQ remains future-gated. | Project Coordinator |
| GUI Scope Gate | Not Applicable | Current direction is CLI-first desktop workflow, not GUI implementation. | Product Coordinator / UX Owner |
| Dependency Gate | Not Applicable | M37-M42 add no dependencies. | Security Engineer |
| Hardware Runtime Gate | Blocked until separate approval | Live DAQ, SDKs, HAL/RTOS, target hardware, runtime loader, and certification work need fresh gates. | User / Technical Director |
| Documentation Gate | Pass | README, workflow docs, requirements, traceability, risk, orchestration, state, and validation log were checked with final M42 validation evidence and PR #177 merge evidence. | Documentation Engineer |

## Hand-Off Note

Role: Product Architect / Project Coordinator
Goal: Define and implement the desktop-user workflow milestone path after M25-M36.
Files changed: `docs/desktop-user-workflow-roadmap.md`, `docs/desktop-user-workflow.md`, README, CLI, examples, requirements, traceability, risk, orchestration, post-MVP roadmap, next-milestones roadmap, project state, and validation log.
Checks run: See `docs/validation-log.md`.
Status: Implemented, validated, and merged through PR #177 for M37-M42.
Known gaps: GUI, live DAQ, vendor SDKs, hardware channel discovery, HAL/RTOS adapters, runtime loader, release publication, and certification scope remain gated.
Next recommended step: Select any future post-M42 follow-up only after an explicit gate while keeping GUI, live DAQ, runtime-loader, hardware, release, dependency, and certification scope gated.
