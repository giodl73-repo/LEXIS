# LEXIS Trace

Status: settled.

VALIDATION reached fixed point. Trace maps the foundation requirements to
settled VTRACE stages and future work packages.

## Requirements trace

| Requirement area | Mission/CONOPS | Baseline/design | Verification/validation | Work packages |
|---|---|---|---|---|
| Scoped language-history question | MISSION, CONOPS, COMMUNICATIONS_STRATEGY | SPECIFICATION_BASELINE, DESIGN | VERIFICATION, VALIDATION | WP-004, WP-005 |
| Claim posture | MISSION, COMMUNICATIONS_STRATEGY, REQUIREMENTS | SPECIFICATION_BASELINE, SPEC_MODEL | VERIFICATION, VALIDATION | WP-002, WP-005 |
| Source custody | MISSION, CONOPS, COMMUNICATIONS_STRATEGY, REQUIREMENTS | CODE_RIGOR | VERIFICATION, VALIDATION | WP-003, WP-004 |
| Relationship separation | COMMUNICATIONS_STRATEGY, REQUIREMENTS | SPECIFICATION_BASELINE, DESIGN | VERIFICATION, VALIDATION | WP-002, WP-005, WP-006 |
| RLINE boundary | REQUIREMENTS | ARCHITECTURE, DESIGN, PACKAGE_BOUNDARIES, CONTRACT_BOUNDARIES, CODE_RIGOR | VERIFICATION | WP-006, WP-007 |
| Chronicle output | MISSION, CONOPS, COMMUNICATIONS_STRATEGY, REQUIREMENTS | DESIGN | VALIDATION | WP-008 |
| Publisher planning | INTERFACES | ARCHITECTURE | TRACE, REVIEW | WP-009 |
| Scenario readiness | CONOPS, REQUIREMENTS | SCENARIO_MODEL, SPEC_MODEL | VERIFICATION, VALIDATION | WP-004, WP-006, WP-008 |
| Source basis | COMMUNICATIONS_STRATEGY, REQUIREMENTS | SOURCE_BASIS, CODE_RIGOR | VERIFICATION, VALIDATION | WP-003, WP-004 |
| Evidence ledger | REQUIREMENTS | EVIDENCE, TRACE | VERIFICATION, REVIEW | all future packages |
| Review lanes | MISSION, ROLE_RECOMMENDATIONS | REVIEW_CHECKLISTS, REVIEW | VALIDATION, REVIEW | all future packages |
| Language profiles | CODE_RIGOR | LANGUAGE_PROFILES, PACKAGE_BOUNDARIES | VERIFICATION | future docs/Rust/fixture/report packages |
| Change control | CHANGE_CONTROL | WORK_PACKAGES, PULSE_EXECUTION | TRACE, REVIEW | future DCRs |
| Domain entity backlog | MISSION, CONOPS, REQUIREMENTS | SPECIFICATION_BASELINE, DOMAIN_BACKLOG | VERIFICATION, VALIDATION | WP-002..WP-009 |
| Language-slice package sets | CONOPS, DOMAIN_BACKLOG | LANGUAGE_SLICE_PACKAGES, WORK_PACKAGES, SCENARIO_MODEL | VERIFICATION, VALIDATION | LEXIS-SLICE-001..005 |
| Research modules and papers | COMMUNICATIONS_STRATEGY, REQUIREMENTS | RESEARCH_PLAN, SOURCE_BASIS, CHANGE_CONTROL | REVIEW | all future packages through DCRs |
| Problem-space map | CONOPS, REQUIREMENTS | PROBLEM_SPACE_MAP, DOMAIN_BACKLOG, LANGUAGE_SLICE_PACKAGES | VERIFICATION, VALIDATION | all slice packages |
| Diagnostic model | REQUIREMENTS, CODE_RIGOR | DIAGNOSTIC_MODEL, REVIEW_CHECKLISTS | VERIFICATION | WP-005 and future CLI packages |
| Fixture model | SCENARIO_MODEL, VERIFICATION | FIXTURE_MODEL, LANGUAGE_SLICE_PACKAGES | VALIDATION | WP-004..WP-008 |
| Implementation-facing specs | REQUIREMENTS, SPECIFICATION_BASELINE, SPEC_MODEL | docs/specs package | VERIFICATION, VALIDATION | WP-002..WP-009 |
| Planned scenarios | CONOPS, REQUIREMENTS | SCENARIO_MODEL, docs/specs package, scenarios package | VALIDATION | WP-003..WP-009 |
| Planned fixture manifests | SCENARIO_MODEL, FIXTURE_MODEL | fixtures package, docs/specs package | VERIFICATION, VALIDATION | WP-004..WP-008 |
| Planned source-custody decisions | REQUIREMENTS, SOURCE_BASIS | source-custody package, docs/specs package, fixtures package | VERIFICATION, VALIDATION | WP-003, WP-004 |
| Source-record contract | REQUIREMENTS, SOURCE_BASIS | source-record-contract spec, source-custody package, fixture manifests | VERIFICATION, VALIDATION | WP-003, WP-004, WP-005 |
| Planned research modules | RESEARCH_PLAN, SOURCE_BASIS | research module plans, source-custody package, docs/specs package | REVIEW | WP-002, WP-003, WP-005, WP-006, LEXIS-SLICE-001..004 |

## Review trace

All settled stages record L-1 through L-8 role findings. Major findings are
closed in the stage files; minor findings are closed or deferred to named later
stages or work packages.

## Evidence status

This foundation wave has planning evidence only. Implementation evidence,
fixtures, test output, source-custody decisions, graph outputs, and chronicles
remain future work.

## Decision

TRACE is settled for the foundation wave. No critical or major actionable role
finding remains. REVIEW is the next VTRACE stage.
