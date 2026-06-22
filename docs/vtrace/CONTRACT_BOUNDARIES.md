# LEXIS Contract Boundaries

Status: settled.

## Scope

Durable contract boundaries for future LEXIS commands, fixtures, source records,
graph slices, chronicle reports, RLINE adoption, and publisher artifacts.

## Boundary Rule

A work package must update this file or explicitly mark the boundary out of
scope when it creates, changes, removes, or claims any durable interface.

## Boundary Classes

| Boundary class | Trigger | Owning spec home | Required controls |
|---|---|---|---|
| Command contract | Future public `lexis` command, flag, output, or transcript. | future CLI spec / `INTERFACES.md` | command status, diagnostics, docs impact, scenario |
| Data/model contract | Language-history records, graph nodes/edges, claims, uncertainty, or custody fields. | `SPECIFICATION_BASELINE.md` / future spec | schema/profile, positive and negative fixtures |
| Graph contract | Graph slice output or RLINE-backed operation. | future graph spec / `ARCHITECTURE.md` | label preservation checks, source-custody preservation |
| Source-custody contract | Source family, source pointer, rights, or redistribution posture. | future source-custody spec | custody review, redaction/redistribution rule |
| Chronicle/report contract | Public narrative, report section, or wording claim. | future chronicle spec / `COMMUNICATIONS_STRATEGY.md` | overclaim review, required sections |
| Diagnostic contract | Stable validation error or warning behavior. | future validation spec | stable ID allocation or queued allocation, negative fixture |
| Scenario contract | Scenario package becomes reusable proof point or fixture seed. | `SCENARIO_MODEL.md` / future scenario root | scenario findings, spec links |
| Publisher contract | CROP, PEBBLE, FLETCH, PROOF, FONTES, or MUNDUS handoff. | future publisher spec | artifact boundary, rights posture, validation evidence |
| Docs/corpus contract | User-facing docs or corpus rows make a behavioral claim. | `COMMUNICATIONS_STRATEGY.md` | evidence pointer, claim boundary, corpus status |

## Package Closeout Requirements

Every future package must report:

1. affected boundary classes,
2. contract specs added or changed,
3. source-custody impact,
4. graph/RLINE preservation impact,
5. diagnostic allocation impact,
6. scenario packages added or updated,
7. fixture candidates and negative cases,
8. docs/corpus impact,
9. public claim allowed on close,
10. public claim still blocked.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: durable contracts must not hide source or theory limits. | Closed by data/model, source-custody, chronicle, and docs/corpus contract classes. |
| Etymology Reviewer | Major: command/report contracts need relationship-collapse controls. | Closed by data/model and diagnostic contract classes. |
| Phonology Reviewer | Minor: diagnostics should later cover inferred pronunciation claims. | Deferred to future diagnostic allocation. |
| Script Systems Reviewer | Minor: script/orthography reports need contract controls when public. | Closed by chronicle/report and data/model classes. |
| Source Custody Reviewer | Major: custody/rights contract must be first-class. | Closed by source-custody and publisher contract classes. |
| Graph Systems Reviewer | Major: RLINE-backed graph output needs an explicit graph contract. | Closed by graph contract class and closeout requirements. |
| Product Chronicle Reviewer | Major: report wording needs durable boundary control. | Closed by chronicle/report contract class. |
| Software Assurance Reviewer | Major: package closeout must report public claims allowed and blocked. | Closed by closeout requirements. |

## Decision

CONTRACT_BOUNDARIES is settled for the foundation wave. No durable public
contract may be claimed until its boundary class is reviewed.
