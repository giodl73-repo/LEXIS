# LEXIS CONOPS

Status: settled.

MISSION reached fixed point. CONOPS defines the operating concepts LEXIS should
support before requirements, schemas, architecture, or implementation begin.

## Operating concept

LEXIS operates as a staged language-history workbench:

1. Select a narrow language-history question.
2. Declare the allowed source family and rights posture.
3. Record attested forms, reconstructed claims, and uncertainty separately.
4. Build a typed lineage graph that distinguishes descent, borrowing, sound
   shift, meaning shift, and script/orthography transitions.
5. Review graph paths and chronicle text with the LEXIS role panel.
6. Publish only bounded artifacts whose evidence and theory boundaries are
   visible.

## Actors

| Actor | Need | Boundary |
|---|---|---|
| Maintainer | Define scoped waves, source posture, and validation gates. | Does not accept broad source ingestion before source-custody review. |
| Language-history reviewer | Inspect etymology, sound, script, meaning, and theory claims. | Does not treat AI-generated synthesis as external peer review. |
| Writer/researcher | Follow a readable chronicle from evidence to theory. | Does not receive unsourced dictionary-style summaries. |
| Downstream repo | Consume portable LEXIS artifacts for naming, lore, identity, or context. | Does not link to LEXIS internals during foundation. |
| Agent workflow | Request bounded, cited language-history context. | Does not expand context without source and rights gates. |

## Core workflows

### Trace a word or root

Input: scoped word, root, or form plus language/time boundaries.

Output: attested forms, possible roots, cognate sets, borrowing candidates,
meaning shifts, sources, uncertainty labels, and chronicle text.

Review emphasis: etymology, source custody, and evidence/theory separation.

### Compare cognates and borrowings

Input: a candidate cognate set or family slice.

Output: graph view that separates descent edges from borrowing/contact edges,
with uncertain and rejected alternatives visible.

Review emphasis: no collapse of cognacy, borrowing, calque, coincidence, and
unknown relationships.

### Inspect sound and script drift

Input: a proposed sound shift, script transition, transliteration lane, or
orthographic variation.

Output: path view over forms, scripts, dates, pronunciations when available,
and reconstruction limits.

Review emphasis: phonology and script-system caution before public chronicle
claims.

### Read a chronicle

Input: a reviewed graph slice.

Output: Lucia-style narrative that names direct evidence, theory claims,
uncertainty, and unresolved alternatives.

Review emphasis: readability without overclaiming.

### Package bounded context

Input: reviewed source/graph slice and downstream purpose.

Output: future CROP/PEBBLE/FLETCH-ready artifact shape, deferred until
publisher planning.

Review emphasis: rights posture, source pointers, and no redistribution of
unapproved source content.

## Operational boundaries

- CONOPS authorizes workflow design only, not implementation.
- Requirements must decide what is testable.
- Specification baseline must decide object and edge vocabularies.
- Architecture must decide where RLINE is used.
- Code rigor must decide validation, source-custody, and overclaim gates.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: workflows need era/contact boundaries before claims can be compared. | Closed by scoped question and language/time boundaries. |
| Etymology Reviewer | Major: cognate and borrowing workflows must remain separate. | Closed by separate workflow and relationship categories. |
| Phonology Reviewer | Minor: pronunciation data may be unavailable for many historic forms. | Deferred to REQUIREMENTS and SPECIFICATION_BASELINE uncertainty labels. |
| Script Systems Reviewer | Major: script drift must include transliteration and orthography boundaries. | Closed in sound/script workflow. |
| Source Custody Reviewer | Major: source family and rights posture must precede ingestion. | Closed in operating concept and package boundary. |
| Graph Systems Reviewer | Major: CONOPS should not commit to RLINE APIs before architecture. | Closed by deferring RLINE placement to ARCHITECTURE. |
| Product Chronicle Reviewer | Minor: chronicle output should start from reviewed graph slices. | Closed in chronicle workflow. |
| Software Assurance Reviewer | Major: workflows need explicit handoff to testable requirements. | Closed by operational boundaries. |

## Decision

CONOPS is settled for the foundation wave. No critical or major actionable role
finding remains. COMMUNICATIONS_STRATEGY is the next VTRACE stage.
