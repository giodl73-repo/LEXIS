# Tier 2 Proof Batch 005

Review date: 2026-06-06

Scope: fifth 10 Tier 2 candidate chains from
`source-review-priority.md`.

This packet records source pointers and graph-shape corrections. It does not
promote generated fixtures because each chain still needs bridge replacement,
compound modeling, semantic-drift labeling, or stronger source-detail review.

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
| 069 | gerere -> gestum -> gesture | https://www.etymonline.com/word/gesture | Supports Medieval Latin `gestura`, Latin `gestus`, and `gerere`. | Hold: generated bridge should use `gestura/gestus`; semantic path through body carriage and action needs labeling. |
| 070 | gradi -> gressum -> progress | https://www.etymonline.com/es/word/progress | Supports Old French `progres`, Latin `progressus`, `progredi`, and `gradi`. | Hold: model compound `pro-` + `gradi`; generated bridge should use `progressus`. |
| 071 | haerere -> haesum -> adhesion | https://www.etymonline.com/word/adhere | Supports French `adhérer`, Latin `adhaerare/adhaerere`, and `haerere`; adhesion-specific pointer still needs final selection. | Hold: model compound `ad-` + `haerere`; add adhesion noun pointer before promotion. |
| 073 | laedere -> laesum -> lesion | https://www.etymonline.com/word/lesion | Supports Old French `lesion`, Latin `laesionem` / `laesio`, and `laedere`, with unknown deeper origin. | Partial: Old French stage is supported; generated bridge should use `laesio/laesionem` and retain deeper-origin uncertainty. |
| 076 | loqui -> locutum -> locution | https://www.catholicculture.org/culture/library/dictionary/index.cfm?id=34640&randomterm=false | Supports Latin `locutio` from `loqui`; source pointer is usable but not sufficient as an accepted etymology record. | Hold: select stronger dictionary pointer and model `locutio`, not only `locutum`. |
| 077 | lucere -> lucem -> lucid | https://www.etymonline.com/word/lucid | Supports Latin `lucidus`, `lucere`, and `lux/lucis`. | Hold: generated bridge should use `lucidus`; semantic shift from shining to clear/perspicuous should be labeled. |
| 078 | manere -> mansum -> mansion | https://www.etymonline.com/word/mansion | Supports Old French `mansion`, Latin `mansionem` / `mansio`, and `manere`. | Partial: Old French stage is supported; generated bridge should use `mansio/mansionem`. |
| 079 | monere -> monitum -> monitor | https://www.etymonline.com/word/monitor and https://www.merriam-webster.com/dictionary/monitor | Supports Latin `monitor`, agent noun from `monere`; Etymonline also notes difficulty around related `Moneta`. | Hold: model agent noun `monitor`; keep deity/name material out of this chain unless separately scoped. |
| 080 | nasci -> natum -> nation | https://www.etymonline.com/word/nation | Supports Old French `nacion`, Latin `nationem` / `natio`, `natus`, and `nasci`. | Partial: Old French stage is supported; generated bridge should use `natio/nationem`; semantic shift from birth/origin to people/political community must be labeled. |
| 083 | orare -> oratum -> orator | https://www.etymonline.com/word/orator | Supports Anglo-French `oratour`, Latin `orator`, and `orare`. | Partial: Anglo-French stage is supported; generated bridge should use agent noun `orator`, not only `oratum`. |

## Requirement Findings

1. `progress` and `adhesion` need compound-edge modeling.
2. `monitor` and `orator` need agent-noun modeling.
3. `nation`, `lucid`, and `gesture` need meaning-shift labels.
4. `lesion` needs deeper-origin uncertainty retained.
5. `locution` needs a stronger source pointer before graph promotion.

## Next Actions

1. Add source-custody decisions for these 10 proof pointers.
2. Generate corrected Tier 2 seed variants with `gestura`, `progressus`,
   `adhaerere/adhesio`, `laesio`, `locutio`, `lucidus`, `mansio`, `monitor`,
   `natio`, and `orator`.
3. Add validator-visible labels for compound formation, agent nouns,
   semantic drift, and weak-source posture before attempting promotion.
