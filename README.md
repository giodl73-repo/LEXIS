# LEXIS

Language history as an evidence graph.

LEXIS traces how words, sounds, scripts, meanings, and languages evolve through
time. It connects roots, cognates, borrowings, sound shifts, semantic drift,
orthography, attestations, source records, and historical theory into
inspectable lineage graphs and chronicles.

LEXIS is a Knowledge Systems repo. It owns linguistic evidence, language-history
models, uncertainty labels, and chronicle output. RLINE may provide reusable
graph packages once implementation starts, but LEXIS keeps linguistic scoring,
source policy, and historical interpretation local.

The current domain plan is in [`DOMAIN_BACKLOG.md`](DOMAIN_BACKLOG.md). It
defines entity build order and slice sequence. The first `scribere` fixture is
source-accepted for a bounded pointer-only slice; the other planned slices remain
candidate or blocked.

Concrete repeatable slice work packages are in
[`LANGUAGE_SLICE_PACKAGES.md`](LANGUAGE_SLICE_PACKAGES.md).

Research modules and paper plans that drive future requirements are in
[`RESEARCH_PLAN.md`](RESEARCH_PLAN.md).
The first planned research module is under [`research/modules/source-custody-and-evidence/`](research/modules/source-custody-and-evidence/).
The second planned research module is under
[`research/modules/ontology-and-relationship-semantics/`](research/modules/ontology-and-relationship-semantics/).
The third planned research module is under
[`research/modules/script-and-reconstruction-caution/`](research/modules/script-and-reconstruction-caution/).
The fourth planned research module is under
[`research/modules/graph-and-chronicle-method/`](research/modules/graph-and-chronicle-method/).
The fifth planned research module is under
[`research/modules/publisher-context/`](research/modules/publisher-context/).

Draft-reviewed implementation-facing specs are in [`docs/specs/`](docs/specs/).
They are not promoted for implementation until scenarios, fixtures, validation
commands, and package evidence exist.

The first Rust implementation scaffold is present as a Cargo workspace. The
minimal `lexis validate <fixture>` command currently fails closed on
`LEXIS-FIX-001`, proving candidate source pointers cannot be promoted as
evidence.
`lexis slice generate` turns a compact seed file into a full graph-facing
fixture so LEXIS can scale beyond hand-authored slices.
`lexis batch validate` and `lexis batch summary` run validation and graph counts
across generated fixture directories.
`lexis trace word` traces a selected wordform inside a fixture, including its
language, source links, connected relationship edges, uncertainty, and
preview-only review posture.
`lexis trace lineage` walks lineage-style relationship edges from a selected
wordform and preserves relationship kind, claim posture, uncertainty, and
review state.
`lexis trace neighborhood` shows local graph adjacency around a node or source
link, including source links and incoming/outgoing relationship edges.
`lexis source status` compares fixture source states with planned
source-custody records and makes promotion blockers visible.
`lexis source list` inventories planned source-custody decisions and their
candidate/deferred/blocked review posture.
`lexis source review` renders a source-custody review packet for one decision,
including pointer posture, rights posture, blockers, and referencing fixtures.
`lexis fixture list` inventories planned fixture manifests so reviewers can
choose which fixture to validate, trace, or review.
`lexis slice list` inventories planned language slice packages and cross-links
them to fixture and source-custody references.
`lexis slice review` renders one slice package set with package steps, linked
fixtures, source decisions, and planning-only posture.
`lexis scenario list` inventories planned validation scenarios, actors,
diagnostics, work-package counts, and fixture candidates.
`lexis scenario review` renders one planned scenario with purpose, specs,
positive and negative paths, expected diagnostics, evidence, and fixture
candidates.
`lexis work-package list` inventories VTRACE work packages and cross-links them
to planned scenarios.
`lexis fixture readiness` summarizes validation, source, graph, chronicle, and
promotion-blocker gates before a fixture can be promoted.
`lexis fixture review` assembles a role-review packet from readiness, source
status, diagnostics, graph summary, and chronicle preview.
`lexis diagnostics explain` groups validator diagnostics by family and keeps
stable diagnostic IDs visible for review.
`lexis artifact list` inventories generated preview graph/chronicle artifact
sets and reports their preview-only posture.

