# LEXIS Architecture

Status: settled.

SPECIFICATION_BASELINE reached fixed point. This architecture defines ownership
boundaries and dependency posture only. It does not select crates, create Rust
packages, or define CLI/API interfaces.

## Architecture principle

LEXIS owns language-history semantics. RLINE may supply reusable graph
mechanics. Source and publisher systems may support custody and artifact flow.
No dependency may own LEXIS claim meaning.

## Layers

| Layer | Owner | Responsibility | Not responsible for |
|---|---|---|---|
| Language-history model | LEXIS | Languages, wordforms, roots, cognate sets, borrowings, sound shifts, meaning shifts, script forms, attestations, theory claims, uncertainty labels. | Generic graph algorithms or source fetching. |
| Evidence and custody | LEXIS, with future FONTES/MUNDUS/FLETCH support | Source posture, rights posture, attestation records, no-redistribution gates. | Linguistic interpretation by source tooling. |
| Graph mechanics | RLINE after interface review | Traversal, connector paths, clusters, neighborhoods, graph storage/helpers, and other product-neutral graph operations. | Deciding whether an edge means descent, borrowing, calque, sound shift, or theory support. |
| Chronicle/report output | LEXIS, with future PROOF support | Evidence/theory/uncertainty narrative, report sections, public wording discipline. | Markdown validation semantics or generated report tooling beyond LEXIS content. |
| Context/artifact publishing | Future CROP/PEBBLE/FLETCH | Bounded context packs, portable bundles, registry/cache distribution. | Expanding source scope or changing claim semantics. |

## RLINE boundary

RLINE is planned as a graph-mechanics dependency only after INTERFACES settle.

Allowed future RLINE uses:

- graph storage or typed graph helpers if they fit LEXIS records,
- traversal over reviewed LEXIS edge kinds,
- connector paths between wordforms, sources, roots, scripts, or meanings,
- cluster/neighborhood summaries over reviewed graph slices,
- path or boundary metrics that preserve edge-kind labels.

Blocked RLINE uses:

- deciding linguistic relationship kinds,
- assigning etymology confidence,
- selecting a winning theory,
- interpreting source quality,
- rewriting chronicle language,
- absorbing LEXIS-specific schema into a product-neutral crate before repeated
  cross-repo need is proven.

## Source and publisher boundaries

- FLETCH may later support source discovery, registry validation, and cache
  distribution.
- FONTES and MUNDUS may later support source-custody and known-asset pointers.
- PROOF may later validate reports and diagrams.
- CROP and PEBBLE may later package bounded language-history context artifacts.
- None of these systems may ingest or publish LEXIS source content without a
  LEXIS source-custody decision.

## Implementation gates

Implementation remains blocked until:

1. INTERFACES names the first CLI/artifact/fixture contracts,
2. DESIGN names the first reviewed views,
3. CODE_RIGOR names validation and source-custody gates,
4. IMPLEMENTATION_PLAN selects one narrow first slice,
5. WORK_PACKAGES splits implementation into testable tasks.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: architecture must keep theory interpretation in LEXIS. | Closed by language-history model and blocked RLINE uses. |
| Etymology Reviewer | Major: graph traversal must not imply relationship certainty. | Closed by preserving edge-kind labels and uncertainty. |
| Phonology Reviewer | Minor: sound data should not require a phonetic engine. | Closed by architecture non-goal and LEXIS-owned model. |
| Script Systems Reviewer | Minor: script forms need representation but not a graphics/glyph engine. | Closed by language-history model boundary. |
| Source Custody Reviewer | Major: publisher tools must not bypass source-custody decisions. | Closed by source and publisher boundaries. |
| Graph Systems Reviewer | Major: RLINE allowed and blocked uses must be explicit. | Closed by RLINE boundary. |
| Product Chronicle Reviewer | Minor: chronicle output should remain LEXIS-owned even if PROOF validates reports. | Closed by chronicle/report output layer. |
| Software Assurance Reviewer | Major: architecture must keep implementation blocked until later VTRACE stages. | Closed by implementation gates. |

## Decision

ARCHITECTURE is settled for the foundation wave. No critical or major
actionable role finding remains. INTERFACES is the next VTRACE stage.
