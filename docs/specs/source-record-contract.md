# Source Record Contract Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This contract defines the future record shape and validation behavior for LEXIS
source-custody records. It does not accept any source, ingest source text,
define Rust types, or authorize fixtures.

## Contract Goal

LEXIS must be able to distinguish a candidate pointer from an accepted source
record before any wordform, relationship, graph slice, chronicle, or publisher
artifact uses that source as evidence.

## Record Kinds

| Kind | Purpose | May support claims? | May appear in fixtures? |
|---|---|---:|---:|
| `candidate_review` | A source pointer is being evaluated for a slice. | no | source-pointer fixture only, as invalid or blocked input |
| `accepted_for_slice` | A reviewed pointer is accepted for one bounded slice. | yes, within named slice only | yes |
| `deferred` | A source family is out of scope for the current slice. | no | only in negative or blocked fixtures |
| `rejected` | A reviewed pointer is not acceptable for the intended use. | no | yes, as negative validation input |
| `blocked` | Rights, access, quality, or review posture blocks use. | no | yes, as negative validation input |

## Required Fields

| Field | Required for all states | Additional rule |
|---|---:|---|
| `decision_id` | yes | Stable local source-custody decision id. |
| `status` | yes | One of the contract record kinds or a more specific blocked substate. |
| `source_family` | yes | Dictionary, lexicon, corpus, inscription, grammar, article, database, or project note. |
| `related_slice_packages` | yes | At least one `*-SOURCE` package. |
| `related_research` | yes | At least one paper or research track. |
| `candidate_use` | yes | Intended use, written narrowly. |
| `pointer` | yes | URL, bibliographic pointer, catalog id, or `none_selected` for deferred records. |
| `rights_posture` | yes | Must not be `unknown` for `accepted_for_slice`. |
| `redistribution_posture` | yes | Defaults to `pointer_only_planned` or stronger restriction. |
| `citation_note` | yes | Human-identifying note that does not replace the source. |
| `date_posture` | yes | Source date, access date, date range, approximate date, or unavailable posture. |
| `language_script_posture` | yes | Language/script coverage claimed by the pointer. |
| `review_state` | yes | Review lifecycle state matching the source record kind. |
| `reviewer` | yes | L-5 required before `accepted_for_slice`. |
| `promotion_allowed` | yes | Must be `false` unless status is `accepted_for_slice`. |
| `blocks` | yes | Names downstream artifacts blocked by the record. |

## State Rules

| Current state | Allowed next state | Required review |
|---|---|---|
| `planned_blocked` | `candidate_review`, `deferred`, `blocked` | L-5 source custody review. |
| `candidate_review` | `accepted_for_slice`, `rejected`, `blocked`, `deferred` | L-5 plus affected domain role. |
| `deferred` | `candidate_review`, `blocked` | L-1 or relevant domain role plus L-5. |
| `rejected` | `candidate_review` | New evidence or source-family decision. |
| `blocked` | `candidate_review` | Explicit blocker removal evidence. |
| `accepted_for_slice` | `blocked`, `rejected`, `accepted_for_slice` with narrowed scope | L-5 and affected package review. |

## Promotion Rules

- Only `accepted_for_slice` source records may support source-backed language
  claims.
- `candidate_review` records may only be used to test blocked source-pointer
  behavior.
- A source record accepted for one slice is not accepted for another slice by
  implication.
- `promotion_allowed: true` is invalid unless `status: accepted_for_slice`.
- Any fixture containing source text must reference a redistribution posture
  stronger than pointer-only; otherwise validation must fail.
- A graph, chronicle, or publisher artifact must fail validation if it depends
  on a source record whose status is not `accepted_for_slice`.

## Diagnostic Contract

Future validation must emit `source_custody` diagnostics for these cases:

| Diagnostic condition | Severity | Suggested disposition |
|---|---|---|
| Missing required source field. | error | Complete source record. |
| `promotion_allowed: true` with non-accepted status. | error | Reset promotion or complete acceptance review. |
| Claim uses `candidate_review`, `deferred`, `rejected`, or `blocked` source. | error | Remove claim or promote source through review. |
| `accepted_for_slice` has unknown rights posture. | error | Resolve rights posture. |
| Fixture includes source text with pointer-only redistribution posture. | error | Remove source text or change posture with evidence. |
| Source is accepted for a different slice. | error | Add slice-specific review or remove dependency. |
| Candidate source lacks source-family or citation note. | warning | Complete candidate metadata before review close. |
| Fixture source state differs from linked source-custody record state. | error | Update the fixture or custody record through review. |
| Linked source-custody decision record is missing or malformed. | error | Add or repair source-custody record before validation. |

## First `scribere` Application

| Decision | Current contract state | Implementation implication |
|---|---|---|
| `LEXIS-SRCDEC-001` | `candidate_review` | Can be used only in a blocked source-pointer fixture. |
| `LEXIS-SRCDEC-002` | `candidate_review` | Can be used only in a blocked source-pointer fixture. |
| `LEXIS-SRCDEC-003` | `deferred_after_first_slice_scope` | Must not appear in first golden fixture unless scope expands and review restarts. |

## Trace

Requirements: `LEXIS-EV-001`, `LEXIS-EV-002`, `LEXIS-EV-003`,
`LEXIS-FR-001`.

Work packages: `LEXIS-WP-003`, `LEXIS-WP-004`, `LEXIS-WP-005`,
`LEXIS-SLICE-001-SOURCE`.

Research: `LEXIS-PAPER-001`, `LEXIS-PAPER-008`.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Major: accepted sources must be scoped to a slice, not universal history. | Closed by slice-specific acceptance rule. |
| L-2 Etymology Reviewer | Major: candidate dictionaries must not become claim support by accident. | Closed by candidate-use promotion block. |
| L-3 Phonology Reviewer | Minor: pronunciation and sound-change sources may need stricter future subfields. | Deferred to future sound fixture. |
| L-4 Script Systems Reviewer | Minor: script coverage must be visible even for word-history sources. | Closed by `language_script_posture`. |
| L-5 Source Custody Reviewer | Major: rights and redistribution posture must block promotion when unresolved. | Closed by diagnostic contract. |
| L-6 Graph Systems Reviewer | Major: graph output must fail on candidate-only sources. | Closed by promotion rules. |
| L-7 Product Chronicle Reviewer | Major: chronicles must not narrate candidate pointers as evidence. | Closed by promotion rules. |
| L-8 Software Assurance Reviewer | Major: validator behavior must be testable before CLI implementation. | Closed by diagnostic table and state rules. |

## Decision

The source-record contract is draft-reviewed. It is ready as planning input for
`LEXIS-WP-003` and `LEXIS-WP-005`, but it is not implementation-promoted and no
source is accepted.
