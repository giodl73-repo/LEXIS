# Tier 2 Proof Batch 003

Review date: 2026-06-06

Scope: third 10 Tier 2 candidate chains from
`source-review-priority.md`.

This packet records source pointers and graph-shape corrections. It does not
promote generated fixtures because every chain still needs bridge replacement,
compound modeling, or semantic caution before acceptance.

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
| 044 | valere -> validum -> valid | https://www.etymonline.com/word/valid | Supports French `valide`, Latin `validus`, and `valere`. | Partial: French stage is supported; generated bridge should use `validus`. |
| 045 | velle -> volitio -> volition | https://www.etymonline.com/word/volition | Supports French `volition`, Medieval Latin `volitionem` / `volitio`, and Latin `velle` / `volo`. | Partial: French stage is supported; bridge correction is already plausible but needs Medieval Latin marking. |
| 047 | docere -> doctum -> doctor | https://www.etymonline.com/word/doctor | Supports Old French `doctour`, Medieval/classical Latin `doctor`, and `docere`. | Partial: Old French stage is supported; generated bridge should use agent noun `doctor`, not only `doctum`. |
| 049 | cavere -> cautum -> caution | https://www.etymonline.com/word/caution | Supports Old French `caution`, Latin `cautionem` / `cautio`, and `cavere`. | Partial: Old French stage is supported; generated bridge should use `cautio/cautionem`. |
| 050 | celare -> celatum -> conceal | https://www.etymonline.com/word/conceal and https://www.merriam-webster.com/dictionary/conceal | Supports Old French/Anglo-French `conceler`, Latin `concelare`, and `celare`. | Hold: model compound/intensive `con-` path rather than direct simple `celatum` path. |
| 051 | clamare -> clamatum -> claim | https://www.etymonline.com/word/claim | Supports Old French `clamer`, Latin `clamare`, and semantic route from calling out to legal demand. | Hold: generated bridge `clamatum` is likely unnecessary; model Old French verbal borrowing and semantic shift. |
| 053 | colere -> cultum -> culture | https://www.etymonline.com/word/culture | Supports Latin `cultura` from the past-participle stem of `colere`. | Hold: generated bridge should use `cultura`, and semantic drift from cultivation to refinement/social practice must be modeled. |
| 054 | componere -> compositum -> compose | https://www.etymonline.com/word/compose and https://www.etymonline.com/word/composition | Supports Old French/French influence, Latin `componere`, `compositus`, and `compositio`. | Hold: model `componere/compositus` with French `composer` influence; generated direct path is too flat. |
| 055 | condicere -> condicio -> condition | https://www.etymonline.com/word/condition and https://www.merriam-webster.com/dictionary/condition | Supports Old French/Anglo-French `condicion`, Latin `condicio/conditionem`, and `condicere`. | Partial: Old French/Anglo-French stage is supported; compound `con-` + `dicere` route should be explicit. |
| 056 | cupere -> cupitum -> cupidity | https://www.etymonline.com/word/cupidity and https://www.merriam-webster.com/dictionary/cupidity | Supports French or Latin `cupiditas`, `cupidus`, and `cupere`. | Hold: generated bridge should use `cupiditas/cupidus`, not `cupitum`; semantic intensity/greed needs a meaning-shift label. |

## Requirement Findings

1. Every chain in this batch needs a corrected bridge before graph promotion.
2. `conceal`, `compose`, and `condition` need explicit compound/prefix modeling.
3. `claim`, `culture`, and `cupidity` need semantic-shift labels, not just
   form lineage.
4. Agent noun and derived noun bridges recur again: `doctor`, `cautio`,
   `cultura`, `cupiditas`.

## Next Actions

1. Add source-custody decisions for these 10 proof pointers.
2. Generate corrected Tier 2 seed variants with `validus`, `doctor`, `cautio`,
   `concelare`, `cultura`, `compositio/compositus`, `condicio`, and
   `cupiditas`.
3. Add validator-visible edge labels for compound formation and meaning-shift
   cases before attempting promotion.
