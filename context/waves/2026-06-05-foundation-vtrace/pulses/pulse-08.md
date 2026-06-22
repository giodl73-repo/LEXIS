# Pulse 08: Design Fixed Point

## Goal

Define and settle `docs/vtrace/DESIGN.md` for LEXIS graph views, lineage views,
drift views, source views, and chronicle output.

## Scope

- View designs.
- Review expectations.
- RLINE design boundary.
- First-slice design preference.

## Non-goals

- No UI implementation.
- No graph library selection.
- No fixtures.
- No CLI.
- No source ingestion.

## Validation

```powershell
git diff --check
```

## Status

complete

## Result

DESIGN reached fixed point. LEXIS now has design-level lineage,
borrowing/contact, sound-shift, meaning-drift, script/orthography,
attestation/source, and chronicle views. The RLINE design boundary requires all
future graph operations to preserve LEXIS edge kinds, claim types, uncertainty,
source custody, and alternatives.

Validation passed with `git diff --check`.
