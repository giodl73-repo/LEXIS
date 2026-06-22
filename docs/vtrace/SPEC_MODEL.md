# LEXIS Spec Model

Status: settled.

## Scope

LEXIS uses this spec model to decide when a language-history behavior is deep
enough to become implementation input.

## Authority Boundary

| Surface | Authority |
|---|---|
| Product specs | `docs/vtrace/SPECIFICATION_BASELINE.md` and draft/future `docs/specs/` files |
| VTRACE planning and trace | `docs/vtrace/` |
| Scenarios and fixtures | future `scenarios/` and `fixtures/` roots |
| Reference inputs | reviewed source pointers only; no source text redistribution by default |
| Implementation evidence | future `docs/vtrace/VERIFICATION.md`, `docs/vtrace/VALIDATION.md`, and work-package records |

Reference inputs do not define target behavior until the behavior is restated in
a LEXIS-owned spec, reviewed, and traced.

## Spec Classes

| Class | Purpose | Required home | Required evidence |
|---|---|---|---|
| Language-history spec | Defines durable LEXIS behavior for forms, roots, claims, edges, views, or chronicles. | `docs/specs/` or `docs/vtrace/SPECIFICATION_BASELINE.md` during foundation | role review, scenario or fixture reference, work-package row |
| Source-custody spec | Defines allowed use of a source family, source pointer, or rights posture. | future source-custody docs | Source Custody Reviewer decision and negative cases |
| Graph contract spec | Defines graph output, edge preservation, or RLINE adoption behavior. | `CONTRACT_BOUNDARIES.md` or future graph spec | graph preservation checks |
| Chronicle/report spec | Defines public report sections and claim wording controls. | future report spec | Product Chronicle and Software Assurance review |
| Scenario spec | Describes a workflow, adversarial path, and expected proof points. | future `scenarios/` root | scenario file, findings file, linked specs |

## Required Spec Shape

Every LEXIS product spec used for implementation must define or explicitly mark
out of scope:

1. status and authority boundary,
2. owning spec area,
3. reference inputs and what was not adopted,
4. target-owned behavior statement,
5. users or actors,
6. data model, schemas, profiles, or descriptors,
7. operations and lifecycle states,
8. invariants,
9. positive examples,
10. negative examples and error paths,
11. diagnostics and allocation status,
12. source-custody impact,
13. graph/edge preservation impact,
14. fixture and scenario impact,
15. proof/evidence impact,
16. compatibility and migration behavior,
17. implementation ownership or explicit deferral,
18. docs/corpus impact,
19. trace and work-package IDs.

## Promotion Rule

A draft LEXIS spec may become implementation input only when:

1. it is in the correct spec home,
2. required fields are present or explicitly out of scope,
3. contract-boundary impact is recorded,
4. source-custody impact is recorded,
5. graph preservation impact is recorded,
6. at least one scenario or fixture path is named,
7. trace, verification, validation, and package ledgers are updated,
8. role review closes critical and major findings.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: source references must not become target behavior by implication. | Closed by authority boundary and reference-input rule. |
| Etymology Reviewer | Major: language-history specs need negative examples for bad etymology relationships. | Closed by required spec shape and promotion rule. |
| Phonology Reviewer | Minor: sound behavior may be out of scope for some specs. | Closed by explicit out-of-scope allowance. |
| Script Systems Reviewer | Minor: script behavior may need separate spec area later. | Deferred to future language-history spec home. |
| Source Custody Reviewer | Major: source-custody impact must be required before implementation input. | Closed by required spec shape and promotion rule. |
| Graph Systems Reviewer | Major: graph/RLINE behavior needs spec evidence before adoption. | Closed by graph contract spec and graph preservation impact fields. |
| Product Chronicle Reviewer | Minor: chronicle/report specs need wording controls. | Closed by chronicle/report spec class. |
| Software Assurance Reviewer | Major: draft specs must not authorize L2 implementation. | Closed by promotion rule. |

## Decision

SPEC_MODEL is settled for the foundation wave. Future implementation specs
remain blocked until scenarios, fixtures, and verification commands exist.
