# LEXIS Communications Strategy

Status: settled.

CONOPS reached fixed point. COMMUNICATIONS_STRATEGY defines how LEXIS should
talk about language-history claims before requirements and public artifacts
turn those claims into commands, reports, or downstream packets.

## Communication principle

LEXIS communication must preserve the reader's ability to tell evidence,
reconstruction, inference, uncertainty, and source limits apart. Public wording
should be memorable, but never at the cost of turning disputed language history
into a false certainty.

## Audience lanes

| Audience | Need | Communication posture |
|---|---|---|
| Portfolio maintainer | Know what LEXIS owns and what remains blocked. | Plain governance language, explicit gates, no implementation claims. |
| Researcher/writer | Follow language-history evidence without losing nuance. | Clear chronicle prose with visible source and theory boundaries. |
| Downstream repo | Consume a bounded artifact safely. | Artifact summaries must name scope, source posture, and unsupported claims. |
| Agent workflow | Request compact cited context. | Prompts and packs must include task boundary, included/excluded claims, and uncertainty labels. |
| Public reader | Understand a word/script/sound story. | Use readable language, but mark disputed or reconstructed claims directly. |

## Public wording rules

- Say "attested" only when a source-backed attestation exists.
- Say "reconstructed" when a form, root, sound, or pathway is reconstructed.
- Say "likely", "possible", "disputed", or "source-limited" when the evidence
  posture requires it.
- Do not say "proved" for reconstruction, inference, or competing theories.
- Do not collapse descent, cognacy, borrowing, calque, coincidence, and unknown
  relationships in summaries.
- Do not imply a source can be redistributed unless source custody allows it.

## Chronicle voice

LEXIS chronicle prose should be readable and compact:

1. name the bounded question,
2. show the direct evidence path,
3. show the theory or reconstruction path,
4. name alternatives and uncertainty,
5. name source-custody limits,
6. hand off to graph or source views for inspection.

## Internal communication

Work packages, pulses, and stage ledgers should communicate:

- what changed,
- what claim type is affected,
- what source posture is involved,
- what role findings were closed,
- what remains deferred or blocked,
- what validation command passed.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: public language must not convert theory into fact. | Closed by public wording rules. |
| Etymology Reviewer | Major: summaries must preserve relationship distinctions. | Closed by public wording rules. |
| Phonology Reviewer | Minor: reconstructed sound claims need explicit labels. | Closed by wording rules. |
| Script Systems Reviewer | Minor: script/glyph language should not imply visual certainty without source posture. | Closed by source-custody and wording rules. |
| Source Custody Reviewer | Major: communication must not imply redistribution rights. | Closed by public and internal communication rules. |
| Graph Systems Reviewer | Minor: graph summaries should not replace inspectable graph views. | Closed by chronicle handoff rule. |
| Product Chronicle Reviewer | Major: chronicle voice must be readable but bounded. | Closed by chronicle voice sequence. |
| Software Assurance Reviewer | Major: status updates must distinguish planned, blocked, validated, and implemented. | Closed by internal communication rules. |

## Decision

COMMUNICATIONS_STRATEGY is settled for the foundation wave. No critical or
major actionable role finding remains. REQUIREMENTS is the next VTRACE stage.

