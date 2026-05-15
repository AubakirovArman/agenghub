# Domain Runtimes

Языки: [English](domain-runtimes.en.md), [Русский](domain-runtimes.ru.md), [中文](domain-runtimes.zh.md), [Қазақша](domain-runtimes.kk.md)

Domain Runtimes описывают специализированные runtime packs поверх workspace profiles. Pack объявляет domain, supported workspace profiles, verifier profiles, ожидаемые effects, report artifacts, memory schemas и required local tools.

## Runtime Packs

- `code.rust`: работа с Rust package через `code.git`, `code_build`, `Cargo.toml`, `cargo` и `code.memory.v1`.
- `infra.terraform`: Terraform plan через `infra.git`, `infra_plan`, Terraform effects, `terraform` и `infra.memory.v1`.
- `data.python`: Python/data quality через `data.git`, `data_quality`, JSON/notebook artifacts, `python` и `data.memory.v1`.
- `media.render`: media render через `media.git`, `media_render`, render artifacts, `ffmpeg` и `content.memory.v1`.
- `research.citations`: research report через `research.git`, `research_report`, citation artifacts и `content.memory.v1`.

## Артефакты

Каждая транзакция пишет `.agent/tx/<tx-id>/domain_runtime.json`. `report.md` содержит секцию Domain Runtime, а browser dashboard payload отдаёт `transactions[].domain_runtime`.

Отсутствующие required tools записываются как structured warnings в `domain_runtime.json`; сами по себе они не вызывают panic транзакции.
