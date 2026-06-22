# LEXIS Language Slice Packages

Status: planning.

This plan expands the generic implementation work packages into repeatable
language-slice package sets. Each slice is a bounded research lane: it starts
with source custody and ends, if validated, with a graph slice and chronicle.

The first `scribere` slice now has an accepted pointer-only fixture and
validated graph artifacts. The remaining slice sets have candidate-only fixture
and preview artifact tracks. These packages still do not authorize source
ingestion, copied source text, or public publication.

## Slice Package Pattern

Each language slice follows the same package pattern:

| Step | Package suffix | Purpose | Required output before next step |
|---:|---|---|---|
| 1 | `SOURCE` | Review source families, rights posture, and allowed use. | Source-custody decision with pointer-only rules. |
| 2 | `SCOPE` | Bound the word/root/script/language question. | Chronicle scope and explicit non-goals. |
| 3 | `NODES` | Draft language, script, wordform, meaning, root, and attestation nodes. | Source-linked entity rows. |
| 4 | `EDGES` | Draft descent, cognate, borrowing, calque, sound, meaning, script, support, and dispute edges. | Relationship rows with uncertainty labels. |
| 5 | `NEGATIVES` | Add rejected, disputed, unknown, source-limited, or unavailable cases. | Negative/rejected alternatives. |
| 6 | `GRAPH` | Produce a bounded graph slice preserving LEXIS labels. | Graph slice artifact or explicit graph deferral. |
| 7 | `CHRONICLE` | Produce a narrative from the reviewed graph slice. | Chronicle report with evidence/theory/source limits. |
| 8 | `PACK` | Plan or emit downstream package when allowed. | Publisher plan or blocked status. |

## Slice Set 1: Latin `scribere`

Goal: prove the first word-history slice around writing without broad
Indo-European reconstruction.

| Package | Work | Notes |
|---|---|---|
| `LEXIS-SLICE-001-SOURCE` | Review source families for Latin `scribere`, English scribe/script/inscription/describe, and allowed dictionary/reference pointers. | Source pointers only; no dictionary text redistribution. |
| `LEXIS-SLICE-001-SCOPE` | Bound the slice to Latin plus selected English descendants/borrowings. | Explicitly exclude full PIE reconstruction unless later reviewed. |
| `LEXIS-SLICE-001-NODES` | Add planned nodes for Latin `scribere`, English `scribe`, `script`, `inscription`, `describe`, selected meanings, and source pointers. | Keep to at most five wordforms. |
| `LEXIS-SLICE-001-EDGES` | Add descent/borrowing/meaning/script edges as source-supported or theory-supported claims. | Distinguish direct Latin inheritance from French/English borrowing pathways if included. |
| `LEXIS-SLICE-001-NEGATIVES` | Record at least one rejected/deferred relation or source-limited claim. | Prevent the first slice from looking falsely complete. |
| `LEXIS-SLICE-001-GRAPH` | Emit bounded graph slice after validation CLI exists. | May use local graph structures before RLINE. |
| `LEXIS-SLICE-001-CHRONICLE` | Write the first LEXIS chronicle around writing/script/scribe. | Only after reviewed graph slice. |
| `LEXIS-SLICE-001-PACK` | Plan CROP/PEBBLE/FLETCH/PROOF handoff. | Blocked until validation and trace evidence exist. |

## Slice Set 2: Greek Alphabet To Latin/Cyrillic Lane

Goal: exercise script and alphabet evolution without making a universal writing
system taxonomy.

| Package | Work | Notes |
|---|---|---|
| `LEXIS-SLICE-002-SOURCE` | Review source families for Greek, Latin, and Cyrillic script-history pointers. | Prefer public reference pointers and source-safe diagrams later. |
| `LEXIS-SLICE-002-SCOPE` | Bound to a small set of letter/script-form transitions. | Avoid full alphabet history. |
| `LEXIS-SLICE-002-NODES` | Add planned language/script nodes and selected script forms. | Use `ScriptForm` first; wordforms optional. |
| `LEXIS-SLICE-002-EDGES` | Add `script_variant_of` / support/dispute edges. | Keep sound values separate from glyph forms. |
| `LEXIS-SLICE-002-NEGATIVES` | Record unavailable or disputed script-transition claims. | Especially where visual similarity is not proof. |
| `LEXIS-SLICE-002-GRAPH` | Emit script-transition graph slice. | Strong RLINE candidate after label preservation is proven. |
| `LEXIS-SLICE-002-CHRONICLE` | Chronicle a bounded alphabet/script lane. | Must name source limits and visual/theory distinction. |
| `LEXIS-SLICE-002-PACK` | Plan downstream script-history artifact. | Blocked until source and graph validation. |

## Slice Set 3: PIE Root Mini-Slice

Goal: exercise reconstruction, uncertainty, and cognate handling.

| Package | Work | Notes |
|---|---|---|
| `LEXIS-SLICE-003-SOURCE` | Review sources for one reconstructed PIE root and selected descendants. | Higher source and overclaim risk. |
| `LEXIS-SLICE-003-SCOPE` | Bound to one root and two to four descendants. | No broad Indo-European tree. |
| `LEXIS-SLICE-003-NODES` | Add root, language, wordform, and meaning nodes. | Root must be marked reconstruction. |
| `LEXIS-SLICE-003-EDGES` | Add cognate/descent/support edges. | Preserve uncertainty and source limits. |
| `LEXIS-SLICE-003-NEGATIVES` | Add competing theory or source-limited alternative. | Required before chronicle. |
| `LEXIS-SLICE-003-GRAPH` | Emit cognate/reconstruction graph slice. | Good test for uncertainty labels. |
| `LEXIS-SLICE-003-CHRONICLE` | Chronicle reconstruction carefully. | Must avoid "proved" language. |
| `LEXIS-SLICE-003-PACK` | Plan reusable reconstruction-context pack. | Blocked until validation. |

