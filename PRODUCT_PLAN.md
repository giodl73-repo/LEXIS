# LEXIS Product Plan

## Vision

LEXIS is the Knowledge Systems repo for word, sound, script, meaning, and
language-family evolution.

It should let downstream systems ask how a word, form, sound, alphabet, or
meaning moved through time without flattening evidence into a single confident
story. LEXIS records lineages, alternatives, source attestations, contact,
borrowing, semantic drift, and theory boundaries.

## Waves

| Wave | Goal | Validation |
|---|---|---|
| Foundation VTRACE | Establish mission, stage order, role panel, and dependency posture before code. | `git diff --check` |
| Research baseline | Plan source-custody, ontology, relationship, script, graph, chronicle, and slice-study papers that drive requirements. | Research plan review plus VTRACE DCRs for accepted requirement changes. |
| Evidence baseline | Define first source-custody rules, object vocabulary, uncertainty labels, and one word-family scope. | VTRACE fixed-point review plus source-custody role review. |
| Graph foundation | Add first RLINE-backed graph model only after requirements and interfaces settle. | Format/tests to be defined in VTRACE work packages. |
| Chronicle output | Emit the first Lucia-style language chronicle with evidence/theory separation. | Role review, fixture checks, and report validation. |
| Publisher planning | Plan FLETCH, PROOF, CROP, and PEBBLE publishing once artifacts stabilize. | Dependency tracker updates and generated artifact sketch. |

## Initial object vocabulary

| Object | Purpose |
|---|---|
| `Language` | Named language, dialect, or reconstructed language node with evidence posture. |
| `Wordform` | Attested or reconstructed form in a language, script, and time/context. |
| `Root` | Reconstructed or cited root with confidence and competing theories. |
| `CognateSet` | Related forms by descent, distinct from borrowing/contact links. |
| `Borrowing` | Contact transfer edge with source/recipient, timing, and evidence. |
| `SoundShift` | Phonological transition or rule with scope and uncertainty. |
| `MeaningShift` | Semantic drift record from one meaning neighborhood to another. |
| `ScriptForm` | Written representation, alphabet/script lane, transliteration, or orthography. |
| `Attestation` | Source-backed occurrence with date, place, text pointer, and rights posture. |
| `TheoryClaim` | Explicit interpretive claim separated from direct evidence. |

Detailed entity sequencing and first-slice candidates live in
[`DOMAIN_BACKLOG.md`](DOMAIN_BACKLOG.md).

Research tracks and paper backlog live in [`RESEARCH_PLAN.md`](RESEARCH_PLAN.md).
The first planned module is
[`research/modules/source-custody-and-evidence/`](research/modules/source-custody-and-evidence/).
The second planned module is
[`research/modules/ontology-and-relationship-semantics/`](research/modules/ontology-and-relationship-semantics/).
The third planned module is
[`research/modules/script-and-reconstruction-caution/`](research/modules/script-and-reconstruction-caution/).
The fourth planned module is
[`research/modules/graph-and-chronicle-method/`](research/modules/graph-and-chronicle-method/).
The fifth planned module is
[`research/modules/publisher-context/`](research/modules/publisher-context/).

Draft-reviewed implementation-facing specs live in
[`docs/specs/`](docs/specs/). They translate the baseline into future source,
model, relationship, graph, diagnostic, fixture, chronicle, and publisher
behavior without authorizing code.

Planned scenario packages live in [`scenarios/`](scenarios/). They connect the
draft-reviewed specs to future validation paths without creating fixtures or
implementation evidence.

Planned fixture manifests live in [`fixtures/`](fixtures/). They map scenarios
to future proof inputs while keeping all source data, graph output, and
chronicles blocked.

Planned source-custody decision stubs live in
[`source-custody/`](source-custody/). They make source blockers explicit before
LEXIS chooses real references.

## Dependency posture

| Dependency | Status | Reason |
|---|---|---|
| RLINE | Intended runtime graph dependency after VTRACE baseline | Lineage paths, cognate/borrowing networks, script-transition graphs, connector paths, clusters, and drift neighborhoods. |
| ROLES | Required governance artifact | Repo-local role panel reviews every VTRACE stage. |
| VTRACE | Required method | Stage-gated mission-to-review process before implementation. |
| FLETCH | Planned source/tool layer | Future source registry, fetch/cache, and dataset discovery support. |
| PROOF | Planned CLI/tool layer | Future Markdown/report/diagram validation. |
| CROP/PEBBLE | Planned publisher/context layers | Future bounded research packs and portable evidence graph bundles. |
| CANON | Possible artifact alignment | Stable ids for languages, works, authors, regions, and cultural continuity. |

## Non-goals

- No broad linguistics encyclopedia in the foundation wave.
- No unsourced etymology claims.
- No dictionary/corpus redistribution before rights review.
- No product-to-product runtime dependency on sibling Knowledge Systems.
- No graph API expansion in RLINE until LEXIS proves concrete repeated needs.
