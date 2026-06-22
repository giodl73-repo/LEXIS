# LEXIS Change Control

Status: settled.

## Design Change Requests

| DCR ID | Change | Reason | Parent IDs | Status | Work Package |
|---|---|---|---|---|---|
| DCR-001 | Create LEXIS VTRACE foundation package. | Establish governance before code. | MISSION / CONOPS / REQUIREMENTS | accepted | foundation wave |
| DCR-002 | Add communications strategy to VTRACE order. | Updated VTRACE requires source-to-docs claim control. | COMMUNICATIONS_STRATEGY | accepted | pulse-03a |
| DCR-003 | Add deep spec and boundary controls. | Updated VTRACE adds spec model, contract boundaries, scenario model, package boundaries, and stage execution. | SPEC_MODEL / CONTRACT_BOUNDARIES / SCENARIO_MODEL / PACKAGE_BOUNDARIES / STAGE_EXECUTION | accepted | foundation alignment |
| DCR-004 | Plan first implementation slice without executing it. | Future code needs work packages, verification, validation, and source custody. | IMPLEMENTATION_PLAN / WORK_PACKAGES | proposed | future implementation wave |

## Change-Control Triggers

Update this file before changing:

- VTRACE stage order,
- public command or artifact claims,
- source-custody posture,
- RLINE dependency posture,
- future fixture or scenario scope,
- work-package status,
- public readiness claim.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: change requests must preserve claim caution. | Closed by trigger list. |
| Source Custody Reviewer | Major: source posture changes need DCR visibility. | Closed by trigger list. |
| Graph Systems Reviewer | Major: RLINE posture changes need DCR visibility. | Closed by trigger list. |
| Software Assurance Reviewer | Major: work-package status changes need change control. | Closed by trigger list. |

## Decision

CHANGE_CONTROL is settled for the foundation wave.

