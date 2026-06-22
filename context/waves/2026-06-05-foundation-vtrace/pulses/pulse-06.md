# Pulse 06: Architecture Fixed Point

## Goal

Define and settle `docs/vtrace/ARCHITECTURE.md`, especially the boundary between
LEXIS linguistic semantics and RLINE graph mechanics.

## Scope

- Architecture layers.
- RLINE dependency boundary.
- Source-custody and publisher boundaries.
- Review gates for implementation.

## Non-goals

- No RLINE crate selection.
- No Rust workspace.
- No interfaces, fixtures, CLI, source ingestion, or graph implementation.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

ARCHITECTURE reached fixed point. LEXIS owns language-history semantics,
evidence policy, source-custody decisions, chronicle content, and uncertainty.
RLINE is planned only for graph mechanics after INTERFACES settle. Publisher and
source systems remain support layers and cannot bypass LEXIS custody decisions.

Validation passed with `git diff --check`.