The graph-facing fixtures cover `scribere`, a narrow Greek/Latin/Cyrillic script
lane, a PIE root mini-slice, a Semitic root-pattern mini-slice, and a
glyph/graph/write contrast slice. `lexis graph emit` now emits validated JSON
and DOT for the accepted `scribere` fixture and blocks emission for invalid or
blocked fixtures.
`lexis graph preview` can render draft or accepted fixtures for review with
explicit preview-only posture.
`lexis graph summary` reports node classes, edge kinds, source postures,
uncertainties, and validation status for quick inspection.
`lexis graph inspect` lists the actual draft nodes and edges with source
posture, claim type, uncertainty, and review state before chronicle work.
`lexis graph path` finds a fixture-local shortest connection between two graph
nodes while preserving stored edge direction.
`lexis graph explain` explains one node or edge claim, including source support,
uncertainty, review state, and adjacent graph context.
`lexis chronicle preview` turns graph-facing fixtures into cautious sectioned
prose for review without publishing them.

Planned scenario packages are in [`scenarios/`](scenarios/). They name positive
and negative validation paths but are not executed evidence yet.

Planned fixture manifests are in [`fixtures/`](fixtures/). They name future
proof inputs and blockers but do not contain executable fixture data.

Preview graph artifacts are in [`artifacts/`](artifacts/). The saved artifacts
render the accepted `scribere` slice and the remaining source-limited candidate
slices as JSON, DOT, and chronicle preview. Regenerate a slice with
`cargo run -p lexis-cli -- artifact write <fixture> <artifact-dir>`.

Seed files are in [`seeds/`](seeds/). Generated fixtures are in
[`fixtures/generated/`](fixtures/generated/) and are meant to be reviewed,
expanded, or promoted into planned fixtures after source and relationship
review.
The 100-slice candidate pipeline is generated by
[`tools/run_candidate_seed_pipeline.ps1`](tools/run_candidate_seed_pipeline.ps1)
and reported under
[`reports/candidate-latin-english-100/`](reports/candidate-latin-english-100/).

Source-custody decisions are in [`source-custody/`](source-custody/). The first
`scribere` source pointers are accepted for bounded pointer-only slice use; no
source text is copied or redistributed.

## Product thesis

Language history is hard to inspect when etymology, script history, sound
change, semantic drift, and source attestations live in separate references.
LEXIS turns those pieces into a typed graph so a reader can follow a word or
language feature through ancestry, borrowing, contact, meaning change, written
forms, and cited evidence.

## First wedge

The first implementation slice should stay narrow:

1. one language-family slice,
2. one small word/root family,
3. one script or orthography lane,
4. explicit attestation/source records,
5. one chronicle report that separates evidence from theory.

## Initial scope

- Define the VTRACE governance process before code.
- Commit a repo-local role panel for linguistic, source, graph, and product
  review.
- Plan PANEL-style research modules and papers before selecting real source
  families or implementing fixtures.
- Record the dependency posture: RLINE as intended runtime graph machinery,
  ROLES/VTRACE as review method, and FLETCH/PROOF/CROP/PEBBLE as later
  publisher/source layers.
- Defer Rust crates, CLI contracts, fixtures, and source ingestion until the
  VTRACE mission, CONOPS, requirements, and baseline reviews settle.

## Portfolio reuse posture

LEXIS is intentionally a specialist language-history product, not a reusable
portfolio foundation. Its fixture schemas, linguistic relationship kinds,
uncertainty labels, source-custody policy, scoring, and chronicle output remain
domain-owned and preview-oriented. The current Rust CLI validates LEXIS evidence
and review workflows; it is not a published graph library or ingestion SDK.

Shared graph mechanics belong in RLINE if that dependency matures, while LEXIS
retains linguistic interpretation and source authority. Other repositories
should not copy candidate fixtures, source records, generated artifacts, or
diagnostic vocabulary as stable APIs. Revisit direct reuse only after an
accepted versioned contract, a real downstream manifest, and consumer-owned
compatibility tests exist.

## Non-goals

- LEXIS is not a general dictionary, translation app, or language-learning
  product.
- LEXIS does not claim certainty where sources only support competing theories.
- LEXIS does not make RLINE own linguistic semantics.
- LEXIS does not scrape or redistribute copyrighted dictionaries or corpora
  without a reviewed source-custody path.
- LEXIS does not start implementation work before the founding VTRACE stages
  have role-reviewed scope.

## Governance

The founding wave is VTRACE-first:

```text
MISSION -> CONOPS -> COMMUNICATIONS_STRATEGY -> REQUIREMENTS
  -> SPECIFICATION_BASELINE -> PROBLEM_SPACE_MAP -> DOMAIN_BACKLOG
  -> RESEARCH_PLAN -> SPEC_MODEL -> ARCHITECTURE -> INTERFACES
  -> DESIGN -> PACKAGE_BOUNDARIES -> CONTRACT_BOUNDARIES -> SCENARIO_MODEL
  -> DIAGNOSTIC_MODEL -> FIXTURE_MODEL -> CODE_RIGOR -> IMPLEMENTATION_PLAN
  -> WORK_PACKAGES -> VERIFICATION -> VALIDATION -> TRACE -> REVIEW
  -> STAGE_EXECUTION
```

