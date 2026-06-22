# Paper Plan: Publisher Shape for Reviewed Language Slices

Paper ID: `LEXIS-PAPER-012`

Status: planned, not written.

## Research Question

What minimal artifact shape and contract boundaries should LEXIS require before
a reviewed language-history slice can be handed to downstream context or
publisher systems?

## Scope

Included:

- Publisher handoff content in `publisher-handoff.md`.
- Source-custody summary requirements.
- Graph slice and chronicle preservation requirements.
- Public claim allowed/blocked posture.
- Future CROP, PEBBLE, FLETCH, PROOF, FONTES, and MUNDUS boundary posture.

Excluded:

- Choosing downstream integration APIs.
- Emitting any artifact or package.
- Publishing a chronicle.
- Creating registry entries.
- Accepting source text redistribution.

## Related Artifacts

| Artifact | Relationship |
|---|---|
| `docs/specs/publisher-handoff.md` | Defines draft handoff behavior and blockers. |
| `docs/vtrace/CONTRACT_BOUNDARIES.md` | Defines publisher and docs/corpus contract classes. |
| `docs/vtrace/INTERFACES.md` | Names future context pack and publisher registry fields. |
| `fixtures/planned/source-limited-claim` | Captures handoff blocker when source posture is pointer-only. |
| `LEXIS-WP-009` | Future publisher planning work package. |

## Expected Outputs

- Minimal reviewed-slice artifact shape.
- Required source, graph, chronicle, validation, and public-claim fields.
- Handoff blocker matrix.
- Dependency posture recommendations for CROP, PEBBLE, FLETCH, PROOF, FONTES,
  and MUNDUS.
- DCR recommendations if current publisher handoff spec is insufficient.

## Expected Negative Findings

- Pointer-only source posture may block artifact types that need source text.
- A reviewed graph without chronicle validation may not be publishable.
- A chronicle with unresolved overclaim findings must block handoff.
- Downstream systems must not inherit unsupported claims by packaging them.

## Review Roles

Required: L-1 through L-8.

Primary: L-5 Source Custody Reviewer, L-7 Product Chronicle Reviewer, and L-8
Software Assurance Reviewer.

## Promotion Block

This plan does not authorize artifact output, downstream integration, publisher
registry entries, source redistribution, or public claims.

