# LEXIS Requirements

Status: settled.

MISSION and CONOPS reached fixed point. Requirements define what the foundation
must later verify before implementation can claim success.

## Requirement format

Each requirement has:

- an id,
- a requirement statement,
- a verification expectation,
- the primary review role.

## Functional requirements

| ID | Requirement | Verification expectation | Role |
|---|---|---|---|
| LEXIS-FR-001 | LEXIS shall support a scoped language-history question over a bounded word, root, script feature, sound shift, or language-family slice. | A future fixture names scope boundaries before records are accepted. | Language Historian |
| LEXIS-FR-002 | LEXIS shall distinguish attested forms from reconstructed forms and theory claims. | Future validation rejects records that omit claim type. | Etymology Reviewer |
| LEXIS-FR-003 | LEXIS shall distinguish descent/cognacy, borrowing/contact, calque, coincidence, rejected, and unknown relationship claims. | Future validation rejects ambiguous relationship kinds. | Etymology Reviewer |
| LEXIS-FR-004 | LEXIS shall support sound-shift and script/orthography transition records without requiring both to exist for every slice. | Future validation permits absent pronunciation or script lanes when explicitly marked unavailable. | Phonology Reviewer |
| LEXIS-FR-005 | LEXIS shall produce a readable chronicle from a reviewed graph slice while preserving evidence, theory, and uncertainty markers. | Future report checks verify required sections and marker presence. | Product Chronicle Reviewer |

## Evidence and source-custody requirements

| ID | Requirement | Verification expectation | Role |
|---|---|---|---|
| LEXIS-EV-001 | Every attestation shall carry a source pointer, date or date range posture, language/script posture, and rights posture. | Future validation rejects unattached attestations. | Source Custody Reviewer |
| LEXIS-EV-002 | LEXIS shall not ingest or redistribute dictionary, corpus, inscription, or source text without an accepted source-custody decision. | Future source intake requires a reviewed custody record before fixture promotion. | Source Custody Reviewer |
| LEXIS-EV-003 | LEXIS shall mark whether a claim is direct evidence, reconstruction, inference, competing theory, or rejected alternative. | Future validation rejects claims without evidence posture. | Language Historian |
| LEXIS-EV-004 | LEXIS shall keep unresolved alternatives visible instead of collapsing them into one confident lineage. | Future chronicle checks require alternative or uncertainty fields when confidence is not settled. | Etymology Reviewer |

## Graph and dependency requirements

| ID | Requirement | Verification expectation | Role |
|---|---|---|---|
| LEXIS-GR-001 | LEXIS shall keep linguistic relationship semantics local to LEXIS. | Architecture review confirms RLINE usage is graph-mechanical only. | Graph Systems Reviewer |
| LEXIS-GR-002 | LEXIS may use RLINE graph packages for traversal, connector paths, clusters, neighborhoods, and graph storage after architecture review. | No RLINE crate dependency is added before ARCHITECTURE and INTERFACES settle. | Graph Systems Reviewer |
| LEXIS-GR-003 | Graph path output shall identify edge kinds so descent, borrowing, sound shift, meaning shift, and script transition are not visually or semantically collapsed. | Future fixture/report validation checks edge kind coverage. | Graph Systems Reviewer |
| LEXIS-GR-004 | Confidence scoring, linguistic ranking, and historical interpretation shall remain LEXIS-owned unless a later extraction decision is reviewed. | Dependency tracker remains candidate/planned until repeated needs are proven. | Software Assurance Reviewer |

## Output requirements

| ID | Requirement | Verification expectation | Role |
|---|---|---|---|
| LEXIS-OUT-001 | Chronicle output shall include a concise evidence path, theory path, uncertainty note, and source-custody note. | Future report validation checks required sections. | Product Chronicle Reviewer |
| LEXIS-OUT-002 | Future context artifacts shall be bounded by task, source posture, and downstream purpose. | Future CROP/PEBBLE/FLETCH planning rejects unbounded packs. | Source Custody Reviewer |
| LEXIS-OUT-003 | Public-facing wording shall avoid claiming proof when the underlying record is reconstructed, inferred, disputed, or source-limited. | Future review checklist flags overclaim language. | Software Assurance Reviewer |

## Foundation gates

Implementation may not begin until these are settled:

1. MISSION,
2. CONOPS,
3. COMMUNICATIONS_STRATEGY,
4. REQUIREMENTS,
5. SPECIFICATION_BASELINE.

RLINE dependency selection may not begin until ARCHITECTURE and INTERFACES
settle.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: requirements must require evidence posture and not only graph structure. | Closed by LEXIS-EV-003 and output requirements. |
| Etymology Reviewer | Major: relationship kinds need explicit separation beyond cognate/borrowing. | Closed by LEXIS-FR-003. |
| Phonology Reviewer | Major: unavailable pronunciation or script data must not fail valid slices. | Closed by LEXIS-FR-004. |
| Script Systems Reviewer | Minor: script and orthography are both needed in the language surface. | Closed by LEXIS-FR-004 and LEXIS-GR-003. |
| Source Custody Reviewer | Major: source rights and intake gates must precede fixture promotion. | Closed by LEXIS-EV-001 and LEXIS-EV-002. |
| Graph Systems Reviewer | Major: RLINE dependency must remain blocked until architecture/interface review. | Closed by LEXIS-GR-001 and LEXIS-GR-002. |
| Product Chronicle Reviewer | Minor: chronicle required sections should be testable. | Closed by LEXIS-OUT-001. |
| Software Assurance Reviewer | Major: implementation and dependency gates need explicit preconditions. | Closed by foundation gates. |

## Decision

REQUIREMENTS is settled for the foundation wave. No critical or major
actionable role finding remains. SPECIFICATION_BASELINE is the next VTRACE
stage.
