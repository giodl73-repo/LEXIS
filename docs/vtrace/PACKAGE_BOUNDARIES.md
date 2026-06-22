# LEXIS Package Boundaries

Status: settled.

## Scope

Repo: LEXIS foundation VTRACE adoption.

## Boundary Inventory

| ID | Boundary Unit | Language / Toolchain | Owner | Responsibility | Public Interfaces | Downstream Consumers |
|---|---|---|---|---|---|---|
| PKG-001 | `docs/vtrace/` | docs | LEXIS | VTRACE control package, stage ledgers, requirements, specs, review, and future implementation gates. | planned `lexis` contracts | maintainers, reviewers |
| PKG-002 | `.roles/` | role Markdown | LEXIS | Language-history, custody, graph, chronicle, and assurance review panel. | role review | maintainers, agents |
| PKG-003 | `.claude/skills/` | skill Markdown | LEXIS | Repo-local wave, pulse, and research workflows. | skill workflows | agents |
| PKG-004 | `context/waves/` | docs | LEXIS | Wave/pulse execution history and VTRACE fixed-point records. | wave ledgers | TRACKER, maintainers |
| PKG-005 | future Rust workspace | Rust | LEXIS | Model, validation CLI, graph slice, and chronicle output. | future CLI and artifacts | downstream language-history consumers |
| PKG-006 | future fixtures/scenarios | JSON/YAML/docs | LEXIS | Source-safe fixtures, scenario packages, negative cases, and evidence seeds. | future validation fixtures | V&V, reviewers |

## Dependency Direction

| From | To | Allowed? | Rationale | Verification |
|---|---|---|---|---|
| Future Rust workspace | RLINE | planned | Graph mechanics only after interface/design/work-package review. | graph preservation checks |
| Future Rust workspace | `docs/vtrace/` | yes | Code must implement settled requirements/specs/work packages. | trace and tests |
| Future fixtures/scenarios | source pointers | yes | Source-safe evidence references only after custody review. | source-custody gate |
| `docs/vtrace/` | TRACKER dependency records | yes | TRACKER records portfolio placement and dependency posture. | tracker diff review |
| RLINE | LEXIS semantics | no | RLINE must not own linguistic meaning, confidence, or theory choice. | architecture/review |

## Boundary Rules

| Boundary ID | Allowed Changes | Forbidden Changes | Change-Control Trigger |
|---|---|---|---|
| PKG-001 | Refine VTRACE controls and trace maps. | Claim implementation evidence without code/tests. | Any stage semantic change. |
| PKG-002 | Add/revise review roles. | Remove custody/graph/assurance review from public claims. | Any role lane change. |
| PKG-003 | Refine workflows. | Let skills bypass VTRACE gates. | Any skill behavior change. |
| PKG-004 | Record pulses and fixed points. | Mark code packages complete without validation. | Any wave/pulse status change. |
| PKG-005 | Add implementation through approved work packages. | Add source ingestion, RLINE adoption, or public CLI claims without gates. | Any code, CLI, or dependency change. |
| PKG-006 | Add reviewed fixtures/scenarios. | Redistribute source text without custody approval. | Any fixture/source/scenario change. |

## Language Tailoring

| Boundary ID | Code Rigor Profile | L0 | L1 | L2 |
|---|---|---|---|---|
| PKG-001 / PKG-002 / PKG-003 / PKG-004 | docs-only | `git diff --check` | role/trace inspection | portfolio review |
| PKG-005 | Rust planned | future `cargo fmt --check` | future tests and validator | role-reviewed scenario proof |
| PKG-006 | fixture/scenario planned | future parse/validate command | negative tests | source-custody and validation review |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: package boundaries must prevent code from making broad historical claims. | Closed by PKG-005 forbidden changes and docs-only controls. |
| Etymology Reviewer | Major: fixtures/scenarios need their own boundary before etymology claims are promoted. | Closed by PKG-006. |
| Phonology Reviewer | Minor: future Rust workspace should not imply phonetic-engine scope. | Closed by PKG-005 responsibility wording and future work gates. |
| Script Systems Reviewer | Minor: future fixtures should handle script-safe evidence separately. | Closed by PKG-006 source-safe responsibility. |
| Source Custody Reviewer | Major: fixture/source boundary must block source-text redistribution. | Closed by PKG-006 forbidden changes. |
| Graph Systems Reviewer | Major: RLINE dependency direction must stay one-way and semantics-free. | Closed by dependency direction table. |
| Product Chronicle Reviewer | Minor: docs and wave records should not claim chronicle implementation. | Closed by PKG-001 and PKG-004 forbidden changes. |
| Software Assurance Reviewer | Major: validation levels must be tailored by package type. | Closed by language tailoring table. |

## Decision

PACKAGE_BOUNDARIES is settled for the foundation wave. Implementation packages
remain blocked until WORK_PACKAGES, VERIFICATION, and VALIDATION are executable.
