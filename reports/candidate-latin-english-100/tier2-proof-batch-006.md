# Tier 2 Proof Batch 006

Review date: 2026-06-06

Scope: final 9 Tier 2 candidate chains from
`source-review-priority.md`.

This packet completes Tier 2 source-pointer proof coverage. It does not promote
generated fixtures because each chain still needs bridge replacement, compound
modeling, semantic-drift labeling, or uncertainty posture before acceptance.

## Batch Result

- chains reviewed: 9
- English etymology pointer found: 9
- Latin root/bridge relationship supported by English etymology pointer: 9
- bridge, compound, semantic-context, or uncertainty correction required: 9
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 9

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 084 | pati -> passum -> passion | https://www.etymonline.com/word/passion | Supports Old French `passion`, Late Latin `passionem` / `passio`, and Latin `pati`, with uncertain deeper origin and religious/emotional semantic development. | Partial: Old French stage is supported; generated bridge should use `passio/passionem`; semantic shift must be labeled. |
| 086 | plicare -> plicatum -> complicate | https://www.etymonline.com/word/complicate | Supports Latin `complicatus`, `complicare`, and `plicare`. | Hold: model compound `com-` + `plicare`; generated bridge should use `complicatus/complicare`. |
| 087 | quaerere -> quaesitum -> question | https://www.etymonline.com/es/word/question | Supports Latin `quaestionem` / `quaestio` from `quaerere`; Old French route still needs direct English pointer confirmation. | Hold: generated bridge should use `quaestio/quaestionem`; select stronger direct English pointer before promotion. |
| 088 | rapere -> raptum -> rapture | https://www.etymonline.com/word/rapture | Supports Medieval Latin `raptura`, Latin `raptus`, and `rapere`, with semantic development from seizure/carrying off to ecstatic transport. | Hold: model `raptura/raptus` and meaning shift explicitly. |
| 089 | ridere -> risum -> risible | https://www.etymonline.com/word/risible | Supports French `risible`, Late Latin `risibilis`, Latin `risus`, and `ridere`, with no good PIE etymology per source. | Hold: generated bridge should use `risibilis`; retain deeper-origin uncertainty. |
| 092 | scire -> scientia -> science | https://www.etymonline.com/word/science | Supports Old French `science`, Latin `scientia`, `sciens`, and `scire`. | Partial: Old French stage is supported; generated bridge is plausible but semantic scope from knowledge to modern science must be labeled. |
| 096 | stringere -> strictum -> strict | https://www.etymonline.com/word/strict | Supports Latin `strictus`, past participle of `stringere`, and Old French/Anglo-French narrow/tight route. | Partial: Romance stage is supported; semantic shift from tight/narrow to exacting must be labeled. |
| 097 | sumere -> sumptum -> assume | https://www.etymonline.com/word/assume | Supports Latin `assumere/adsumere` from `ad-` plus `sumere`. | Hold: model compound `ad-` + `sumere`; generated bridge should use `assumere`, not only `sumptum`. |
| 100 | uti -> usum -> use | https://www.etymonline.com/word/use and https://www.merriam-webster.com/dictionary/use | Supports Middle English/Anglo-French `use`, Latin `usus`, and `uti`; verb route also involves Old French `user` and Vulgar Latin `usare`. | Hold: model noun and verb routes separately; generated bridge should use `usus` and/or `usare` with route distinction. |

## Requirement Findings

1. Tier 2 source-pointer proof coverage is complete.
2. This batch adds several important graph patterns:
   - religious/emotional semantic development: `passion`
   - compound formation: `complicate`, `assume`
   - noun/verb route split: `use`
   - modern domain narrowing: `science`
   - deeper-origin uncertainty: `passion`, `risible`
3. `question` needs a stronger direct English etymology pointer before any
   accepted fixture promotion.

## Next Actions

1. Add source-custody decisions for these 9 proof pointers.
2. Generate corrected Tier 2 seed variants with `passio`, `complicatus`,
   `quaestio`, `raptura`, `risibilis`, `scientia`, `strictus`, `assumere`, and
   `usus/usare`.
3. Move to Tier 3 proof packets and treat replacement/alternate-chain decisions
   as first-class outcomes.
