# Paper Plan: Claim Types in Etymology Graphs

Paper ID: `LEXIS-PAPER-002`

Status: planned, not written.

## Research Question

Are LEXIS's current claim types sufficient to represent attestation,
reconstruction, inference, competing theory, rejected alternative, and unknown
posture in language-history graph records?

## Scope

Included:

- Baseline claim types in `SPECIFICATION_BASELINE.md`.
- Draft model records in `language-history-model.md`.
- Diagnostic allocation for missing or contradictory claim posture.
- First-slice implications for `LEXIS-SLICE-001`.

Excluded:

- Selecting real source-backed claims.
- Adding Rust enum definitions.
- Changing edge kinds without a reviewed DCR.
- Writing fixture data.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `docs/specs/language-history-model.md` | Defines draft record classes and lifecycle states. |
| `docs/specs/relationship-edges.md` | Uses claim type and uncertainty on every promoted edge. |
| `docs/specs/validation-diagnostics.md` | Allocates `claim_type` diagnostics. |
| `fixtures/planned/negative-borrowing-descent` | Needs invalid missing/ambiguous claim posture cases. |
| `LEXIS-WP-002` | Future foundation model work package. |
| `LEXIS-WP-005` | Future validation CLI work package. |

## Expected Outputs

- Recommendation to keep, rename, split, or add claim types.
- Decision on whether `unknown` is enough for first-slice gaps.
- Diagnostic requirements for missing claim type.
- Negative fixture requirements for contradictory claim posture.

## Expected Negative Findings

- A reconstructed root cannot use `direct_evidence`.
- A disputed claim cannot be labeled `settled_for_slice` without review.
- An edge with no claim type cannot promote into graph output.

## Review Roles

Required: L-1 through L-8.

Primary: L-2 Etymology Reviewer and L-8 Software Assurance Reviewer.

## Promotion Block

This plan does not authorize model changes, fixture data, graph output, or CLI
diagnostics.

