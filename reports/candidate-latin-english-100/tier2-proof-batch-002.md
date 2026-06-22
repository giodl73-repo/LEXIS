# Tier 2 Proof Batch 002

Review date: 2026-06-06

Scope: second 10 Tier 2 candidate chains from
`source-review-priority.md`.

Tier 2 chains remain candidate review material. This packet records source
pointers and bridge corrections; it does not promote generated fixtures.

## Batch Result

- chains reviewed: 10
- English etymology pointer found: 10
- Latin root/bridge relationship supported by English etymology pointer: 10
- bridge, compound, or semantic-context correction required: 10
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 10

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 027 | stare -> statum -> state | https://www.etymonline.com/word/state | Supports Old French `estat`, Latin `status`, and `stare`. | Partial: Old French stage is supported; generated bridge should use `status`. |
| 029 | tenere -> tentum -> tenant | https://www.merriam-webster.com/dictionary/tenant and https://en.wiktionary.org/wiki/tenant | Supports Middle English/Anglo-French/French `tenant`, from French `tenir`, ultimately Latin `tenere`; Wiktionary records Old French present participle route. | Hold: generated bridge should use `tenant/tenens` present-participle path, not `tentum`. |
| 032 | volvere -> volutum -> volume | https://www.etymonline.com/word/volume and https://www.merriam-webster.com/wordplay/parts-of-a-book-terms-and-meanings/volume | Supports Old French/French `volume`, Latin `volumen`, and `volvere`. | Hold: generated bridge should use `volumen`, not `volutum`; semantic drift from scroll/book to quantity must be modeled. |
| 033 | currere -> cursum -> course | https://www.etymonline.com/word/course | Supports Old French `cors`, Latin `cursus`, and `currere`. | Partial: Old French stage is supported; generated bridge should use `cursus`. |
| 034 | pellere -> pulsum -> pulse | https://www.etymonline.com/word/pulse and https://www.merriam-webster.com/dictionary/pulses | Supports Old French `pous`/`pulse`, Latin `pulsus`, and `pellere`; Merriam-Webster flags uncertainty in the deeper etymology of `pellere`. | Hold: bridge is plausible but needs uncertainty note for deeper root posture. |
| 035 | tendere -> tensum -> tension | https://www.etymonline.com/word/tension | Supports French `tension`, Latin `tensionem` / `tensio`, `tensus`, and `tendere`. | Partial: French stage is supported; generated bridge should use `tensio/tensionem`. |
| 037 | venire -> ventum -> advent | https://www.etymonline.com/word/advent | Supports Latin `adventus`, `advenire`, and `venire`. | Hold: model compound `ad-` + `venire`; generated bridge should use `adventus`. |
| 038 | vincere -> victum -> victory | https://www.etymonline.com/word/victory | Supports Anglo-French/Old French `victorie`, Latin `victoria`, and `vincere`. | Partial: Old French/Anglo-French stage is supported; generated bridge should use `victoria`. |
| 040 | sedere -> sessum -> session | https://www.etymonline.com/word/session | Supports Old French `session`, Latin `sessionem` / `sessio`, and `sedere`. | Partial: Old French stage is supported; generated bridge should use `sessio/sessionem`. |
| 043 | trahere -> tractum -> tractor | https://www.etymonline.com/word/tractor | Supports Modern Latin `tractor`, agent noun from the past-participle stem of `trahere`. | Hold: model agent-noun and modern technical route; not a normal medieval borrowing chain. |

## Requirement Findings

1. All ten chains have a source pointer, but all ten require graph-shape
   correction before acceptance.
2. This batch adds several recurring correction patterns:
   - present participle path: `tenant`
   - derived noun bridge: `state`, `volume`, `course`, `tension`, `victory`,
     `session`
   - compound path: `advent`
   - modern technical agent noun: `tractor`
3. `pulse` needs explicit uncertainty posture for the deeper `pellere` root
   even though the Latin-to-English path is source-supported.

## Next Actions

1. Add source-custody decisions for the 10 pointers in this proof batch.
2. Generate corrected Tier 2 seed variants using `status`, `tenens/tenant`,
   `volumen`, `cursus`, `tensio`, `adventus`, `victoria`, `sessio`, and
   `tractor` as appropriate.
3. Add graph edge labels for present-participle, compound, agent-noun, and
   semantic-drift cases before attempting fixture promotion.
