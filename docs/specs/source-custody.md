# Source Custody Spec

Status: draft-reviewed, not promoted for implementation.

## Authority Boundary

This spec defines LEXIS-owned source-custody behavior. It does not authorize
source ingestion or redistribution. Real source-family decisions must be added
through future source-custody records and role review.

## Target Behavior

LEXIS records source pointers and custody posture before any source-backed
language-history claim is accepted into a fixture, graph slice, chronicle, or
publisher artifact.

## Required Source Record Fields

| Field | Required | Meaning |
|---|---|---|
| `source_id` | yes | Stable local identifier for a reviewed source pointer. |
| `source_family` | yes | Dictionary, lexicon, corpus, inscription, grammar, article, database, project note, or other reviewed family. |
| `pointer` | yes | Bibliographic pointer, URL, catalog id, or local redacted reference. |
| `rights_posture` | yes | Public-domain, permissive, copyrighted-pointer-only, restricted, unknown, or blocked. |
| `redistribution_posture` | yes | Allowed, brief-quote-only, summary-only, pointer-only, local-cache-only, or blocked. |
| `citation_note` | yes | Short human note sufficient to identify the source without replacing it. |
| `date_posture` | yes | Date, date range, approximate date, source date, or unavailable. |
| `language_script_posture` | yes | Language/script coverage claimed by the source pointer. |
| `review_state` | yes | Proposed, reviewed, accepted_for_slice, rejected, or blocked. |
| `reviewer` | yes when accepted | Source Custody Reviewer or named review lane. |

## Invariants

- Pointer-only is the default posture for dictionaries, corpora, and reference
  works unless review accepts a stronger posture.
- A source record may support a claim only when `review_state` is
  `accepted_for_slice`.
- Source text may not be stored in fixtures unless redistribution posture
  explicitly permits it.
- Source records support evidence posture; they do not by themselves prove a
  theory claim.

## Positive Example

A first-slice source pointer for Latin `scribere` can be accepted when it has a
stable bibliographic pointer, pointer-only redistribution posture, date posture,
and Source Custody Reviewer approval.

## Negative Examples

- A dictionary entry copied into a fixture without rights review is invalid.
- A URL with no source family or redistribution posture is invalid.
- A source pointer marked `unknown` rights posture cannot promote a graph or
  chronicle claim.

## Diagnostics

Future validation should allocate failures to `source_custody`. Detailed source
state, promotion, and diagnostic behavior is defined in
`source-record-contract.md`.

## Trace

Requirements: `LEXIS-EV-001`, `LEXIS-EV-002`, `LEXIS-EV-003`.

Work packages: `LEXIS-WP-003`, `LEXIS-WP-004`, `LEXIS-WP-005`, all `*-SOURCE`
packages.

## Role Review

| Role | Finding | Decision |
|---|---|---|
| L-1 Language Historian | Minor: sources can support different historical periods and should not imply universal coverage. | Closed by date and language/script posture fields. |
| L-2 Etymology Reviewer | Minor: source pointers should not prove theory claims by themselves. | Closed by source-record invariant. |
| L-3 Phonology Reviewer | Minor: pronunciation sources may require separate posture later. | Deferred to future source-family decisions and sound fixtures. |
| L-4 Script Systems Reviewer | Minor: script coverage needs explicit posture. | Closed by `language_script_posture`. |
| L-5 Source Custody Reviewer | Major: default posture must prevent accidental text redistribution. | Closed by pointer-only default and redistribution fields. |
| L-6 Graph Systems Reviewer | Minor: graph slices need source posture preserved. | Closed by target behavior and trace to graph packages. |
| L-7 Product Chronicle Reviewer | Minor: chronicles need source-limit wording. | Closed by source record support for source-custody notes. |
| L-8 Software Assurance Reviewer | Major: invalid source states need future negative tests. | Closed by negative examples and diagnostic allocation. |

Decision: draft-reviewed. No critical or major actionable finding remains; no
source ingestion is authorized.
