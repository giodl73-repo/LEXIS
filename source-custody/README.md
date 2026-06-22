# LEXIS Source Custody

Status: first `scribere` source pointers accepted for bounded pointer-only
slice use; other source families remain blocked.

This directory records source-family custody decisions before LEXIS promotes
source-backed claims into fixtures, graph slices, chronicles, or publisher
artifacts.

The `scribere` pilot has accepted pointer-only source records for the bounded
first slice. No language-history source text has been ingested, cached, quoted,
or redistributed.

The first research plans that may later resolve these stubs are
`LEXIS-PAPER-001` and `LEXIS-PAPER-008` under
`research/modules/source-custody-and-evidence/`.
Script and reconstruction source blockers are also covered by plans in
`research/modules/script-and-reconstruction-caution/`.

## Decision Index

| Decision | Source family | Related slice | Status |
|---|---|---|---|
| `LEXIS-SRCDEC-001-latin-lexicographic-reference` | Latin lexicographic or dictionary reference pointers. | `LEXIS-SLICE-001-SOURCE` | accepted_for_slice |
| `LEXIS-SRCDEC-002-english-etymology-reference` | English etymology reference pointers. | `LEXIS-SLICE-001-SOURCE` | accepted_for_slice |
| `LEXIS-SRCDEC-003-general-language-history-reference` | General scholarly language-history references. | `LEXIS-SLICE-001-SOURCE`, future slices | deferred_after_first_slice_scope |
| `LEXIS-SRCDEC-004-script-history-reference` | Script and alphabet-history reference pointers. | `LEXIS-SLICE-002-SOURCE` | planned_blocked |
| `LEXIS-SRCDEC-005-source-limited-placeholder` | Unknown, restricted, or source-limited examples for negative validation. | `LEXIS-SLICE-001-NEGATIVES`, `LEXIS-SLICE-005-NEGATIVES` | planned_blocked |
| `LEXIS-SRCDEC-006-pie-reconstruction-reference` | Proto-Indo-European reconstruction reference pointers. | `LEXIS-SLICE-003-SOURCE` | planned_blocked |
| `LEXIS-SRCDEC-007-semitic-root-reference` | Semitic root and pattern reference pointers. | `LEXIS-SLICE-004-SOURCE` | planned_blocked |
| `LEXIS-SRCDEC-008-meta-etymology-reference` | Glyph/graph/write contrastive etymology reference pointers. | `LEXIS-SLICE-005-SOURCE` | planned_blocked |
| `LEXIS-SRCDEC-009-latin-english-batch-candidate-reference` | Shared placeholder for generated Latin-to-English seed batches. | `LEXIS-SLICE-001-SOURCE` | candidate_review |

## Latin-English Proof Pointer Batch 001

These records are source-detail candidates from
[`../reports/candidate-latin-english-100/tier1-proof-batch-001.md`](../reports/candidate-latin-english-100/tier1-proof-batch-001.md).
They prove that a real source pointer exists for each chain, but they do not
yet permit graph promotion.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-010-tier1-proof-act` | `agere -> actum -> act` | candidate_review |
| `LEXIS-SRCDEC-011-tier1-proof-capture` | `capere -> captum -> capture` | candidate_review |
| `LEXIS-SRCDEC-012-tier1-proof-cession` | `cedere -> cessum -> cession` | candidate_review |
| `LEXIS-SRCDEC-013-tier1-proof-credit` | `credere -> creditum -> credit` | candidate_review |
| `LEXIS-SRCDEC-014-tier1-proof-conduct` | `ducere -> ductum -> conduct` | candidate_review |
| `LEXIS-SRCDEC-015-tier1-proof-factor` | `facere -> factum -> factor` | candidate_review |
| `LEXIS-SRCDEC-016-tier1-proof-junction` | `jungere -> junctum -> junction` | candidate_review |
| `LEXIS-SRCDEC-017-tier1-proof-lecture` | `legere -> lectum -> lecture` | candidate_review |
| `LEXIS-SRCDEC-018-tier1-proof-mission` | `mittere -> missum -> mission` | candidate_review |
| `LEXIS-SRCDEC-019-tier1-proof-motion` | `movere -> motum -> motion` | candidate_review |

## Latin-English Proof Pointer Batch 002

These records continue the Tier 1 proof pass from
[`../reports/candidate-latin-english-100/tier1-proof-batch-002.md`](../reports/candidate-latin-english-100/tier1-proof-batch-002.md).
They identify source pointers and bridge corrections, but remain blocked from
graph promotion.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-020-tier1-proof-position` | `ponere -> positum -> position` | candidate_review |
| `LEXIS-SRCDEC-021-tier1-proof-pressure` | `premere -> pressum -> pressure` | candidate_review |
| `LEXIS-SRCDEC-022-tier1-proof-rupture` | `rumpere -> ruptum -> rupture` | candidate_review |
| `LEXIS-SRCDEC-023-tier1-proof-script` | `scribere -> scriptum -> script` | candidate_review |
| `LEXIS-SRCDEC-024-tier1-proof-sense` | `sentire -> sensum -> sense` | candidate_review |
| `LEXIS-SRCDEC-025-tier1-proof-contact` | `tangere -> tactum -> contact` | candidate_review |
| `LEXIS-SRCDEC-026-tier1-proof-vision` | `videre -> visum -> vision` | candidate_review |
| `LEXIS-SRCDEC-027-tier1-proof-vocation` | `vocare -> vocatum -> vocation` | candidate_review |
| `LEXIS-SRCDEC-028-tier1-proof-version` | `vertere -> versum -> version` | candidate_review |
| `LEXIS-SRCDEC-029-tier1-proof-section` | `secare -> sectum -> section` | candidate_review |

