# LEXIS Domain Backlog

Status: planning.

This backlog turns the LEXIS entity vocabulary into a domain build-out plan. It
does not contain real language data yet. It defines the order in which language,
word, sound, script, meaning, source, graph, and chronicle entities should be
researched and implemented.

## Build Principle

Build from the most source-safe and reviewable entities outward:

1. source posture,
2. bounded scope,
3. language and script identifiers,
4. attested wordforms,
5. meaning senses,
6. relationship claims,
7. sound/script/meaning drift,
8. graph slices,
9. chronicle output,
10. publisher artifacts.

The first real domain slice should be small enough that every claim can be
reviewed by hand.

## Entity Backlog

| Order | Entity | Purpose | First questions | Needs before implementation | First artifact |
|---:|---|---|---|---|---|
| 1 | `SourceRecord` / `Attestation` | Anchor claims to source pointers and rights posture. | What source families are allowed? What can be quoted, summarized, or only pointed to? | Source-custody review. | Source-custody stub and source pointer rows. |
| 2 | `ChronicleScope` | Bound the language-history question. | Which word/root? Which languages? Which date range? Which non-goals? | Source candidate and reviewer approval. | Scope record. |
| 3 | `Language` | Identify language or bounded variety nodes. | What label, period, geography, and evidence posture are needed? | Canonical naming convention. | Language nodes for first slice. |
| 4 | `ScriptForm` | Represent written form, script, orthography, transliteration, or glyph variant. | Which script lane matters? Is transliteration source-backed? | Source and script-system review. | Script lane records. |
| 5 | `Wordform` | Represent attested or reconstructed lexical forms. | Which forms are attested? Which are reconstructed? What source/date posture applies? | Source records and language nodes. | Wordform nodes. |
| 6 | `MeaningSense` | Track meaning and semantic neighborhood. | What meaning is attested? What drift is claimed? What is inferred? | Wordforms and source posture. | Meaning nodes. |
| 7 | `Root` | Represent cited or reconstructed roots. | Is the root source-backed, reconstructed, disputed, or out of scope? | Wordform and theory posture. | Root node or explicit deferral. |
| 8 | `CognateSet` | Group descent-related forms. | Which forms are proposed cognates? What alternatives are rejected? | Wordforms, roots, relationship rules. | Cognate set record. |
| 9 | `Borrowing` / contact claim | Represent transfer across languages. | Is this descent, borrowing, calque, coincidence, or unknown? | Source/theory review and negative examples. | Borrowing/contact edge. |
| 10 | `SoundFeature` / `SoundShift` | Track pronunciation or sound-change claims. | Is sound data available? Is it reconstructed? Does it matter to the first slice? | Phonology review. | Optional sound lane. |
| 11 | `MeaningShift` | Track semantic drift. | What changed in meaning, and when? Is the shift attested or inferred? | Meaning senses and source posture. | Meaning-shift edge. |
| 12 | `TheoryClaim` | Separate interpretation from evidence. | What explanation is being proposed? What alternatives remain? | Evidence links and uncertainty labels. | Theory claim record. |
| 13 | `GraphSlice` / `ChronicleSlice` | Package reviewed nodes and edges for inspection. | Which claims are included, excluded, rejected, or deferred? | Model, fixture, validation CLI. | Graph slice artifact. |
| 14 | `Chronicle` | Narrative output from a reviewed slice. | Can a reader follow evidence, theory, uncertainty, and source limits? | Reviewed graph slice and overclaim gate. | Chronicle report. |
| 15 | Publisher artifact | Future CROP/PEBBLE/FLETCH/PROOF handoff. | What downstream task needs this slice? What rights posture applies? | Validation and trace. | Bounded language-history pack. |

## Candidate First Domain Slices

| Rank | Slice | Entities exercised | Why it fits | Risk |
|---:|---|---|---|---|
| 1 | Latin `scribere` to scribe/script/inscription/describe family | source, language, wordform, meaning, borrowing, script lane, chronicle | Strong thematic fit for writing and LEXIS; likely source-safe with pointers; good semantic drift. | Need careful source selection and no dictionary text redistribution. |
| 2 | Greek alphabet to Latin/Cyrillic script lane | script, language, source, theory, graph slice | Strong alphabet evolution story and visual graph. | Script history can get broad fast; needs tight scope. |
| 3 | Proto-Indo-European root to English/Latin/Greek descendants | root, cognate set, reconstruction, uncertainty | Classic lineage graph. | Reconstruction and disputed theories raise overclaim risk. |
| 4 | Semitic root pattern across Hebrew/Arabic/Aramaic | root, language, wordform, meaning, script | Shows non-Indo-European structure and script/language distinction. | Requires careful transliteration/source handling. |
| 5 | The word family around glyph/graph/write | thematic meta-slice | Ties LEXIS identity to language history. | May mix unrelated roots if not tightly scoped. |

## Recommended First Slice

Start with a `scribere` / scribe / script / inscription / describe slice.

First-slice constraints:

- Use source pointers only.
- Limit to Latin plus selected English descendants/borrowings.
- Include at most five wordforms.
- Include one script/orthography lane only if source-safe.
- Include at least one rejected or deferred relationship claim.
- Include one chronicle report after validation, not before.

## Entity Dependency Graph

```text
SourceRecord
  -> Attestation
  -> Language
  -> ScriptForm
  -> Wordform
       -> MeaningSense
       -> Root
       -> CognateSet
       -> Borrowing/contact claim
       -> SoundFeature/SoundShift
       -> MeaningShift
       -> TheoryClaim
            -> GraphSlice/ChronicleSlice
                 -> Chronicle
                      -> Publisher artifact
```

## Work-Package Mapping

| Backlog area | Work package |
|---|---|
| Source and attestation posture | LEXIS-WP-003 |
| First bounded domain scope | LEXIS-WP-004 |
| Foundation model entities | LEXIS-WP-002 |
| Validation of entity records | LEXIS-WP-005 |
| Graph slice and relationship edges | LEXIS-WP-006 |
| Optional RLINE adoption | LEXIS-WP-007 |
| Chronicle report | LEXIS-WP-008 |
| Publisher handoff | LEXIS-WP-009 |

Repeatable language-slice package sets live in
[`LANGUAGE_SLICE_PACKAGES.md`](LANGUAGE_SLICE_PACKAGES.md).

Research papers that de-risk and refine these entities live in
[`RESEARCH_PLAN.md`](RESEARCH_PLAN.md). Research findings must flow through a
VTRACE DCR before changing entity requirements or slice order.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: backlog must avoid universal language-family scope. | Closed by first-slice constraints. |
| Etymology Reviewer | Major: first slice needs rejected/deferred relationship claims. | Closed by first-slice constraints. |
| Phonology Reviewer | Minor: sound lane can be optional in the first slice. | Closed by entity order and first-slice constraints. |
| Script Systems Reviewer | Minor: script lane should be included only when source-safe. | Closed by first-slice constraints. |
| Source Custody Reviewer | Major: source pointers must precede wordform fixtures. | Closed by build principle and entity order. |
| Graph Systems Reviewer | Major: graph slices should come after validated records. | Closed by dependency graph and work-package mapping. |
| Product Chronicle Reviewer | Major: chronicle should be last, not first. | Closed by entity order. |
| Software Assurance Reviewer | Major: backlog must map to work packages. | Closed by work-package mapping. |

## Decision

DOMAIN_BACKLOG is accepted as planning input. It does not authorize source
ingestion, fixtures, code, graph output, or chronicle claims.
