# LEXIS Scenario Model

Status: settled.

## Scope

Scenario package model for future LEXIS validation of language-history specs,
source custody, graph preservation, and chronicle usefulness.

## Scenario Root

Preferred future root:

```text
scenarios/language-history/<package-or-slice>/
```

Each scenario package should contain:

```text
scenario.yaml
99-findings.md
00-workspace/
```

Fixtures may live under a repo-local fixture root when implementation begins,
but scenario findings must remain linked to requirements, specs, and work
packages.

## Required Scenario Shape

| Field | Required content |
|---|---|
| Scenario ID | Stable `LEXIS-SC-*` scenario ID. |
| Actor | Maintainer, language-history reviewer, writer, downstream repo, or agent workflow. |
| Purpose | What language-history question the scenario answers. |
| Specs exercised | Requirement/spec/contract IDs and spec file paths. |
| Boundary classes | Contract and package boundaries crossed. |
| Positive path | Expected successful workflow. |
| Negative paths | Missing, invalid, disputed, source-limited, stale, or adversarial cases. |
| Diagnostics | Expected stable IDs or allocation queue. |
| Evidence | Source pointers, validation output, graph slice, chronicle report, or review findings. |
| Fixture candidates | Files or commands that should become reusable tests. |
| Findings | Issues found in specs and how they were resolved or deferred. |

## First Scenario Candidates

| Scenario | Purpose | Blocker |
|---|---|---|
| `LEXIS-SC-001-word-root-slice` | Prove one word/root family with evidence, theory, alternatives, and chronicle output. | Source-custody decision and fixture work package. |
| `LEXIS-SC-002-borrowing-vs-descent` | Prove descent and borrowing edges stay distinct. | Fixture and negative test package. |
| `LEXIS-SC-003-source-limited-claim` | Prove source-limited claims do not overstate confidence. | Source-custody and chronicle checks. |
| `LEXIS-SC-004-rline-preservation` | Prove any RLINE-backed graph operation preserves LEXIS labels. | Graph package and RLINE adoption package. |

These scenarios should be instantiated through the slice package sets in
[`../../LANGUAGE_SLICE_PACKAGES.md`](../../LANGUAGE_SLICE_PACKAGES.md), starting
with `LEXIS-SLICE-001-*`.

## Findings Rule

Scenarios should usually find issues. A scenario with no findings needs an
explicit explanation of why the exercised spec surface was already sufficient.

Findings classify as:

- critical: blocks the spec from implementation input,
- major: must be fixed before L2 implementation,
- minor: can be fixed before public readiness,
- observation: useful but not blocking.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: scenarios must validate a bounded language-history question. | Closed by required scenario shape and `LEXIS-SC-001`. |
| Etymology Reviewer | Major: scenarios need negative cases for descent versus borrowing. | Closed by `LEXIS-SC-002` and negative paths field. |
| Phonology Reviewer | Minor: source-limited sound claims should be scenario candidates later. | Deferred to future scenario expansion. |
| Script Systems Reviewer | Minor: script lanes may need a first scenario once sources are chosen. | Deferred to future scenario expansion. |
| Source Custody Reviewer | Major: source-limited and custody cases need scenarios before public claims. | Closed by `LEXIS-SC-003` and evidence fields. |
| Graph Systems Reviewer | Major: RLINE preservation needs a scenario before adoption. | Closed by `LEXIS-SC-004`. |
| Product Chronicle Reviewer | Major: scenarios should feed chronicle usefulness, not just fixtures. | Closed by evidence and findings fields. |
| Software Assurance Reviewer | Major: scenarios should usually produce findings. | Closed by findings rule. |

## Decision

SCENARIO_MODEL is settled for the foundation wave. Scenario execution remains
blocked until future source-custody and fixture work packages begin.
