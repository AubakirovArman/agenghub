# Smart Sync

Тілдер: [English](smart-sync.en.md), [Русский](smart-sync.ru.md), [中文](smart-sync.zh.md), [Қазақша](smart-sync.kk.md)

Smart Sync ескі HEAD-only block орнына file-level overlap detection қолданады. Transaction кезінде project `HEAD` өзгерсе, AgentHub main changed files пен transaction changed files салыстырады.

## Decisions

- `clean`: project `HEAD` өзгермеді.
- `rebase_required`: project `HEAD` өзгерді, бірақ file sets overlap жоқ. AgentHub `git rebase --autostash` іске қосады, diff guard және verifier қайта жүргізеді, содан кейін commit жасайды.
- `blocked_overlap`: project `HEAD` өзгерді және екі жақ бір same path өзгертті. AgentHub blind merge жасамай, human block қояды.

## Artifacts

```text
.agent/tx/<tx-id>/baseline.json
.agent/tx/<tx-id>/sync.json
.agent/tx/<tx-id>/report.md
```

`baseline.json` transaction `base_head`, `scope.allow` ішінен scoped file hashes және context maps арқылы таңдалған relevant file hashes сақтайды.

`sync.json` base/current heads, main changed files, transaction changed files, overlaps және verifier rerun flag сақтайды.

Мысал:

```json
{
  "decision": "rebase_required",
  "overlapping_files": [],
  "verifier_rerun_required": true
}
```
