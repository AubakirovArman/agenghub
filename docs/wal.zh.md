# Write-Ahead Log

语言: [English](wal.en.md), [Русский](wal.ru.md), [中文](wal.zh.md), [Қазақша](wal.kk.md)

每个 transaction 现在都会在 `journal.jsonl` 旁写入 formal WAL。

## 文件

```text
.agent/tx/<tx-id>/wal.jsonl
.agent/tx/<tx-id>/wal_replay.json
```

`wal.jsonl` 是 append-only。每条记录包含 sequence number、timestamp、transaction id、state、message、data 和 SHA-256 checksum。AgentHub 会先写 WAL record，再写对应的 journal event，并执行 fsync。

`wal_replay.json` 在 transaction 到达 `CLOSED` 时生成。Replay 会验证 sequence 顺序和 checksums，然后记录 `record_count`、`latest_state` 和所有 replayed records。

## 使用

查看 WAL state：

```bash
sed -n '1,20p' .agent/tx/<tx-id>/wal.jsonl
cat .agent/tx/<tx-id>/wal_replay.json
```

如果 WAL record 被修改或重新排序，replay 会因 checksum 或 sequence error 失败。
