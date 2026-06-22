# LEXIS Validation

Status: settled.

VERIFICATION reached fixed point. Validation defines how future outputs prove
they are useful and appropriately cautious. It does not run validation studies.

## Validation principle

A LEXIS output is valid only if a reviewer can understand what is evidenced,
what is inferred, what is disputed, what is unavailable, and what source limits
apply.

## Future validation criteria

| Criterion | Acceptance question |
|---|---|
| Scope clarity | Can a reviewer state the bounded language-history question and non-goals? |
| Evidence clarity | Can a reviewer identify direct source-backed evidence? |
| Theory clarity | Can a reviewer identify reconstruction, inference, and competing theories? |
| Relationship clarity | Are descent, borrowing, calque, sound shift, meaning shift, and script variation visibly distinct? |
| Source custody | Are rights and redistribution limits visible? |
| Uncertainty | Are disputed, source-limited, unavailable, and rejected alternatives visible? |
| Chronicle readability | Can a reader follow the narrative without losing evidence boundaries? |
| Graph usefulness | Does the graph help inspect the claim rather than decorate it? |
| Downstream safety | Can a downstream repo consume the artifact without inheriting unsupported claims? |
| Spec depth | Does any implementation input have enough spec, boundary, scenario, and fixture context? |
| Scenario usefulness | Does the scenario reveal findings or explain why the exercised surface was already sufficient? |
| Evidence honesty | Does the evidence ledger distinguish docs-only proof from implementation proof? |
| Review completeness | Do required review lanes cover source custody, graph boundaries, and release readiness? |

## Planned Scenario Coverage

| Scenario | Validation focus | Current status |
|---|---|---|
| `LEXIS-SC-001-word-root-slice` | Scope, source posture, model records, relationship edges, graph planning, chronicle review. | planned, not executed |
| `LEXIS-SC-002-borrowing-vs-descent` | Relationship clarity and negative relationship diagnostics. | planned, not executed |
| `LEXIS-SC-003-source-limited-claim` | Source custody, source-limited claims, chronicle overclaim, publisher blockers. | planned, not executed |
| `LEXIS-SC-004-rline-preservation` | Graph-label preservation and RLINE adoption gate. | planned, not executed |

## Planned Fixture Coverage

| Fixture plan | Validation focus | Current status |
|---|---|---|
| `LEXIS-FIX-001-source-pointer-scribere` | Source-custody metadata shape and pointer-only posture. | draft shape exists; minimal validator should fail it closed; fixture promotion blocked |
| `LEXIS-FIX-002-golden-scribere-slice` | First valid word/root slice after source and model promotion. | draft node/edge shape exists; invalid until source custody is accepted |
| `LEXIS-FIX-003-negative-borrowing-descent` | Relationship-collapse negative validation. | planned manifest only |
| `LEXIS-FIX-004-source-limited-claim` | Source-limited claim and chronicle overclaim blocking. | planned manifest only |
| `LEXIS-FIX-005-rline-preservation` | Graph-label preservation before RLINE adoption. | planned manifest only |

## Planned Source-Record Contract Coverage

| Contract surface | Validation focus | Current status |
|---|---|---|
| source record states | Candidate, accepted, deferred, rejected, and blocked source records do not collapse. | draft-reviewed |
| source promotion rule | Only `accepted_for_slice` records support claims. | draft-reviewed |
| source redistribution rule | Pointer-only records cannot carry copied source text. | draft-reviewed |
| graph/chronicle blocker | Candidate-only sources block graph, chronicle, and publisher output. | draft-reviewed |

## Planned Source-Custody Coverage

| Decision | Validation focus | Current status |
|---|---|---|
| `LEXIS-SRCDEC-001` | Latin lexicographic pointer posture for first slice. | candidate_review |
| `LEXIS-SRCDEC-002` | English etymology pointer posture for first slice. | candidate_review |
| `LEXIS-SRCDEC-003` | General scholarly source posture for theory/reconstruction. | deferred_after_first_slice_scope |
| `LEXIS-SRCDEC-004` | Script-history source posture for alphabet lane. | planned_blocked |
| `LEXIS-SRCDEC-005` | Source-limited negative validation posture. | planned_blocked |

## Planned Research Coverage

| Research item | Validation focus | Current status |
|---|---|---|
| `LEXIS-PAPER-001` | Source-family custody rules and rights posture. | plan only |
| `LEXIS-PAPER-008` | First `scribere` slice source safety and fixture readiness. | draft-reviewed, open blockers |
| `LEXIS-PAPER-002` | Claim type sufficiency and invalid claim posture. | plan only |
| `LEXIS-PAPER-003` | Relationship boundary and edge-kind sufficiency. | plan only |
| `LEXIS-PAPER-010` | Negative relationship product value and wording rules. | plan only |
| `LEXIS-PAPER-004` | Script form, sound value, and transliteration separation. | plan only |
| `LEXIS-PAPER-005` | Reconstruction labels and overclaim controls. | plan only |
| `LEXIS-PAPER-009` | Greek alphabet lane scenario and graph shape. | plan only |
| `LEXIS-PAPER-011` | Semitic root-pattern morphology and transliteration gaps. | plan only |
| `LEXIS-PAPER-006` | Graph/RLINE preservation and graph-label invariants. | plan only |
| `LEXIS-PAPER-007` | Chronicle wording, evidence-theory separation, and overclaim controls. | plan only |
| `LEXIS-PAPER-012` | Reviewed-slice publisher shape and handoff blockers. | plan only |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: validation must judge historical caution. | Closed by evidence/theory/uncertainty criteria. |
| Etymology Reviewer | Major: relationship clarity must be a validation criterion. | Closed by relationship clarity. |
| Phonology Reviewer | Minor: unavailable sound data should be visible to readers. | Closed by uncertainty criterion. |
| Script Systems Reviewer | Minor: script variation should not be decorative only. | Closed by graph usefulness and relationship clarity. |
| Source Custody Reviewer | Major: source limits must be visible to readers and downstream repos. | Closed by source custody and downstream safety. |
| Graph Systems Reviewer | Major: graph output must support inspection, not just aesthetics. | Closed by graph usefulness. |
| Product Chronicle Reviewer | Major: readable chronicle must preserve boundaries. | Closed by chronicle readability. |
| Software Assurance Reviewer | Major: downstream safety must be explicit. | Closed by downstream safety. |

## Decision

VALIDATION is settled for the foundation wave. No critical or major actionable
role finding remains. TRACE is the next VTRACE stage.
