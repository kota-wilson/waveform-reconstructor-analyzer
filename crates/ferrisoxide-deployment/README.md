# ferrisoxide-deployment

`ferrisoxide-deployment` defines the reviewable deployment package manifest format for controller-in-the-loop workflows.

The crate is intentionally schema- and validation-focused. It does not export packages, sign artifacts, load RTOS configs, talk to hardware, bind HALs, or claim hardware qualification/certification status.

Current scope:

- deployment package metadata,
- target profile metadata,
- required artifact roles,
- manifest validation,
- checksum index text generation,
- explicit non-signing and non-certification wording.

Required artifact roles:

- `production-control-config.toml`
- `test-verification-config.toml`
- `channel-map.toml`
- `manifest.json`
- `checksum.txt`
- `qualification-report.json`
- `qualification-evidence.svg`
- `generated-at.txt`

See `docs/rtos-deployment-package-format.md` for the human-readable package contract.
