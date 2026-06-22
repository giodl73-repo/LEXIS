# Fixture Promotion Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec defines how future research or scenario artifacts may become
controlled fixtures. It does not create fixtures.

## Target Behavior

LEXIS fixtures are proof inputs, not examples or research notes. A fixture may
promote only after source custody, scope, expected result, diagnostics, and
review posture are explicit.

## Fixture Classes

Use the classes in `docs/vtrace/FIXTURE_MODEL.md`:

- source-pointer fixture,
- golden slice fixture,
- negative relationship fixture,
- script-lane fixture,
- reconstruction fixture,
- graph-preservation fixture,
- chronicle fixture.

## Required Promotion Fields

Every promoted fixture must define:

1. fixture id,
2. owning work package,
3. linked research paper or scenario,
4. linked source-custody decision,
5. linked scope,
6. fixture class,
7. expected valid or invalid result,
8. expected diagnostic family for invalid cases,
9. records and edges included,
10. source text redistribution posture,
11. graph/chronicle impact,
12. review state.

## Promotion Blockers

- No accepted source-custody posture.
- Candidate source pointers used as evidence rather than blocked test input.
- No bounded scope.
- No expected result.
- No negative case for relationship-heavy fixtures.
- No diagnostic allocation.
- Source text included without redistribution permission.

## First Fixture Target

The first fixture shape is `LEXIS-FIX-001-source-pointer-scribere`, a blocked
source-pointer fixture that exercises candidate source records without language
claims. The first golden slice remains later and requires Papers 001, 002, 003,
and 008 in `RESEARCH_PLAN.md` to settle source and relationship posture.

## Trace

Requirements: `LEXIS-FR-001`, `LEXIS-EV-001`, `LEXIS-EV-002`,
`LEXIS-EV-004`.

Work packages: `LEXIS-WP-004`, `LEXIS-WP-005`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: first fixture must not imply broad Latin or Indo-European coverage. | Closed by first-fixture target and bounded scope blocker. |
| L-2 Etymology Reviewer | Major: relationship-heavy fixtures need negative cases. | Closed by promotion blockers. |
| L-3 Phonology Reviewer | Minor: first fixture can omit sound data if marked unavailable. | Deferred to fixture content review. |
| L-4 Script Systems Reviewer | Minor: script-lane fixtures need separate class. | Closed by fixture classes. |
| L-5 Source Custody Reviewer | Major: fixture promotion must require accepted source posture. | Closed by required fields and blockers. |
| L-6 Graph Systems Reviewer | Minor: graph fixture class should be separate from source fixtures. | Closed by fixture classes. |
| L-7 Product Chronicle Reviewer | Minor: chronicle fixtures should follow graph fixtures. | Closed by graph/chronicle impact field and class order. |
| L-8 Software Assurance Reviewer | Major: fixtures need expected valid/invalid result and diagnostics. | Closed by required promotion fields. |

Decision: draft-reviewed. No critical or major actionable finding remains; no
fixture exists yet.
