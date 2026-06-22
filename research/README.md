# LEXIS Research

Status: planning.

LEXIS research is organized as reviewed modules and papers. The controlling
backlog is [`../RESEARCH_PLAN.md`](../RESEARCH_PLAN.md).

## Expected Paper Layout

Future papers should use this shape:

```text
research/
  modules/
    <module-slug>/
      README.md
      REVIEW_PANEL.md
      publications/
        <paper-slug>/
          plan.md
          paper.md
          review.md
```

## Current State

No research paper is written yet. Planned module directories exist for:

- `modules/source-custody-and-evidence/` with `LEXIS-PAPER-001` and
  `LEXIS-PAPER-008`,
- `modules/ontology-and-relationship-semantics/` with `LEXIS-PAPER-002`,
  `LEXIS-PAPER-003`, and `LEXIS-PAPER-010`.
- `modules/script-and-reconstruction-caution/` with `LEXIS-PAPER-004`,
  `LEXIS-PAPER-005`, `LEXIS-PAPER-009`, and `LEXIS-PAPER-011`.
- `modules/graph-and-chronicle-method/` with `LEXIS-PAPER-006` and
  `LEXIS-PAPER-007`.
- `modules/publisher-context/` with `LEXIS-PAPER-012`.

## Research Gates

- Paper plans must name related work packages and language slices.
- Papers must separate evidence, theory, uncertainty, and source limits.
- Module reviews must decide whether findings require a VTRACE DCR.
- No paper may promote unreviewed source text, fixtures, graph outputs, or
  chronicle claims as finished LEXIS evidence.
