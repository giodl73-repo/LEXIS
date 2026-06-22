# Pulse 07: Interfaces Fixed Point

## Goal

Define and settle `docs/vtrace/INTERFACES.md` without creating implementation
code or selecting concrete RLINE crates.

## Scope

- Future CLI command surface.
- Future fixture contract.
- Future source-record contract.
- Future graph/report/artifact contract names.
- Interface review gates.

## Non-goals

- No Rust workspace.
- No CLI implementation.
- No fixture files.
- No source ingestion.
- No RLINE crate selection.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

INTERFACES reached fixed point. It names future CLI, fixture, source-record,
graph, report, and artifact contracts while keeping them as planning anchors.
Implementation, fixture creation, source ingestion, and RLINE crate selection
remain gated to later VTRACE stages.

Validation passed with `git diff --check`.
