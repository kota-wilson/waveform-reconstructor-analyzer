# wra-measurements

`wra-measurements` contains reusable signal-measurement primitives for the Waveform Reconstructor and Analyzer workspace.

The crate is `#![no_std]`, allocation-free, and has no third-party dependencies. It works on caller-owned time and sample slices so desktop criteria, future reports, evidence plots, and future adapters can reuse the same measured facts without pulling in CSV parsing, file I/O, plotting, or report rendering.

## Current Measurements

- Minimum sample value.
- Maximum sample value.
- State transition count.
- Shortest or longest state run duration.
- Rise time between configured low/high thresholds.
- Fall time between configured high/low thresholds.

## Scope Limits

This crate does not define pass/fail policy, TOML configuration, report schemas, SVG rendering, DAQ acquisition, RTOS adapters, or certification evidence. `wra-core` remains responsible for converting measurement facts into criteria results and reports.
