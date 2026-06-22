# LEXIS Implementation Plan

Status: settled.

CODE_RIGOR reached fixed point. This plan selects a narrow first implementation
slice, but does not authorize code until WORK_PACKAGES, VERIFICATION, and
VALIDATION define execution and checks.

## First-slice selection

The first slice should be a small, source-custody-safe language-history graph:

- one word/root family,
- two to five wordforms,
- one language-family boundary,
- one sound or script lane,
- one competing or rejected alternative where available,
- source pointers rather than redistributed source text,
- one chronicle slice.

The exact word/root family remains deferred to WORK_PACKAGES so source-custody
review can happen before fixtures are created.

## Implementation sequence

| Step | Planned outcome | Gate |
|---|---|---|
| 1 | Create Rust workspace skeleton and docs validation contract. | WORK_PACKAGES and VERIFICATION. |
| 2 | Add data model for baseline vocabulary without external source ingestion. | SPECIFICATION_BASELINE and CODE_RIGOR. |
| 3 | Add one hand-authored fixture with source pointers only. | Source-custody review. |
| 4 | Add validation CLI for scope, claims, sources, uncertainty, and edge kinds. | VERIFICATION. |
| 5 | Add graph-slice output preserving LEXIS labels. | RLINE preservation review. |
| 6 | Add chronicle report output. | VALIDATION and product chronicle review. |

## RLINE adoption timing

RLINE should be adopted only after the first local validation contract proves
what fields must be preserved. The first implementation may start with local
data structures if that keeps the validation gate clearer. RLINE adoption should
then be a separate work package with explicit preservation checks.

## Validation expectations

Future implementation should introduce validation in this order:

1. format/check command for docs and code,
2. fixture validation,
3. negative fixture validation,
4. graph-slice preservation validation,
5. chronicle overclaim validation.

## Blocks before code

Code remains blocked until WORK_PACKAGES defines:

- exact crates or files,
- exact first fixture scope,
- validation commands,
- source-custody decision,
- RLINE adoption decision or deferral,
- negative tests,
- role-review closure.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: first slice must avoid broad language-family claims. | Closed by narrow first-slice selection. |
| Etymology Reviewer | Major: first fixture needs a competing or rejected alternative when available. | Closed by first-slice preference and work-package deferral. |
| Phonology Reviewer | Minor: first slice may use sound or script, not necessarily both. | Closed by first-slice selection. |
| Script Systems Reviewer | Minor: script lane can be the first visual proof if source-safe. | Deferred to WORK_PACKAGES source choice. |
| Source Custody Reviewer | Major: exact source family must be reviewed before fixtures. | Closed by source-custody block before code. |
| Graph Systems Reviewer | Major: RLINE adoption should be separate from first model validation if needed. | Closed by RLINE adoption timing. |
| Product Chronicle Reviewer | Major: chronicle output must not precede reviewed graph slice. | Closed by implementation sequence. |
| Software Assurance Reviewer | Major: code must wait for work packages and validation commands. | Closed by blocks before code. |

## Decision

IMPLEMENTATION_PLAN is settled for the foundation wave. No critical or major
actionable role finding remains. WORK_PACKAGES is the next VTRACE stage.
