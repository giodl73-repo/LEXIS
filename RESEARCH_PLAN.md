# LEXIS Research Plan

Status: planning.

LEXIS research turns the work packages and language slices into reviewed
requirements input. It follows a PANEL-style rhythm: bounded research modules,
paper plans, paper-level review, module-level review, and VTRACE change records
when findings need to modify requirements, models, source rules, or package
order.

No paper below is written yet. This plan does not authorize source ingestion,
fixture creation, graph output, chronicle claims, or publication.

The first planned module exists at
`research/modules/source-custody-and-evidence/` with plan files for
`LEXIS-PAPER-001` and `LEXIS-PAPER-008`.

The second planned module exists at
`research/modules/ontology-and-relationship-semantics/` with plan files for
`LEXIS-PAPER-002`, `LEXIS-PAPER-003`, and `LEXIS-PAPER-010`.

The third planned module exists at
`research/modules/script-and-reconstruction-caution/` with plan files for
`LEXIS-PAPER-004`, `LEXIS-PAPER-005`, `LEXIS-PAPER-009`, and
`LEXIS-PAPER-011`.

The fourth planned module exists at
`research/modules/graph-and-chronicle-method/` with plan files for
`LEXIS-PAPER-006` and `LEXIS-PAPER-007`.

The fifth planned module exists at
`research/modules/publisher-context/` with a plan file for `LEXIS-PAPER-012`.

## Research Operating Model

Each research item uses this sequence:

| Step | Output | Purpose |
|---:|---|---|
| 1 | `plan.md` | State the question, scope, source families, expected requirement impact, and review roles. |
| 2 | `paper.md` | Write a bounded argument with source pointers, evidence/theory separation, and open questions. |
| 3 | paper review | Record L-1 through L-8 findings, including overclaim and source-custody checks. |
| 4 | module review | Decide whether related papers change LEXIS requirements or package order. |
| 5 | VTRACE feedback | Promote accepted findings through DCRs into `REQUIREMENTS.md`, `SPEC_MODEL.md`, `SOURCE_BASIS.md`, `WORK_PACKAGES.md`, `LANGUAGE_SLICE_PACKAGES.md`, or future implementation specs. |

Research must be useful even when it blocks a slice. A rejected source family,
ambiguous relationship, or impossible scope is a valid research result if it
prevents misleading requirements.

## Research Tracks

| Track | Module | Drives | First requirement pressure |
|---|---|---|---|
| R-1 | Source Custody and Lexicographic Evidence | `LEXIS-WP-003`, all `*-SOURCE` packages | Which references can be pointed to, summarized, quoted briefly, or excluded. |
| R-2 | Entity Ontology and Claim Semantics | `LEXIS-WP-002`, `*-NODES`, `*-EDGES` | Whether current entities and claim types can represent real language-history cases. |
| R-3 | Etymology Relationship Typology | `LEXIS-WP-002`, `LEXIS-WP-005`, `*-NEGATIVES` | How descent, borrowing, cognate, calque, analogy, coincidence, disputed, and unknown differ. |
| R-4 | Script, Orthography, and Transliteration | `LEXIS-SLICE-002-*`, `LEXIS-SLICE-004-*` | How to keep glyph form, script lineage, sound value, and transliteration separate. |
| R-5 | Sound Change and Reconstruction | `LEXIS-SLICE-003-*`, `LEXIS-SLICE-004-*` | How reconstructed forms, uncertainty, sound shifts, and competing theories are labeled. |
| R-6 | Graph Method and RLINE Preservation | `LEXIS-WP-006`, `LEXIS-WP-007` | What graph labels and edge semantics must survive RLINE integration. |
| R-7 | Chronicle Communication and Overclaim Control | `LEXIS-WP-008` | How chronicles explain evidence, theory, uncertainty, and source limits without sounding settled when they are not. |
| R-8 | Slice Case Studies | `LEXIS-SLICE-001..005` | Which actual slice should run first, and what each slice teaches the model. |
| R-9 | Publisher and Context Artifacts | `LEXIS-WP-009` | What downstream CROP/PEBBLE/FLETCH/PROOF handoff would require after validated slices exist. |

## Initial Paper Backlog

