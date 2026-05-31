# ARM64 Embedded Track

The ARM64 track is intentionally staged:

- `qemu/`: first bare-metal proof target.
- `bare-metal/`: future board-agnostic notes and examples.
- `zephyr/`: optional feasibility work after no_std primitives and adapter boundaries exist.

Current status:

- `qemu/` contains a host-checkable no_std proof slice for the adapter path.
- `zephyr/` contains an isolated feasibility sketch and risk notes.
- `crates/wra-embedded` defines the runtime adapter traits.

The current milestone does not add a production RTOS integration, board HAL, DAQ path, or certification evidence.
