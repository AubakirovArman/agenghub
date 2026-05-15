# Write-Ahead Log

Языки: [English](wal.en.md), [Русский](wal.ru.md), [中文](wal.zh.md), [Қазақша](wal.kk.md)

Каждая transaction теперь пишет formal WAL рядом с `journal.jsonl`.

## Файлы

```text
.agent/tx/<tx-id>/wal.jsonl
.agent/tx/<tx-id>/wal_replay.json
```

`wal.jsonl` append-only. В каждой записи есть sequence number, timestamp, transaction id, state, message, data и SHA-256 checksum. AgentHub пишет WAL record до соответствующего journal event и делает fsync.

`wal_replay.json` создаётся, когда transaction достигает `CLOSED`. Replay проверяет порядок sequence и checksums, затем записывает `record_count`, `latest_state` и все replayed records.

## Использование

Посмотреть WAL state:

```bash
sed -n '1,20p' .agent/tx/<tx-id>/wal.jsonl
cat .agent/tx/<tx-id>/wal_replay.json
```

Если WAL record отредактирован или переставлен, replay падает с checksum или sequence error.
