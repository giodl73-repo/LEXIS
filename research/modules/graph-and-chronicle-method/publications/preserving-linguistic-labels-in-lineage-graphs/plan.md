# Paper Plan: Preserving Linguistic Labels in Lineage Graphs

Paper ID: `LEXIS-PAPER-006`

Status: planned, not written.

## Research Question

What LEXIS-owned labels, source posture, uncertainty, and relationship
distinctions must survive local graph output and any later RLINE-backed graph
operation?

## Scope

Included:

- Graph slice content and preservation invariants.
- RLINE adoption criteria and deferral conditions.
- Graph-preservation fixture plan.
- Script-lane and reconstruction graph risks.

Excluded:

- Choosing RLINE APIs or crates.
- Emitting graph output.
- Creating graph fixtures.
- Changing graph contracts without DCR review.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `docs/specs/graph-slice.md` | Defines graph slice content and preservation invariants. |
| `docs/vtrace/CONTRACT_BOUNDARIES.md` | Defines graph contract boundary class. |
| `fixtures/planned/rline-preservation` | Planned graph-preservation fixture. |
| `scenarios/language-history/rline-preservation` | Planned RLINE preservation scenario. |
| `dependency-systems/rline-usage.md` | TRACKER dependency posture for RLINE. |
| `LEXIS-WP-006`, `LEXIS-WP-007` | Future graph and RLINE work packages. |

## Expected Outputs

- Required graph-label preservation matrix.
- Local graph baseline criteria.
- RLINE adoption pass/fail criteria.
- Negative cases for dropped edge kind, claim type, source posture, uncertainty,
  and rejected alternatives.
- DCR recommendations if graph contracts need refinement.

## Expected Negative Findings

- Topology-only output is insufficient.
- RLINE-backed output cannot rename LEXIS semantics without a mapping.
- Source posture and uncertainty must survive graph output.

## Review Roles

Required: L-1 through L-8.

Primary: L-6 Graph Systems Reviewer and L-8 Software Assurance Reviewer.

## Promotion Block

This plan does not authorize graph output, RLINE adoption, graph fixtures, or
graph API commitments.

