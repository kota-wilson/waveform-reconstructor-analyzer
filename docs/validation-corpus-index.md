# Validation Corpus Index

Date: 2026-06-02

Status: M19 validation corpus and benchmark baseline index for the implemented desktop product surface after local MVP exit, updated through M36 comprehensive-suite closure. This index maps current fixture evidence to workflows and known gaps. It is software validation evidence only.

## Coverage Matrix

| Workflow / Behavior | Fixture Or Config | Expected Evidence | Test / Command |
|---|---|---|---|
| Basic CSV analysis | `examples/basic-waveform.csv`, `examples/basic-config.toml` | Passing text/JSON report with min/max evidence. | CLI tests and README commands. |
| Criteria DSL parity | `examples/basic-dsl-config.toml`; `tests/configs/*-dsl.toml` | DSL and legacy reports match representative behavior. | Core/CLI DSL parity tests. |
| Exact golden reports | `tests/golden/*.json` | Stable JSON report structure. | Golden report tests. |
| Known-answer square wave | `validation/known_answer/` | Expected measurements documented before analyzer execution. | Known-answer validation test. |
| Measurement engine | `validation/measurement_engine/` | Known extrema, transitions, pulse width, transient duration, stable state, rise/fall behavior. | Measurement-engine exact report test. |
| Environmental dropout/contact bounce | `validation/environmental_cases/` | Failing environmental-style reports with expected measurements. | Environmental validation report tests. |
| Pointwise/windowed transforms | `examples/m11-transform-config.toml` | Derived transform metadata for offset, gain, invert, clamp, deadband, DC removal, baseline subtraction, moving median. | CLI M11 transform test. |
| High-pass baseline correction | `examples/m14-high-pass-baseline-config.toml` | Derived metadata, timing checks, desktop-only profile, package export rejection. | Core/CLI M14 tests. |
| Data cleaning and timing conditioning | `examples/m26-data-cleaning-waveform.csv`, `examples/m26-data-cleaning-config.toml` | Timestamp sort/dedupe, NaN interpolation/removal, crop, fixed delay, gap fill, fixed-rate resampling, channel delay metadata, and post-cleaning criteria evidence. | Core M26 tests and CLI M26 transform test. |
| Pointwise normalization and nonlinear conditioning | `examples/m27-pointwise-waveform.csv`, `examples/m27-pointwise-config.toml` | Absolute/square/square-root/log/exp/normalize/tanh/sigmoid/soft-limit/piecewise/polynomial metadata, domain checks, and post-transform criteria evidence. | Core M27 tests and CLI M27 transform test. |
| Smoothing, detrending, baseline, and spike cleanup | `examples/m28-smoothing-waveform.csv`, `examples/m28-smoothing-config.toml` | Weighted/EMA/boxcar/Gaussian/Savitzky-Golay/median smoothing, rolling baseline, detrending, Hampel, spike cleanup metadata, edge behavior, and post-transform criteria evidence. | Core M28 tests and CLI M28 transform test. |
| Standard frequency filters | `examples/m29-frequency-waveform.csv`, `examples/m29-frequency-config.toml` | FIR/IIR coefficient filters, zero-phase FIR/IIR, high/band/notch/comb filters, Butterworth/Chebyshev/Bessel metadata, sample-rate validation, generated frequency-response checks, and post-transform criteria evidence. | Core M29 tests and CLI M29 transform test. |
| Resampling and timing alignment | `examples/m30-resampling-waveform.csv`, `examples/m30-resampling-config.toml` | Fixed-grid resampling/interpolation, downsample/decimate/upsample/rational conversion, sample/zero/first-order holds, fractional delay, cross-correlation lag/confidence metadata, jitter correction, clock-drift correction, and post-transform criteria evidence. | Core M30 tests and CLI M30 transform test. |
| Envelope, energy, and calculus | `examples/m31-calculus-waveform.csv`, `examples/m31-calculus-config.toml` | Rectification, envelope, moving RMS, peak hold, derivatives, integrals, leaky integration, slope detection, scalar feature records, unit evidence, and post-transform criteria evidence. | Core M31 tests and CLI M31 transform/feature test. |
| Statistics and correlation | `examples/m32-statistics-waveform.csv`, `examples/m32-statistics-config.toml`, `examples/m32-statistics-filters-config.toml` | Rolling statistics, z-score, outlier flags, quantile clipping, scalar statistics, histogram bin records, covariance, correlation, autocorrelation, cross-correlation, method context, and post-transform criteria evidence. | Core M32 tests and CLI M32 transform/feature test. |
| Spectrum, windows, and time-frequency | `examples/m33-spectrum-waveform.csv`, `examples/m33-spectrum-config.toml` | Window coefficients, DFT/FFT/IFFT, power spectrum, PSD, Welch, cross-spectrum, coherence, transfer estimate, harmonic metrics, STFT, spectrogram, centroid, bandwidth, rolloff, band power, and method context for sine, square, harmonic, and deterministic-noise fixtures. | Core M33 tests and CLI M33 feature test. |
| Fault injection and ADC/DAC simulation | `examples/m34-fault-adc-waveform.csv`, `examples/m34-fault-adc-config.toml` | Seeded deterministic noise/fault/drift/interference simulation, quantizer variants, dithering, sample-clock jitter, ADC missing-code/INL/DNL/gain/offset simulation, `evidence_scope = simulation_only`, and post-transform criteria evidence. | Core M34 tests, catalog metadata test, CLI M34 transform test, and rule-package export rejection matrix. |
| Multi-channel, sensor, and domain conditioning | `examples/m35-domain-waveform.csv`, `examples/m35-domain-config.toml` | Derived-channel evidence for multi-channel arithmetic, vector/matrix/coordinate transforms, software sensor conversions, vibration integration/severity, control transforms, explicit output units, and desktop-only package rejection metadata. | Core M35 tests, catalog metadata test, CLI M35 transform test, direct CLI JSON fixture run, and rule-package export rejection matrix. |
| Event validation | `examples/switch-bounce-waveform.csv`, `examples/m12-event-validation-config.toml` | Event records and validation records for switch/bounce workflows. | CLI M12 event validation test. |
| Heated actuator qualification scenario | `tests/e2e/heated_actuator/` | Passing and failing JSON reports for response latency, transient, and supply dropout behavior. | E2E exact report tests. |
| Rule-package export guardrails | `tests/expected/rule-package-basic/`; M14 high-pass config | Exact export artifacts and unsupported transform rejection. | CLI export tests. |
| Linear pointwise rule packages | `examples/rule-package/linear-pointwise-rules.toml`, `examples/rule-package/linear-pointwise-rules.json`, `examples/m21-linear-pointwise-package-config.toml` | Parse-equivalent TOML/JSON package fixtures plus export support for `offset`, `gain`, and `invert`. | Rule-schema linear pointwise tests; CLI linear pointwise export/parity tests. |
| Desktop simulation workflow | `examples/control-config/`, `examples/test-verification-config/`, `examples/simulation/` | Simulation trace and verification evidence. | CLI simulation tests. |
| Controller parity | `tests/parity/`, `tests/controller_parity/` | Desktop/embedded-compatible evidence parity for approved software subset. | Parity tests. |
| Batch analysis | `examples/batch-analysis.toml` | Per-run reports plus deterministic summary. | M17 CLI batch tests. |
| Benchmark baseline | `scripts/benchmark-large-csv.sh`, `validation/benchmarks/README.md`; `examples/m35-domain-waveform.csv`, `examples/m35-domain-config.toml` | Local timing baseline with environment and scope limits, plus M36 benchmark-readiness evidence over the comprehensive domain fixture. | Benchmark command when performance evidence is refreshed; M36 benchmark helper run in `docs/validation-log.md`. |