## Latin-English Proof Pointer Batch 003

These records complete the current Tier 1 source-pointer proof pass from
[`../reports/candidate-latin-english-100/tier1-proof-batch-003.md`](../reports/candidate-latin-english-100/tier1-proof-batch-003.md).
They remain blocked from graph promotion until bridge and omitted-stage
decisions are modeled in corrected fixtures.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-030-tier1-proof-solution` | `solvere -> solutum -> solution` | candidate_review |
| `LEXIS-SRCDEC-031-tier1-proof-structure` | `struere -> structum -> structure` | candidate_review |
| `LEXIS-SRCDEC-032-tier1-proof-dictate` | `dicere -> dictum -> dictate` | candidate_review |
| `LEXIS-SRCDEC-033-tier1-proof-data` | `dare -> datum -> data` | candidate_review |
| `LEXIS-SRCDEC-034-tier1-proof-clause` | `claudere -> clausum -> clause` | candidate_review |
| `LEXIS-SRCDEC-035-tier1-proof-delete` | `delere -> deletum -> delete` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 001

These records start Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-001.md`](../reports/candidate-latin-english-100/tier2-proof-batch-001.md).
They are blocked from graph promotion until compound, agent-noun, and
bridge-correction decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-036-tier2-proof-amateur` | `amare -> amator -> amateur` | candidate_review |
| `LEXIS-SRCDEC-037-tier2-proof-auditor` | `audire -> auditum -> auditor` | candidate_review |
| `LEXIS-SRCDEC-038-tier2-proof-transfer` | `ferre -> transferre -> transfer` | candidate_review |
| `LEXIS-SRCDEC-039-tier2-proof-finite` | `finire -> finitum -> finite` | candidate_review |
| `LEXIS-SRCDEC-040-tier2-proof-flower` | `flos -> florem -> flower` | candidate_review |
| `LEXIS-SRCDEC-041-tier2-proof-habit` | `habere -> habitum -> habit` | candidate_review |
| `LEXIS-SRCDEC-042-tier2-proof-project` | `jacere -> projectum -> project` | candidate_review |
| `LEXIS-SRCDEC-043-tier2-proof-portable` | `portare -> portatum -> portable` | candidate_review |
| `LEXIS-SRCDEC-044-tier2-proof-rector` | `regere -> rectum -> rector` | candidate_review |
| `LEXIS-SRCDEC-045-tier2-proof-spirit` | `spirare -> spiritum -> spirit` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 002

These records continue Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-002.md`](../reports/candidate-latin-english-100/tier2-proof-batch-002.md).
They are blocked from graph promotion until bridge, compound, semantic-drift,
and uncertainty decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-046-tier2-proof-state` | `stare -> statum -> state` | candidate_review |
| `LEXIS-SRCDEC-047-tier2-proof-tenant` | `tenere -> tentum -> tenant` | candidate_review |
| `LEXIS-SRCDEC-048-tier2-proof-volume` | `volvere -> volutum -> volume` | candidate_review |
| `LEXIS-SRCDEC-049-tier2-proof-course` | `currere -> cursum -> course` | candidate_review |
| `LEXIS-SRCDEC-050-tier2-proof-pulse` | `pellere -> pulsum -> pulse` | candidate_review |
| `LEXIS-SRCDEC-051-tier2-proof-tension` | `tendere -> tensum -> tension` | candidate_review |
| `LEXIS-SRCDEC-052-tier2-proof-advent` | `venire -> ventum -> advent` | candidate_review |
| `LEXIS-SRCDEC-053-tier2-proof-victory` | `vincere -> victum -> victory` | candidate_review |
| `LEXIS-SRCDEC-054-tier2-proof-session` | `sedere -> sessum -> session` | candidate_review |
| `LEXIS-SRCDEC-055-tier2-proof-tractor` | `trahere -> tractum -> tractor` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 003

These records continue Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-003.md`](../reports/candidate-latin-english-100/tier2-proof-batch-003.md).
They are blocked from graph promotion until bridge, compound, and semantic-shift
decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-056-tier2-proof-valid` | `valere -> validum -> valid` | candidate_review |
| `LEXIS-SRCDEC-057-tier2-proof-volition` | `velle -> volitio -> volition` | candidate_review |
| `LEXIS-SRCDEC-058-tier2-proof-doctor` | `docere -> doctum -> doctor` | candidate_review |
| `LEXIS-SRCDEC-059-tier2-proof-caution` | `cavere -> cautum -> caution` | candidate_review |
| `LEXIS-SRCDEC-060-tier2-proof-conceal` | `celare -> celatum -> conceal` | candidate_review |
| `LEXIS-SRCDEC-061-tier2-proof-claim` | `clamare -> clamatum -> claim` | candidate_review |
| `LEXIS-SRCDEC-062-tier2-proof-culture` | `colere -> cultum -> culture` | candidate_review |
| `LEXIS-SRCDEC-063-tier2-proof-compose` | `componere -> compositum -> compose` | candidate_review |
| `LEXIS-SRCDEC-064-tier2-proof-condition` | `condicere -> condicio -> condition` | candidate_review |
| `LEXIS-SRCDEC-065-tier2-proof-cupidity` | `cupere -> cupitum -> cupidity` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 004

These records continue Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-004.md`](../reports/candidate-latin-english-100/tier2-proof-batch-004.md).
They are blocked from graph promotion until compound, derived-noun, coinage,
and uncertainty decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-066-tier2-proof-dolor` | `dolere -> dolorem -> dolor` | candidate_review |
| `LEXIS-SRCDEC-067-tier2-proof-exempt` | `eximere -> exemptum -> exempt` | candidate_review |
| `LEXIS-SRCDEC-068-tier2-proof-error` | `errare -> erratum -> error` | candidate_review |
| `LEXIS-SRCDEC-069-tier2-proof-essence` | `esse -> essentia -> essence` | candidate_review |
| `LEXIS-SRCDEC-070-tier2-proof-false` | `fallere -> falsum -> false` | candidate_review |
| `LEXIS-SRCDEC-071-tier2-proof-confess` | `fateri -> fassum -> confess` | candidate_review |
| `LEXIS-SRCDEC-072-tier2-proof-flex` | `flectere -> flexum -> flex` | candidate_review |
| `LEXIS-SRCDEC-073-tier2-proof-flux` | `fluere -> fluxum -> flux` | candidate_review |
| `LEXIS-SRCDEC-074-tier2-proof-fracture` | `frangere -> fractum -> fracture` | candidate_review |
| `LEXIS-SRCDEC-075-tier2-proof-fusion` | `fundere -> fusum -> fusion` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 005

These records continue Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-005.md`](../reports/candidate-latin-english-100/tier2-proof-batch-005.md).
They are blocked from graph promotion until compound, agent-noun, semantic-drift,
and weak-source posture decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-076-tier2-proof-gesture` | `gerere -> gestum -> gesture` | candidate_review |
| `LEXIS-SRCDEC-077-tier2-proof-progress` | `gradi -> gressum -> progress` | candidate_review |
| `LEXIS-SRCDEC-078-tier2-proof-adhesion` | `haerere -> haesum -> adhesion` | candidate_review |
| `LEXIS-SRCDEC-079-tier2-proof-lesion` | `laedere -> laesum -> lesion` | candidate_review |
| `LEXIS-SRCDEC-080-tier2-proof-locution` | `loqui -> locutum -> locution` | candidate_review |
| `LEXIS-SRCDEC-081-tier2-proof-lucid` | `lucere -> lucem -> lucid` | candidate_review |
| `LEXIS-SRCDEC-082-tier2-proof-mansion` | `manere -> mansum -> mansion` | candidate_review |
| `LEXIS-SRCDEC-083-tier2-proof-monitor` | `monere -> monitum -> monitor` | candidate_review |
| `LEXIS-SRCDEC-084-tier2-proof-nation` | `nasci -> natum -> nation` | candidate_review |
| `LEXIS-SRCDEC-085-tier2-proof-orator` | `orare -> oratum -> orator` | candidate_review |

## Latin-English Proof Pointer Tier 2 Batch 006

These records complete Tier 2 proof coverage from
[`../reports/candidate-latin-english-100/tier2-proof-batch-006.md`](../reports/candidate-latin-english-100/tier2-proof-batch-006.md).
They are blocked from graph promotion until compound, semantic-drift,
uncertainty, and noun/verb route decisions are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-086-tier2-proof-passion` | `pati -> passum -> passion` | candidate_review |
| `LEXIS-SRCDEC-087-tier2-proof-complicate` | `plicare -> plicatum -> complicate` | candidate_review |
| `LEXIS-SRCDEC-088-tier2-proof-question` | `quaerere -> quaesitum -> question` | candidate_review |
| `LEXIS-SRCDEC-089-tier2-proof-rapture` | `rapere -> raptum -> rapture` | candidate_review |
| `LEXIS-SRCDEC-090-tier2-proof-risible` | `ridere -> risum -> risible` | candidate_review |
| `LEXIS-SRCDEC-091-tier2-proof-science` | `scire -> scientia -> science` | candidate_review |
| `LEXIS-SRCDEC-092-tier2-proof-strict` | `stringere -> strictum -> strict` | candidate_review |
| `LEXIS-SRCDEC-093-tier2-proof-assume` | `sumere -> sumptum -> assume` | candidate_review |
| `LEXIS-SRCDEC-094-tier2-proof-use` | `uti -> usum -> use` | candidate_review |

