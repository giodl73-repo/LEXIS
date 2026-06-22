# Tier 1 Proof Batch 002

Review date: 2026-06-06

Scope: next 10 Tier 1 candidate chains from
`source-review-priority.md`, after `tier1-proof-batch-001.md`.

This packet records source pointers and claim posture for high-priority
Latin-to-English chains. It does not promote generated fixtures to
`accepted_for_slice`, because generated Old French and Middle English stages
remain placeholders until source review resolves actual intermediate forms.

## Batch Result

- chains reviewed: 10
- English etymology pointer found: 10
- Latin root/bridge relationship supported by English etymology pointer: 10
- Old French, Anglo-French, or direct French stage identified: 9 partial
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 10

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 018 | ponere -> positum -> position | https://www.etymonline.com/word/position | Supports Old French `posicion`, Latin `positionem` / `positio`, and `ponere`. | Hold: model `positio/positionem` bridge and decide whether Middle English is attested or omitted. |
| 020 | premere -> pressum -> pressure | https://www.etymonline.com/word/pressure | Supports Old French `presseure`, Latin `pressura`, `pressus`, and `premere`. | Partial: Old French stage is supported; generated bridge should use `pressura` for the English noun path. |
| 022 | rumpere -> ruptum -> rupture | https://www.etymonline.com/word/rupture | Supports Old French `rupture`, Latin `ruptura`, and the past-participle stem of `rumpere`. | Partial: Old French stage is supported; generated bridge should use `ruptura` for the English noun path. |
| 023 | scribere -> scriptum -> script | https://www.etymonline.com/word/script | Supports Anglo-French/Old French `scrit`/`escrit`, Latin `scriptum`, and `scribere`. | Partial: aligns with accepted scribere work, but generated batch fixture still needs per-chain source decisions. |
| 024 | sentire -> sensum -> sense | https://www.etymonline.com/word/sense | Supports Old French `sens`, Latin `sensus`, and `sentire`; also flags possible Germanic interaction. | Hold: model mixed-source caution and use `sensus` rather than only `sensum` where appropriate. |
| 028 | tangere -> tactum -> contact | https://www.etymonline.com/word/contact | Supports Latin `contactus` from `contingere`, built from `con-` plus `tangere`. | Hold: model compound path `contingere/contactus`, not simple `tactum` path. |
| 030 | videre -> visum -> vision | https://www.etymonline.com/word/vision | Supports Anglo-French/Old French `vision`, Latin `visionem` / `visio`, and `videre`. | Partial: Old French/Anglo-French stage is supported; generated bridge should use `visio/visionem`. |
| 031 | vocare -> vocatum -> vocation | https://www.etymonline.com/word/vocation | Supports Old French `vocacion`, Latin `vocationem` / `vocatio`, `vocatus`, and `vocare`. | Partial: Old French stage is supported; generated bridge should use `vocatio/vocationem`. |
| 036 | vertere -> versum -> version | https://www.etymonline.com/word/version | Supports French `version`, Medieval Latin `versionem` / `versio`, and Latin `vertere`. | Partial: French stage is supported; generated bridge should use `versio/versionem`. |
| 039 | secare -> sectum -> section | https://www.etymonline.com/word/section | Supports Old French `section`, Latin `sectionem` / `sectio`, and `secare`. | Partial: Old French stage is supported; generated bridge should use `sectio/sectionem`. |

## Requirement Findings

1. The second proof batch confirms that noun-path bridges are usually more
   precise than raw supine or participial bridge forms.
2. Several chains include a compound route that must be modeled explicitly:
   - `contact`: `contingere/contactus`, not a direct `tactum` path.
3. Several chains identify French or Old French intermediates from the same
   pointer, so generated placeholders can be replaced by reviewed named forms.
4. `sense` needs mixed-source caution because the source pointer records both
   Latin/French and possible Germanic contribution.

## Next Actions

1. Add source-custody decisions for the 10 English etymology pointers in this
   proof batch.
2. Generate corrected seed variants for these 10 chains using precise noun or
   compound bridge forms.
3. Keep every generated fixture blocked until source records are linked per
   chain and placeholder language stages are either replaced or removed.
