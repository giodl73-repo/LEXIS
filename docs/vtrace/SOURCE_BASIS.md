# LEXIS Source Basis

Status: settled.

LEXIS foundation work is source-pointer only. No dictionary, corpus,
inscription, article, or book text has been ingested or redistributed. The
first `scribere` source pointers are candidates only, not accepted source
records.

## Current Source Basis

| Source class | Status | Allowed use |
|---|---|---|
| VTRACE guidance in `repos/standards-protocols/vtrace` | local method reference | Derive LEXIS VTRACE artifact shape. |
| TRACKER portfolio docs | local portfolio context | Record dependency posture and repo placement. |
| Language-history external sources | candidate pointers for first `scribere` review only | Blocked from source-backed claims until source-custody acceptance. |
| Dictionaries/corpora/inscriptions | candidate dictionary pointers for first `scribere` review only | Blocked until rights and redistribution posture are reviewed. |
| Source-custody decision records | candidate / planned / deferred | Name source-family blockers; do not accept or ingest sources. |

## Planned Source-Custody Decisions

| Decision ID | Source family | Status | Allowed use now |
|---|---|---|---|
| `LEXIS-SRCDEC-001` | Latin lexicographic or dictionary reference. | candidate_review | Candidate pointer metadata only. |
| `LEXIS-SRCDEC-002` | English etymology reference. | candidate_review | Candidate pointer metadata only. |
| `LEXIS-SRCDEC-003` | General scholarly language-history reference. | deferred_after_first_slice_scope | Deferred blocker only. |
| `LEXIS-SRCDEC-004` | Script and alphabet-history reference. | planned_blocked | Planning blocker only. |
| `LEXIS-SRCDEC-005` | Unknown, restricted, or source-limited placeholder. | planned_blocked | Negative validation planning only. |

## Source Rules

- Use source pointers before source content.
- Record rights and redistribution posture before fixture promotion.
- Keep source quality separate from linguistic interpretation.
- Do not infer source permission from public availability.

## Decision

SOURCE_BASIS is settled for the foundation wave and first `scribere` candidate
review. Real source acceptance remains future work.
