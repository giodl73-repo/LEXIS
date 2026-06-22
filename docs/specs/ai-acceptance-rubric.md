# AI Acceptance Rubric

Status: draft, advisory only.

## Authority Boundary

AI acceptance is a review recommendation. It cannot promote a fixture, accept a
source-custody record, override validation diagnostics, or authorize source
redistribution.

## Hard Gates

An AI reviewer must recommend `block_promotion` when any hard gate fails:

1. Source custody is not `accepted_for_slice`.
2. The fixture or graph artifact has validation errors.
3. The route still has unresolved correction blockers.
4. Homonym, compound, borrowing, or semantic-shift edges are collapsed or
   unlabeled.
5. Source text redistribution posture is unknown, blocked, or overclaimed.

## Advisory Score

The score is out of 100:

| Dimension | Points | Requirement |
|---|---:|---|
| Source custody | 30 | Evidence source is accepted for slice use and may support claims. |
| Fixture validation | 25 | Corrected fixture validates with zero diagnostics. |
| Route correction | 20 | Replacement bases, bridges, compounds, homonyms, and intermediate forms are modeled. |
| Relationship semantics | 15 | Edge kinds and uncertainty labels are explicit and not collapsed. |
| Rights and chronicle posture | 10 | Pointer-only or stronger rights posture is honored in graph and chronicle outputs. |

AI reviewers may also record a compact `0-5` score profile for review notes:
source custody strength, form-chain correctness, relationship edge quality,
semantic drift handling, homonym/compound/variant handling, validation
readiness, and promotion risk. The `0-5` profile is explanatory; the `0-100`
score and hard gates control the advisory recommendation.

## Recommendations

| Recommendation | Meaning |
|---|---|
| `accept_for_human_promotion_review` | AI sees no rubric blocker, but human/source-custody promotion is still required. |
| `revise_before_acceptance` | Evidence exists, but correction or modeling gaps remain. |
| `block_promotion` | At least one hard gate fails; graph must remain preview-only. |

## Current Tier 3 Posture

The corrected Tier 3 graph artifacts are source-backed candidates, not accepted
chains. They should receive `block_promotion` until candidate source-custody
records become accepted and generated fixtures validate without diagnostics.
