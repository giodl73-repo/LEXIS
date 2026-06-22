# Wave: Foundation VTRACE

## Goal

Establish LEXIS as the Knowledge Systems repo for language evolution while
defining the VTRACE stage process, role-review gates, dependency posture, and
initial mission before any implementation code begins.

## Thesis

Language-history tooling is unusually prone to overconfident summaries. LEXIS
needs source custody, uncertainty labels, linguistic review, and graph-boundary
discipline from the start. A VTRACE-first foundation keeps the first code slice
small, reviewable, and evidence-backed.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|---|---|---|
| 01 | Governance scaffold | complete | Created docs, skills, role panel, VTRACE stage ledger, and dependency records. |
| 02 | Mission fixed point | complete | Settled `docs/vtrace/MISSION.md` at role-review fixed point. |
| 03 | CONOPS fixed point | complete | Settled user workflows and operating concepts at role-review fixed point. |
| 03a | Communications strategy fixed point | complete | Settled audience lanes, wording rules, chronicle voice, and status communication. |
| 04 | Requirements baseline | complete | Settled source, graph, uncertainty, and output requirements at role-review fixed point. |
| 05 | Specification baseline | complete | Settled object vocabulary, claim types, edge kinds, uncertainty labels, and source posture. |
| 05a | Spec model | complete | Settled deep spec shape, authority boundaries, and promotion rules. |
| 06 | Architecture boundary | complete | Settled LEXIS/RLINE/source/publisher ownership boundaries. |
| 07 | Interfaces boundary | complete | Settled future CLI, fixture, source, graph, report, and artifact contracts. |
| 08 | Design boundary | complete | Settled graph views, lineage views, drift views, and chronicles. |
| 08a | Package boundaries | complete | Settled package ownership, dependency direction, and boundary rules. |
| 08b | Contract boundaries | complete | Settled durable contract classes and closeout requirements. |
| 08c | Scenario model | complete | Settled future scenario package shape and findings rules. |
| 09 | Code rigor boundary | complete | Settled fixtures, validation, source custody, and overclaim gates. |
| 10 | Implementation planning | complete | Settled the first narrow implementation-slice plan. |
| 11 | Work packages | complete | Settled implementation work packages and negative-test expectations. |
| 12 | Verification | complete | Settled future tests and preservation checks. |
| 13 | Validation | complete | Settled usefulness and historical-caution criteria. |
| 14 | Trace | complete | Settled requirements-to-stage and work-package trace. |
| 15 | Review | complete | Settled foundation readiness and remaining blocks. |
| 16 | Stage execution | complete | Settled S0-S6 stage board and evidence mapping. |
| 17 | Support artifacts | complete | Settled evidence, change control, review checklists, role recommendations, language profiles, source basis, pulse execution, and VTRACE README. |
| 18 | Domain backlog | complete | Planned entity build order, candidate first slices, and work-package mapping without source ingestion. |
| 19 | Language slice packages | complete | Planned repeatable SOURCE/SCOPE/NODES/EDGES/NEGATIVES/GRAPH/CHRONICLE/PACK package sets for five candidate slices. |
| 20 | Research plan | complete | Planned PANEL-style research tracks and papers that feed requirements, work packages, and language slices through DCRs. |
| 21 | VTRACE large-domain alignment | complete | Added problem-space map, research-stage pointer, diagnostic model, and fixture model controls from updated VTRACE. |
| 22 | Draft specs | complete | Added draft implementation-facing specs for source custody, model, relationships, diagnostics, fixtures, graph slices, chronicles, and publisher handoff. |
| 23 | Draft spec role review | complete | Reviewed all draft specs with L-1..L-8 and recorded fixed-point decisions without promoting implementation. |
| 24 | Planned scenarios | complete | Instantiated planned scenario packages for word/root slice, borrowing/descent separation, source-limited claims, and RLINE preservation. |
| 25 | Planned fixture manifests | complete | Added planned fixture manifests for source-pointer, golden slice, negative relationship, source-limited, and graph-preservation proof inputs. |
| 26 | Planned source-custody decisions | complete | Added planned source-family custody decision stubs for first-slice blockers without accepting real sources. |
| 27 | First research module plans | complete | Added Source Custody and Evidence module with plans for `LEXIS-PAPER-001` and `LEXIS-PAPER-008` without writing papers. |
| 28 | Relationship research module plans | complete | Added Ontology and Relationship Semantics module with plans for `LEXIS-PAPER-002`, `LEXIS-PAPER-003`, and `LEXIS-PAPER-010` without writing papers. |
| 29 | Script and reconstruction research module plans | complete | Added Script and Reconstruction Caution module with plans for `LEXIS-PAPER-004`, `LEXIS-PAPER-005`, `LEXIS-PAPER-009`, and `LEXIS-PAPER-011` without writing papers. |
| 30 | Graph and chronicle research module plans | complete | Added Graph and Chronicle Method module with plans for `LEXIS-PAPER-006` and `LEXIS-PAPER-007` without writing papers. |
| 31 | Publisher context research module plan | complete | Added Publisher Context module with plan for `LEXIS-PAPER-012` without writing the paper. |
| 32 | Scribere source candidate review | complete | Recorded first candidate source pointers and drafted `LEXIS-PAPER-008` as planning input without accepting sources or promoting fixtures. |
| 33 | Source-record contract | complete | Added draft-reviewed source-record states, promotion rules, and validator diagnostic contract without implementation. |
| 34 | Source-pointer fixture shape | complete | Added invalid-by-design `LEXIS-FIX-001` fixture shape and expected diagnostics without executable validation or source acceptance. |
| 35 | Rust scaffold and minimal validator | complete | Added Rust workspace and `lexis validate` skeleton targeting `LEXIS-FIX-001`; command evidence passed with expected fail-closed source-custody diagnostics. |
| 36 | Stable source diagnostic IDs | complete | Added diagnostic IDs to validator output and expected diagnostics; command evidence passed with `LEXIS-DIAG-SRC-000`, `LEXIS-DIAG-SRC-001`, and `LEXIS-DIAG-SRC-002`. |
| 37 | Linked source-custody validation | complete | Validator resolves source-custody decision records and can detect fixture/source state mismatch; command evidence passed. |
| 38 | Source-custody index coverage | complete | Validator loads planned source-custody records through an index and tests missing, malformed, and mismatched records; command evidence passed. |
| 39 | First node-edge graph slice | complete | Added draft `scribere -> scribe` node/edge fixture, graph-facing validator checks, and blocked JSON/DOT graph emission evidence. |
| 40 | Graph preview output | complete | Added preview-only JSON/DOT graph rendering for blocked fixtures with validation error count; command evidence passed. |
| 41 | Graph label preservation | complete | Graph preview output now carries source posture, claim type, and uncertainty labels; command evidence passed. |
| 42 | Graph summary output | complete | Added graph summary command for node classes, edge kinds, source postures, uncertainty labels, and validation status; command evidence passed. |
| 43 | Graph inspection output | complete | Added graph inspection command for concrete node and edge review before chronicle work; command evidence passed. |
| 44 | Chronicle preview output | complete | Added chronicle preview command with required sections, source-limit wording, and non-published posture; command evidence passed. |
| 45 | Word trace output | complete | Added fixture-local word trace command for source links and connected relationship edges; command evidence passed. |
| 46 | Lineage trace output | complete | Added fixture-local lineage trace command separating lineage paths from supporting evidence edges; command evidence passed. |
| 47 | Neighborhood trace output | complete | Added fixture-local neighborhood trace command for local source links and adjacent relationship edges; command evidence passed. |
| 48 | Source status output | complete | Added source-custody status command comparing fixture source states with custody record states; command evidence passed. |
| 49 | Fixture readiness output | complete | Added promotion-readiness command for validation, source, graph, chronicle, and blocker gates; command evidence passed. |
| 50 | Diagnostic explanation output | complete | Added grouped diagnostic explanation command preserving IDs, affected records, and messages; command evidence passed. |
| 51 | Fixture review packet | complete | Added role-review packet command combining readiness, source status, diagnostics, graph summary, and chronicle preview; command evidence passed. |
| 52 | Fixture inventory output | complete | Added planned fixture inventory command listing fixture manifest status, class, scope, expected result, and blocker counts; command evidence passed. |
| 53 | Source inventory output | complete | Added planned source-custody inventory command listing source decision status, family, review state, promotion posture, and blocker counts; command evidence passed. |
| 54 | Source review packet | complete | Added pointer-only source review command listing source decision posture, blockers, citation note, and referencing fixture manifests; command evidence passed. |
| 55 | Slice inventory output | complete | Added language-slice inventory command linking slice package sets to fixture and source references; command evidence passed. |
| 56 | Slice review packet | complete | Added focused slice review command listing package steps, linked fixtures, linked source decisions, and planning-only posture; command evidence passed. |
| 57 | Scenario inventory output | complete | Added planned scenario inventory command listing actors, slice packages, diagnostics, work-package counts, and fixture candidates; command evidence passed. |
| 58 | Scenario review packet | complete | Added focused scenario review command listing purpose, specs, positive/negative paths, expected diagnostics, evidence, and fixture candidates; command evidence passed. |
| 59 | Work-package inventory output | complete | Added VTRACE work-package inventory command listing outcomes, gates, and planned scenario references; command evidence passed. |

