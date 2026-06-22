# Paper Plan: Descent, Borrowing, Calque, and Coincidence Boundaries

Paper ID: `LEXIS-PAPER-003`

Status: planned, not written.

## Research Question

How should LEXIS distinguish descent, cognacy, borrowing, calque, coincidence,
unknown relation, competing theory, and rejected alternative without flattening
them into a generic relationship edge?

## Scope

Included:

- Baseline relationship and edge kinds.
- Non-collapse rules in `relationship-edges.md`.
- Negative fixture needs for `LEXIS-SC-002`.
- First-slice pressure around Latin and English writing-related terms.

Excluded:

- Deciding actual `scribere` pathways.
- Selecting sources or source text.
- Emitting graph output.
- Adding a new edge kind without a DCR.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `docs/specs/relationship-edges.md` | Defines current edge kinds and non-collapse rules. |
| `scenarios/language-history/borrowing-vs-descent` | Planned scenario for relationship separation. |
| `fixtures/planned/negative-borrowing-descent` | Planned negative fixture. |
| `docs/specs/graph-slice.md` | Requires edge-kind preservation. |
| `LEXIS-WP-005` | Future validation work package. |
| `LEXIS-WP-006` | Future graph output work package. |

## Expected Outputs

- Relationship boundary matrix.
- Recommendation on whether coincidence needs a distinct edge kind.
- Recommendation on how analogy should be represented or deferred.
- Required negative cases for validation.
- DCR recommendations if current edge kinds are insufficient.

## Expected Negative Findings

- `related_to` is not acceptable as a promoted edge.
- Visual similarity is not etymological proof.
- Borrowing and descent cannot be merged for readability.
- Unknown is a meaningful state, not an omission.

## Review Roles

Required: L-1 through L-8.

Primary: L-2 Etymology Reviewer, L-4 Script Systems Reviewer, and L-6 Graph
Systems Reviewer.

## Promotion Block

This plan does not authorize edge-kind changes, fixture data, graph output, or
chronicle wording.

