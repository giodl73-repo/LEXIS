# LEXIS Mission

## Mission

LEXIS traces the evolution of words, sounds, scripts, meanings, and language
families through evidence-backed graphs and readable chronicles.

## Mission boundaries

LEXIS should make language evolution inspectable without pretending that every
etymology, reconstruction, borrowing, or semantic path is settled. The repo
separates direct evidence from theory, marks uncertainty, and records source
custody before publishing claims.

LEXIS treats evidence, theory, and uncertainty as first-class mission outputs:
an attested form, a reconstructed root, a proposed borrowing path, and a
narrative chronicle are different claim types and must remain distinguishable.

## Primary users

- Portfolio maintainers designing Knowledge Systems and world-history evidence.
- Researchers or writers inspecting word, script, sound, or meaning evolution.
- Downstream systems that need portable language-history artifacts without
  owning linguistic source policy.
- Agent workflows that need bounded, cited context about language lineage.

## Mission success

LEXIS succeeds when a user can choose a scoped word, root, script feature, or
language-family slice and inspect:

- attested forms and sources,
- possible roots and cognate sets,
- descent versus borrowing edges,
- sound and meaning shifts,
- script or orthography transitions,
- competing theory claims,
- uncertainty labels and rejected alternatives,
- readable chronicle output with evidence boundaries.

The first success target is not breadth. The first success target is one narrow
language-history slice that can be inspected, reviewed, and validated without
overstating what the sources prove.

## Mission non-goals

- No general dictionary, thesaurus, translation, or language-learning product.
- No unsourced etymology summaries.
- No source redistribution without rights review.
- No broad implementation before the VTRACE baseline settles.
- No implementation packages before MISSION, CONOPS,
  COMMUNICATIONS_STRATEGY, REQUIREMENTS, and SPECIFICATION_BASELINE reach
  role-review fixed points.
- No transfer of linguistic semantics into RLINE.

## Role-review checkpoint

| Role | Initial finding | Decision |
|---|---|---|
| Language Historian | Major: scope must separate evidence from migration/contact theory. | Closed by first-class evidence/theory/uncertainty claim types. |
| Etymology Reviewer | Major: cognate and borrowing links must not be collapsed. | Closed in mission success and deferred edge detail to SPECIFICATION_BASELINE. |
| Phonology Reviewer | Minor: sound shifts need explicit uncertainty and reconstruction limits later. | Deferred to REQUIREMENTS and SPECIFICATION_BASELINE. |
| Script Systems Reviewer | Minor: written forms should be first-class but not dominate spoken language. | Closed by balanced mission wording. |
| Source Custody Reviewer | Major: rights posture must exist before source ingestion. | Closed as source-custody boundary; detailed rules deferred to CODE_RIGOR. |
| Graph Systems Reviewer | Major: RLINE must stay graph-mechanical, not linguistic. | Closed in mission non-goals. |
| Product Chronicle Reviewer | Minor: chronicle output is core to the value proposition. | Closed in mission success. |
| Software Assurance Reviewer | Major: validation must begin as docs-only and implementation must stay gated. | Closed by explicit early-stage fixed-point gate. |

## Decision

MISSION is settled for the foundation wave. No critical or major actionable role
finding remains. CONOPS is the next VTRACE stage.
