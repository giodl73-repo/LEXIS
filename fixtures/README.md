# LEXIS Fixtures

Status: planned and draft fixture shapes only.

This directory contains fixture plans and draft fixture shapes. Candidate-only
language data is stored here for validation and preview graph generation, but no
source text, promoted graph output, validation transcript, or published
chronicle output is stored here.

## Fixture Plan Index

| Fixture plan | Class | Scenario | Status |
|---|---|---|---|
| `planned/source-pointer-scribere` | source-pointer fixture | `LEXIS-SC-001`, `LEXIS-SC-003` | fixture_shape_drafted_blocked |
| `planned/golden-scribere-slice` | golden slice fixture | `LEXIS-SC-001` | draft_node_edge_shape_blocked |
| `planned/negative-borrowing-descent` | negative relationship fixture | `LEXIS-SC-002` | executable_negative_fixture_not_promoted |
| `planned/source-limited-claim` | source-pointer / chronicle fixture | `LEXIS-SC-003` | planned_blocked |
| `planned/rline-preservation` | graph-preservation fixture | `LEXIS-SC-004` | planned_blocked |
| `planned/script-alphabet-slice` | script lane fixture | `LEXIS-SC-004` | draft_node_edge_shape_blocked |
| `planned/pie-root-mini-slice` | reconstruction fixture | `LEXIS-SC-002` | draft_node_edge_shape_blocked |
| `planned/semitic-root-pattern-slice` | root-pattern fixture | `LEXIS-SC-002` | draft_node_edge_shape_blocked |
| `planned/glyph-graph-write-meta-slice` | contrastive meta-language fixture | `LEXIS-SC-003` | draft_node_edge_shape_blocked |

## Blockers

All fixture plans remain blocked until:

1. source-custody research and accepted source-family decisions exist,
2. a validation command exists,
3. expected input/output file formats are specified,
4. role review accepts fixture content,
5. VTRACE evidence records executable proof.

## Research Dependencies

| Fixture plan | Required research plans |
|---|---|
| `planned/source-pointer-scribere` | `LEXIS-PAPER-001`, `LEXIS-PAPER-008` |
| `planned/golden-scribere-slice` | `LEXIS-PAPER-001`, `LEXIS-PAPER-002`, `LEXIS-PAPER-003`, `LEXIS-PAPER-008` |
| `planned/negative-borrowing-descent` | `LEXIS-PAPER-002`, `LEXIS-PAPER-003`, `LEXIS-PAPER-010` |
| `planned/source-limited-claim` | `LEXIS-PAPER-001`, `LEXIS-PAPER-007`, `LEXIS-PAPER-008` |
| `planned/rline-preservation` | `LEXIS-PAPER-006` |
| `planned/script-alphabet-slice` | `LEXIS-PAPER-004`, `LEXIS-PAPER-009` |
| `planned/pie-root-mini-slice` | `LEXIS-PAPER-003` |
| `planned/semitic-root-pattern-slice` | `LEXIS-PAPER-005` |
| `planned/glyph-graph-write-meta-slice` | `LEXIS-PAPER-011` |

## Draft Fixture Shapes

| Fixture | Shape | Expected diagnostics | Status |
|---|---|---|---|
| `LEXIS-FIX-001-source-pointer-scribere` | `planned/source-pointer-scribere/fixture.yaml` | `planned/source-pointer-scribere/expected-diagnostics.yaml` | invalid by design until validator and accepted source records exist |
| `LEXIS-FIX-002-golden-scribere-slice` | `planned/golden-scribere-slice/fixture.yaml` | n/a | draft node/edge shape, graph output blocked until validation passes |
| `LEXIS-FIX-003-negative-borrowing-descent` | `planned/negative-borrowing-descent/fixture.yaml` | n/a | invalid by design; must emit relationship-collapse diagnostics |
| `LEXIS-FIX-006-script-alphabet-slice` | `planned/script-alphabet-slice/fixture.yaml` | n/a | draft script lane shape, graph output blocked until validation passes |
| `LEXIS-FIX-007-pie-root-mini-slice` | `planned/pie-root-mini-slice/fixture.yaml` | n/a | draft root/cognate shape, graph output blocked until validation passes |
| `LEXIS-FIX-008-semitic-root-pattern-slice` | `planned/semitic-root-pattern-slice/fixture.yaml` | n/a | draft root-pattern shape, graph output blocked until validation passes |
| `LEXIS-FIX-009-glyph-graph-write-meta-slice` | `planned/glyph-graph-write-meta-slice/fixture.yaml` | n/a | draft contrastive meta-language shape, graph output blocked until validation passes |
