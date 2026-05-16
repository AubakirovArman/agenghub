# V4.03 Checksum-Verified Installers

## Status

Done.

## Completed

- POSIX installer verifies SHA-256 before extracting release archives.
- Windows installer verifies SHA-256 before extracting release archives.
- Remote installs download the matching `.sha256` asset automatically.
- Local artifact installs use adjacent `<archive>.sha256`, `AGENTHUB_CHECKSUM`, or `AGENTHUB_CHECKSUM_FILE`.
- Emergency/debug installs can opt out with `AGENTHUB_SKIP_CHECKSUM=1`.
- Install and release engineering docs were updated in English, Russian, Chinese, and Kazakh.

## 1.0 Relevance

This closes a supply-chain gap in the local release path. Package artifacts are already signed by GitHub release provenance indirectly through the release workflow; checksum verification gives users and release-readiness checks a deterministic local integrity gate before installation.
