# LEXIS Problem Space Map

Status: settled.

This map controls the large language-history world before LEXIS turns any
research into fixtures, graphs, chronicles, or public claims. It is the parent
map for `LANGUAGE_SLICE_PACKAGES.md`: slices are traversal paths through this
world, not isolated tasks.

## Map Rule

Every future language-history discovery must land in a region below or create a
reviewed new region. Work packages and slice packages must name the region they
touch and any adjacent region they affect.

## World Regions

| Region | Responsibility | Primary artifacts | Boundary pressure |
|---|---|---|---|
| LEXIS-RGN-001 | Source custody and attestations | `SOURCE_BASIS.md`, `RESEARCH_PLAN.md`, `*-SOURCE` packages | Source rights, citation limits, source confidence, no source-text redistribution. |
| LEXIS-RGN-002 | Languages, periods, and varieties | `DOMAIN_BACKLOG.md`, future model specs | Names, periods, geography, dialect scope, reconstructed-language caution. |
| LEXIS-RGN-003 | Scripts, orthography, and transliteration | `LANGUAGE_SLICE_PACKAGES.md`, `RESEARCH_PLAN.md` | Glyph form must not collapse into sound value or language identity. |
| LEXIS-RGN-004 | Wordforms, roots, and meaning senses | `SPECIFICATION_BASELINE.md`, `DOMAIN_BACKLOG.md` | Attested forms, reconstructed roots, and meanings need separate claim posture. |
| LEXIS-RGN-005 | Relationship and theory claims | `SPEC_MODEL.md`, `WORK_PACKAGES.md` | Descent, borrowing, cognate, calque, coincidence, disputed, unknown, and rejected claims must not collapse. |
| LEXIS-RGN-006 | Sound change and reconstruction | `RESEARCH_PLAN.md`, future phonology fixtures | Uncertainty and competing theories must remain visible. |
| LEXIS-RGN-007 | Graph slices and RLINE boundary | `CONTRACT_BOUNDARIES.md`, `PACKAGE_BOUNDARIES.md` | Graph mechanics must preserve LEXIS edge kinds, labels, and source posture. |
| LEXIS-RGN-008 | Chronicles and publisher artifacts | `COMMUNICATIONS_STRATEGY.md`, `REVIEW_CHECKLISTS.md` | Public narrative must separate evidence, theory, uncertainty, and source limits. |

## Traversal Order

Default traversal for LEXIS:

1. source families and rights posture,
2. bounded language-history question,
3. language/script/period identifiers,
4. wordforms, roots, and meaning senses,
5. relationship/theory claims and negative alternatives,
6. optional sound/reconstruction lane,
7. graph slice and RLINE preservation check,
8. chronicle wording and overclaim review,
9. publisher/context artifact planning.

## Slice Map

| Slice | Regions crossed | Traversal purpose |
|---|---|---|
| `LEXIS-SLICE-001` Latin `scribere` | RGN-001, RGN-002, RGN-004, RGN-005, RGN-007, RGN-008 | First source-safe word-family path from source pointers to graph and chronicle. |
| `LEXIS-SLICE-002` Greek alphabet lane | RGN-001, RGN-002, RGN-003, RGN-007, RGN-008 | Script-form traversal without flattening glyph form into sound value. |
| `LEXIS-SLICE-003` PIE root mini-slice | RGN-001, RGN-002, RGN-004, RGN-005, RGN-006, RGN-007, RGN-008 | Reconstruction and uncertainty traversal. |
| `LEXIS-SLICE-004` Semitic root pattern | RGN-001, RGN-002, RGN-003, RGN-004, RGN-005, RGN-006 | Non-Indo-European morphology and transliteration traversal. |
| `LEXIS-SLICE-005` glyph/graph/write meta-slice | RGN-001, RGN-003, RGN-004, RGN-005, RGN-008 | Negative relationship and public-explanation traversal. |

## Cross-Region Risks

| Risk | Regions | Required control |
|---|---|---|
| Source pointer becomes implied linguistic proof. | RGN-001, RGN-005 | Source custody, theory-claim labels, review checklist. |
| Visual similarity becomes false script or word lineage. | RGN-003, RGN-005 | Script Systems review and negative claims. |
| Reconstruction is written as fact. | RGN-004, RGN-006, RGN-008 | Uncertainty labels and chronicle wording review. |
| RLINE graph output drops linguistic labels. | RGN-007 | Contract-boundary preservation tests. |
| Chronicle leads the evidence instead of following it. | RGN-008 | Graph and validation gates before chronicle output. |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: world map must prevent universal language-history scope. | Closed by bounded regions and slice traversal. |
| Etymology Reviewer | Major: relationship claims need their own region and negative alternatives. | Closed by RGN-005 and slice requirements. |
| Phonology Reviewer | Minor: sound change should be explicit but not forced into every slice. | Closed by RGN-006 and optional traversal. |
| Script Systems Reviewer | Major: script form needs first-class region status. | Closed by RGN-003. |
| Source Custody Reviewer | Major: all traversal starts with source posture. | Closed by traversal order. |
| Graph Systems Reviewer | Major: graph region must preserve LEXIS labels before RLINE adoption. | Closed by RGN-007. |
| Product Chronicle Reviewer | Major: chronicle region must remain downstream of evidence and graph review. | Closed by traversal order. |
| Software Assurance Reviewer | Major: slices must map to regions. | Closed by slice map. |

## Decision

PROBLEM_SPACE_MAP is settled for the foundation wave. It accepts
`LANGUAGE_SLICE_PACKAGES.md` as LEXIS's repo-specific slice traversal plan.