## Negative Case Matrix

| Invalid Or Failing Case | Evidence |
|---|---|
| Bad TOML syntax | `tests/configs/invalid-bad-syntax.toml` and CLI invalid-config tests. |
| Empty channels | `tests/configs/invalid-empty-channels.toml`. |
| Missing criteria | `tests/configs/invalid-missing-criteria.toml`. |
| Unsupported criterion | `tests/configs/invalid-unsupported-criterion.toml`. |
| Missing ADC field | `tests/configs/invalid-missing-adc-field.toml`. |
| Negative tolerance | `tests/configs/invalid-negative-tolerance.toml`. |
| Mixed legacy/DSL criterion shape | `tests/configs/invalid-mixed-legacy-dsl-criterion.toml`. |
| Unknown DSL operator | `tests/configs/invalid-dsl-unknown-operator.toml`. |
| Missing DSL requirement fields | `tests/configs/invalid-dsl-missing-*`. |
| Invalid event config | M12 config tests. |
| Invalid M26 cleaning/timing transform config or data | M26 config/core tests cover missing fields, non-finite fields, all-NaN channels, invalid crops, missing channels, and invalid fixed-grid intervals. |
| Invalid M27 pointwise/nonlinear config or data | M27 config/core tests cover missing fields, inverted normalization ranges, invalid point/coefficients, invalid log/exp bases, negative square-root inputs, non-positive log inputs, and constant normalization inputs. |
| Invalid M28 smoothing/baseline config or data | M28 config/core tests cover missing fields, invalid weights, invalid alpha, zero windows, invalid sigma/order/outlier/threshold values, invalid time axes, and non-finite samples. |
| Invalid M29 frequency-filter config or data | M29 config/core tests cover missing fields, invalid biquad coefficient counts and unstable poles, nonuniform timing, above-Nyquist frequencies, invalid Q, invalid comb delay/gain, and invalid ripple/attenuation values. |
| Invalid M30 resampling/timing config or data | M30 config/core tests cover missing fields, invalid factors, invalid intervals, invalid delays, missing alignment channels, nonuniform timing where uniform timing is required, decimation cutoffs above target Nyquist, and cross-correlation cases without a finite estimate. |
| Invalid M31 envelope/calculus config or data | M31 config/core/feature tests cover invalid envelope alpha, missing/invalid windows and time constants, invalid slope thresholds, invalid time axes, unknown feature channels, unsupported feature types, and zero-RMS crest factor. |
| Invalid M32 statistics/correlation config or data | M32 config/core/feature tests cover invalid rolling windows, sigma thresholds, quantiles, histogram bins/ranges, missing comparison channels/lags, constant z-score/skew/correlation inputs, invalid lag values, and unknown feature channels. |
| Invalid M33 spectrum/time-frequency config or data | M33 config/core/feature tests cover unknown windows, invalid Welch/STFT overlap, missing window sizes, missing band limits, invalid fundamentals, above-Nyquist bands, missing sample-rate evidence, and too-short spectra. |
| Invalid M34 fault/ADC simulation config or data | M34 config/core tests cover missing seeds, missing probabilities, invalid companding mode, missing jitter or ADC-code fields, empty INL/DNL coefficients, invalid probabilities, invalid ADC ranges, invalid missing-code indexes, excessive jitter, invalid stuck-at windows, invalid quantizer ranges, and non-finite simulation input. |
| Invalid M35 multi-channel/sensor/domain config or data | M35 config/core tests cover missing output channel names, too-short channel lists, empty matrix shapes, missing sensor parameters, zero vibration windows, missing PID gains, zero rate limits, missing channels, mismatched units, duplicate output names, and invalid finite-parameter cases. |
| Unsupported transform export | M14/M18 export guardrail tests. |
| Unsupported nonlinear package transform | `examples/rule-package/unsupported-clamp-rules.toml` and `.json`; schema rejects `clamp` as `UnknownFilter`. |
| Empty batch manifest | M17 CLI batch test. |

