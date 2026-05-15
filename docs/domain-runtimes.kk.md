# Domain Runtimes

Тілдер: [English](domain-runtimes.en.md), [Русский](domain-runtimes.ru.md), [中文](domain-runtimes.zh.md), [Қазақша](domain-runtimes.kk.md)

Domain Runtimes workspace profiles үстіндегі specialized runtime packs қабатын сипаттайды. Әр pack domain, supported workspace profiles, verifier profiles, expected effects, report artifacts, memory schemas және required local tools жариялайды.

## Runtime Packs

- `code.rust`: Rust package work үшін `code.git`, `code_build`, `Cargo.toml`, `cargo` және `code.memory.v1`.
- `infra.terraform`: Terraform plan work үшін `infra.git`, `infra_plan`, Terraform effects, `terraform` және `infra.memory.v1`.
- `data.python`: Python/data quality work үшін `data.git`, `data_quality`, JSON/notebook artifacts, `python` және `data.memory.v1`.
- `media.render`: media render work үшін `media.git`, `media_render`, render artifacts, `ffmpeg` және `content.memory.v1`.
- `research.citations`: research report work үшін `research.git`, `research_report`, citation artifacts және `content.memory.v1`.

## Artifacts

Әр transaction `.agent/tx/<tx-id>/domain_runtime.json` жазады. `report.md` Domain Runtime section қосады, ал browser dashboard payload `transactions[].domain_runtime` береді.

Жоқ required tools `domain_runtime.json` ішінде structured warnings ретінде жазылады; олар transaction-ды өздігінен panic етпейді.
