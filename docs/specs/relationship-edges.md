# Relationship Edge Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

LEXIS owns linguistic relationship semantics. RLINE may later provide graph
mechanics, but it does not define what a LEXIS edge means.

## Target Behavior

LEXIS relationship edges preserve the difference between evidence, descent,
cognacy, borrowing, calque, sound change, meaning change, script variation,
support, dispute, unknowns, and rejected alternatives.

## Edge Requirements

Every promoted edge must define:

1. edge id,
2. edge kind,
3. source record id,
4. target record id,
5. claim type,
6. uncertainty label,
7. supporting source or theory claim,
8. review state,
9. negative or alternative relation when applicable.

## Edge Kinds

Use the baseline edge kinds:

- `attested_as`,
- `descends_from`,
- `cognate_with`,
- `borrowed_from`,
- `calque_of`,
- `sound_shift_to`,
- `meaning_shift_to`,
- `script_variant_of`,
- `supports_claim`,
- `disputes_claim`.

## Non-Collapse Rules

- `descends_from` must not be used for borrowing or contact transfer.
- `cognate_with` must not imply direct borrowing.
- `calque_of` must not imply direct lexical borrowing.
- `script_variant_of` must not imply sound change.
- `supports_claim` must not be rendered as proof.
- A rejected superficial similarity should be represented as a rejected
  alternative or dispute, not omitted.

## Positive Example

A `scribere` first slice may include `borrowed_from` or `descends_from` only
when the source/theory posture supports the chosen pathway and alternatives are
marked unknown, rejected, or out of scope.

## Negative Examples

- A graph path that labels every connection as `related_to` is invalid.
- A visual glyph similarity edge used as etymological proof is invalid.
- A disputed cognate relation without `disputed` or `competing_theory` posture
  is invalid.

## Diagnostics

Future validation should allocate failures to `relationship`, `claim_type`,
`script`, or `reconstruction`.

## Trace

Requirements: `LEXIS-FR-003`, `LEXIS-EV-004`, `LEXIS-GR-003`.

Work packages: `LEXIS-WP-002`, `LEXIS-WP-005`, `LEXIS-WP-006`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: edge labels must not imply a complete lineage. | Closed by uncertainty and alternative requirements. |
| L-2 Etymology Reviewer | Major: descent, cognacy, borrowing, calque, and superficial similarity must not collapse. | Closed by non-collapse rules and negative examples. |
| L-3 Phonology Reviewer | Minor: sound-shift edges require reconstruction posture when inferred. | Closed by claim type and uncertainty requirements. |
| L-4 Script Systems Reviewer | Major: visual similarity must not become etymological proof. | Closed by `script_variant_of` non-collapse and negative examples. |
| L-5 Source Custody Reviewer | Minor: edges need support pointers before promotion. | Closed by edge requirements. |
| L-6 Graph Systems Reviewer | Major: graph output must preserve edge kinds. | Closed by trace to `LEXIS-GR-003` and graph spec. |
| L-7 Product Chronicle Reviewer | Minor: relationship labels need readable chronicle translation later. | Deferred to `chronicle-output.md`. |
| L-8 Software Assurance Reviewer | Major: ambiguous relationship validation needs stable diagnostic allocation. | Closed by diagnostic families and negative examples. |

Decision: draft-reviewed. No critical or major actionable finding remains;
relationship fixtures remain future work.
