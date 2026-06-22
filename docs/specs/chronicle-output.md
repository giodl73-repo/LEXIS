# Chronicle Output Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec defines future chronicle report behavior. No chronicle may be
published or treated as validated output until a reviewed graph slice and
validation evidence exist.

## Target Behavior

A LEXIS chronicle converts a reviewed graph slice into readable language-history
narrative while preserving evidence, theory, uncertainty, alternatives, and
source limits.

## Required Sections

| Section | Purpose |
|---|---|
| Scope | Names the bounded question and explicit non-goals. |
| Evidence path | Summarizes source-backed records and attestations. |
| Theory path | Summarizes inferred, reconstructed, or disputed relationships. |
| Alternatives | Names competing, rejected, unknown, or deferred claims. |
| Source limits | States pointer-only, rights, date, access, or source-quality limits. |
| Graph summary | Names the graph slice and preserved edge kinds. |
| Review state | States whether output is draft, reviewed, blocked, or accepted for slice. |

## Wording Controls

- Do not use proof language for reconstruction, inference, disputed, possible,
  source-limited, or unknown claims.
- Do not imply a full language-family history from a slice.
- Do not imply source text was ingested when only source pointers exist.
- Do not collapse "not related", "unknown", and "not enough evidence".

## Negative Examples

- "This proves the word came from..." is invalid for disputed or inferred paths.
- "Greek letters became Cyrillic letters" is too broad unless the scoped script
  lane supports each transition.
- A chronicle with no source-custody note is invalid.

## Trace

Requirements: `LEXIS-FR-005`, `LEXIS-OUT-001`, `LEXIS-OUT-003`.

Work packages: `LEXIS-WP-008`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: chronicle scope must prevent broad historical overclaim. | Closed by required Scope section and wording controls. |
| L-2 Etymology Reviewer | Major: competing and rejected alternatives must remain visible. | Closed by Alternatives section. |
| L-3 Phonology Reviewer | Minor: reconstructed sounds need cautious wording when present. | Closed by wording controls for reconstruction and inference. |
| L-4 Script Systems Reviewer | Major: script-lane chronicles must not overstate transitions. | Closed by negative example and graph-summary requirement. |
| L-5 Source Custody Reviewer | Major: source limits must be visible in every chronicle. | Closed by required Source limits section. |
| L-6 Graph Systems Reviewer | Minor: chronicle needs graph slice pointer. | Closed by Graph summary section. |
| L-7 Product Chronicle Reviewer | Major: narrative must preserve evidence/theory separation. | Closed by required Evidence path and Theory path sections. |
| L-8 Software Assurance Reviewer | Major: overclaim wording needs validation hooks. | Closed by wording controls and `chronicle_overclaim` diagnostic. |

Decision: draft-reviewed. No critical or major actionable finding remains; no
chronicle output is authorized.
