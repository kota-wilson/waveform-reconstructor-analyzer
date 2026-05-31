# Risk Register

| ID | Risk | Probability | Impact | Mitigation | Owner | Status | Review Trigger |
|---|---|---|---|---|---|---|---|
| WRA-RISK-001 | CSV files may have inconsistent delimiters, headers, units, or missing values. | High | High | Start with explicit MVP CSV dialect and structured parser errors; add fixtures for malformed data. | Software Architect | Active | Parser implementation. |
| WRA-RISK-002 | Sample-rate mismatch or unlabeled time units may produce invalid analysis results. | Medium | High | Require time-unit metadata and sample interval checks in waveform construction. | Systems Engineer | Active | Waveform model implementation. |
| WRA-RISK-003 | Filter phase delay or edge behavior may be misinterpreted as signal failure. | Medium | High | Document filter assumptions, edge handling, and latency; test synthetic signals. | Systems Engineer | Active | Filter implementation. |
| WRA-RISK-004 | MVP scope may expand into GUI, DAQ, or certification claims. | Medium | High | Keep non-goals in charter and stop at approval gates for scope expansion. | Project Coordinator | Active | Milestone planning. |
| WRA-RISK-005 | Third-party crate choices may introduce license or supply-chain risk. | Medium | Medium | Require dependency review before adding crates. | Security Engineer | Active | Dependency proposal. |
| WRA-RISK-006 | Performance may degrade on large waveform files. | Medium | Medium | Design streaming-friendly interfaces and benchmark before performance claims. | Performance Engineer | Active | Large fixture or streaming work. |
| WRA-RISK-007 | Users may treat this tool as certified aerospace validation software. | Low | High | Use clear disclaimers; avoid certification claims. | Documentation Engineer | Active | README and release docs. |
| WRA-RISK-008 | ADC quantization settings may hide analog excursions if users choose an unrealistic range or resolution. | Medium | High | Document clipping and ideal-code assumptions; keep raw data preserved; require tests that prove criteria evaluate the derived waveform. | Electrical Signal Integrity Engineer / Documentation Engineer | Active | ADC quantization transform changes. |
| WRA-RISK-009 | Software validation fixtures may be mistaken for hardware qualification evidence. | Medium | High | Label validation fixtures, report confidence notes, and environmental examples as software validation aids only; keep DAQ/hardware/certification out of scope. | Verification and Validation Engineer / Documentation Engineer | Active | Validation dataset or report schema changes. |
| WRA-RISK-010 | Benchmark results may be overread as production performance guarantees. | Medium | Medium | Record environment, generated fixture strategy, timing categories, and explicit non-goals; avoid DAQ throughput or real-time claims. | Performance Engineer | Active | Benchmark updates or performance claims. |

## Escalation

Critical impact or safety/security risks must be escalated to the Technical Director before release.
