# Paper Plan: The `scribere` First-Slice Source Plan

Paper ID: `LEXIS-PAPER-008`

Status: planned, not written.

## Research Question

Can the Latin `scribere` to selected English writing-related terms slice be
made source-safe enough to serve as LEXIS's first executable language-history
fixture?

## Scope

Included:

- Source-family plan for Latin `scribere`.
- Source-family plan for selected English forms such as `scribe`, `script`,
  `inscription`, and `describe`.
- Decision criteria for limiting the slice to at most five wordforms.
- Criteria for excluding full Proto-Indo-European reconstruction in the first
  slice.
- Required rejected, deferred, unknown, or source-limited alternative.

Excluded:

- Selecting real source pointers.
- Writing source-backed wordform records.
- Ingesting dictionary entries.
- Deciding final descent or borrowing pathways.
- Creating graph or chronicle output.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `LANGUAGE_SLICE_PACKAGES.md` | Defines `LEXIS-SLICE-001-*`. |
| `source-custody/` | Holds planned source-family decision stubs. |
| `fixtures/planned/source-pointer-scribere` | First source-pointer fixture plan. |
| `fixtures/planned/golden-scribere-slice` | First golden fixture plan. |
| `scenarios/language-history/word-root-slice` | First validation scenario. |
| `LEXIS-WP-004` | First fixture work package. |

## Source Decisions To Resolve Later

| Decision | Needed for |
|---|---|
| `LEXIS-SRCDEC-001` | Latin source posture. |
| `LEXIS-SRCDEC-002` | English etymology source posture. |
| `LEXIS-SRCDEC-003` | Theory or general language-history support if needed. |
| `LEXIS-SRCDEC-005` | Source-limited negative case if included. |

## Expected Outputs

- Recommendation on whether `LEXIS-SLICE-001` remains the first fixture target.
- Source-family acceptance/blocker matrix.
- First-slice scope statement.
- Explicit non-goals for PIE reconstruction and broad language-family coverage.
- Fixture readiness checklist for `LEXIS-FIX-001` and `LEXIS-FIX-002`.

## Expected Negative Findings

- If source-family decisions remain unknown, the first fixture remains blocked.
- If the slice requires broad reconstruction to explain properly, it should not
  be the first fixture.
- If selected wordforms require copied dictionary text, the fixture must remain
  pointer-only or blocked.

## Review Roles

Required: L-1 through L-8.

Primary: L-1 Language Historian, L-2 Etymology Reviewer, and L-5 Source Custody
Reviewer.

## Promotion Block

This plan does not authorize source selection, wordform fixtures, graph output,
chronicle output, or publication.

