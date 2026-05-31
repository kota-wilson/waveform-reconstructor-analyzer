# Validation Dataset Area

This folder is reserved for v0.3.0 signal accuracy and validation work.

The validation area is not a production qualification package and does not provide hardware validation, environmental qualification, or certification evidence. It is for known-answer fixtures, expected reports, filter-response checks, and engineering review examples that strengthen confidence in the analyzer.

## Layout

| Folder | Purpose |
|---|---|
| `known_answer/` | Waveforms with precomputed expected measurements. |
| `environmental_cases/` | Environmental-style examples with stated intent and limits. |
| `filter_response/` | Filter behavior fixtures and equation-backed expected outputs. |
| `reports/` | Expected reports generated from validation fixtures. |
| `benchmarks/` | Repeatable large-CSV benchmark strategy and baseline notes. |

Each future validation case should include source notes, expected values, tolerance policy, analyzer command, expected output, and scope limits.
