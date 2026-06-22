# Language-History Model Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec refines the VTRACE baseline object vocabulary for future data-model
work. It does not define Rust structs, schemas, storage, CLI format, or fixture
files yet.

## Target Behavior

LEXIS represents language-history slices as source-linked records with explicit
scope, claim posture, uncertainty, and review state.

## Record Classes

| Record | Required minimum | Notes |
|---|---|---|
| `ChronicleScope` | scope id, question, included languages/scripts/forms, excluded claims, review state | Must exist before records promote into a slice. |
| `Language` | id, label, kind, time/geography posture, source posture | Kind may include language, dialect, variety, reconstructed language, or out-of-scope. |
| `ScriptForm` | id, script label, form text or pointer, transliteration posture, source posture | Must not imply sound value unless a sound record exists. |
| `Wordform` | id, language id, form posture, claim type, source posture, uncertainty | Attested and reconstructed forms are distinct. |
| `Root` | id, reconstruction posture, theory claim, source posture, uncertainty | Required only when a slice uses root analysis. |
| `MeaningSense` | id, gloss, context posture, claim type, source posture | Glosses are LEXIS summaries, not copied dictionary text by default. |
| `SoundFeature` | id, sound value or pointer, availability posture, reconstruction posture | Optional in slices where sound data is unavailable. |
| `TheoryClaim` | id, claim type, statement summary, supporting records, uncertainty | Must name alternatives or mark them unavailable when relevant. |
| `ChronicleSlice` | scope id, included records, included/rejected claims, review state | Required before graph or chronicle output. |

## Lifecycle States

| State | Meaning |
|---|---|
| `proposed` | Draft record, not usable as evidence. |
| `source_reviewed` | Source posture has been reviewed. |
| `accepted_for_slice` | Accepted for the current bounded slice only. |
| `rejected_for_slice` | Reviewed and rejected for the current slice. |
| `blocked` | Cannot promote without new source, scope, or review action. |

## Invariants

- Every record promoted beyond `proposed` must link to a `ChronicleScope`.
- Every source-backed record must link to at least one accepted source record.
- Every reconstructed or inferred record must carry an uncertainty label.
- `settled_for_slice` never means universally settled language history.
- Missing sound or script data must be represented as `unavailable`, not
  silently inferred.

## Negative Examples

- A wordform with no claim type is invalid.
- A reconstructed root marked as direct evidence is invalid.
- A language node with no scope can stay proposed but cannot enter a slice.

## Diagnostics

Future validation should allocate failures to `scope`, `claim_type`,
`reconstruction`, `script`, or `source_custody`.

## Trace

Requirements: `LEXIS-FR-001`, `LEXIS-FR-002`, `LEXIS-FR-004`,
`LEXIS-EV-003`.

Work packages: `LEXIS-WP-002`, `LEXIS-WP-004`, `LEXIS-WP-005`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: records need scoped acceptance, not global historical truth. | Closed by `ChronicleScope`, lifecycle states, and `settled_for_slice` invariant. |
| L-2 Etymology Reviewer | Major: reconstructed forms and theory claims must remain distinct. | Closed by `Wordform`, `Root`, and `TheoryClaim` requirements. |
| L-3 Phonology Reviewer | Minor: sound data may be absent in early slices. | Closed by optional `SoundFeature` and unavailable invariant. |
| L-4 Script Systems Reviewer | Major: script form must not imply sound value. | Closed by `ScriptForm` notes and diagnostics. |
| L-5 Source Custody Reviewer | Major: source-backed records need accepted source links before promotion. | Closed by invariants. |
| L-6 Graph Systems Reviewer | Minor: graph node classes should survive later graph output. | Deferred to `graph-slice.md` preservation checks. |
| L-7 Product Chronicle Reviewer | Minor: chronicle output needs reviewed slice packaging. | Closed by `ChronicleSlice`. |
| L-8 Software Assurance Reviewer | Major: model states need deterministic validation targets. | Closed by lifecycle states and negative examples. |

Decision: draft-reviewed. No critical or major actionable finding remains; Rust
types and schemas remain unimplemented.