## Latin-English Proof Pointer Tier 3 Batch 001

These records complete generated 100-chain proof coverage from
[`../reports/candidate-latin-english-100/tier3-proof-batch-001.md`](../reports/candidate-latin-english-100/tier3-proof-batch-001.md).
They are blocked from graph promotion until replacement bridges, replacement
bases, compound routes, homonym splits, and low-value direct paths are modeled.

| Decision | Chain | Status |
|---|---|---|
| `LEXIS-SRCDEC-095-tier3-proof-spectator` | `spectare -> spectator -> spectator` | candidate_review |
| `LEXIS-SRCDEC-096-tier3-proof-joy` | `gaudere -> gaudium -> joy` | candidate_review |
| `LEXIS-SRCDEC-097-tier3-proof-itinerary` | `ire -> itum -> itinerary` | candidate_review |
| `LEXIS-SRCDEC-098-tier3-proof-laud` | `laudare -> laudatum -> laud` | candidate_review |
| `LEXIS-SRCDEC-099-tier3-proof-relic` | `linquere -> lictum -> relic` | candidate_review |
| `LEXIS-SRCDEC-100-tier3-proof-noxious` | `nocere -> nocitum -> noxious` | candidate_review |
| `LEXIS-SRCDEC-101-tier3-proof-novel` | `novare -> novatum -> novel` | candidate_review |
| `LEXIS-SRCDEC-102-tier3-proof-placid` | `placere -> placitum -> placid` | candidate_review |
| `LEXIS-SRCDEC-103-tier3-proof-salient` | `salire -> saltum -> salient` | candidate_review |
| `LEXIS-SRCDEC-104-tier3-proof-sanitary` | `sanare -> sanatum -> sanitary` | candidate_review |
| `LEXIS-SRCDEC-105-tier3-proof-serve` | `servire -> servitum -> serve` | candidate_review |
| `LEXIS-SRCDEC-106-tier3-proof-sound` | `sonare -> sonitum -> sound` | candidate_review |
| `LEXIS-SRCDEC-107-tier3-proof-despair` | `sperare -> speratum -> despair` | candidate_review |
| `LEXIS-SRCDEC-108-tier3-proof-timid` | `timere -> timorem -> timid` | candidate_review |
| `LEXIS-SRCDEC-109-tier3-proof-urgent` | `urgere -> ursum -> urgent` | candidate_review |

## Candidate Review Records

| Record | Purpose | Status |
|---|---|---|
| `candidate-reviews/scribere-pilot-source-review.md` | First-pass source candidate matrix for `LEXIS-SLICE-001-SOURCE`. | superseded by accepted pointer-only source records |

## Rules

- Source-family records are not source acceptance unless their status is
  `accepted_for_slice`.
- Pointer-only is the default planned posture.
- Any real source pointer must be reviewed before use in fixture data.
- Unknown rights posture blocks fixture, graph, chronicle, and publisher
  promotion.
- Source-record states and future validator behavior are specified in
  [`../docs/specs/source-record-contract.md`](../docs/specs/source-record-contract.md).
