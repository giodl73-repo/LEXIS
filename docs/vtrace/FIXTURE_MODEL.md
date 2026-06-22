# LEXIS Fixture Model

Status: settled.

Fixtures are blocked until source custody, model requirements, and validation
commands exist. This file defines how future language-history artifacts can
become controlled proof inputs. `LEXIS-FIX-001` now has a draft fixture shape,
but it is invalid by design and not executable evidence.

## Fixture Classes

| Class | Purpose | First candidate |
|---|---|---|
| Source-pointer fixture | Proves allowed source metadata shape without redistributing source text. | `LEXIS-SLICE-001-SOURCE`. |
| Golden slice fixture | Proves a narrow valid language-history path. | Latin `scribere` minimal slice. |
| Negative relationship fixture | Proves rejected, disputed, unknown, or source-limited claims stay visible. | glyph/graph/write contrast claim. |
| Script-lane fixture | Proves script form is separate from sound value. | Greek alphabet lane. |
| Reconstruction fixture | Proves reconstructed roots and uncertainty labels survive validation. | PIE root mini-slice. |
| Graph-preservation fixture | Proves graph output keeps LEXIS labels. | First validated graph slice before RLINE adoption. |
| Chronicle fixture | Proves narrative output includes evidence, theory, uncertainty, and source limits. | First reviewed chronicle. |

## Promotion Rule

A LEXIS research or scenario artifact becomes a fixture only when:

1. source-custody posture is accepted,
2. scope is bounded,
3. entities and edge kinds are accepted,
4. positive or negative expected result is explicit,
5. diagnostics are allocated,
6. graph/chronicle/public-claim impact is recorded,
7. validation command is known,
8. role review closes major findings.

## Draft Shape Status

| Fixture | Status | Reason |
|---|---|---|
| `LEXIS-FIX-001-source-pointer-scribere` | draft shape, blocked | Candidate source records exist, but accepted source decisions and validator do not. |
| `LEXIS-FIX-002-golden-scribere-slice` | draft node/edge shape, blocked | Candidate source records and graph output blockers prevent promotion. |

## Decision

FIXTURE_MODEL is settled as planning input. No promoted fixture exists yet.
