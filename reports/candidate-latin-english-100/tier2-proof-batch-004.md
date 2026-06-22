# Tier 2 Proof Batch 004

Review date: 2026-06-06

Scope: fourth 10 Tier 2 candidate chains from
`source-review-priority.md`.

This packet records source pointers and graph-shape corrections. It does not
promote generated fixtures because every chain still needs bridge replacement,
compound modeling, or uncertainty labeling before acceptance.

## Batch Result

- chains reviewed: 10
- English etymology pointer found: 10
- Latin root/bridge relationship supported by English etymology pointer: 10
- bridge, compound, or uncertainty correction required: 10
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 10

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 058 | dolere -> dolorem -> dolor | https://www.merriam-webster.com/dictionary/dolor | Supports Middle English/Anglo-French `dolour`, Latin `dolor`, and `dolere`. | Hold: model `dolor` noun path and keep target as lower-priority English learned/medical word. |
| 059 | eximere -> exemptum -> exempt | https://www.etymonline.com/word/exempt and https://www.merriam-webster.com/dictionary/exempt | Supports Old French/Anglo-French `exempt`, Latin `exemptus`, and `eximere`. | Hold: model compound `ex-` + `emere/eximere`; generated path should not hide the compound. |
| 060 | errare -> erratum -> error | https://www.etymonline.com/word/error and https://www.merriam-webster.com/dictionary/err | Supports Old French `errer`, Latin `errare`, and English `error`; `erratum` is a related neuter participial form. | Hold: generated bridge `erratum` may be useful for `errata`, but `error` route should model `errare/error` directly. |
| 061 | esse -> essentia -> essence | https://www.etymonline.com/word/essence | Supports Latin `essentia`, formed from the present-participle stem of `esse`, with philosophical Greek-translation context. | Hold: model philosophical coinage/translation context, not a simple lexical descent edge. |
| 062 | fallere -> falsum -> false | https://www.etymonline.com/word/false | Supports Old French `fals/faus`, Latin `falsus`, and `fallere`, with uncertain deeper origin. | Partial: Old French stage is supported; retain uncertainty for deeper root posture. |
| 063 | fateri -> fassum -> confess | https://www.etymonline.com/word/confession and https://www.merriam-webster.com/dictionary/confess | Supports Old French/Anglo-French `confesser`, Latin `confiteri`, and `fateri`. | Hold: model compound `con-` + `fateri`; generated bridge `fassum` is too flat. |
| 064 | flectere -> flexum -> flex | https://www.etymonline.com/word/flex | Supports Latin `flex-` from the past-participle stem of `flectere`, with uncertain deeper origin. | Hold: generated bridge is plausible, but modern English `flex` needs route and uncertainty decisions. |
| 065 | fluere -> fluxum -> flux | https://www.etymonline.com/word/flux | Supports Old French `flus` or direct Latin `fluxus`, from `fluere`. | Partial: Old French/direct Latin route is supported; generated bridge should use `fluxus`. |
| 066 | frangere -> fractum -> fracture | https://www.etymonline.com/word/fracture and https://www.merriam-webster.com/dictionary/fracture | Supports Old French `fracture`, Latin `fractura`, `fractus`, and `frangere`. | Partial: Old French stage is supported; generated bridge should use `fractura`. |
| 067 | fundere -> fusum -> fusion | https://www.etymonline.com/word/fusion | Supports French `fusion` or direct Latin `fusionem` / `fusio`, `fusus`, and `fundere`. | Partial: French/direct Latin stage is supported; generated bridge should use `fusio/fusionem`. |

## Requirement Findings

1. `exempt` and `confess` need compound-edge modeling.
2. `essence` needs a philosophical coinage/translation context label.
3. `false` and `flex` need uncertainty labels for deeper origin posture.
4. Derived noun bridges recur: `dolor`, `fluxus`, `fractura`, and
   `fusio/fusionem`.

## Next Actions

1. Add source-custody decisions for these 10 proof pointers.
2. Generate corrected Tier 2 seed variants with `dolor`, `exemptus`,
   `error/errare`, `essentia`, `falsus`, `confiteri/confessus`, `fluxus`,
   `fractura`, and `fusio/fusionem`.
3. Add validator-visible labels for compound formation, philosophical coinage,
   and uncertain deeper origin before attempting promotion.
