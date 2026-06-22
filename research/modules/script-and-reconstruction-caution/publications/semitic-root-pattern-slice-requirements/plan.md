# Paper Plan: Semitic Root-Pattern Slice Requirements

Paper ID: `LEXIS-PAPER-011`

Status: planned, not written.

## Research Question

What model, source-custody, transliteration, morphology, and relationship gaps
must LEXIS resolve before planning a Semitic root-pattern mini-slice?

## Scope

Included:

- `LEXIS-SLICE-004-*` package sequence.
- Root-pattern morphology pressure on the current model.
- Script/transliteration caution for Hebrew, Arabic, and Aramaic examples.
- Whether a morphology edge or entity may be needed.

Excluded:

- Selecting real Semitic roots.
- Selecting real sources.
- Creating transliteration fixtures.
- Making broad claims about Semitic grammar.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `LANGUAGE_SLICE_PACKAGES.md` | Defines `LEXIS-SLICE-004-*`. |
| `docs/vtrace/PROBLEM_SPACE_MAP.md` | Maps Semitic root-pattern traversal regions. |
| `docs/specs/language-history-model.md` | May need morphology pressure review. |
| `docs/specs/relationship-edges.md` | May need edge-kind review. |
| `source-custody/planned/general-language-history-reference.yaml` | Planned source-family blocker. |

## Expected Outputs

- Requirement gap list for root-pattern morphology.
- Transliteration and script posture requirements.
- Recommendation on whether current entities and edges are enough.
- DCR recommendations if new morphology model pieces are needed.

## Expected Negative Findings

- Root-pattern examples cannot be treated like simple wordform descent paths.
- Transliteration cannot replace original-script posture.
- Broad Semitic grammar is out of scope for one slice.

## Review Roles

Required: L-1 through L-8.

Primary: L-2 Etymology Reviewer, L-3 Phonology Reviewer, and L-4 Script Systems
Reviewer.

## Promotion Block

This plan does not authorize Semitic source selection, root-pattern fixtures,
new model entities, graph output, or chronicle output.

