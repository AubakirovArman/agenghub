# 13. Workspace Runtime

## 13.1 Назначение

Workspace — изолированная среда исполнения.

Интерфейс:

```text
Workspace.prepare()
Workspace.snapshot()
Workspace.run(command)
Workspace.diff()
Workspace.verify()
Workspace.commit()
Workspace.rollback()
Workspace.cleanup()
```

## 13.2 CodeWorkspace

Для программных проектов.

Backend:

* git worktree;
* branch isolation;
* diff;
* sync check;
* build/test/runtime verifier;
* dependency effects;
* source maps.

## 13.3 DataWorkspace

Для анализа данных и ML.

Backend:

* isolated Python venv;
* Jupyter kernel;
* dataset snapshots;
* artifact folder;
* notebook execution;
* metrics verifier.

## 13.4 InfraWorkspace

Для DevOps и инфраструктуры.

Backend:

* Terraform workspace;
* isolated state file;
* terraform plan;
* policy check;
* cost estimate;
* apply requires approval;
* rollback plan.

## 13.5 ContentWorkspace

Для текстов, сценариев, документов.

Backend:

* virtual filesystem;
* document diff;
* style verifier;
* repetition check;
* brand voice check;
* plagiarism/factuality check if needed.

## 13.6 MediaWorkspace

Для видео, аудио, визуального production.

Backend:

* asset workspace;
* render pipeline;
* prompt archive;
* timeline artifacts;
* TTS/STT outputs;
* render verifier.

---

