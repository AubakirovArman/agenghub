# Write-Ahead Log

Тілдер: [English](wal.en.md), [Русский](wal.ru.md), [中文](wal.zh.md), [Қазақша](wal.kk.md)

Әр transaction енді `journal.jsonl` жанында formal WAL жазады.

## Файлдар

```text
.agent/tx/<tx-id>/wal.jsonl
.agent/tx/<tx-id>/wal_replay.json
```

`wal.jsonl` append-only. Әр record ішінде sequence number, timestamp, transaction id, state, message, data және SHA-256 checksum бар. AgentHub WAL record жазбасын matching journal event алдында жазады және fsync жасайды.

`wal_replay.json` transaction `CLOSED` күйіне жеткенде жасалады. Replay sequence order және checksums тексереді, кейін `record_count`, `latest_state` және барлық replayed records жазады.

## Қолдану

WAL state көру:

```bash
sed -n '1,20p' .agent/tx/<tx-id>/wal.jsonl
cat .agent/tx/<tx-id>/wal_replay.json
```

WAL record өзгертілсе немесе орындары ауысса, replay checksum немесе sequence error арқылы fail болады.
