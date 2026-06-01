# DAQ Input Abstraction

Status: implemented fixture/test-double boundary for M9-004 / issue #79.

Crate: `crates/ferrisoxide-daq`

## Purpose

The DAQ input abstraction provides a deterministic way to feed controller-in-the-loop workflows without vendor SDKs, live hardware, drivers, global setup, or operating-system device APIs.

Use this crate for:

- fixture sample sources,
- test-double sample sources,
- deterministic channel descriptors,
- monotonic timestamp validation,
- missing-channel validation,
- non-finite analog value validation,
- future desktop simulation workflow input plumbing.

Do not use this crate for:

- live DAQ hardware access,
- vendor SDK binding,
- driver installation,
- global system configuration,
- controller output I/O,
- hardware qualification evidence,
- real-time acquisition guarantees.

## Current API Boundary

The crate defines:

| Type | Purpose |
|---|---|
| `DaqChannel` | Channel ID, source string, and unit metadata. |
| `DaqSampleFrame` | One timestamped set of channel values. |
| `DaqSampleValue` | Analog or digital sample value. |
| `DaqSourceDescriptor` | Source name, source kind, and channel list. |
| `DaqSampleSource` | Trait for deterministic frame producers. |
| `FixtureDaqSource` | In-memory fixture/test-double source. |
| `collect_frames` | Helper to collect bounded frame batches from a source. |
| `DaqError` | Structured errors for bad descriptors or frames. |

## Fixture Source Example

```rust
use ferrisoxide_daq::{
    DaqChannel, DaqSampleFrame, DaqSampleValue, DaqSourceDescriptor, DaqSourceKind,
    FixtureDaqSource, collect_frames,
};

let descriptor = DaqSourceDescriptor {
    name: "heated-actuator-fixture".to_string(),
    kind: DaqSourceKind::Fixture,
    channels: vec![
        DaqChannel::new("command", "fixture.command_v", "V"),
        DaqChannel::new("feedback", "fixture.feedback_high", "bool"),
    ],
};

let frames = vec![
    DaqSampleFrame::new(0.000)
        .with_value("command", DaqSampleValue::Analog { value: 0.0 })
        .with_value("feedback", DaqSampleValue::Digital { high: false }),
    DaqSampleFrame::new(0.001)
        .with_value("command", DaqSampleValue::Analog { value: 5.0 })
        .with_value("feedback", DaqSampleValue::Digital { high: true }),
];

let mut source = FixtureDaqSource::new(descriptor, frames)?;
let collected = collect_frames(&mut source, 16)?;
```

## Validation Rules

`FixtureDaqSource::new()` validates:

- at least one channel exists,
- channel IDs, sources, and units are non-empty,
- channel IDs are unique,
- at least one frame exists,
- timestamps are finite and strictly increasing,
- every frame includes every declared channel,
- analog values are finite.

Errors are structured `DaqError` values. Bad fixtures should fail clearly before simulator or analysis code consumes them.

## Future SDK Gates

Live DAQ SDK work is intentionally out of scope. Before any vendor SDK, driver, or hardware acquisition path is added, run these gates:

| Gate | Required Evidence |
|---|---|
| Dependency Gate | SDK crate/license review, transitive dependency review, version pinning, and rollback plan. |
| Environment Gate | Project-local setup instructions, no global installation by default, supported OS/architecture, and cleanup plan. |
| Security Gate | No secrets in config, no unnecessary device permissions, no unsafe FFI without review, and explicit trust boundary notes. |
| Hardware Gate | Required hardware, cabling, calibration assumptions, sampling limits, and safe failure behavior. |
| V&V Gate | Fixture parity tests that prove live-source mapping matches deterministic fixture behavior for known samples. |

## Current Limits

This crate does not yet map DAQ channels into simulator input IDs, does not parse CSV files, does not stream from hardware, and does not own controller output I/O. The desktop simulation workflow issue should connect this abstraction to simulator input mapping later.
