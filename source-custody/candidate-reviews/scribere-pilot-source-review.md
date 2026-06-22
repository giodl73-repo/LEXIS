# Scribere Pilot Source Review

Status: candidate review, not accepted.

This record captures first-pass source candidates for `LEXIS-SLICE-001-SOURCE`.
It does not accept any source, quote source text, ingest dictionary entries,
cache source material, or authorize source-backed LEXIS claims.

## Candidate Matrix

| Decision | Candidate pointer | Candidate use | Current posture | Promotion blocker |
|---|---|---|---|---|
| `LEXIS-SRCDEC-001` | Lewis and Short `scribo` entry via Scaife ATLAS / Perseus | Latin source pointer for `scribere` / `scribo` posture. | Candidate pointer-only review. | Confirm rights, citation format, stable identifier, and whether a second Latin reference is required. |
| `LEXIS-SRCDEC-002` | Merriam-Webster `scribe` entry | English pointer for one first-slice English form. | Candidate copyrighted pointer-only review. | Confirm whether source is sufficient for etymology authority or only useful as public pointer context. |
| `LEXIS-SRCDEC-002` | Online Etymology Dictionary family pointer | English etymology candidate family for selected forms. | Candidate pointer-only review. | Review source methodology, licensing, citation, and whether entries may be cited only by pointer. |
| `LEXIS-SRCDEC-003` | None selected | General theory support. | Deferred. | Required only if first slice expands into reconstruction, broad language-family theory, or disputed historical explanation. |

## First-Slice Source Rule

`LEXIS-SLICE-001` may move toward a source-pointer fixture only if every source
record remains pointer-only or stronger, has an explicit rights posture, and is
reviewed by L-5. The first fixture must not store copied dictionary text.

## Scope Recommendation

Keep the pilot to a Latin headword/source pointer plus no more than four
selected English wordforms. `scribe` should be the first English form because it
keeps the source review narrow. `script`, `inscription`, and `describe` remain
candidate expansions after source-family posture is reviewed.

## Non-Goals

- No Proto-Indo-European reconstruction in the pilot.
- No broad history of writing.
- No copied dictionary definitions, example citations, or entry text.
- No graph or chronicle output until accepted source records and fixture schema
  exist.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: first slice should not need broad reconstruction to be useful. | Closed by deferring `LEXIS-SRCDEC-003` and excluding PIE reconstruction. |
| L-2 Etymology Reviewer | Major: public dictionary pointers can be helpful but may not be sufficient final etymology authority. | Open as promotion blocker for `LEXIS-SRCDEC-002`. |
| L-3 Phonology Reviewer | Minor: first slice does not exercise pronunciation or sound-change proof. | Accepted as a deliberate scope limit. |
| L-4 Script Systems Reviewer | Minor: writing/script words should not become an alphabet-history claim. | Closed by non-goals. |
| L-5 Source Custody Reviewer | Major: candidate pointers must not imply source acceptance or text redistribution. | Closed by pointer-only posture and promotion block. |
| L-6 Graph Systems Reviewer | Major: graph output remains blocked until accepted source records and model fixtures exist. | Closed by source rule. |
| L-7 Product Chronicle Reviewer | Major: no public chronicle should be written from candidate-only sources. | Closed by non-goals. |
| L-8 Software Assurance Reviewer | Major: candidate review needs validation checks before fixture promotion. | Open until source record schema and validator exist. |

## Decision

The `scribere` pilot can proceed to source-scope planning with candidate source
pointers. It cannot promote source-backed fixtures, graph slices, chronicle
output, or public claims.
