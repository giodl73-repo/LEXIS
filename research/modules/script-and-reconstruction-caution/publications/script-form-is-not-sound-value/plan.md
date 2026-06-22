# Paper Plan: Script Form Is Not Sound Value

Paper ID: `LEXIS-PAPER-004`

Status: planned, not written.

## Research Question

How should LEXIS keep script form, glyph shape, orthography, transliteration,
language identity, and sound value separate in records, graph slices, and
chronicles?

## Scope

Included:

- `ScriptForm` behavior in `language-history-model.md`.
- `script_variant_of` boundaries in `relationship-edges.md`.
- Greek alphabet lane and Semitic root-pattern implications.
- Source-family blockers for script-history references.

Excluded:

- Selecting real script-history sources.
- Creating glyph diagrams or visual assets.
- Emitting graph output.
- Changing edge kinds without a DCR.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `source-custody/planned/script-history-reference.yaml` | Planned source-family blocker. |
| `LANGUAGE_SLICE_PACKAGES.md` | Defines `LEXIS-SLICE-002` and `LEXIS-SLICE-004`. |
| `docs/specs/language-history-model.md` | Defines `ScriptForm`. |
| `docs/specs/relationship-edges.md` | Defines `script_variant_of`. |
| `docs/specs/chronicle-output.md` | Controls public wording. |

## Expected Outputs

- Script/sound/transliteration boundary matrix.
- Recommendation on whether current `ScriptForm` is enough.
- Negative cases for visual similarity and sound-value confusion.
- Chronicle wording rules for script lanes.

## Expected Negative Findings

- Visual similarity does not prove lineage.
- Script transition does not prove sound transition.
- Transliteration is not the same as original script form.

## Review Roles

Required: L-1 through L-8.

Primary: L-4 Script Systems Reviewer and L-3 Phonology Reviewer.

## Promotion Block

This plan does not authorize script fixtures, script sources, graph output, or
chronicle output.

