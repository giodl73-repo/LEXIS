# Paper Plan: Source Custody for Language-History References

Paper ID: `LEXIS-PAPER-001`

Status: planned, not written.

## Research Question

What source-family categories, rights postures, citation rules, and
redistribution limits should LEXIS require before any language-history source
supports a fixture, graph slice, chronicle, or publisher artifact?

## Scope

Included:

- Latin lexicographic or dictionary reference families.
- English etymology reference families.
- General scholarly language-history reference families.
- Script-history reference families.
- Source-limited or restricted placeholders for negative validation.

Excluded:

- Selecting specific real sources.
- Quoting source text.
- Building fixture records.
- Deciding the actual `scribere` source set.
- Defining final schema or Rust types.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `docs/specs/source-custody.md` | Defines draft-reviewed source-custody behavior. |
| `source-custody/` | Holds planned source-family decision stubs. |
| `fixtures/` | Holds fixture manifests blocked by source decisions. |
| `LEXIS-WP-003` | Future source-custody stub work package. |
| `LEXIS-SLICE-001-SOURCE` | First slice source review package. |

## Source Families To Investigate

This plan names families only. The paper may later identify candidate pointers,
but no source is accepted in the plan.

| Family | Current planned decision |
|---|---|
| Latin lexicographic reference | `LEXIS-SRCDEC-001` |
| English etymology reference | `LEXIS-SRCDEC-002` |
| General scholarly language-history reference | `LEXIS-SRCDEC-003` |
| Script-history reference | `LEXIS-SRCDEC-004` |
| Source-limited placeholder | `LEXIS-SRCDEC-005` |

## Expected Outputs

- Source-family category taxonomy.
- Rights and redistribution posture recommendations.
- Pointer-only default rules.
- Blocked and unknown posture rules.
- Required source-custody decision fields.
- DCR recommendations if the current source-custody spec is insufficient.

## Expected Negative Findings

- Public web availability does not imply redistribution permission.
- A dictionary source pointer does not prove an etymology theory.
- Unknown rights posture blocks fixture promotion.
- A source-limited placeholder can test overclaim blocking without naming a real
  restricted source.

## Review Roles

Required: L-1 through L-8.

Primary: L-5 Source Custody Reviewer and L-8 Software Assurance Reviewer.

## Promotion Block

This plan does not authorize source selection, source ingestion, fixture data,
graph output, chronicle output, or publication.

