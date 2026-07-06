# Graph Report - wallpaper  (2026-07-06)

## Corpus Check
- 152 files · ~102,948 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1308 nodes · 1749 edges · 107 communities (90 shown, 17 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 20 edges (avg confidence: 0.74)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e391f951`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- [[_COMMUNITY_runtime.ts|runtime.ts]]
- [[_COMMUNITY_mod.rs|mod.rs]]
- [[_COMMUNITY_WallpaperApp|WallpaperApp]]
- [[_COMMUNITY_Invocation|Invocation]]
- [[_COMMUNITY_session-catchup.py|session-catchup.py]]
- [[_COMMUNITY_session-catchup.py|session-catchup.py]]
- [[_COMMUNITY_run_worker|run_worker]]
- [[_COMMUNITY_Issue tracker GitHub|Issue tracker: GitHub]]
- [[_COMMUNITY_Triage|Triage]]
- [[_COMMUNITY_compress.py|compress.py]]
- [[_COMMUNITY_Process|Process]]
- [[_COMMUNITY_SKILL|SKILL.md]]
- [[_COMMUNITY_validate.py|validate.py]]
- [[_COMMUNITY_What You Must Do When Invoked|What You Must Do When Invoked]]
- [[_COMMUNITY_Codebase Design|Codebase Design]]
- [[_COMMUNITY_README|README.md]]
- [[_COMMUNITY_During the session|During the session]]
- [[_COMMUNITY_HTML Report Format|HTML Report Format]]
- [[_COMMUNITY_package.json|package.json]]
- [[_COMMUNITY_Reference Manus Context Engineering Principles|Reference: Manus Context Engineering Principles]]
- [[_COMMUNITY_Planning with Files|Planning with Files]]
- [[_COMMUNITY_Reference Manus Context Engineering Principles|Reference: Manus Context Engineering Principles]]
- [[_COMMUNITY_Task Plan Brief Description|Task Plan: [Brief Description]]]
- [[_COMMUNITY_Examples Planning with Files in Action|Examples: Planning with Files in Action]]
- [[_COMMUNITY_Examples Planning with Files in Action|Examples: Planning with Files in Action]]
- [[_COMMUNITY_Planning with Files|Planning with Files]]
- [[_COMMUNITY_SKILL|SKILL.md]]
- [[_COMMUNITY_Diagnosing Bugs|Diagnosing Bugs]]
- [[_COMMUNITY_package.json|package.json]]
- [[_COMMUNITY_Task Plan Analytics Project Description|Task Plan: [Analytics Project Description]]]
- [[_COMMUNITY_Task Plan Analytics Project Description|Task Plan: [Analytics Project Description]]]
- [[_COMMUNITY_Test-Driven Development|Test-Driven Development]]
- [[_COMMUNITY_Caveman Compress|Caveman Compress]]
- [[_COMMUNITY_SKILL|SKILL.md]]
- [[_COMMUNITY_Process|Process]]
- [[_COMMUNITY_caveman-commit|caveman-commit]]
- [[_COMMUNITY_caveman-review|caveman-review]]
- [[_COMMUNITY_Pi Planning With Files|Pi Planning With Files]]
- [[_COMMUNITY_Findings & Decisions|Findings & Decisions]]
- [[_COMMUNITY_init-session.sh|init-session.sh]]
- [[_COMMUNITY_Findings & Decisions|Findings & Decisions]]
- [[_COMMUNITY_init-session.sh|init-session.sh]]
- [[_COMMUNITY_Ask Matt|Ask Matt]]
- [[_COMMUNITY_Findings & Decisions|Findings & Decisions]]
- [[_COMMUNITY_Findings & Decisions|Findings & Decisions]]
- [[_COMMUNITY_Repository Guidelines|Repository Guidelines]]
- [[_COMMUNITY_Repository Guidelines|Repository Guidelines]]
- [[_COMMUNITY_Wallpaper Manager|Wallpaper Manager]]
- [[_COMMUNITY_resolve-plan-dir.sh|resolve-plan-dir.sh]]
- [[_COMMUNITY_ledger-append.sh|ledger-append.sh]]
- [[_COMMUNITY_resolve-plan-dir.sh|resolve-plan-dir.sh]]
- [[_COMMUNITY_SKILL|SKILL.md]]
- [[_COMMUNITY_Repository Guidelines Template|Repository Guidelines Template]]
- [[_COMMUNITY_graphify reference extra exports and benchmark|graphify reference: extra exports and benchmark]]
- [[_COMMUNITY_Process|Process]]
- [[_COMMUNITY_Critical Rules|Critical Rules]]
- [[_COMMUNITY_Progress Log|Progress Log]]
- [[_COMMUNITY_Progress Log|Progress Log]]
- [[_COMMUNITY_caveman-stats|caveman-stats]]
- [[_COMMUNITY_Task Plan Brief Description|Task Plan: [Brief Description]]]
- [[_COMMUNITY_Task Plan Brief Description|Task Plan: [Brief Description]]]
- [[_COMMUNITY_check-complete.sh|check-complete.sh]]
- [[_COMMUNITY_Phases|Phases]]
- [[_COMMUNITY_check-complete.sh|check-complete.sh]]
- [[_COMMUNITY_phase-status.sh|phase-status.sh]]
- [[_COMMUNITY_Phases|Phases]]
- [[_COMMUNITY_graphify reference query, path, explain|graphify reference: query, path, explain]]
- [[_COMMUNITY_attest-plan.sh|attest-plan.sh]]
- [[_COMMUNITY_attest-plan.sh|attest-plan.sh]]
- [[_COMMUNITY_hitl-loop.template.sh|hitl-loop.template.sh]]
- [[_COMMUNITY_planning-with-files Pi Extension|planning-with-files Pi Extension]]
- [[_COMMUNITY_inject-plan.sh|inject-plan.sh]]
- [[_COMMUNITY_GLOSSARY.md Format|GLOSSARY.md Format]]
- [[_COMMUNITY_graphify reference add a URL and watch a folder|graphify reference: add a URL and watch a folder]]
- [[_COMMUNITY_graphify reference commit hook and native CLAUDE.md integration|graphify reference: commit hook and native CLAUDE.md integration]]
- [[_COMMUNITY_graphify reference incremental update and cluster-only|graphify reference: incremental update and cluster-only]]
- [[_COMMUNITY_ledger-summary.sh|ledger-summary.sh]]
- [[_COMMUNITY_graphify reference GitHub clone and cross-repo merge|graphify reference: GitHub clone and cross-repo merge]]
- [[_COMMUNITY_graphify reference transcribe video and audio|graphify reference: transcribe video and audio]]
- [[_COMMUNITY_main|main]]
- [[_COMMUNITY_graphify|graphify.md]]
- [[_COMMUNITY___init__.py|__init__.py]]
- [[_COMMUNITY_set-active-plan.sh script|set-active-plan.sh script]]
- [[_COMMUNITY_gate-stop.sh|gate-stop.sh]]
- [[_COMMUNITY_set-active-plan.sh script|set-active-plan.sh script]]
- [[_COMMUNITY_graphify|graphify.md]]
- [[_COMMUNITY_extraction-spec|extraction-spec.md]]

## God Nodes (most connected - your core abstractions)
1. `WallpaperApp` - 38 edges
2. `planningWithFilesExtension()` - 24 edges
3. `Planning with Files` - 18 edges
4. `run_worker()` - 17 edges
5. `Planning with Files` - 17 edges
6. `AppSettings` - 16 edges
7. `validate()` - 14 edges
8. `StitchOrientation` - 13 edges
9. `AppState` - 13 edges
10. `compress_file()` - 12 edges

## Surprising Connections (you probably didn't know these)
- `run_worker()` --calls--> `process_image()`  [INFERRED]
  src/slideshow/mod.rs → src/image_ops/mod.rs
- `run_worker()` --calls--> `stitch_images()`  [INFERRED]
  src/slideshow/mod.rs → src/image_ops/mod.rs
- `run_worker()` --calls--> `set_wallpaper()`  [INFERRED]
  src/slideshow/mod.rs → src/wallpaper/wallpaper.rs
- `WallpaperApp` --references--> `AppSettings`  [EXTRACTED]
  src/app/mod.rs → src/settings/mod.rs
- `WallpaperApp` --references--> `SlideshowWorker`  [EXTRACTED]
  src/app/mod.rs → src/slideshow/mod.rs

## Import Cycles
- None detected.

## Communities (107 total, 17 thin omitted)

### Community 0 - "runtime.ts"
Cohesion: 0.05
Nodes (59): AttestationCheck, checkPlanAttestation(), normalizeHash(), sha256File(), isAllPhasesComplete(), isPlanIncomplete(), isSessionAttached(), PlanPaths (+51 more)

### Community 1 - "mod.rs"
Cohesion: 0.07
Nodes (56): Color32, Default, DynamicImage, RgbImage, style_label(), Language, apply_smart_rotation(), cache_file_path() (+48 more)

### Community 2 - "WallpaperApp"
Cohesion: 0.08
Nodes (37): App, Arc, AtomicBool, CreationContext, Drop, FnOnce, Frame, HWND (+29 more)

### Community 3 - "Invocation"
Cohesion: 0.05
Nodes (40): Branch, Co-location, Cognitive Load, Completion Criterion, Context Load, Context Pointer, Description, Duplication (+32 more)

### Community 4 - "session-catchup.py"
Cohesion: 0.12
Nodes (40): codex_meta_cwd(), codex_planning_update(), extract_messages_after(), find_current_codex_session(), find_last_planning_update(), _format_opencode_part(), get_claude_project_dir(), get_codex_sessions() (+32 more)

### Community 5 - "session-catchup.py"
Cohesion: 0.12
Nodes (40): codex_meta_cwd(), codex_planning_update(), extract_messages_after(), find_current_codex_session(), find_last_planning_update(), _format_opencode_part(), get_claude_project_dir(), get_codex_sessions() (+32 more)

### Community 6 - "run_worker"
Cohesion: 0.20
Nodes (18): ChaChaRng, Duration, JoinHandle, Receiver, Sender, pick_random_with_rng(), Option, PathBuf (+10 more)

### Community 7 - "Issue tracker: GitHub"
Cohesion: 0.06
Nodes (30): Before exploring, read these, Domain Docs, File structure, Flag ADR conflicts, Use the glossary's vocabulary, Conventions, Issue tracker: GitHub, Pull requests as a triage surface (+22 more)

### Community 8 - "Triage"
Cohesion: 0.06
Nodes (29): Bad agent brief, Behavioral, not procedural, Complete acceptance criteria, Durability over precision, Examples, Explicit scope boundaries, Good agent brief (bug), Good agent brief (enhancement) (+21 more)

### Community 9 - "compress.py"
Cohesion: 0.07
Nodes (49): benchmark_pair(), count_tokens(), main(), print_table(), Path, main(), print_usage(), backup_dir_for() (+41 more)

### Community 10 - "Process"
Cohesion: 0.07
Nodes (28): 1. State the question, 2. Pick the language, 3. Isolate the logic in a portable module, 4. Build the smallest TUI that exposes the state, 5. Make it runnable in one command, 6. Hand it over, 7. Capture the answer, Anti-patterns (+20 more)

### Community 11 - "SKILL.md"
Cohesion: 0.07
Nodes (25): Learning Record Format, Numbering, Optional sections, Supersession, Template, What does _not_ qualify, When to write a learning record, MISSION.md Format (+17 more)

### Community 12 - "validate.py"
Cohesion: 0.29
Nodes (7): Principle 1: Design Around KV-Cache, Principle 2: Mask, Don't Remove, Principle 3: Filesystem as External Memory, Principle 4: Manipulate Attention Through Recitation, Principle 5: Keep the Wrong Stuff In, Principle 6: Don't Get Few-Shotted, The 6 Manus Principles

### Community 13 - "What You Must Do When Invoked"
Cohesion: 0.08
Nodes (24): For /graphify add and --watch, For /graphify query, For the commit hook and native CLAUDE.md integration, For --update and --cluster-only, /graphify, Honesty Rules, Interpreter guard for subcommands, Part A - Structural extraction for code files (+16 more)

### Community 14 - "Codebase Design"
Cohesion: 0.09
Nodes (21): 1. In-process, 2. Local-substitutable, 3. Remote but owned (Ports & Adapters), 4. True external (Mock), Deepening, Dependency categories, Seam discipline, Testing strategy: replace, don't layer (+13 more)

### Community 15 - "README.md"
Cohesion: 0.09
Nodes (20): Before / After, Benchmarks, How It Work, <img src="../../docs/assets/dancing-rock.svg" width="20" height="20" alt="rock"/> Caveman (285 tokens), Install, 📄 Original (706 tokens), Part of Caveman, Security (+12 more)

### Community 16 - "During the session"
Cohesion: 0.09
Nodes (19): ADR Format, Numbering, Optional sections, Template, What qualifies, When to offer an ADR, CONTEXT.md Format, Rules (+11 more)

### Community 17 - "HTML Report Format"
Cohesion: 0.10
Nodes (18): Call-graph collapse, Candidate card, Cross-section (good for layered shallowness), Diagram patterns, Hand-built boxes-and-arrows (when Mermaid's layout fights you), Header, HTML Report Format, Mass diagram (good for "interface as wide as implementation") (+10 more)

### Community 18 - "package.json"
Cohesion: 0.11
Nodes (18): author, bugs, url, description, files, homepage, keywords, license (+10 more)

### Community 19 - "Reference: Manus Context Engineering Principles"
Cohesion: 0.11
Nodes (18): Critical Constraints, File Types Manus Creates, Key Quotes, Manus Statistics, Principle 1: Design Around KV-Cache, Principle 2: Mask, Don't Remove, Principle 3: Filesystem as External Memory, Principle 4: Manipulate Attention Through Recitation (+10 more)

### Community 20 - "Planning with Files"
Cohesion: 0.11
Nodes (18): Advanced Topics, Anti-Patterns, File Purposes, FIRST: Restore Context, Important: Where Files Go, Parallel task workflow, Pi Extension Hooks (mode-based), Planning with Files (+10 more)

### Community 21 - "Reference: Manus Context Engineering Principles"
Cohesion: 0.18
Nodes (11): Critical Constraints, File Types Manus Creates, Key Quotes, Manus Statistics, Reference: Manus Context Engineering Principles, Source, Strategy 1: Context Reduction, Strategy 2: Context Isolation (Multi-Agent) (+3 more)

### Community 22 - "Task Plan: [Brief Description]"
Cohesion: 0.12
Nodes (15): Current Phase, Decisions Made, Errors Encountered, Goal, Key Questions, Model Routing, Notes, Phase 1: Requirements & Discovery (+7 more)

### Community 23 - "Examples: Planning with Files in Action"
Cohesion: 0.13
Nodes (14): After (Correct), Before (Wrong), Example 1: Research Task, Example 2: Bug Fix Task, Example 3: Feature Development, Example 4: Error Recovery Pattern, Examples: Planning with Files in Action, Loop 1: Create Plan (+6 more)

### Community 24 - "Examples: Planning with Files in Action"
Cohesion: 0.13
Nodes (14): After (Correct), Before (Wrong), Example 1: Research Task, Example 2: Bug Fix Task, Example 3: Feature Development, Example 4: Error Recovery Pattern, Examples: Planning with Files in Action, Loop 1: Create Plan (+6 more)

### Community 25 - "Planning with Files"
Cohesion: 0.05
Nodes (41): 1. Create Plan First, 2. The 2-Action Rule, 3. Read Before Decide, 4. Update After Act, 5. Log ALL Errors, 6. Never Repeat Failures, 7. Continue After Completion, Advanced Topics (+33 more)

### Community 26 - "SKILL.md"
Cohesion: 0.14
Nodes (12): cavecrew, Example chaining, How to invoke, Model overrides, See also, What it does, Auto-clarity (inherited), Chaining patterns (+4 more)

### Community 27 - "Diagnosing Bugs"
Cohesion: 0.14
Nodes (13): Completion criterion — a tight loop that goes red, Diagnosing Bugs, Minimise, Non-deterministic bugs, Phase 1 — Build a feedback loop, Phase 2 — Reproduce + minimise, Phase 3 — Hypothesise, Phase 4 — Instrument (+5 more)

### Community 28 - "package.json"
Cohesion: 0.15
Nodes (12): devDependencies, @types/node, typescript, vitest, name, peerDependencies, @earendil-works/pi-coding-agent, private (+4 more)

### Community 29 - "Task Plan: [Analytics Project Description]"
Cohesion: 0.15
Nodes (12): Current Phase, Decisions Made, Errors Encountered, Goal, Hypotheses, Notes, Phase 1: Data Discovery, Phase 2: Exploratory Analysis (+4 more)

### Community 30 - "Task Plan: [Analytics Project Description]"
Cohesion: 0.15
Nodes (12): Current Phase, Decisions Made, Errors Encountered, Goal, Hypotheses, Notes, Phase 1: Data Discovery, Phase 2: Exploratory Analysis (+4 more)

### Community 31 - "Test-Driven Development"
Cohesion: 0.15
Nodes (10): Designing for Mockability, When to Mock, Anti-patterns, Rules of the loop, Seams — where tests go, Test-Driven Development, What a good test is, Bad Tests (+2 more)

### Community 32 - "Caveman Compress"
Cohesion: 0.17
Nodes (11): Boundaries, Caveman Compress, Compress, Compression Rules, Pattern, Preserve EXACTLY (never modify), Preserve Structure, Process (+3 more)

### Community 33 - "SKILL.md"
Cohesion: 0.17
Nodes (10): caveman, Example output, How to invoke, See also, What it does, Auto-Clarity, Boundaries, Intensity (+2 more)

### Community 34 - "Process"
Cohesion: 0.17
Nodes (11): 1. Gather context, 2. Explore the codebase (optional), 3. Draft vertical slices, 4. Quiz the user, 5. Publish the issues to the issue tracker, Acceptance criteria, Blocked by, Parent (+3 more)

### Community 35 - "caveman-commit"
Cohesion: 0.18
Nodes (9): caveman-commit, Example output, How to invoke, See also, What it does, Auto-Clarity, Boundaries, Examples (+1 more)

### Community 36 - "caveman-review"
Cohesion: 0.18
Nodes (9): caveman-review, Example output, How to invoke, See also, What it does, Auto-Clarity, Boundaries, Examples (+1 more)

### Community 37 - "Pi Planning With Files"
Cohesion: 0.18
Nodes (10): Commands, File Structure, Hook Parity in Pi, Installation, Manual Install, Mode System, Pi Install, Pi Planning With Files (+2 more)

### Community 38 - "Findings & Decisions"
Cohesion: 0.18
Nodes (7): Findings & Decisions, Issues Encountered, Requirements, Research Findings, Resources, Technical Decisions, Visual/Browser Findings

### Community 39 - "init-session.sh"
Cohesion: 0.31
Nodes (8): apply_v3_mode(), create_files_in(), gen_nonce(), init-session.sh script, write_analytics_progress(), write_default_findings(), write_default_progress(), write_default_task_plan()

### Community 40 - "Findings & Decisions"
Cohesion: 0.29
Nodes (7): Findings & Decisions, Issues Encountered, Requirements, Research Findings, Resources, Technical Decisions, Visual/Browser Findings

### Community 41 - "init-session.sh"
Cohesion: 0.31
Nodes (8): apply_v3_mode(), create_files_in(), gen_nonce(), init-session.sh script, write_analytics_progress(), write_default_findings(), write_default_progress(), write_default_task_plan()

### Community 42 - "Ask Matt"
Cohesion: 0.20
Nodes (9): Ask Matt, Codebase health, Context hygiene, Crossing sessions, On-ramps, Precondition, Standalone, The main flow: idea → ship (+1 more)

### Community 43 - "Findings & Decisions"
Cohesion: 0.20
Nodes (9): Data Sources, Findings & Decisions, Hypothesis Log, Issues Encountered, Query Results, Resources, Statistical Findings, Technical Decisions (+1 more)

### Community 44 - "Findings & Decisions"
Cohesion: 0.20
Nodes (9): Data Sources, Findings & Decisions, Hypothesis Log, Issues Encountered, Query Results, Resources, Statistical Findings, Technical Decisions (+1 more)

### Community 45 - "Repository Guidelines"
Cohesion: 0.20
Nodes (9): Build, Test, and Development Commands, Coding Style & Naming Conventions, Commit & Pull Request Guidelines, **DO NOT send optional commentary**, graphify, Platform & Repo Notes, Project Structure & Module Organization, Repository Guidelines (+1 more)

### Community 46 - "Repository Guidelines"
Cohesion: 0.62
Nodes (6): RegKey, disable(), enable(), is_enabled(), open_run_key(), Result

### Community 47 - "Wallpaper Manager"
Cohesion: 0.20
Nodes (9): Build And Run, Contributor Notes, Data And Cache Paths, How It Works, License, Project Layout, UI Layout, Wallpaper Manager (+1 more)

### Community 48 - "resolve-plan-dir.sh"
Cohesion: 0.47
Nodes (6): is_within_root(), resolve_from_active_file(), resolve_from_env(), resolve_latest_dir(), resolve-plan-dir.sh script, slug_is_valid()

### Community 50 - "resolve-plan-dir.sh"
Cohesion: 0.47
Nodes (6): is_within_root(), resolve_from_active_file(), resolve_from_env(), resolve_latest_dir(), resolve-plan-dir.sh script, slug_is_valid()

### Community 51 - "SKILL.md"
Cohesion: 0.22
Nodes (8): Further Notes, Implementation Decisions, Out of Scope, Problem Statement, Process, Solution, Testing Decisions, User Stories

### Community 52 - "Repository Guidelines Template"
Cohesion: 0.22
Nodes (8): Build, Test, and Development Commands, Coding Style & Naming Conventions, Commit & Pull Request Guidelines, Engineering Principles, Optional Sections, Project Structure & Module Organization, Repository Guidelines Template, Testing Guidelines

### Community 53 - "graphify reference: extra exports and benchmark"
Cohesion: 0.22
Nodes (8): graphify reference: extra exports and benchmark, Step 6b - Wiki (only if --wiki flag), Step 7 - Neo4j export (only if --neo4j or --neo4j-push flag), Step 7a - FalkorDB export (only if --falkordb or --falkordb-push flag), Step 7b - SVG export (only if --svg flag), Step 7c - GraphML export (only if --graphml flag), Step 7d - MCP server (only if --mcp flag), Step 8 - Token reduction benchmark (only if total_words > 5000)

### Community 54 - "Process"
Cohesion: 0.25
Nodes (7): 1. Pin the fixed point, 2. Identify the spec source, 3. Identify the standards sources, 4. Spawn both sub-agents in parallel, 5. Aggregate, Process, Why two axes

### Community 55 - "Critical Rules"
Cohesion: 0.25
Nodes (8): 1. Create Plan First, 2. The 2-Action Rule, 3. Read Before Decide, 4. Update After Act, 5. Log ALL Errors, 6. Never Repeat Failures, 7. Continue After Completion, Critical Rules

### Community 56 - "Progress Log"
Cohesion: 0.25
Nodes (7): 5-Question Reboot Check, Error Log, Phase 1: [Title], Phase 2: [Title], Progress Log, Session: [DATE], Test Results

### Community 59 - "Progress Log"
Cohesion: 0.25
Nodes (7): 5-Question Reboot Check, Error Log, Phase 1: [Title], Phase 2: [Title], Progress Log, Session: [DATE], Test Results

### Community 60 - "caveman-stats"
Cohesion: 0.29
Nodes (5): caveman-stats, Example output, How to invoke, See also, What it does

### Community 61 - "Task Plan: [Brief Description]"
Cohesion: 0.29
Nodes (7): Current Phase, Decisions Made, Errors Encountered, Goal, Key Questions, Notes, Task Plan: [Brief Description]

### Community 63 - "Task Plan: [Brief Description]"
Cohesion: 0.29
Nodes (7): Current Phase, Decisions Made, Errors Encountered, Goal, Key Questions, Notes, Task Plan: [Brief Description]

### Community 65 - "Phases"
Cohesion: 0.33
Nodes (6): Phase 1: Requirements & Discovery, Phase 2: Planning & Structure, Phase 3: Implementation, Phase 4: Testing & Verification, Phase 5: Delivery, Phases

### Community 67 - "phase-status.sh"
Cohesion: 0.53
Nodes (4): do_write(), rewrite(), phase-status.sh script, usage()

### Community 68 - "Phases"
Cohesion: 0.33
Nodes (6): Phase 1: Requirements & Discovery, Phase 2: Planning & Structure, Phase 3: Implementation, Phase 4: Testing & Verification, Phase 5: Delivery, Phases

### Community 69 - "graphify reference: query, path, explain"
Cohesion: 0.33
Nodes (5): For /graphify explain, For /graphify path, graphify reference: query, path, explain, Step 0 — Constrained query expansion (REQUIRED before traversal), Step 1 — Traversal

### Community 74 - "hitl-loop.template.sh"
Cohesion: 0.83
Nodes (3): capture(), hitl-loop.template.sh script, step()

### Community 75 - "planning-with-files Pi Extension"
Cohesion: 0.50
Nodes (3): Events mapped, Modes, planning-with-files Pi Extension

### Community 78 - "GLOSSARY.md Format"
Cohesion: 0.50
Nodes (3): GLOSSARY.md Format, Rules, Structure

### Community 79 - "graphify reference: add a URL and watch a folder"
Cohesion: 0.50
Nodes (3): For /graphify add, For --watch, graphify reference: add a URL and watch a folder

### Community 80 - "graphify reference: commit hook and native CLAUDE.md integration"
Cohesion: 0.50
Nodes (3): For git commit hook, For native CLAUDE.md integration, graphify reference: commit hook and native CLAUDE.md integration

### Community 81 - "graphify reference: incremental update and cluster-only"
Cohesion: 0.50
Nodes (3): For --cluster-only, For --update (incremental re-extraction), graphify reference: incremental update and cluster-only

## Knowledge Gaps
- **601 isolated node(s):** `tempRoots`, `EventHandler`, `MockPi`, `MockContext`, `tempRoots` (+596 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **17 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `WallpaperApp` connect `WallpaperApp` to `mod.rs`, `run_worker`?**
  _High betweenness centrality (0.006) - this node is a cross-community bridge._
- **Why does `StitchOrientation` connect `mod.rs` to `run_worker`?**
  _High betweenness centrality (0.005) - this node is a cross-community bridge._
- **Why does `AppSettings` connect `mod.rs` to `WallpaperApp`?**
  _High betweenness centrality (0.004) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `run_worker()` (e.g. with `process_image()` and `stitch_images()`) actually correct?**
  _`run_worker()` has 3 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Caveman compress scripts.  This package provides tools to compress natural langu`, `Split YAML frontmatter from body. Returns (frontmatter, body).      Memory files`, `Resolve the out-of-tree backup directory for a given source file.      Backups m` to the rest of the system?**
  _637 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.ts` be split into smaller, more focused modules?**
  _Cohesion score 0.05257312106627175 - nodes in this community are weakly interconnected._
- **Should `mod.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06829488919041157 - nodes in this community are weakly interconnected._