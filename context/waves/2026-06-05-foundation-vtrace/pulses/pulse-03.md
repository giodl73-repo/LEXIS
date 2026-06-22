# Pulse 03: CONOPS Fixed Point

## Goal

Define and settle `docs/vtrace/CONOPS.md` with the LEXIS role panel.

## Scope

- Define the operating concepts for the first LEXIS workflows.
- State actor roles, inputs, outputs, and review boundaries.
- Record role findings and dispositions.
- Mark CONOPS settled only if no critical or major actionable findings remain.

## Non-goals

- No requirements matrix.
- No object schema baseline.
- No architecture or RLINE crate selection.
- No fixtures, CLI, or source ingestion.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

CONOPS reached fixed point. The operating concept now defines scoped
language-history questions, source posture, evidence/theory separation, graph
review, chronicle review, and bounded publishing. Major findings on contact
boundaries, cognate/borrowing separation, source posture, script drift, RLINE API
deferral, and testable-requirements handoff were closed.

Validation passed with `git diff --check`.
