# Domain Runtimes

语言: [English](domain-runtimes.en.md), [Русский](domain-runtimes.ru.md), [中文](domain-runtimes.zh.md), [Қазақша](domain-runtimes.kk.md)

Domain Runtimes 在 workspace profiles 之上描述 specialized runtime packs。每个 pack 声明 domain、supported workspace profiles、verifier profiles、expected effects、report artifacts、memory schemas 和 required local tools。

## Runtime Packs

- `code.rust`: Rust package work，使用 `code.git`、`code_build`、`Cargo.toml`、`cargo` 和 `code.memory.v1`。
- `infra.terraform`: Terraform plan work，使用 `infra.git`、`infra_plan`、Terraform effects、`terraform` 和 `infra.memory.v1`。
- `data.python`: Python/data quality work，使用 `data.git`、`data_quality`、JSON/notebook artifacts、`python` 和 `data.memory.v1`。
- `media.render`: media render work，使用 `media.git`、`media_render`、render artifacts、`ffmpeg` 和 `content.memory.v1`。
- `research.citations`: research report work，使用 `research.git`、`research_report`、citation artifacts 和 `content.memory.v1`。

## Artifacts

每个 transaction 都会写入 `.agent/tx/<tx-id>/domain_runtime.json`。`report.md` 包含 Domain Runtime section，browser dashboard payload 暴露 `transactions[].domain_runtime`。

缺失的 required tools 会作为 structured warnings 写入 `domain_runtime.json`；它们本身不会让 transaction panic。
