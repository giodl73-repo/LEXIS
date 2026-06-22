# Validation Diagnostics Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec defines diagnostic expectations. A minimal text diagnostic surface is
implemented for the first `source_custody` validator path; broader CLI text,
exit-code, JSON schema, and fixture coverage remain future work.

## Target Behavior

Future `lexis validate` behavior should produce stable diagnostic families that
make failed source, scope, claim, relationship, graph, and chronicle checks
visible to reviewers.

## Diagnostic Families

| Family | Blocking by default | Purpose |
|---|---|---|
| `source_custody` | yes | Unreviewed, blocked, or rights-incompatible source posture. |
| `scope` | yes | Missing or overbroad slice boundaries. |
| `claim_type` | yes | Missing or contradictory evidence/theory posture. |
| `relationship` | yes | Collapsed, ambiguous, or unsupported relationship semantics. |
| `script` | yes when script edge is promoted | Script, orthography, transliteration, glyph, or sound-value confusion. |
| `reconstruction` | yes when reconstruction is promoted | Reconstructed forms written as attested or under-labeled. |
| `graph_preservation` | yes | Graph output drops LEXIS labels, uncertainty, or source posture. |
| `chronicle_overclaim` | yes | Narrative wording exceeds evidence posture. |

## Required Diagnostic Shape

Future diagnostics should include:

1. stable diagnostic id,
2. family,
3. severity,
4. affected record or edge id,
5. affected source or scope id when available,
6. requirement or spec pointer,
7. reviewer lane,
8. suggested disposition.

The current minimal text output includes diagnostic id, family, severity,
affected id, and message.

## Implemented Diagnostic IDs

| Diagnostic | Family | Condition |
|---|---|---|
| `LEXIS-DIAG-SCOPE-001` | `scope` | Graph-bearing fixture does not declare a bounded scope. |
| `LEXIS-DIAG-NODE-001` | `claim_type` | Duplicate node id. |
| `LEXIS-DIAG-NODE-002` | `claim_type` | Node is missing required id. |
| `LEXIS-DIAG-NODE-003` | `claim_type` | Claim-bearing root, wordform, or meaning node has no source links. |
| `LEXIS-DIAG-NODE-004` | `claim_type` | Root node lacks reconstruction posture or source-limited/disputed uncertainty. |
| `LEXIS-DIAG-EDGE-001` | `relationship` | Edge source or target does not resolve. |
| `LEXIS-DIAG-EDGE-002` | `relationship` | Edge kind is unsupported. |
| `LEXIS-DIAG-EDGE-003` | `relationship` | Source link or supporting source is not accepted for slice. |
| `LEXIS-DIAG-EDGE-004` | `relationship` | Same ordered pair collapses `borrowed_from` and `descends_from`. |
| `LEXIS-DIAG-EDGE-005` | `relationship` | Relationship edge has no supporting source references. |
| `LEXIS-DIAG-EDGE-006` | `relationship` | `disputes_claim` is not marked as rejected and disputed. |
| `LEXIS-DIAG-CHRON-001` | `chronicle_overclaim` | Draft fixture contains chronicle output. |
| `LEXIS-DIAG-CHRON-002` | `chronicle_overclaim` | Chronicle output uses overclaim wording. |

## Severity

| Severity | Meaning |
|---|---|
| `error` | Blocks promotion or package close. |
| `warning` | Requires reviewer disposition before close. |
| `info` | Non-blocking note for trace or review. |

## Negative Examples

- A generic "invalid fixture" message without family is insufficient.
- A source-backed claim using a `candidate_review` source record must not pass
  validation.
- A graph-preservation failure cannot be downgraded without Graph Systems
  Reviewer disposition.
- A source-custody failure cannot be hidden as informational.

## Trace

Requirements: `LEXIS-EV-001`, `LEXIS-EV-002`, `LEXIS-GR-003`,
`LEXIS-OUT-003`.

Work packages: `LEXIS-WP-003`, `LEXIS-WP-005`, `LEXIS-WP-006`,
`LEXIS-WP-008`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Minor: diagnostics should point reviewers to the historical-scope issue. | Closed by required scope/source pointers. |
| L-2 Etymology Reviewer | Major: relationship collapse needs blocking diagnostics. | Closed by `relationship` family. |
| L-3 Phonology Reviewer | Minor: reconstructed sound failures need a family. | Closed by `reconstruction` and `script` families. |
| L-4 Script Systems Reviewer | Major: script/sound confusion must be blocking when promoted. | Closed by `script` blocking rule. |
| L-5 Source Custody Reviewer | Major: custody failures cannot be informational. | Closed by `source_custody` blocking rule and negative examples. |
| L-6 Graph Systems Reviewer | Major: graph preservation failures cannot be downgraded casually. | Closed by blocking default and reviewer disposition rule. |
| L-7 Product Chronicle Reviewer | Major: chronicle overclaim needs its own diagnostic family. | Closed by `chronicle_overclaim`. |
| L-8 Software Assurance Reviewer | Major: diagnostics need stable shape before CLI work. | Closed by required diagnostic shape. |

Decision: draft-reviewed. No critical or major actionable finding remains; CLI
text, exit codes, and schemas remain future work.
