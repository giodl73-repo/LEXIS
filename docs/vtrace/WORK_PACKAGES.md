# LEXIS Work Packages

Status: settled.

IMPLEMENTATION_PLAN reached fixed point. Work packages define the future
execution sequence only. They do not authorize implementation until
VERIFICATION and VALIDATION settle.

## Package sequence

| WP | Name | Outcome | Primary gate |
|---|---|---|---|
| LEXIS-WP-001 | Repo implementation scaffold | Rust workspace skeleton, README validation update, no promoted domain fixture. | `cargo test`, `cargo fmt --check`, `git diff --check`. |
| LEXIS-WP-002 | Foundation model | Local model for settled baseline vocabulary, claim types, edge kinds, and uncertainty labels. | Unit tests for valid/invalid records. |
| LEXIS-WP-003 | Source-custody stub | Source record review structure with rights and redistribution posture. | Negative tests for unreviewed source promotion. |
| LEXIS-WP-004 | First fixture | One narrow word/root-family fixture using source pointers only. | Source Custody Reviewer approval and fixture validation. |
| LEXIS-WP-005 | Validation CLI | `lexis validate <fixture>` starts with source-custody validation for the blocked `LEXIS-FIX-001` fixture. | Positive and negative fixture tests. |
| LEXIS-WP-006 | Graph slice output | Bounded graph slice preserving LEXIS labels. | Graph preservation tests; RLINE adoption decision or deferral. |
| LEXIS-WP-007 | RLINE adoption | Optional RLINE graph-mechanics integration after local preservation checks. | No dropped labels; dependency tracker update. |
| LEXIS-WP-008 | Chronicle output | Narrative report from reviewed graph slice. | Overclaim review and required-section checks. |
| LEXIS-WP-009 | Publisher planning | Sketch CROP/PEBBLE/FLETCH/PROOF artifact handoff. | No artifact publication before VALIDATION and TRACE. |

## Language-Slice Package Sets

The generic work packages above are executed through repeatable language-slice
sets in [`../../LANGUAGE_SLICE_PACKAGES.md`](../../LANGUAGE_SLICE_PACKAGES.md).
Each slice uses the sequence `SOURCE -> SCOPE -> NODES -> EDGES -> NEGATIVES ->
GRAPH -> CHRONICLE -> PACK`.

Initial planned slice sets:

| Slice set | Focus | Status |
|---|---|---|
| `LEXIS-SLICE-001-*` | Latin `scribere` to scribe/script/inscription/describe. | planned |
| `LEXIS-SLICE-002-*` | Greek alphabet to Latin/Cyrillic script lane. | planned |
| `LEXIS-SLICE-003-*` | PIE root mini-slice. | planned |
| `LEXIS-SLICE-004-*` | Semitic root-pattern mini-slice. | planned |
| `LEXIS-SLICE-005-*` | glyph/graph/write meta-slice. | planned |

## Acceptance gates

Each work package must record:

- exact changed files,
- validation commands,
- role findings,
- source-custody posture,
- RLINE posture if graph work is touched,
- public wording/overclaim posture,
- deferred risks.

## Required negative tests

Future implementation must include negative cases for:

- missing scope,
- missing claim type,
- missing uncertainty where needed,
- unreviewed source promotion,
- source redistribution not allowed by rights posture,
- collapsed descent/borrowing relationship,
- graph output that drops edge kind or source posture,
- chronicle wording that overclaims disputed evidence.

## Implementation block

These packages remain planning records until VERIFICATION and VALIDATION define
how the package outputs are checked and judged useful. REVIEW decides whether
any package may be promoted as a release-ready foundation.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: first fixture must not imply broad language-family coverage. | Closed by WP-004 narrow fixture gate. |
| Etymology Reviewer | Major: negative tests need relationship collapse failures. | Closed by required negative tests. |
| Phonology Reviewer | Minor: first fixture can defer sound details if unavailable. | Deferred to WP-004 source choice. |
| Script Systems Reviewer | Minor: first fixture can choose script lane if source-safe. | Deferred to WP-004 source choice. |
| Source Custody Reviewer | Major: source-custody work must precede fixture promotion. | Closed by WP-003 before WP-004. |
| Graph Systems Reviewer | Major: RLINE adoption must be optional and separately reviewed. | Closed by WP-006/WP-007 split. |
| Product Chronicle Reviewer | Major: chronicle output waits for reviewed graph slice. | Closed by WP-008 placement after graph slice output. |
| Software Assurance Reviewer | Major: work packages need negative tests and validation commands. | Closed by acceptance gates and required negative tests. |

## Decision

WORK_PACKAGES is settled for the foundation wave. No critical or major
actionable role finding remains. VERIFICATION is the next VTRACE stage.
