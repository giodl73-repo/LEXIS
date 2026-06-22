# LEXIS VTRACE Process

LEXIS uses VTRACE before implementation. Each stage advances one deliverable,
passes repo-local role review, records trace implications, and then unlocks the
next stage.

## Stage ledger

| Stage | File | Status | Roles | Decision | Next |
|---|---|---|---|---|---|
| MISSION | `docs/vtrace/MISSION.md` | settled | L-1..L-8 | Scope language-history evidence graph; fixed point reached. | CONOPS |
| CONOPS | `docs/vtrace/CONOPS.md` | settled | L-1..L-8 | Define user workflows and operating concepts; fixed point reached. | COMMUNICATIONS_STRATEGY |
| COMMUNICATIONS_STRATEGY | `docs/vtrace/COMMUNICATIONS_STRATEGY.md` | settled | L-1..L-8 | Define audience lanes, wording rules, chronicle voice, and status communication; fixed point reached. | REQUIREMENTS |
| REQUIREMENTS | `docs/vtrace/REQUIREMENTS.md` | settled | L-1..L-8 | Make functional/evidence/source/graph requirements testable; fixed point reached. | SPECIFICATION_BASELINE |
| SPECIFICATION_BASELINE | `docs/vtrace/SPECIFICATION_BASELINE.md` | settled | L-1..L-8 | Stabilize object vocabulary and claim boundaries; fixed point reached. | PROBLEM_SPACE_MAP |
| PROBLEM_SPACE_MAP | `docs/vtrace/PROBLEM_SPACE_MAP.md` | settled | L-1..L-8 | Map language-history world regions and slice traversal; fixed point reached. | DOMAIN_BACKLOG |
| DOMAIN_BACKLOG | `DOMAIN_BACKLOG.md` | settled | L-1..L-8 | Sequence entities and first-slice candidates; fixed point reached. | RESEARCH_PLAN |
| RESEARCH_PLAN | `docs/vtrace/RESEARCH_PLAN.md`, `RESEARCH_PLAN.md` | settled | L-1..L-8 | Plan research tracks and paper backlog as requirements input; fixed point reached. | SPEC_MODEL |
| SPEC_MODEL | `docs/vtrace/SPEC_MODEL.md` | settled | L-1..L-8 | Define deep spec shape and promotion rules; fixed point reached. | ARCHITECTURE |
| ARCHITECTURE | `docs/vtrace/ARCHITECTURE.md` | settled | L-1..L-8 | Place RLINE and future publisher layers correctly; fixed point reached. | INTERFACES |
| INTERFACES | `docs/vtrace/INTERFACES.md` | settled | L-1..L-8 | Name CLI/API/artifact contracts without premature code; fixed point reached. | DESIGN |
| DESIGN | `docs/vtrace/DESIGN.md` | settled | L-1..L-8 | Define graph views, lineage views, drift views, and chronicles; fixed point reached. | PACKAGE_BOUNDARIES |
| PACKAGE_BOUNDARIES | `docs/vtrace/PACKAGE_BOUNDARIES.md` | settled | L-1..L-8 | Define package/dependency ownership boundaries; fixed point reached. | CONTRACT_BOUNDARIES |
| CONTRACT_BOUNDARIES | `docs/vtrace/CONTRACT_BOUNDARIES.md` | settled | L-1..L-8 | Define durable contract classes and closeout requirements; fixed point reached. | SCENARIO_MODEL |
| SCENARIO_MODEL | `docs/vtrace/SCENARIO_MODEL.md` | settled | L-1..L-8 | Define future scenario package shape and findings rules; fixed point reached. | DIAGNOSTIC_MODEL |
| DIAGNOSTIC_MODEL | `docs/vtrace/DIAGNOSTIC_MODEL.md` | settled | L-1..L-8 | Define future diagnostic families and allocation rules; fixed point reached. | FIXTURE_MODEL |
| FIXTURE_MODEL | `docs/vtrace/FIXTURE_MODEL.md` | settled | L-1..L-8 | Define future fixture classes and promotion rules; fixed point reached. | CODE_RIGOR |
| CODE_RIGOR | `docs/vtrace/CODE_RIGOR.md` | settled | L-1..L-8 | Define fixtures, validation, source custody, and overclaim gates; fixed point reached. | IMPLEMENTATION_PLAN |
| IMPLEMENTATION_PLAN | `docs/vtrace/IMPLEMENTATION_PLAN.md` | settled | L-1..L-8 | Select the first narrow implementation slice; fixed point reached. | WORK_PACKAGES |
| WORK_PACKAGES | `docs/vtrace/WORK_PACKAGES.md` | settled | L-1..L-8 | Split schema, fixtures, graph, CLI, reports, and tests; fixed point reached. | VERIFICATION |
| VERIFICATION | `docs/vtrace/VERIFICATION.md` | settled | L-1..L-8 | Prove implementation matches requirements; fixed point reached. | VALIDATION |
| VALIDATION | `docs/vtrace/VALIDATION.md` | settled | L-1..L-8 | Prove outputs are historically useful and not overstated; fixed point reached. | TRACE |
| TRACE | `docs/vtrace/TRACE.md` | settled | L-1..L-8 | Map requirements to design, work, tests, evidence, and reviews; fixed point reached. | REVIEW |
| REVIEW | `docs/vtrace/REVIEW.md` | settled | L-1..L-8 | Decide release readiness and remaining gaps; fixed point reached. | complete |
| STAGE_EXECUTION | `docs/vtrace/STAGE_EXECUTION.md` | settled | L-1..L-8 | Summarize S0-S6 readiness and blocked execution stages; fixed point reached. | complete |

## Support Artifacts

| Artifact | Status | Purpose |
|---|---|---|
| `README.md` | settled | Local VTRACE source-of-truth map. |
| `EVIDENCE.md` | settled | Docs-only objective evidence ledger. |
| `CHANGE_CONTROL.md` | settled | DCRs and change-control triggers. |
| `PROBLEM_SPACE_MAP.md` | settled | Language-history world regions and slice traversal. |
| `RESEARCH_PLAN.md` | settled | Research tracks as requirements input. |
| `DIAGNOSTIC_MODEL.md` | settled | Future validation finding families. |
| `FIXTURE_MODEL.md` | settled | Future controlled fixture classes. |
| `REVIEW_CHECKLISTS.md` | settled | Required checklist rows and readiness decisions. |
| `ROLE_RECOMMENDATIONS.md` | settled | VTRACE lanes mapped to LEXIS roles. |
| `LANGUAGE_PROFILES.md` | settled | Docs/current and Rust/fixture/report planned validation profiles. |
| `SOURCE_BASIS.md` | settled | Pointer-only source posture. |
| `PULSE_EXECUTION.md` | settled | VTRACE package discipline mapped to LEXIS pulses. |

## Review mechanics

```text
Draft one stage file.
Run the LEXIS role panel.
Classify findings as critical, major, minor, or deferred.
Revise the same stage file only.
Repeat until no critical or major actionable finding remains.
Record deferred items with a named later stage or work package.
Move to the next stage.
```

## Stage settlement criteria

- The deliverable has one clear scope.
- No critical or major actionable role finding remains.
- Deferred feedback names a later VTRACE stage or work package.
- Trace implications are recorded.
- Validation expectations are listed.
- The wave ledger records the decision and next stage.
