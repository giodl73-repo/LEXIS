# LEXIS Specification Baseline

Status: settled.

MISSION, CONOPS, COMMUNICATIONS_STRATEGY, and REQUIREMENTS reached fixed point.
This baseline names the foundation vocabulary only. It does not define Rust
types, storage, APIs, RLINE crates, fixtures, or source ingestion.

## Baseline object vocabulary

| Object | Meaning | Required posture |
|---|---|---|
| `Language` | A named language, dialect, reconstructed language, or bounded language variety. | Identifier, label, time/geography posture when known, source posture. |
| `Wordform` | An attested or reconstructed lexical form in a language and optional script. | Claim type, language, form text or normalized pointer, date posture, source posture. |
| `Root` | A reconstructed or cited root used to relate wordforms. | Reconstruction posture, theory source, confidence/uncertainty posture. |
| `CognateSet` | A reviewed group of wordforms related by descent or proposed descent. | Membership posture, competing/rejected alternatives. |
| `Attestation` | Source-backed occurrence or source pointer for a form, script, meaning, or claim. | Source pointer, rights posture, date/date-range posture, citation note. |
| `MeaningSense` | A meaning or semantic neighborhood attached to a wordform or root. | Gloss, time/context posture, evidence/theory posture. |
| `ScriptForm` | Written representation, script lane, orthography, transliteration, or glyph/form variant. | Script/orthography posture, transliteration posture when applicable. |
| `SoundFeature` | Pronunciation, phoneme, reconstructed sound, or sound-change element. | Availability posture, reconstruction posture, uncertainty posture. |
| `TheoryClaim` | Interpretive statement that connects evidence into an explanation. | Claim type, evidence links, uncertainty/rejected-alternative posture. |
| `ChronicleSlice` | Bounded reviewed graph slice prepared for narrative or artifact output. | Scope, included claims, source posture, reviewer state. |

## Claim types

| Claim type | Use |
|---|---|
| `direct_evidence` | A source-backed observation or attestation. |
| `reconstruction` | A scholarly or project-local reconstruction from evidence. |
| `inference` | A derived relation or explanation that is not directly attested. |
| `competing_theory` | A plausible alternative retained for comparison. |
| `rejected_alternative` | A considered alternative retained to prevent rediscovery or overclaiming. |
| `unknown` | A known gap where evidence does not support a stronger label. |

## Relationship and edge kinds

| Edge kind | Meaning | Must not be collapsed with |
|---|---|---|
| `attested_as` | Source links a form, meaning, script, or claim to evidence. | theory or inference |
| `descends_from` | Proposed descent relation between forms/languages/roots. | borrowing/contact |
| `cognate_with` | Membership or relation inside a cognate set. | borrowing/contact or coincidence |
| `borrowed_from` | Contact transfer from donor to recipient. | descent/cognacy |
| `calque_of` | Loan translation or structural borrowing. | direct lexical borrowing |
| `sound_shift_to` | Sound-change relation or path. | meaning/script transitions |
| `meaning_shift_to` | Semantic drift relation or path. | sound/script transitions |
| `script_variant_of` | Orthographic, transliteration, glyph, or script-form relation. | sound or meaning shift |
| `supports_claim` | Evidence supports a theory or inference. | proves_claim |
| `disputes_claim` | Evidence or theory conflicts with a claim. | rejected_alternative |

## Uncertainty labels

| Label | Meaning |
|---|---|
| `settled_for_slice` | Accepted for the current bounded slice, not a universal claim. |
| `likely` | Evidence supports the claim but alternatives remain plausible. |
| `possible` | Claim is plausible but weakly supported. |
| `disputed` | Competing theories remain active. |
| `source_limited` | Source quality, access, date, or rights posture limits confidence. |
| `unavailable` | Data is absent and should not be silently inferred. |

## Source and rights posture

Every source-backed record must be able to carry:

- source pointer,
- citation note,
- date or date-range posture,
- language/script posture,
- rights posture,
- redistribution posture,
- reviewer state.

## Baseline non-goals

- No universal language taxonomy.
- No full dictionary schema.
- No phonetic engine.
- No translation engine.
- No RLINE API commitment.
- No source ingestion format.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: `settled` labels could read like universal historical certainty. | Closed by `settled_for_slice`. |
| Etymology Reviewer | Major: relationship kinds need calque, coincidence/unknown, and rejected alternatives. | Closed by claim types plus relationship separation; coincidence remains represented through `unknown` or rejected alternatives until design needs a separate edge. |
| Phonology Reviewer | Major: sound data availability must be explicit. | Closed by `SoundFeature` and `unavailable`. |
| Script Systems Reviewer | Major: script, orthography, transliteration, and glyph variants need one lane without overcommitting implementation. | Closed by `ScriptForm` and `script_variant_of`. |
| Source Custody Reviewer | Major: rights and redistribution posture must be baseline fields. | Closed by source and rights posture section. |
| Graph Systems Reviewer | Major: edge semantics are LEXIS-owned and RLINE API-neutral. | Closed by no RLINE API commitment. |
| Product Chronicle Reviewer | Minor: chronicle output needs a bounded slice object. | Closed by `ChronicleSlice`. |
| Software Assurance Reviewer | Major: baseline must avoid premature Rust schema and fixture commitments. | Closed by baseline scope and non-goals. |

## Decision

SPECIFICATION_BASELINE is settled for the foundation wave. No critical or major
actionable role finding remains. ARCHITECTURE is the next VTRACE stage.
