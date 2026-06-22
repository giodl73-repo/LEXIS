# LEXIS Interfaces

Status: settled.

ARCHITECTURE reached fixed point. Interfaces name future contracts only; they do
not create code, schemas, fixture files, source ingestion, or RLINE crate
dependencies.

## Interface principle

LEXIS interfaces should make scope, claim posture, source posture, and graph
edge semantics visible at every boundary. No interface should accept or emit a
language-history claim without enough metadata for review.

## Future CLI surface

The first command is now minimally implemented for the blocked source-pointer
fixture. Other command names remain planning anchors, not implementation
commitments.

| Command | Purpose | Gate |
|---|---|---|
| `lexis validate <fixture>` | Validate a LEXIS fixture or artifact for required scope, claim, source, uncertainty, and graph fields. | Minimal source-custody implementation exists for `LEXIS-FIX-001`; broader validation remains gated. |
| `lexis trace word <id>` | Inspect word/root lineage, attestations, cognates, borrowings, shifts, and alternatives. | Requires DESIGN and first fixture. |
| `lexis graph slice <id>` | Emit a bounded graph slice with edge-kind labels preserved. | Requires RLINE boundary review in DESIGN. |
| `lexis chronicle <slice>` | Emit a readable evidence/theory/uncertainty chronicle. | Requires DESIGN and VALIDATION. |
| `lexis source review <source>` | Review source rights and custody posture before ingestion or promotion. | Requires CODE_RIGOR. |

## Future fixture contract

A future fixture must carry these top-level sections:

| Section | Purpose |
|---|---|
| `scope` | Bounded language-history question, language/time/script boundaries, and non-goals. |
| `sources` | Source pointers, citation notes, rights posture, redistribution posture, and reviewer state. |
| `claims` | Direct evidence, reconstructions, inferences, competing theories, rejected alternatives, and unknowns. |
| `nodes` | Languages, wordforms, roots, cognate sets, attestations, meaning senses, script forms, sound features, and chronicle slices. |
| `edges` | LEXIS-owned relationship kinds with uncertainty and evidence links. |
| `review` | Role-review findings, decision state, and deferred work. |

## Future source-record contract

Every future source record must expose:

- source pointer,
- citation note,
- source family,
- date/date-range posture,
- language/script posture,
- rights posture,
- redistribution posture,
- review decision,
- allowed use in fixtures, chronicles, and artifacts.

## Future graph interface

Graph output must preserve LEXIS edge kinds and claim posture. A graph consumer
may traverse, cluster, summarize, or package the graph, but must not drop:

- edge kind,
- claim type,
- uncertainty label,
- source links,
- rejected or competing alternatives,
- source-custody status.

## Future report and artifact interfaces

| Interface | Required visible fields |
|---|---|
| Chronicle report | Scope, evidence path, theory path, uncertainty note, source-custody note, alternatives. |
| Graph slice artifact | Nodes, edges, edge kinds, claim posture, source posture, review state. |
| Context pack | Task boundary, included records, excluded records, source posture, downstream purpose. |
| Publisher registry | Artifact id, version/posture, source pointers, rights posture, validation status. |

## Interface gates

- CLI implementation waits for CODE_RIGOR and WORK_PACKAGES.
- RLINE crate selection waits for DESIGN and WORK_PACKAGES.
- Source ingestion waits for CODE_RIGOR.
- Publisher artifacts wait for VALIDATION and TRACE.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: interfaces must preserve scope and theory posture. | Closed by interface principle, fixture sections, and report fields. |
| Etymology Reviewer | Major: graph interfaces must preserve rejected and competing alternatives. | Closed by graph interface requirements. |
| Phonology Reviewer | Minor: sound-feature gaps must remain representable in fixtures. | Deferred to DESIGN and first fixture work package. |
| Script Systems Reviewer | Minor: script posture should remain visible in sources and fixtures. | Closed by fixture and source-record contracts. |
| Source Custody Reviewer | Major: source review must be an explicit interface before ingestion. | Closed by future source-record contract and `lexis source review`. |
| Graph Systems Reviewer | Major: graph consumers must not erase LEXIS edge semantics. | Closed by graph interface requirements. |
| Product Chronicle Reviewer | Minor: chronicle output needs stable visible sections. | Closed by report interface fields. |
| Software Assurance Reviewer | Major: command names must not imply implementation is approved. | Closed by planning-anchor wording and interface gates. |

## Decision

INTERFACES is settled for the foundation wave. No critical or major actionable
role finding remains. DESIGN is the next VTRACE stage.