| Paper ID | Working title | Track | Related packages | Requirement output |
|---|---|---|---|---|
| LEXIS-PAPER-001 | Source Custody for Language-History References | R-1 | `LEXIS-WP-003`, all `*-SOURCE` | Source-family categories, rights posture, citation/pointer rules, exclusion rules. |
| LEXIS-PAPER-002 | Claim Types in Etymology Graphs | R-2, R-3 | `LEXIS-WP-002`, `LEXIS-WP-005` | Final candidate claim types and negative-test classes. |
| LEXIS-PAPER-003 | Descent, Borrowing, Calque, and Coincidence Boundaries | R-3 | `*-EDGES`, `*-NEGATIVES` | Relationship taxonomy and edge validation rules. |
| LEXIS-PAPER-004 | Script Form Is Not Sound Value | R-4 | `LEXIS-SLICE-002-*`, `LEXIS-SLICE-004-*` | Script/transliteration model constraints and chronicle wording rules. |
| LEXIS-PAPER-005 | Uncertainty Labels for Reconstructed Roots | R-5 | `LEXIS-SLICE-003-*` | Reconstruction labels, disputed-theory posture, and overclaim tests. |
| LEXIS-PAPER-006 | Preserving Linguistic Labels in Lineage Graphs | R-6 | `LEXIS-WP-006`, `LEXIS-WP-007` | RLINE adoption criteria and graph-output invariants. |
| LEXIS-PAPER-007 | Chronicle Writing with Evidence and Theory Boundaries | R-7 | `LEXIS-WP-008` | Required chronicle sections, phrasing bans, and review checklist updates. |
| LEXIS-PAPER-008 | The `scribere` First-Slice Source Plan | R-8 | `LEXIS-SLICE-001-*` | Decision on whether Slice 1 is source-safe enough to execute first. |
| LEXIS-PAPER-009 | Greek Alphabet Lane Scenario Model | R-4, R-8 | `LEXIS-SLICE-002-*` | Script-lane scenario constraints and candidate graph shape. |
| LEXIS-PAPER-010 | Negative Relationship Claims as Product Value | R-3, R-7, R-8 | `LEXIS-SLICE-005-*` | Model and chronicle rules for "not related", unknown, and folk-etymology rejection. |
| LEXIS-PAPER-011 | Semitic Root-Pattern Slice Requirements | R-4, R-5, R-8 | `LEXIS-SLICE-004-*` | Morphology and transliteration gaps that may require new entities or edges. |
| LEXIS-PAPER-012 | Publisher Shape for Reviewed Language Slices | R-9 | `LEXIS-WP-009`, all `*-PACK` | Minimal artifact contract for later context packs without premature publication. |

## Module Delivery Order

| Order | Module | Papers | Why now |
|---:|---|---|---|
| 1 | Source Custody and Evidence | 001, 008 | Must decide whether the recommended `scribere` slice can be researched safely. |
| 2 | Ontology and Relationship Semantics | 002, 003, 010 | Must harden entities, edge kinds, and negative tests before fixtures. |
| 3 | Script and Reconstruction Caution | 004, 005, 009, 011 | Exercises non-word-only cases and higher-uncertainty slices. |
| 4 | Graph and Chronicle Method | 006, 007 | Converts reviewed slice data into graph and narrative requirements. |
| 5 | Publisher Context | 012 | Runs only after validation requirements and artifact boundaries are clearer. |

## Slice-To-Research Map

| Slice | Required research before execution | Optional follow-up |
|---|---|---|
| `LEXIS-SLICE-001` Latin `scribere` | Papers 001, 002, 003, 008 | Paper 007 before chronicle. |
| `LEXIS-SLICE-002` Greek alphabet lane | Papers 001, 004, 006, 009 | Paper 007 before public-facing chronicle. |
| `LEXIS-SLICE-003` PIE root mini-slice | Papers 001, 003, 005, 006 | Paper 010 for competing theories. |
| `LEXIS-SLICE-004` Semitic root-pattern mini-slice | Papers 001, 004, 011 | Paper 005 if reconstruction enters scope. |
| `LEXIS-SLICE-005` glyph/graph/write meta-slice | Papers 002, 003, 010 | Paper 007 for public explanation. |

## Requirements Feedback Rules

Research can change requirements only through explicit review:

- If a paper identifies a missing entity, edge kind, uncertainty label, or source
  posture, open a DCR before changing VTRACE files.
- If a paper shows that a slice is too broad, update
  `LANGUAGE_SLICE_PACKAGES.md` before any fixture work.
- If a paper changes graph semantics, update `CONTRACT_BOUNDARIES.md`,
  `TRACE.md`, and the RLINE dependency posture before implementation.
- If a paper changes chronicle wording, update `COMMUNICATIONS_STRATEGY.md` and
  `REVIEW_CHECKLISTS.md`.
- If a paper blocks a source family, record the block in `SOURCE_BASIS.md` and
  the relevant `*-SOURCE` package.

## Role-Review Checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: research must be tied to bounded language slices, not broad language evolution speculation. | Closed by slice-to-research map. |
| Etymology Reviewer | Major: relationship typology needs dedicated papers before fixtures. | Closed by Papers 002, 003, and 010. |
| Phonology Reviewer | Minor: sound change appears later than source and relationship foundations. | Accepted by delivery order; Paper 005 covers reconstruction before PIE execution. |
| Script Systems Reviewer | Major: script form, sound value, and transliteration need a separate track. | Closed by R-4 and Papers 004, 009, 011. |
| Source Custody Reviewer | Major: every paper touching real references needs custody posture. | Closed by Paper 001 and feedback rules. |
| Graph Systems Reviewer | Major: graph research must define preservation criteria before RLINE adoption. | Closed by Paper 006. |
| Product Chronicle Reviewer | Major: papers must drive chronicle requirements without publishing unreviewed claims. | Closed by Paper 007 and publication block. |
| Software Assurance Reviewer | Major: research outputs need traceable requirements impact. | Closed by requirement-output column and feedback rules. |

## Decision

RESEARCH_PLAN is accepted as planning input. It creates a research backlog that
drives future requirements, slice execution, graph contracts, and chronicle
rules without authorizing implementation or publication.
