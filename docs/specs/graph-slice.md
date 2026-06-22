# Graph Slice Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec defines LEXIS graph output behavior. RLINE adoption remains optional
and blocked until local graph-preservation checks prove that LEXIS labels
survive graph output.

## Target Behavior

A graph slice renders a bounded `ChronicleSlice` as inspectable nodes and edges
while preserving linguistic edge kind, claim type, uncertainty, source posture,
and review state.

Validated graph emission and draft graph preview are separate operations.
Preview output may render invalid draft fixtures for inspection, but must carry
preview-only posture and validation error count.

## Required Graph Slice Content

| Content | Required | Notes |
|---|---|---|
| slice id | yes | Links to `ChronicleScope`. |
| node list | yes | Includes record ids and record classes. |
| node list | yes | Includes record ids, record classes, source posture, and claim/uncertainty where applicable. |
| edge list | yes | Includes edge kinds, claim posture, and uncertainty. |
| source posture summary | yes | Names pointer-only, blocked, or accepted source state. |
| uncertainty summary | yes | Lists disputed, source-limited, possible, unavailable cases. |
| rejected/deferred claims | yes when applicable | Prevents false completeness. |
| graph engine posture | yes | local-only, RLINE-candidate, RLINE-backed, or deferred. |
| graph output posture | yes | validated or preview-only. |
| validation error count | yes for preview | Prevents preview output from looking promoted. |

## Preservation Invariants

- Edge kind must survive graph output.
- Claim type must survive graph output.
- Uncertainty label must survive graph output.
- Source-custody posture must survive graph output.
- Rejected or disputed alternatives must not disappear from reviewed slices.
- RLINE-backed output must not rename linguistic semantics into generic graph
  labels without a LEXIS-owned mapping.

## Negative Examples

- A path that shows only unlabeled connections is invalid.
- A RLINE-backed graph that drops `borrowed_from` versus `descends_from` is
  invalid.
- A graph slice that omits `source_limited` posture is invalid.

## Trace

Requirements: `LEXIS-GR-001`, `LEXIS-GR-002`, `LEXIS-GR-003`,
`LEXIS-GR-004`.

Work packages: `LEXIS-WP-006`, `LEXIS-WP-007`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Minor: graph slices should not read as complete history maps. | Closed by scope and rejected/deferred claim content. |
| L-2 Etymology Reviewer | Major: graph paths must preserve descent versus borrowing labels. | Closed by preservation invariants and negative examples. |
| L-3 Phonology Reviewer | Minor: sound-shift paths need uncertainty labels. | Closed by uncertainty preservation. |
| L-4 Script Systems Reviewer | Major: script transitions must not collapse into sound or meaning paths. | Closed by edge-kind preservation. |
| L-5 Source Custody Reviewer | Major: source posture must survive graph output. | Closed by required content and preservation invariants. |
| L-6 Graph Systems Reviewer | Major: RLINE adoption needs local preservation proof first. | Closed by authority boundary and graph engine posture. |
| L-7 Product Chronicle Reviewer | Minor: graph summary should feed chronicle structure. | Deferred to `chronicle-output.md`. |
| L-8 Software Assurance Reviewer | Major: graph preservation needs negative tests. | Closed by negative examples and trace to WP-006/WP-007. |

Decision: draft-reviewed. No critical or major actionable finding remains;
RLINE adoption remains blocked.