See [`docs/vtrace/VTRACE_PROCESS.md`](docs/vtrace/VTRACE_PROCESS.md) and
[`context/waves/2026-06-05-foundation-vtrace/WAVE.md`](context/waves/2026-06-05-foundation-vtrace/WAVE.md).

Support controls also live under `docs/vtrace/`: `EVIDENCE.md`,
`CHANGE_CONTROL.md`, `REVIEW_CHECKLISTS.md`, `ROLE_RECOMMENDATIONS.md`,
`LANGUAGE_PROFILES.md`, `SOURCE_BASIS.md`, and `PULSE_EXECUTION.md`.

## Validation

Current validation:

```powershell
git diff --check
cargo fmt --check
cargo test
cargo run -p lexis-cli -- slice generate seeds/scribere-family.yaml fixtures/generated/scribere-family/fixture.yaml
powershell -ExecutionPolicy Bypass -File tools/run_candidate_seed_pipeline.ps1
cargo run -p lexis-cli -- batch validate fixtures/generated
cargo run -p lexis-cli -- batch summary fixtures/generated
cargo run -p lexis-cli -- validate fixtures/planned/source-pointer-scribere/fixture.yaml
cargo run -p lexis-cli -- validate fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- validate fixtures/generated/scribere-family/fixture.yaml
cargo run -p lexis-cli -- fixture list
cargo run -p lexis-cli -- artifact list
cargo run -p lexis-cli -- source list
cargo run -p lexis-cli -- slice list
cargo run -p lexis-cli -- slice review 001
cargo run -p lexis-cli -- scenario list
cargo run -p lexis-cli -- scenario review 001
cargo run -p lexis-cli -- work-package list
cargo run -p lexis-cli -- source review LEXIS-SRCDEC-002-english-etymology-reference
cargo run -p lexis-cli -- diagnostics explain fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- diagnostics explain fixtures/planned/negative-borrowing-descent/fixture.yaml
cargo run -p lexis-cli -- fixture readiness fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- fixture review fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- source status fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- trace word fixtures/planned/golden-scribere-slice/fixture.yaml wf-en-scribe
cargo run -p lexis-cli -- trace lineage fixtures/planned/golden-scribere-slice/fixture.yaml wf-en-scribe
cargo run -p lexis-cli -- trace neighborhood fixtures/planned/golden-scribere-slice/fixture.yaml wf-en-scribe
cargo run -p lexis-cli -- graph emit fixtures/planned/golden-scribere-slice/fixture.yaml --format json
cargo run -p lexis-cli -- graph preview fixtures/planned/golden-scribere-slice/fixture.yaml --format dot
cargo run -p lexis-cli -- graph summary fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- graph inspect fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- graph path fixtures/planned/golden-scribere-slice/fixture.yaml wf-lat-scribere wf-en-describe
cargo run -p lexis-cli -- graph explain fixtures/planned/golden-scribere-slice/fixture.yaml edge-describe-borrowed-from-describere
cargo run -p lexis-cli -- chronicle preview fixtures/planned/golden-scribere-slice/fixture.yaml
cargo run -p lexis-cli -- artifact write fixtures/planned/golden-scribere-slice/fixture.yaml artifacts/planned/golden-scribere-slice
cargo run -p lexis-cli -- artifact write fixtures/planned/script-alphabet-slice/fixture.yaml artifacts/planned/script-alphabet-slice
cargo run -p lexis-cli -- artifact write fixtures/planned/pie-root-mini-slice/fixture.yaml artifacts/planned/pie-root-mini-slice
cargo run -p lexis-cli -- artifact write fixtures/planned/semitic-root-pattern-slice/fixture.yaml artifacts/planned/semitic-root-pattern-slice
cargo run -p lexis-cli -- artifact write fixtures/planned/glyph-graph-write-meta-slice/fixture.yaml artifacts/planned/glyph-graph-write-meta-slice
```

The accepted `scribere` fixture validates and emits graph JSON/DOT with exit
code `0`. Planned fixtures with candidate-only or planned-blocked source custody
still return exit code `1` because they are invalid or blocked by design.

## License

LEXIS uses separate licenses for software and content. Source code,
executable scripts, tests, configuration, and ordinary software
documentation are MIT-licensed (copyright Gio Della-Libera). Original
non-software content is licensed CC BY-NC 4.0 (copyright Gio Della-Libera);
commercial use of that content requires separate written permission.
Third-party material remains under its own terms.
See [LICENSE](./LICENSE) for the complete notice.