## Benchmark Scope

The benchmark helper records local software timing only. It is not:

- live DAQ throughput evidence,
- hard real-time evidence,
- embedded runtime evidence,
- hardware qualification,
- production capacity planning,
- certification evidence.

Benchmark refreshes should record:

- command,
- generated sample count and channel count,
- machine/platform,
- Rust/Cargo versions,
- read/parse/transform/criteria/report/total timing categories,
- explicit scope limits.

M36 benchmark-readiness evidence:

- Command: `cargo run --quiet -p ferrisoxide-cli --bin ferrisoxide-signal-bench -- --input examples/m35-domain-waveform.csv --config examples/m35-domain-config.toml --iterations 5`
- Result: Pass; 4 samples, 58 channels, 54059 report bytes, and 2.164 ms average total local software time.
- Scope: local helper evidence only; no throughput, real-time, live DAQ, embedded runtime, hardware, production, qualification, or certification claim.

## MVP-Exit Validation Gaps

The current corpus is sufficient for MVP exit because every implemented surface has at least one local software evidence path and current high-risk future work is separated. Remaining post-MVP validation gaps are:

- automated Markdown link checking,
- visual regression checks for SVG output,
- hardware execution evidence,
- live DAQ SDK evidence,
- RTOS timing evidence,
- binary package loader evidence,
- certification evidence.

Those gaps do not block MVP exit because the features they would validate are outside the MVP exit scope.

## Hand-Off Note

Role: Verification and Validation Engineer / Performance Engineer
Goal: Index validation corpus and benchmark baseline evidence before MVP exit.
Files changed: `docs/validation-corpus-index.md`, docs, validation log, traceability, risk, and readiness artifacts.
Checks run: See `docs/validation-log.md`.
Status: M19 validation corpus index complete and updated through M36; M25-M36 merged through PR #175.
Known gaps: Automated corpus coverage and benchmark refresh automation remain future work; existing benchmark helper remains available.
Next recommended step: Choose one separately gated advanced follow-up or release-publication plan.
