# Publisher Handoff Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec sketches future handoff behavior for downstream artifact systems. It
does not authorize CROP, PEBBLE, FLETCH, PROOF, FONTES, or MUNDUS integration.

## Target Behavior

LEXIS may later package reviewed language-history slices for downstream context
or publication only when source posture, validation evidence, graph labels,
chronicle wording, and public-claim limits are preserved.

## Candidate Handoff Content

| Content | Required before handoff |
|---|---|
| scope summary | Reviewed `ChronicleScope`. |
| source-custody summary | Accepted source records and redistribution posture. |
| graph slice | Preserved node/edge labels and uncertainty. |
| chronicle report | Reviewed wording and source limits. |
| validation evidence | Commands or inspections recorded in VTRACE evidence. |
| public claim boundary | Allowed and blocked claims. |

## Handoff Blockers

- Source posture is pointer-only but artifact requires source text.
- Graph output drops LEXIS labels.
- Chronicle has unresolved overclaim findings.
- Validation evidence is missing.
- Downstream purpose is unbounded.

## Trace

Requirements: `LEXIS-OUT-002`, `LEXIS-OUT-003`, `LEXIS-GR-003`.

Work packages: `LEXIS-WP-009`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Minor: artifact handoff must preserve bounded scope. | Closed by required scope summary. |
| L-2 Etymology Reviewer | Minor: downstream packs must preserve alternatives and uncertainty. | Closed by graph slice and public-claim boundary requirements. |
| L-3 Phonology Reviewer | Minor: sound/reconstruction posture must survive if present. | Deferred to graph slice and chronicle content. |
| L-4 Script Systems Reviewer | Minor: script posture must survive if present. | Deferred to graph slice and chronicle content. |
| L-5 Source Custody Reviewer | Major: downstream artifacts must not require source text when posture is pointer-only. | Closed by handoff blockers. |
| L-6 Graph Systems Reviewer | Major: handoff must not drop LEXIS labels. | Closed by graph slice requirement and blocker. |
| L-7 Product Chronicle Reviewer | Major: unresolved overclaim findings must block publication. | Closed by handoff blockers. |
| L-8 Software Assurance Reviewer | Major: validation evidence must exist before handoff. | Closed by candidate content and blockers. |

Decision: draft-reviewed. No critical or major actionable finding remains;
publisher integration remains unauthorized.
