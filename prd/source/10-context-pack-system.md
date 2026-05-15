# 10. Context Pack System

## 10.1 Назначение

Context Pack — минимально достаточный пакет информации для агента.

Он должен включать только то, что нужно для текущей задачи.

## 10.2 Источники Context Pack

* AgentSpec / AgentIR;
* agent.lock;
* relevant memory;
* failed attempt fingerprints;
* skill instructions;
* workspace maps;
* relevant files/fragments;
* policies;
* verifier expectations;
* current task scope.

## 10.3 Принцип Least Context

Плохо:

```text
Вся история чата + весь проект + все логи + все правила.
```

Хорошо:

```text
15 фактов проекта + 3 решения + 2 ошибки + 4 фрагмента файлов + 1 skill + verifier profile.
```

## 10.4 Context Pack Trace

Каждый context pack должен иметь trace:

* какие memory ids были включены;
* какие skills включены;
* какие файлы включены;
* какие карты использованы;
* какие правила активны;
* какой общий token estimate.

---

