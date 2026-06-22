# LEXIS Design

Status: settled.

INTERFACES reached fixed point. Design defines the first views and output
shapes LEXIS should support. It does not implement UI, CLI, graph storage,
fixtures, source ingestion, or RLINE calls.

## Design principle

Every LEXIS view must preserve the distinction between evidence, theory,
uncertainty, and rejected alternatives. A beautiful graph or chronicle is a
failure if it makes disputed language history look settled.

## Core views

| View | Purpose | Required visible information |
|---|---|---|
| Lineage view | Follow a word/root/language feature through descent and proposed descent. | Scope, nodes, `descends_from`, `cognate_with`, uncertainty, competing theories. |
| Borrowing/contact view | Show donor/recipient paths and contact alternatives. | `borrowed_from`, `calque_of`, language/time posture, source links, rejected alternatives. |
| Sound-shift view | Inspect sound-change claims and reconstruction limits. | Sound features, unavailable data markers, source links, uncertainty, scope. |
| Meaning-drift view | Follow semantic neighborhoods through time. | Meaning senses, `meaning_shift_to`, date posture, competing theories. |
| Script/orthography view | Follow written forms, transliteration, glyph, or orthographic variants. | Script forms, script posture, transliteration posture, source links. |
| Attestation/source view | Inspect source-backed claims and custody posture. | Source pointer, rights posture, redistribution posture, citation note, reviewer state. |
| Chronicle view | Read the reviewed slice as a narrative. | Evidence path, theory path, uncertainty note, alternatives, source-custody note. |

## View composition

A `ChronicleSlice` should be the design unit that ties views together:

1. Scope defines the bounded question.
2. Source view defines allowed evidence.
3. Lineage/borrowing/sound/meaning/script views expose graph paths.
4. Chronicle view narrates only reviewed paths.
5. Review state records what is accepted, deferred, or blocked.

## RLINE design boundary

RLINE may later help compute or traverse view graphs, but design labels remain
LEXIS-owned. Any future RLINE-backed view must preserve:

- LEXIS edge kind,
- claim type,
- uncertainty label,
- source-custody state,
- rejected or competing alternatives.

If an RLINE operation cannot preserve those fields, it is not acceptable for the
first LEXIS slice.

## First-slice design preference

The first slice should prefer a small, inspectable path over breadth:

- one root or word family,
- two to five related wordforms,
- one borrowing or rejected-borrowing alternative if available,
- one sound or script lane,
- a small number of source records,
- one chronicle output.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: views must keep disputed history visible. | Closed by design principle and view fields. |
| Etymology Reviewer | Major: lineage and borrowing/contact need separate views. | Closed by distinct core views. |
| Phonology Reviewer | Major: unavailable sound data must remain visible, not guessed. | Closed by sound-shift view and RLINE preservation requirements. |
| Script Systems Reviewer | Major: script and orthography need their own view. | Closed by script/orthography view. |
| Source Custody Reviewer | Major: source and rights posture need a first-class view. | Closed by attestation/source view. |
| Graph Systems Reviewer | Major: RLINE operations must preserve LEXIS labels. | Closed by RLINE design boundary. |
| Product Chronicle Reviewer | Major: chronicle should narrate reviewed slices only. | Closed by view composition. |
| Software Assurance Reviewer | Minor: first slice should be small enough to validate. | Closed by first-slice design preference. |

## Decision

DESIGN is settled for the foundation wave. No critical or major actionable role
finding remains. CODE_RIGOR is the next VTRACE stage.
