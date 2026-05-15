# Phase 13 — Marketplace / Plugin Ecosystem

Status: Done

Existing evidence: `ddd84ca Add plugin package installation foundation`

Closing evidence: `ab46f97`.

## Deliverables

- Skill package format: done.
- Workspace plugin format: done.
- Verifier plugin format: done.
- Versioning: done.
- Trust model: done.
- Signing optional: done.

## Acceptance

- External author can publish a skill: done.
- Project can install and lock skill versions: done.

## Verification

- Added `agenthub plugins scaffold` authoring flow for publishable packages.
- Manifest validation checks semver package versions, safe paths, referenced skill manifests, and workspace schemas.
- Workspace/verifier plugin metadata supports profiles, capabilities, artifacts, timeout, and lock snapshots.
- Optional signing is a metadata stub recorded in locks; trust enforcement remains `--trust`.
- Updated plugin ecosystem docs and README examples on 4 languages.
- Verified inspect, scaffold, install, and list CLI flows.
