# Write-Ahead Log

Languages: [English](wal.en.md), [Русский](wal.ru.md), [中文](wal.zh.md), [Қазақша](wal.kk.md)

Every transaction now writes a formal WAL beside `journal.jsonl`.

## Files

```text
.agent/tx/<tx-id>/wal.jsonl
.agent/tx/<tx-id>/wal_replay.json
```

`wal.jsonl` is append-only. Each record has a sequence number, timestamp, transaction id, state, message, data, and SHA-256 checksum. AgentHub writes the WAL record before the matching journal event and fsyncs it.

`wal_replay.json` is generated when the transaction reaches `CLOSED`. Replay validates sequence order and checksums, then records `record_count`, `latest_state`, and all replayed records.

## Use

Inspect WAL state:

```bash
sed -n '1,20p' .agent/tx/<tx-id>/wal.jsonl
cat .agent/tx/<tx-id>/wal_replay.json
```

If a WAL record is edited or reordered, replay fails with a checksum or sequence error.
