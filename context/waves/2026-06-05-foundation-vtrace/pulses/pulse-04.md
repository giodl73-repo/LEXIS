# Pulse 04: Requirements Baseline

## Goal

Define and settle `docs/vtrace/REQUIREMENTS.md` with testable requirements for
the LEXIS foundation.

## Scope

- Functional requirements.
- Evidence and source-custody requirements.
- Linguistic uncertainty and claim-type requirements.
- Graph and RLINE-boundary requirements.
- Chronicle/output requirements.
- Verification handoff expectations.

## Non-goals

- No object schema baseline.
- No architecture or RLINE API choice.
- No crates, fixtures, CLI, or source ingestion.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

REQUIREMENTS reached fixed point. The file now defines functional, evidence,
source-custody, graph/dependency, output, and foundation-gate requirements with
verification expectations. Major findings on evidence posture, relationship
kinds, unavailable pronunciation/script data, source rights, RLINE gating, and
implementation preconditions were closed.

Validation passed with `git diff --check`.
