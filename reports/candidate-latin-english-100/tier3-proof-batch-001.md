# Tier 3 Proof Batch 001

Review date: 2026-06-06

Scope: all 15 Tier 3 candidate chains from `source-review-priority.md`.

This packet completes source-pointer proof coverage for the generated
Latin-English 100. Tier 3 deliberately includes replacement and ambiguity
cases. A pointer here proves that a reviewable etymology trail exists; it does
not approve the generated fixture path for graph promotion.

## Batch Result

- chains reviewed: 15
- English etymology pointer found: 15
- Latin root or corrected bridge relationship supported by English etymology pointer: 15
- replacement, ambiguity, semantic-drift, or route correction required: 15
- ready for accepted graph promotion: 0
- total generated 100-chain proof-pointer coverage: 100 of 100

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 025 | spectare -> spectator -> spectator | https://www.etymonline.com/word/spectator | Supports Latin `spectator` as an agent noun from the past-participle stem of `spectare`; related `spectacle` gives a stronger Old French route. | Hold: keep as direct learned/agent-noun study or replace target with `spectacle`/`inspect` for richer graph value. |
| 068 | gaudere -> gaudium -> joy | https://www.etymonline.com/word/joy | Supports Middle English `joy`, Old French `joie`, Latin plural `gaudia`, singular `gaudium`, and `gaudere`. | Partial: path is plausible, but bridge should model `gaudia/gaudium` and Old French `joie`; semantic development should be explicit. |
| 072 | ire -> itum -> itinerary | https://www.etymonline.com/word/itinerary | Supports Late Latin `itinerarium`, Latin `itineris/iter`, and `ire`; current `itum` bridge is not the preferred route. | Hold: replace bridge with `iter/itinerarium` and label journey/account semantics. |
| 074 | laudare -> laudatum -> laud | https://www.etymonline.com/word/laud | Supports Old French `lauder`, Latin `laudare`, and `laus/laudis`. | Partial: usable but lower value; bridge should use `laudare/laus` rather than `laudatum`. |
| 075 | linquere -> lictum -> relic | https://www.etymonline.com/word/relic | Supports Old French `relique`, Late Latin `reliquiae`, `reliquus`, and relation to `relinquere` from `re-` plus `linquere`. | Hold: replace generated bridge with `relinquere/reliquus/reliquiae`; current `lictum` is not promotion-ready. |
| 081 | nocere -> nocitum -> noxious | https://www.etymonline.com/word/noxious | Supports Latin `noxius`, `noxa`, and relation to `nocere`. | Hold: replace bridge with `noxius/noxa`; model harm/death-family uncertainty. |
| 082 | novare -> novatum -> novel | https://www.etymonline.com/word/novel | Supports Old French `novel/nouvel`, Latin `novellus`, and `novus`; route is not primarily from `novare`. | Hold: replace base/bridge with `novus -> novellus`; keep `novare` only as related-family context if needed. |
| 085 | placere -> placitum -> placid | https://www.etymonline.com/word/placid | Supports French `placide`, Latin `placidus`, and `placere`. | Hold: replace bridge with `placidus`; distinguish pleasing/peaceful semantic route from `placitum`. |
| 090 | salire -> saltum -> salient | https://www.etymonline.com/word/salient | Supports Latin `salientem/saliens`, present participle of `salire`, and semantic shift from leaping to projecting/prominent. | Hold: replace bridge with `saliens/salientem`; model semantic drift. |
| 091 | sanare -> sanatum -> sanitary | https://www.etymonline.com/word/sanitary | Supports French `sanitaire`, Latin `sanitas`, and `sanus`; source does not route through `sanare/sanatum`. | Hold: replace base/bridge with `sanus -> sanitas` or record `sanare` as related-family context. |
| 093 | servire -> servitum -> serve | https://www.etymonline.com/word/serve | Supports Old French `servir`, Latin `servire`, and relation to `servus`, with deeper-origin uncertainty. | Partial: generated root is usable, but route should model `servir/servire/servus` and uncertainty; `servitum` is not the main bridge. |
| 094 | sonare -> sonitum -> sound | https://www.etymonline.com/word/sound | Supports `sound` noun/verb from Old French `son/soner`, Latin `sonus/sonare`, while the adjective and water-channel senses have separate Germanic/Norse paths. | Hold: split homonym routes before promotion; only the acoustic noun/verb belongs to `sonare`. |
| 095 | sperare -> speratum -> despair | https://www.etymonline.com/word/despair | Supports Old French `desperer`, Latin `desperare`, `de-` plus `sperare`, and `spes`. | Hold: model compound/negative prefix route; generated bridge should use `desperare`. |
| 098 | timere -> timorem -> timid | https://www.etymonline.com/word/timid | Supports French `timide`, Latin `timidus`, and `timere`, with uncertain deeper origin. | Hold: replace bridge with `timidus`; retain deeper-origin uncertainty. |
| 099 | urgere -> ursum -> urgent | https://www.etymonline.com/word/urgent | Supports Old French `urgent`, Latin `urgentem/urgens`, present participle of `urgere`, with uncertain PIE alternatives. | Hold: replace bridge with `urgens/urgentem`; current `ursum` is not promotion-ready. |

## Requirement Findings

1. The 100-chain generated corpus now has complete source-pointer review
   coverage.
2. Tier 3 proves the pipeline needs first-class correction outcomes:
   `replace_bridge`, `replace_base`, `split_homonym_route`, `compound_route`,
   and `low_value_but_supported`.
3. The next real system milestone is not more candidate creation; it is
   corrected fixture generation from these proof decisions, starting with the
   high-value Tier 1 chains and selected Tier 3 replacement cases.

## Next Actions

1. Add source-custody decisions for these 15 proof pointers.
2. Add a correction-plan artifact that maps generated chain IDs to replacement
   bases, bridges, intermediate languages, and graph edge decisions.
3. Generate a corrected accepted-candidate subset instead of promoting the
   original placeholder fixtures.
