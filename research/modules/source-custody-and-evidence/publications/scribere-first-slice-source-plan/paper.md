# The `scribere` First-Slice Source Plan

Paper ID: `LEXIS-PAPER-008`

Status: draft for role review, not published.

## Research Question

Can the Latin `scribere` to selected English writing-related terms slice be
made source-safe enough to serve as LEXIS's first executable language-history
fixture?

## Draft Finding

Yes, but only as a source-pointer pilot first. The slice is suitable because it
can be bounded to one Latin source-family candidate, one English source-family
candidate, and a small wordform set. It should not begin as a broad
Proto-Indo-European reconstruction, a history of writing, or a public chronicle.

The source-custody outcome is candidate review, not acceptance. `LEXIS-SRCDEC-001`
and `LEXIS-SRCDEC-002` now have candidate pointers. `LEXIS-SRCDEC-003` remains
deferred unless the slice expands into reconstruction or theory comparison.

## Candidate Sources

| Decision | Candidate posture | Use allowed now | Use blocked now |
|---|---|---|---|
| `LEXIS-SRCDEC-001` | Lewis and Short `scribo` pointer via Scaife ATLAS / Perseus. | Record as candidate pointer metadata. | Copying entry text, accepting Latin claims, graph output, chronicle wording. |
| `LEXIS-SRCDEC-002` | Merriam-Webster `scribe` pointer and Etymonline family pointer. | Record as candidate pointer metadata. | Treating either source as sufficient final authority before review. |
| `LEXIS-SRCDEC-003` | None selected. | Defer. | Reconstruction, broad theory claims, and competing-theory fixtures. |

## Scope Decision

The first executable fixture should start with:

- one Latin headword/source pointer lane around `scribere` / `scribo`;
- one English form lane for `scribe`;
- optional later expansion to `script`, `inscription`, and `describe`;
- one required negative or deferred claim showing that candidate-only sources do
  not authorize a complete lineage graph.

## Fixture Readiness

| Fixture | Current readiness | Reason |
|---|---|---|
| `LEXIS-FIX-001-source-pointer-scribere` | partially unblocked for schema planning | Candidate pointers exist, but accepted source records and validator do not. |
| `LEXIS-FIX-002-golden-scribere-slice` | blocked | Requires accepted source records, model schema, relationship rules, and validation command. |

## Requirements Pressure

This paper pushes the next implementation requirements toward:

- a source-record schema that can represent candidate, accepted, deferred, and
  blocked postures;
- pointer-only fixture rows that preserve source IDs without copying source
  text;
- diagnostics that block graph and chronicle promotion when source records are
  candidate-only;
- a first-slice scope file before wordform and relationship rows exist.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: keep the first slice historically useful but narrow. | Closed by limiting the first fixture to source pointers and `scribe` first. |
| L-2 Etymology Reviewer | Major: English candidate sources may need stronger scholarly backup. | Open; blocks golden fixture promotion. |
| L-3 Phonology Reviewer | Minor: no sound-change proof is attempted. | Accepted for first source-pointer fixture. |
| L-4 Script Systems Reviewer | Minor: writing-related words should not imply alphabet-history modeling. | Closed by excluding broad writing history. |
| L-5 Source Custody Reviewer | Major: no source text can enter fixtures under candidate review. | Closed by pointer-only posture. |
| L-6 Graph Systems Reviewer | Major: graph slices cannot consume candidate-only source records. | Closed as a validator requirement. |
| L-7 Product Chronicle Reviewer | Major: no chronicle can be written from candidate-only posture. | Closed as a promotion blocker. |
| L-8 Software Assurance Reviewer | Major: next work needs schema and validation evidence. | Open; becomes next implementation input. |

## Conclusion

`LEXIS-SLICE-001` remains the right first slice, but only if LEXIS promotes a
source-pointer fixture before a golden language-history fixture. The next work
package should define the source-record contract and validator behavior for
candidate, accepted, deferred, and blocked source states.
