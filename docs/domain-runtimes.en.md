# Domain Runtimes

Languages: [English](domain-runtimes.en.md), [Русский](domain-runtimes.ru.md), [中文](domain-runtimes.zh.md), [Қазақша](domain-runtimes.kk.md)

Domain Runtimes describe specialized runtime packs on top of workspace profiles. A pack declares the domain, supported workspace profiles, verifier profiles, expected effects, report artifacts, memory schemas, and required local tools.

## Runtime Packs

- `code.rust`: Rust package work using `code.git`, `code_build`, `Cargo.toml`, `cargo`, and `code.memory.v1`.
- `infra.terraform`: Terraform plan work using `infra.git`, `infra_plan`, Terraform effects, `terraform`, and `infra.memory.v1`.
- `data.python`: Python/data quality work using `data.git`, `data_quality`, JSON/notebook artifacts, `python`, and `data.memory.v1`.
- `media.render`: media render work using `media.git`, `media_render`, render artifacts, `ffmpeg`, and `content.memory.v1`.
- `research.citations`: research report work using `research.git`, `research_report`, citation artifacts, and `content.memory.v1`.

## Artifacts

Every transaction writes `.agent/tx/<tx-id>/domain_runtime.json`. `report.md` includes a Domain Runtime section, and the browser dashboard payload exposes `transactions[].domain_runtime`.

Missing required tools are recorded as structured warnings in `domain_runtime.json`; they do not panic the transaction by themselves.
