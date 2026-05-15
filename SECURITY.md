# Security Policy

AgentHub is a local-first runtime that can execute commands through configured agents and verifier profiles. Treat security reports as high priority, especially issues involving command policy bypass, sandbox escape, secret leakage, unsafe plugin execution, rollback corruption, or untrusted release artifacts.

## Supported Versions

The project is pre-1.0. Security fixes target the current `main` branch until tagged release support is formalized.

## Reporting

Do not publish sensitive vulnerability details in public issues. Use GitHub private vulnerability reporting if it is enabled for the repository, or contact the maintainer privately with:

- affected commit or release;
- reproduction steps;
- expected and actual behavior;
- logs or artifacts with secrets removed;
- impact assessment.

## Handling

Maintainers should acknowledge reports, reproduce the issue, create a private fix branch when needed, and publish a short advisory after a patched release or mitigation is available.
