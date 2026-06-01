# Heated Actuator Deployment Package Example

This directory is a reviewable RTOS/controller deployment package format example for M9-007.

It is software evidence only. It is not a signed release, not an authenticated package, not a hardware qualification artifact, and not certification evidence.

## Artifact Set

| Artifact | Role |
|---|---|
| `production-control-config.toml` | Production control behavior config. |
| `test-verification-config.toml` | Test verification criteria config. |
| `channel-map.toml` | Mapping from CSV/fixture channels to logical control and verification channels. |
| `manifest.json` | Deployment package manifest consumed by the schema validator. |
| `checksum.txt` | Non-cryptographic artifact drift index. |
| `qualification-report.json` | Example software qualification evidence report. |
| `qualification-evidence.svg` | Example visual evidence artifact. |
| `generated-at.txt` | Deterministic generation timestamp for this example. |

## Scope

The package keeps production and test configs separate and links them through the manifest. Future RTOS or bare-metal loaders should validate this package shape before consuming any constrained runtime subset.
