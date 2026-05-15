# PRD v4

Тілдер: [English](prd-v4.en.md), [Русский](prd-v4.ru.md), [中文](prd-v4.zh.md), [Қазақша](prd-v4.kk.md)

PRD v4 AgentHub-ты бірінші tagged local developer preview үшін дайындайды: `v0.2.0-local-preview`.

## Scope

- Package version мәнін `0.2.0-local-preview` деңгейіне көтеру.
- Known limitations төрт тілде жазу.
- Repeatable local product checks үшін `scripts/dogfood.sh` қосу.
- Release validation, packaging, local install, `version` және `doctor` үшін `scripts/release-readiness.sh` қосу.
- GitHub Release assets тек Linux, macOS және Windows CI жасыл болғаннан кейін жариялау.
- Owner approval кейін project `UNLICENSED` күйінен Apache-2.0 open source күйіне ауыстыру.

## Scope құрамына кірмейді

PRD v4 hosted SaaS қоспайды және full security sandbox бар деп мәлімдемейді. Бұлар later product немесе hardening tracks болып қалады.
