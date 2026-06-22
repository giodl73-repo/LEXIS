# LEXIS Specs

Status: draft-reviewed, not promoted for implementation.

These specs translate the settled VTRACE foundation into implementation-facing
behavior. They are not promoted for implementation yet because LEXIS has no
accepted scenarios, fixtures, validation CLI, source-custody decisions, graph
output, or chronicle output.

## Spec Index

| Spec | Area | Primary work packages | Status |
|---|---|---|---|
| `source-custody.md` | Source records, rights posture, redistribution posture. | `LEXIS-WP-003`, all `*-SOURCE` packages | draft-reviewed |
| `source-record-contract.md` | Source record states, promotion rules, and validator diagnostics. | `LEXIS-WP-003`, `LEXIS-WP-004`, `LEXIS-WP-005`, `LEXIS-SLICE-001-SOURCE` | draft-reviewed |
| `language-history-model.md` | Core records, claim posture, uncertainty, lifecycle states. | `LEXIS-WP-002`, `LEXIS-WP-004` | draft-reviewed |
| `relationship-edges.md` | Descent, cognacy, borrowing, calque, shifts, support, disputes, negatives. | `LEXIS-WP-002`, `LEXIS-WP-005`, `*-EDGES`, `*-NEGATIVES` | draft-reviewed |
| `validation-diagnostics.md` | Future validation diagnostic families and blocking rules. | `LEXIS-WP-005` | draft-reviewed |
| `fixture-promotion.md` | Scenario/research artifact promotion into controlled fixtures. | `LEXIS-WP-004`, `LEXIS-WP-005` | draft-reviewed |
| `graph-slice.md` | Graph slice output and RLINE preservation contract. | `LEXIS-WP-006`, `LEXIS-WP-007` | draft-reviewed |
| `chronicle-output.md` | Narrative report shape and overclaim controls. | `LEXIS-WP-008` | draft-reviewed |
| `publisher-handoff.md` | Future artifact handoff posture. | `LEXIS-WP-009` | draft-reviewed |

## Promotion Rule

A spec becomes implementation input only when:

1. at least one scenario or fixture path is named,
2. source-custody impact is reviewed,
3. positive and negative examples are present or explicitly out of scope,
4. diagnostic impact is allocated,
5. graph and public-claim impact are reviewed,
6. trace, verification, validation, and work-package rows are updated,
7. L-1 through L-8 review closes critical and major findings.

## Scenario Coverage

| Spec | Planned scenario coverage | Promotion impact |
|---|---|---|
| `source-custody.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/source-limited-claim` | Scenario path named; execution and source decisions still blocked. |
| `source-record-contract.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/source-limited-claim` | Scenario path named; source acceptance and validator remain blocked. |
| `language-history-model.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/borrowing-vs-descent` | Scenario path named; fixtures still blocked. |
| `relationship-edges.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/borrowing-vs-descent`, `scenarios/language-history/rline-preservation` | Scenario path named; negative fixtures still blocked. |
| `validation-diagnostics.md` | all planned scenarios | Scenario path named; CLI diagnostics still blocked. |
| `fixture-promotion.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/source-limited-claim` | Scenario path named; no fixture promoted. |
| `graph-slice.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/rline-preservation` | Scenario path named; graph output still blocked. |
| `chronicle-output.md` | `scenarios/language-history/word-root-slice`, `scenarios/language-history/source-limited-claim` | Scenario path named; chronicle output still blocked. |
| `publisher-handoff.md` | `scenarios/language-history/source-limited-claim` | Scenario path named; publisher handoff still blocked. |

## Fixture Plan Coverage

| Spec | Planned fixture coverage | Promotion impact |
|---|---|---|
| `source-custody.md` | `fixtures/planned/source-pointer-scribere`, `fixtures/planned/source-limited-claim` | Fixture plans named; source decisions still blocked. |
| `source-record-contract.md` | `fixtures/planned/source-pointer-scribere`, `fixtures/planned/golden-scribere-slice`, `fixtures/planned/source-limited-claim` | Candidate-only source fixture shape drafted; executable validation still blocked. |
| `language-history-model.md` | `fixtures/planned/golden-scribere-slice` | Fixture plan named; model schema still blocked. |
| `relationship-edges.md` | `fixtures/planned/negative-borrowing-descent`, `fixtures/planned/rline-preservation` | Negative and graph fixture plans named; executable validation still blocked. |
| `validation-diagnostics.md` | all planned fixture manifests with `expected_diagnostics` | Diagnostic targets named; CLI diagnostics still blocked. |
| `fixture-promotion.md` | all planned fixture manifests plus `LEXIS-FIX-001` draft shape | Promotion shape named; no fixture promoted. |
| `graph-slice.md` | `fixtures/planned/rline-preservation` | Graph preservation fixture plan named; graph output still blocked. |
| `chronicle-output.md` | `fixtures/planned/source-limited-claim` | Chronicle fixture plan named; chronicle output still blocked. |
| `publisher-handoff.md` | `fixtures/planned/source-limited-claim` | Publisher blocker fixture plan named; handoff still blocked. |

