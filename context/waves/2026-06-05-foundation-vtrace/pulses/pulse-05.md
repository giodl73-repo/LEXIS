# Pulse 05: Specification Baseline Fixed Point

## Goal

Define and settle `docs/vtrace/SPECIFICATION_BASELINE.md` so LEXIS has a stable
foundation vocabulary before architecture or implementation planning.

## Scope

- Core object vocabulary.
- Claim types.
- Relationship and edge kinds.
- Uncertainty labels.
- Source and rights posture fields.
- Explicit non-goals for the baseline.

## Non-goals

- No Rust schema.
- No RLINE API choice.
- No fixtures, CLI, source ingestion, or graph implementation.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

SPECIFICATION_BASELINE reached fixed point. The file now defines the foundation
object vocabulary, claim types, edge kinds, uncertainty labels, source/rights
posture, and baseline non-goals without committing to Rust schemas, fixtures,
source ingestion, or RLINE APIs.

Validation passed with `git diff --check`.
