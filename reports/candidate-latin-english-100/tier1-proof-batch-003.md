# Tier 1 Proof Batch 003

Review date: 2026-06-06

Scope: remaining Tier 1 candidate chains from
`source-review-priority.md`, after `tier1-proof-batch-001.md` and
`tier1-proof-batch-002.md`.

This packet completes Tier 1 source-pointer proof coverage. It does not promote
generated fixtures to `accepted_for_slice`, because generated Old French and
Middle English stages remain placeholders until source review resolves actual
intermediate forms.

## Batch Result

- chains reviewed: 6
- English etymology pointer found: 6
- Latin root/bridge relationship supported by English etymology pointer: 6
- Old French, French, or direct Latin stage identified: 5 partial
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 6

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 041 | solvere -> solutum -> solution | https://www.etymonline.com/word/solution | Supports Old French `solucion`, Latin `solutionem` / `solutio`, and `solvere`. | Partial: Old French stage is supported; generated bridge should use `solutio/solutionem`. |
| 042 | struere -> structum -> structure | https://www.etymonline.com/word/structure | Supports Latin `structura`, `structus`, and `struere`. | Hold: model `structura` bridge and decide whether an intermediate French stage is attested or absent. |
| 046 | dicere -> dictum -> dictate | https://www.etymonline.com/word/dictate and https://www.merriam-webster.com/dictionary/dictate | Supports Latin `dictare`, `dictatus`, and `dicere`; Merriam-Webster records Medieval Latin `dictatum`. | Hold: model frequentative/Medieval Latin route, not only `dictum`. |
| 048 | dare -> datum -> data | https://www.etymonline.com/word/datum | Supports Latin `datum`, neuter past participle of `dare`; English `data` is the classical plural. | Hold: model plural/singular relation explicitly and avoid treating `data` as a simple inherited singular. |
| 052 | claudere -> clausum -> clause | https://www.etymonline.com/word/clause | Supports Old French `clause`, Medieval Latin `clausa`, classical Latin `clausula`, and `claudere`. | Partial: Old French stage is supported; generated bridge should use `clausa/clausula`. |
| 057 | delere -> deletum -> delete | https://www.etymonline.com/word/delete and https://www.merriam-webster.com/dictionary/delete | Supports Latin `deletus` from `delere`; Merriam-Webster records the same Latin past participle route. | Hold: source route into English appears direct Latin; placeholder French/Middle English stages should likely be removed unless separately attested. |

## Requirement Findings

1. Tier 1 is source-pointer proven, but still not fixture-accepted.
2. The final Tier 1 group adds two important graph patterns:
   - morphological number/usage relation: `datum` -> `data`
   - frequentative/Medieval Latin route: `dicere` -> `dictare/dictatum` -> `dictate`
3. Most generated bridge forms should become noun or derived-form bridges before
   graph promotion.
4. Several chains have no clear need for both Old French and Middle English
   placeholders, so LEXIS needs explicit omitted-stage decisions.

## Next Actions

1. Add source-custody decisions for the 6 English etymology pointers in this
   proof batch.
2. Generate corrected Tier 1 seed variants with precise bridge forms.
3. Add per-chain omitted-stage decisions where the source route is direct Latin
   or otherwise does not support the generated placeholder stages.