## Slice Set 4: Semitic Root Pattern Mini-Slice

Goal: exercise root-and-pattern morphology plus script/transliteration caution.

| Package | Work | Notes |
|---|---|---|
| `LEXIS-SLICE-004-SOURCE` | Review source families for Hebrew/Arabic/Aramaic root-pattern examples. | Requires careful transliteration and rights posture. |
| `LEXIS-SLICE-004-SCOPE` | Bound to one root pattern and a small language set. | Avoid broad Semitic grammar. |
| `LEXIS-SLICE-004-NODES` | Add language, script, root, wordform, and meaning nodes. | Script/transliteration posture is first-class. |
| `LEXIS-SLICE-004-EDGES` | Add root-pattern, meaning, script, and support/dispute edges. | May require a future edge kind if baseline is insufficient. |
| `LEXIS-SLICE-004-NEGATIVES` | Record transliteration/source-limited cases. | Prevent false equivalence across scripts. |
| `LEXIS-SLICE-004-GRAPH` | Emit root-pattern graph slice. | Good test for whether LEXIS needs a morphology edge. |
| `LEXIS-SLICE-004-CHRONICLE` | Chronicle the root-pattern example. | Must be readable to non-specialists. |
| `LEXIS-SLICE-004-PACK` | Plan reusable non-Indo-European example pack. | Blocked until source review. |

## Slice Set 5: Glyph/Graph/Write Meta-Slice

Goal: connect LEXIS identity to writing/marking words while avoiding unrelated
root conflation.

| Package | Work | Notes |
|---|---|---|
| `LEXIS-SLICE-005-SOURCE` | Review sources for glyph, graph, write, and related terms. | Likely multiple unrelated roots. |
| `LEXIS-SLICE-005-SCOPE` | Decide whether this is one slice or multiple contrastive slices. | May split into separate packages. |
| `LEXIS-SLICE-005-NODES` | Add planned wordform/root/script nodes only after scope review. | Keep unrelated roots separate. |
| `LEXIS-SLICE-005-EDGES` | Add support/dispute/unknown edges where relation is unclear. | This slice should demonstrate non-relationship as useful knowledge. |
| `LEXIS-SLICE-005-NEGATIVES` | Explicitly reject folk-etymology or superficial similarity claims. | Strong overclaim test. |
| `LEXIS-SLICE-005-GRAPH` | Emit contrast graph if validated. | Good graph demonstration of "not related". |
| `LEXIS-SLICE-005-CHRONICLE` | Chronicle why similar-looking terms may not share lineage. | Good public-facing education piece. |
| `LEXIS-SLICE-005-PACK` | Plan identity/meta-language pack. | Later than slice 1. |

## Cross-Slice Learning Order

| Phase | Slice focus | What it proves |
|---|---|---|
| A | `scribere` word family | Basic source -> wordform -> meaning -> borrowing -> chronicle path. |
| B | Greek alphabet lane | Script-form and alphabet-transition handling. |
| C | PIE mini-slice | Reconstruction, cognates, uncertainty, and disputed alternatives. |
| D | Semitic root pattern | Non-Indo-European root/script/transliteration handling. |
| E | Glyph/graph/write meta-slice | Negative relationship claims and public explanation. |

## Mapping To Generic Work Packages

| Slice step | Generic WP |
|---|---|
| `SOURCE` | LEXIS-WP-003 |
| `SCOPE` / `NODES` | LEXIS-WP-004 |
| `NODES` / `EDGES` model support | LEXIS-WP-002 |
| `NEGATIVES` / validation | LEXIS-WP-005 |
| `GRAPH` | LEXIS-WP-006 |
| RLINE-backed `GRAPH` | LEXIS-WP-007 |
| `CHRONICLE` | LEXIS-WP-008 |
| `PACK` | LEXIS-WP-009 |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: slices need bounded scope and learning order. | Closed by slice sets and cross-slice order. |
| Etymology Reviewer | Major: slice packages must include negative/rejected relationship work. | Closed by `NEGATIVES` step in every slice. |
| Phonology Reviewer | Minor: phonology is not equally central in all slices. | Closed by optional sound handling and PIE/Semitic later slices. |
| Script Systems Reviewer | Major: script/alphabet work needs its own slice, not a side note. | Closed by Slice Set 2. |
| Source Custody Reviewer | Major: every slice must begin with source review. | Closed by `SOURCE` step in every slice. |
| Graph Systems Reviewer | Major: graph work should not precede validated nodes/edges. | Closed by package pattern. |
| Product Chronicle Reviewer | Major: chronicles must follow graph/review, not lead it. | Closed by package pattern. |
| Software Assurance Reviewer | Major: slice packages must map back to generic WPs. | Closed by mapping table. |

## Decision

LANGUAGE_SLICE_PACKAGES is accepted as planning input. It does not authorize
source ingestion, fixtures, code, graph output, or chronicle claims.