## Source-Custody Decision Coverage

| Spec | Planned source-custody coverage | Promotion impact |
|---|---|---|
| `source-custody.md` | `source-custody/planned/*.yaml`, `source-custody/candidate-reviews/scribere-pilot-source-review.md` | Candidate source pointers exist for `LEXIS-SRCDEC-001` and `LEXIS-SRCDEC-002`; all remain unaccepted. |
| `source-record-contract.md` | `LEXIS-SRCDEC-001`, `LEXIS-SRCDEC-002`, `LEXIS-SRCDEC-003` | Contract states exist; candidate and deferred records remain blocked from claim support. |
| `language-history-model.md` | `LEXIS-SRCDEC-001`, `LEXIS-SRCDEC-002`, `LEXIS-SRCDEC-003` | Source-backed records remain blocked. |
| `relationship-edges.md` | `LEXIS-SRCDEC-002`, `LEXIS-SRCDEC-003` | Real relationship claims remain blocked until sources are accepted. |
| `fixture-promotion.md` | fixture manifests link to custody decision IDs | Fixture promotion remains blocked by unaccepted custody decisions. |
| `graph-slice.md` | graph fixture links to custody decision IDs | Graph output remains blocked until source posture survives validation. |
| `chronicle-output.md` | `LEXIS-SRCDEC-005` | Source-limited chronicle proof remains blocked. |
| `publisher-handoff.md` | `LEXIS-SRCDEC-005` | Publisher handoff remains blocked. |

## Research Plan Coverage

| Spec | Planned research coverage | Promotion impact |
|---|---|---|
| `source-custody.md` | `LEXIS-PAPER-001` plan exists; `LEXIS-PAPER-008` draft exists. | Source acceptance and source rules remain unpromoted. |
| `source-record-contract.md` | `LEXIS-PAPER-001` plan exists; `LEXIS-PAPER-008` draft exists. | Contract is draft-reviewed; validator and source acceptance remain unimplemented. |
| `language-history-model.md` | `LEXIS-PAPER-002` plan exists; `LEXIS-PAPER-008` draft exists. | Model records remain blocked. |
| `relationship-edges.md` | `LEXIS-PAPER-002`, `LEXIS-PAPER-003`, `LEXIS-PAPER-010` plans exist. | Relationship edge promotion remains blocked. |
| `validation-diagnostics.md` | `LEXIS-PAPER-002`, `LEXIS-PAPER-003`, `LEXIS-PAPER-010` plans exist. | Diagnostic schema remains blocked. |
| `fixture-promotion.md` | `LEXIS-PAPER-008` draft exists; `LEXIS-PAPER-010` plan exists. | Fixture promotion remains blocked. |
| `graph-slice.md` | `LEXIS-PAPER-006`, `LEXIS-PAPER-009` plans exist for graph preservation and script-lane graph shape. | Graph output remains blocked. |
| `chronicle-output.md` | `LEXIS-PAPER-004`, `LEXIS-PAPER-005`, `LEXIS-PAPER-007`, `LEXIS-PAPER-010` plans exist for cautious wording. | Chronicle output remains blocked. |
| `publisher-handoff.md` | `LEXIS-PAPER-012` plan exists for reviewed-slice artifact shape. | Publisher handoff remains blocked. |

## Role Review Summary

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: specs must keep slice scope and historical uncertainty visible. | Closed by scope, lifecycle, graph, and chronicle controls. |
| L-2 Etymology Reviewer | Major: relationship collapse must remain a spec-level negative case. | Closed by `relationship-edges.md` and diagnostic blocking rules. |
| L-3 Phonology Reviewer | Minor: phonology is optional in early slices but must not be silently inferred. | Closed by model and diagnostic rules for unavailable/reconstructed sound data. |
| L-4 Script Systems Reviewer | Major: script form, glyph relation, transliteration, and sound value must stay separate. | Closed by model, relationship, graph, and chronicle controls. |
| L-5 Source Custody Reviewer | Major: source records must not authorize ingestion or redistribution by implication. | Closed by source-custody and fixture-promotion blockers. |
| L-6 Graph Systems Reviewer | Major: graph/RLINE output must preserve LEXIS-owned labels. | Closed by graph-slice preservation invariants. |
| L-7 Product Chronicle Reviewer | Major: public narrative must not lead evidence or overclaim theory. | Closed by chronicle required sections and wording controls. |
| L-8 Software Assurance Reviewer | Major: draft specs need testable blockers but cannot promote without fixtures. | Closed by promotion rule and draft-reviewed status. |

## Review Decision

The spec package is draft-reviewed by the LEXIS role panel. No critical or major
actionable finding remains in the docs. The package remains blocked from
implementation promotion until scenarios, fixtures, validation commands, and
work-package evidence exist.
