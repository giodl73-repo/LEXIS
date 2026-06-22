# LEXIS Review Checklists

Status: settled.

| Checklist | Required | Decision | Evidence / Rationale |
|---|---|---|---|
| Mission Review | yes | pass | `MISSION.md` bounds LEXIS and blocks implementation claims. |
| CONOPS Review | yes | pass | `CONOPS.md` defines scoped workflows. |
| Communications Review | yes | pass | `COMMUNICATIONS_STRATEGY.md` controls public wording and status language. |
| Specification Review | yes | pass | `REQUIREMENTS.md`, `SPECIFICATION_BASELINE.md`, `PROBLEM_SPACE_MAP.md`, `DOMAIN_BACKLOG.md`, `RESEARCH_PLAN.md`, and `SPEC_MODEL.md` are settled. |
| Draft Specs Review | yes | pass_with_risk | `docs/specs/` is draft-reviewed by L-1..L-8, but no spec is promoted. |
| Scenario Planning Review | yes | pass_with_risk | `scenarios/` names planned validation paths, but no scenario has executed. |
| Fixture Planning Review | yes | pass_with_risk | `fixtures/` names planned fixture manifests, but no fixture is executable or promoted. |
| Source-Custody Planning Review | yes | pass_with_risk | `source-custody/` names planned source-family decision stubs, but no source is accepted. |
| Research Planning Review | yes | pass_with_risk | First research module and paper plans exist, but no paper has been written or reviewed. |
| Design Review | yes | pass_with_risk | `ARCHITECTURE.md`, `INTERFACES.md`, `DESIGN.md`, `PACKAGE_BOUNDARIES.md`, `CONTRACT_BOUNDARIES.md`, `SCENARIO_MODEL.md`, `DIAGNOSTIC_MODEL.md`, and `FIXTURE_MODEL.md` are docs-only controls. |
| Source Custody Review | yes | pass_with_risk | Source ingestion is blocked; source basis is pointer-only. |
| Graph/RLINE Review | yes | pass_with_risk | RLINE is planned for graph mechanics only and remains unadopted. |
| Test Readiness Review | yes | pass_with_risk | Verification/validation are planned; executable tests do not exist yet. |
| Release Readiness Review | yes | blocked | No code, fixtures, source-custody decisions, graph output, or chronicle output exist. |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Source Custody Reviewer | Major: source review must be required even before source ingestion. | Closed by checklist row. |
| Graph Systems Reviewer | Major: RLINE review must be required before adoption. | Closed by checklist row. |
| Software Assurance Reviewer | Major: release readiness must remain blocked. | Closed by checklist row. |

## Decision

REVIEW_CHECKLISTS is settled for the foundation wave.
