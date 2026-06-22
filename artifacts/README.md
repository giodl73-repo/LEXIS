# LEXIS Artifacts

Artifacts are generated or curated outputs from fixture data.

Preview artifacts preserve fixture posture, source posture, review state, and
validation blockers. The `scribere` artifact set is validated for its bounded
pointer-only slice; other artifact sets remain source-limited previews.

Current preview artifacts:

| Artifact set | Source fixture | Posture |
| --- | --- | --- |
| `planned/golden-scribere-slice/` | `fixtures/planned/golden-scribere-slice/fixture.yaml` | validated, pointer-only source accepted |
| `planned/script-alphabet-slice/` | `fixtures/planned/script-alphabet-slice/fixture.yaml` | preview only, source limited |
| `planned/pie-root-mini-slice/` | `fixtures/planned/pie-root-mini-slice/fixture.yaml` | preview only, source limited |
| `planned/semitic-root-pattern-slice/` | `fixtures/planned/semitic-root-pattern-slice/fixture.yaml` | preview only, source limited |
| `planned/glyph-graph-write-meta-slice/` | `fixtures/planned/glyph-graph-write-meta-slice/fixture.yaml` | preview only, source limited |

Regenerate a slice artifact set with:

```powershell
cargo run -p lexis-cli -- artifact write fixtures/planned/golden-scribere-slice/fixture.yaml artifacts/planned/golden-scribere-slice
```

Regenerate a fixture directory as preview artifact sets with:

```powershell
cargo run -p lexis-cli -- artifact write-batch fixtures/generated/corrected-latin-english-tier3 artifacts/generated/corrected-latin-english-tier3
```

List generated preview artifact sets with:

```powershell
cargo run -p lexis-cli -- artifact list
```

Summarize a generated artifact corpus with:

```powershell
cargo run -p lexis-cli -- artifact summarize artifacts/generated/corrected-latin-english-tier3
```

Write a durable corpus analysis report with:

```powershell
cargo run -p lexis-cli -- artifact report artifacts/generated/corrected-latin-english-tier3 reports/candidate-latin-english-100/corrected-tier3-artifact-analysis.md
```

Write a correction-aware promotion worklist with:

```powershell
cargo run -p lexis-cli -- correction artifact-report artifacts/generated/corrected-latin-english-tier3 reports/candidate-latin-english-100/corrected-tier3-promotion-worklist.md
```

Write an AI-advisory acceptance review with:

```powershell
cargo run -p lexis-cli -- acceptance ai-report artifacts/generated/corrected-latin-english-tier3 reports/candidate-latin-english-100/corrected-tier3-ai-acceptance-review.md
```