## VTRACE stage ledger

| Repo | Stage | File | Status | Input SHA | Output SHA | Roles | Findings | Decision | Next |
|---|---|---|---|---|---|---|---|---|---|
| LEXIS | MISSION | `docs/vtrace/MISSION.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | CONOPS |
| LEXIS | CONOPS | `docs/vtrace/CONOPS.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | COMMUNICATIONS_STRATEGY |
| LEXIS | COMMUNICATIONS_STRATEGY | `docs/vtrace/COMMUNICATIONS_STRATEGY.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | REQUIREMENTS |
| LEXIS | REQUIREMENTS | `docs/vtrace/REQUIREMENTS.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | SPECIFICATION_BASELINE |
| LEXIS | SPECIFICATION_BASELINE | `docs/vtrace/SPECIFICATION_BASELINE.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | PROBLEM_SPACE_MAP |
| LEXIS | PROBLEM_SPACE_MAP | `docs/vtrace/PROBLEM_SPACE_MAP.md` | settled | n/a | pending | L-1..L-8 | Language-history world regions and slice traversal accepted as planning input only. | Fixed point reached. | DOMAIN_BACKLOG |
| LEXIS | SPEC_MODEL | `docs/vtrace/SPEC_MODEL.md` | settled | n/a | pending | L-1..L-8 | Explicit L-1..L-8 review table added; implementation specs blocked until scenarios/fixtures exist. | Fixed point reached. | ARCHITECTURE |
| LEXIS | ARCHITECTURE | `docs/vtrace/ARCHITECTURE.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | INTERFACES |
| LEXIS | INTERFACES | `docs/vtrace/INTERFACES.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | DESIGN |
| LEXIS | DESIGN | `docs/vtrace/DESIGN.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | PACKAGE_BOUNDARIES |
| LEXIS | PACKAGE_BOUNDARIES | `docs/vtrace/PACKAGE_BOUNDARIES.md` | settled | n/a | pending | L-1..L-8 | Explicit L-1..L-8 review table added; package ownership and dependency direction recorded. | Fixed point reached. | CONTRACT_BOUNDARIES |
| LEXIS | CONTRACT_BOUNDARIES | `docs/vtrace/CONTRACT_BOUNDARIES.md` | settled | n/a | pending | L-1..L-8 | Explicit L-1..L-8 review table added; durable contract classes and closeout requirements recorded. | Fixed point reached. | SCENARIO_MODEL |
| LEXIS | SCENARIO_MODEL | `docs/vtrace/SCENARIO_MODEL.md` | settled | n/a | pending | L-1..L-8 | Explicit L-1..L-8 review table added; scenario shape and first candidates recorded. | Fixed point reached. | DIAGNOSTIC_MODEL |
| LEXIS | DIAGNOSTIC_MODEL | `docs/vtrace/DIAGNOSTIC_MODEL.md` | settled | n/a | pending | L-1..L-8 | Future diagnostic families accepted as planning input only; no CLI implementation exists. | Fixed point reached. | FIXTURE_MODEL |
| LEXIS | FIXTURE_MODEL | `docs/vtrace/FIXTURE_MODEL.md` | settled | n/a | pending | L-1..L-8 | Future fixture classes accepted as planning input only; no fixtures exist. | Fixed point reached. | CODE_RIGOR |
| LEXIS | DRAFT_SPECS | `docs/specs/` | draft-reviewed | n/a | pending | L-1..L-8 | Draft specs reviewed by L-1..L-8; no spec is promoted for implementation. | Fixed point reached. | complete |
| LEXIS | PLANNED_SCENARIOS | `scenarios/` | planned | n/a | pending | L-1..L-8 | Scenario packages accepted as planning input only; no scenario is executed or promoted as validation evidence. | Fixed point reached. | complete |
| LEXIS | PLANNED_FIXTURES | `fixtures/` | draft_shape | n/a | pending | L-1..L-8 | Fixture manifests accepted as planning input; `LEXIS-FIX-001` has an invalid-by-design draft shape, but no fixture is executable or promoted as validation evidence. | Fixed point reached. | complete |
| LEXIS | PLANNED_SOURCE_CUSTODY | `source-custody/` | candidate_review | n/a | pending | L-1..L-8 | First `scribere` candidate pointers recorded for review; no real source is accepted, ingested, quoted, cached, or redistributed. | Fixed point reached. | complete |
| LEXIS | PLANNED_RESEARCH_MODULES | `research/modules/` | partial_draft | n/a | pending | L-1..L-8 | All five research module plans and twelve paper plans exist; `LEXIS-PAPER-008` is drafted as planning input, while remaining papers are not written. | Fixed point reached. | complete |
| LEXIS | CODE_RIGOR | `docs/vtrace/CODE_RIGOR.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | IMPLEMENTATION_PLAN |
| LEXIS | IMPLEMENTATION_PLAN | `docs/vtrace/IMPLEMENTATION_PLAN.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | WORK_PACKAGES |
| LEXIS | WORK_PACKAGES | `docs/vtrace/WORK_PACKAGES.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | VERIFICATION |
| LEXIS | VERIFICATION | `docs/vtrace/VERIFICATION.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | VALIDATION |
| LEXIS | VALIDATION | `docs/vtrace/VALIDATION.md` | settled | n/a | pending | L-1..L-8 | Major findings closed; minor findings closed or deferred. | Fixed point reached. | TRACE |
| LEXIS | TRACE | `docs/vtrace/TRACE.md` | settled | n/a | pending | L-1..L-8 | Trace map recorded. | Fixed point reached. | REVIEW |
| LEXIS | REVIEW | `docs/vtrace/REVIEW.md` | settled | n/a | pending | L-1..L-8 | Foundation review recorded. | Fixed point reached. | complete |
| LEXIS | STAGE_EXECUTION | `docs/vtrace/STAGE_EXECUTION.md` | settled | n/a | pending | L-1..L-8 | Explicit L-1..L-8 review table added; S0-S6 stage board and evidence map recorded. | Fixed point reached. | complete |
| LEXIS | SUPPORT_ARTIFACTS | `docs/vtrace/{README,EVIDENCE,CHANGE_CONTROL,REVIEW_CHECKLISTS,ROLE_RECOMMENDATIONS,LANGUAGE_PROFILES,SOURCE_BASIS,PULSE_EXECUTION}.md` | settled | n/a | pending | L-1..L-8 | Support artifacts reviewed in file-local checkpoints or REVIEW summary; implementation proof remains blocked. | Fixed point reached. | complete |
| LEXIS | DOMAIN_BACKLOG | `DOMAIN_BACKLOG.md` | settled | n/a | pending | L-1..L-8 | Entity sequence, first-slice candidates, and work-package mapping accepted as planning input only. | Fixed point reached. | RESEARCH_PLAN |
| LEXIS | RESEARCH_PLAN | `docs/vtrace/RESEARCH_PLAN.md`, `RESEARCH_PLAN.md`, `research/README.md` | settled | n/a | pending | L-1..L-8 | Research modules and papers accepted as planning input only; source ingestion, implementation, graph output, and publication remain blocked. | Fixed point reached. | SPEC_MODEL |
| LEXIS | LANGUAGE_SLICE_PACKAGES | `LANGUAGE_SLICE_PACKAGES.md` | settled | n/a | pending | L-1..L-8 | Slice package sets accepted as planning input only; execution remains blocked. | Fixed point reached. | complete |

## Success criteria

- README explains the repo purpose, boundaries, and docs-only validation.
- Product plan names waves and non-goals.
- `.roles/ROLE.md` defines the LEXIS review panel.
- VTRACE stage files exist with pending stages clearly marked.
- Dependency posture records RLINE as intended graph runtime support, not a
  linguistics owner.
- `git diff --check` passes.

## Validation

```powershell
git diff --check
```
