# PRD Heading Index

Generated from [`../../prd.md`](../../prd.md).

| Line | Level | Heading |
|---:|---:|---|
| 1 | 1 | PRD v1 |
| 3 | 2 | 0. Назначение документа |
| 13 | 1 | 1. Executive Summary |
| 15 | 2 | 1.1 Что такое AgentHub |
| 35 | 2 | 1.2 Что именно строится |
| 57 | 2 | 1.3 Главная формула системы |
| 91 | 1 | 2. Product Vision |
| 93 | 2 | 2.1 Видение |
| 117 | 2 | 2.2 Философия |
| 134 | 1 | 3. Positioning |
| 136 | 2 | 3.1 Чем AgentHub не является |
| 152 | 2 | 3.2 Чем AgentHub является |
| 167 | 2 | 3.3 Короткое позиционирование |
| 193 | 1 | 4. Problem Statement |
| 195 | 2 | 4.1 Современная проблема AI-агентов |
| 201 | 3 | 4.1.1 State Drift |
| 211 | 3 | 4.1.2 Context Bloat |
| 224 | 3 | 4.1.3 No Transactionality |
| 237 | 3 | 4.1.4 Memory Pollution |
| 247 | 3 | 4.1.5 Weak Verification |
| 266 | 3 | 4.1.6 Poor Observability |
| 280 | 3 | 4.1.7 No Cross-Agent Continuity |
| 288 | 1 | 5. Target Users |
| 290 | 2 | 5.1 Primary Users |
| 292 | 3 | 5.1.1 AI-heavy developers |
| 307 | 3 | 5.1.2 AI engineers / agent builders |
| 320 | 3 | 5.1.3 Solo founders / product builders |
| 333 | 3 | 5.1.4 Enterprise teams |
| 348 | 2 | 5.2 Secondary Users |
| 363 | 1 | 6. Core Principles / Kernel Invariants |
| 365 | 2 | 6.1 Главные законы AgentHub |
| 367 | 3 | Law 1 — Atomicity |
| 375 | 3 | Law 2 — Memory Consistency |
| 383 | 3 | Law 3 — Isolation |
| 391 | 3 | Law 4 — Rollbackability |
| 399 | 3 | Law 5 — Failed Experience Durability |
| 407 | 3 | Law 6 — No Blind Merge |
| 415 | 3 | Law 7 — Scope Enforcement |
| 423 | 3 | Law 8 — Observability First |
| 431 | 3 | Law 9 — Least Context |
| 437 | 3 | Law 10 — Domain via Plugins |
| 445 | 1 | 7. High-Level Architecture |
| 447 | 2 | 7.1 Macro Architecture |
| 546 | 2 | 7.2 Core Architectural Decision |
| 581 | 1 | 8. AgentHub Layers |
| 583 | 2 | 8.1 Interface Layer |
| 585 | 3 | 8.1.1 CLI / SLI |
| 603 | 3 | 8.1.2 TUI |
| 615 | 3 | 8.1.3 Web Dashboard |
| 628 | 3 | 8.1.4 VS Code Extension |
| 643 | 2 | 8.2 Intent Layer |
| 645 | 3 | 8.2.1 Natural Language Input |
| 653 | 3 | 8.2.2 Intent Normalizer |
| 666 | 3 | 8.2.3 Clarification Engine |
| 687 | 3 | 8.2.4 Default Resolver |
| 706 | 2 | 8.3 Language Layer |
| 708 | 2 | 8.3.1 AAL — Agent Action Language |
| 764 | 2 | 8.3.2 AgentSpec |
| 811 | 2 | 8.3.3 AgentIR |
| 833 | 1 | 9. VCM-OS Memory Layer |
| 835 | 2 | 9.1 Назначение |
| 846 | 2 | 9.2 Core Memory Types |
| 868 | 2 | 9.3 Domain Schemas |
| 872 | 3 | 9.3.1 Code Project Schema |
| 888 | 3 | 9.3.2 Infra Project Schema |
| 899 | 3 | 9.3.3 Data/ML Schema |
| 911 | 3 | 9.3.4 Content Schema |
| 924 | 3 | 9.3.5 Media Schema |
| 935 | 2 | 9.4 Memory Lifecycle |
| 953 | 2 | 9.5 Staging Memory vs Committed Memory |
| 974 | 2 | 9.6 Failed Attempt Memory |
| 1004 | 2 | 9.7 Memory Compaction |
| 1039 | 1 | 10. Context Pack System |
| 1041 | 2 | 10.1 Назначение |
| 1047 | 2 | 10.2 Источники Context Pack |
| 1060 | 2 | 10.3 Принцип Least Context |
| 1074 | 2 | 10.4 Context Pack Trace |
| 1087 | 1 | 11. Agent Lock |
| 1089 | 2 | 11.1 Назначение |
| 1095 | 2 | 11.2 Содержимое agent.lock |
| 1138 | 2 | 11.3 Правила |
| 1147 | 1 | 12. Skill Registry |
| 1149 | 2 | 12.1 Назначение |
| 1168 | 2 | 12.2 Skill Manifest |
| 1209 | 2 | 12.3 Skill Types |
| 1211 | 3 | Code Skills |
| 1226 | 3 | Design Skills |
| 1237 | 3 | Infra Skills |
| 1246 | 3 | Data Skills |
| 1254 | 3 | Content Skills |
| 1263 | 2 | 12.4 Progressive Disclosure |
| 1291 | 1 | 13. Workspace Runtime |
| 1293 | 2 | 13.1 Назначение |
| 1310 | 2 | 13.2 CodeWorkspace |
| 1324 | 2 | 13.3 DataWorkspace |
| 1337 | 2 | 13.4 InfraWorkspace |
| 1351 | 2 | 13.5 ContentWorkspace |
| 1364 | 2 | 13.6 MediaWorkspace |
| 1379 | 1 | 14. Transaction Manager |
| 1381 | 2 | 14.1 Назначение |
| 1399 | 2 | 14.2 Transaction Lifecycle |
| 1431 | 2 | 14.3 Failure States |
| 1441 | 2 | 14.4 BLOCKED_ON_HUMAN |
| 1455 | 2 | 14.5 Sync Check |
| 1472 | 2 | 14.6 Diff Guard |
| 1494 | 2 | 14.7 Effect Tracking |
| 1518 | 1 | 15. Verifier Layer |
| 1520 | 2 | 15.1 Назначение |
| 1524 | 2 | 15.2 Verifier Profiles |
| 1526 | 3 | 15.2.1 code_build |
| 1533 | 3 | 15.2.2 web_runtime_smoke |
| 1542 | 3 | 15.2.3 backend_tdd |
| 1549 | 3 | 15.2.4 db_migration |
| 1556 | 3 | 15.2.5 infra_plan |
| 1564 | 3 | 15.2.6 data_quality |
| 1572 | 3 | 15.2.7 content_quality |
| 1581 | 2 | 15.3 Runtime Smoke Example |
| 1602 | 1 | 16. Agent Orchestration |
| 1604 | 2 | 16.1 Agent Adapters |
| 1617 | 2 | 16.2 Topologies |
| 1619 | 3 | 16.2.1 Single Executor |
| 1623 | 3 | 16.2.2 Planner → Executor |
| 1627 | 3 | 16.2.3 Generator → Critic |
| 1631 | 3 | 16.2.4 Executor → Reviewer → Repair |
| 1635 | 3 | 16.2.5 Swarm Research |
| 1639 | 3 | 16.2.6 Manager / Worker |
| 1643 | 3 | 16.2.7 Tournament |
| 1647 | 2 | 16.3 Routing Policy |
| 1664 | 1 | 17. LLM Gateway and Observability |
| 1666 | 2 | 17.1 Назначение |
| 1685 | 2 | 17.2 Redaction |
| 1708 | 2 | 17.3 Cost Profiler |
| 1728 | 2 | 17.4 Transaction Report |
| 1756 | 1 | 18. `.agent/` Project Structure |
| 1758 | 2 | 18.1 Proposed Structure |
| 1810 | 2 | 18.2 Source of Truth |
| 1821 | 1 | 19. Security and Policy |
| 1823 | 2 | 19.1 Security Principles |
| 1835 | 2 | 19.2 Command Policy |
| 1857 | 2 | 19.3 Sandbox Levels |
| 1859 | 3 | Level 0 — Local Controlled |
| 1867 | 3 | Level 1 — Local Sandbox |
| 1873 | 3 | Level 2 — Strong Isolation |
| 1880 | 3 | Level 3 — Enterprise Runner |
| 1889 | 1 | 20. Domain Profiles |
| 1891 | 2 | 20.1 AgentHub Code |
| 1907 | 2 | 20.2 AgentHub Infra |
| 1921 | 2 | 20.3 AgentHub Data |
| 1934 | 2 | 20.4 AgentHub Content |
| 1947 | 2 | 20.5 AgentHub Media |
| 1959 | 2 | 20.6 AgentHub Research |
| 1974 | 1 | 21. Development Roadmap |
| 1978 | 2 | Phase 1 — Execution Kernel Foundation |
| 2004 | 2 | Phase 2 — Observability and LLM Gateway |
| 2022 | 2 | Phase 3 — VCM-OS Core |
| 2040 | 2 | Phase 4 — AgentSpec YAML and Compiler |
| 2057 | 2 | Phase 5 — Skill Registry v1 |
| 2074 | 2 | Phase 6 — Agent Adapters v1 |
| 2090 | 2 | Phase 7 — Runtime Smoke and Repair Loop |
| 2107 | 2 | Phase 8 — Context Maps |
| 2122 | 2 | Phase 9 — Natural Language to AgentSpec |
| 2138 | 2 | Phase 10 — Advanced Agent Topologies |
| 2154 | 2 | Phase 11 — Additional Workspaces |
| 2168 | 2 | Phase 12 — IDE and Visual Layer |
| 2183 | 2 | Phase 13 — Marketplace / Plugin Ecosystem |
| 2199 | 2 | Phase 14 — Enterprise Layer |
| 2217 | 1 | 22. Technical Stack Recommendation |
| 2219 | 2 | 22.1 Core |
| 2239 | 2 | 22.2 UI / IDE |
| 2253 | 2 | 22.3 Research / ML Plugins |
| 2269 | 2 | 22.4 Storage |
| 2280 | 1 | 23. Success Metrics |
| 2282 | 2 | 23.1 Reliability Metrics |
| 2291 | 2 | 23.2 Context Efficiency Metrics |
| 2299 | 2 | 23.3 Quality Metrics |
| 2309 | 2 | 23.4 User Trust Metrics |
| 2317 | 2 | 23.5 Cost Metrics |
| 2327 | 1 | 24. Major Risks |
| 2329 | 2 | 24.1 Over-Abstraction Risk |
| 2339 | 2 | 24.2 Weak Verifier Risk |
| 2351 | 2 | 24.3 Memory Pollution Risk |
| 2361 | 2 | 24.4 Security Risk |
| 2373 | 2 | 24.5 Cost Explosion Risk |
| 2385 | 2 | 24.6 Skill Quality Risk |
| 2399 | 1 | 25. Open Questions |
| 2401 | 2 | 25.1 Language |
| 2408 | 2 | 25.2 Memory |
| 2415 | 2 | 25.3 Workspace |
| 2421 | 2 | 25.4 Skills |
| 2428 | 2 | 25.5 Agent Routing |
| 2435 | 2 | 25.6 Enterprise |
| 2444 | 1 | 26. First Concrete Reference Scenario |
| 2448 | 2 | Scenario |
| 2492 | 1 | 27. Strategic Connection to WAL and VCM-OS |
| 2494 | 2 | 27.1 VCM-OS |
| 2507 | 2 | 27.2 WAL |
| 2530 | 1 | 28. Final Product Thesis |
| 2544 | 1 | 29. Immediate Next Document Candidates |
| 2571 | 1 | 30. Current Decision Snapshot |
| 2595 | 1 | 31. Short North Star |
