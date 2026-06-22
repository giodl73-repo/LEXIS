# Tier 1 Proof Batch 001

Review date: 2026-06-06

Scope: first 10 Tier 1 candidate chains from
`source-review-priority.md`.

This packet records source pointers and claim posture for high-priority
Latin-to-English chains. It does not yet promote generated fixtures to
`accepted_for_slice`, because the generated Old French and Middle English stages
are placeholders until source review resolves actual intermediate forms.

## Batch Result

- chains reviewed: 10
- English etymology pointer found: 10
- Latin root/bridge relationship supported by English etymology pointer: 10
- Old French or Middle English placeholder resolved to exact forms: 2 partial
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 10

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 001 | agere -> actum -> act | https://www.etymonline.com/word/act | Supports Latin `actus` / `actum` from `agere` as the English `act` source family. | Hold: replace placeholder intermediate stages or mark absent/direct Latin borrowing. |
| 004 | capere -> captum -> capture | https://www.etymonline.com/word/capture | Supports French `capture`, Latin `captura`, `captus`, and `capere`. | Hold: source the French stage and decide if Middle English is applicable. |
| 005 | cedere -> cessum -> cession | https://www.etymonline.com/word/cession | Supports Old French `cession`, Latin `cessionem` / `cessio`, and `cedere`. | Partial: Old French stage is supported; Middle English still needs exact attestation or omission. |
| 006 | credere -> creditum -> credit | https://www.etymonline.com/word/credit | Supports French/Italian route, Latin `creditum`, and `credere`. | Hold: source exact French/Italian intermediate handling before promotion. |
| 007 | ducere -> ductum -> conduct | https://www.etymonline.com/word/conduct and https://www.merriam-webster.com/dictionary/conduct | Supports Medieval Latin `conductus` and Latin `conducere` / `ducere`; Merriam-Webster records Middle English and Anglo-French details. | Partial: Middle English/Anglo-French forms need modeled replacement instead of generic placeholders. |
| 008 | facere -> factum -> factor | https://www.etymonline.com/word/factor | Supports French/Old French `factor`/`faitor`, Latin `factor`, and `facere`. | Hold: model agent-noun path, not only `factum`. |
| 014 | jungere -> junctum -> junction | https://www.merriam-webster.com/word-of-the-day/juncture-2020-09-21 | Supports the `jungere` family behind `junction`/`juncture` class words. | Hold: source a direct `junction` entry and exact `junctio` bridge before promotion. |
| 015 | legere -> lectum -> lecture | https://www.etymonline.com/word/lecture | Supports Medieval Latin `lectura`, Latin `lectus`, and `legere`. | Hold: model `lectura` bridge and source intermediate stages. |
| 016 | mittere -> missum -> mission | https://www.etymonline.com/word/mission | Supports Latin `missionem` / `missio` from the past-participle stem of `mittere`. | Hold: model `missio/missionem` bridge and source route into English. |
| 017 | movere -> motum -> motion | https://www.etymonline.com/word/motion | Supports Old French `mocion`, Latin `motionem` / `motio`, and `movere`. | Partial: Old French stage is supported; Middle English still needs exact attestation or omission. |

## Requirement Findings

1. The generated chain model is right to include intermediate stages, but the
   intermediate stages must not be generic placeholders at promotion time.
2. Some chains should use a more precise bridge node than the generated Latin
   bridge:
   - `factor`: likely `factor`, not only `factum`.
   - `lecture`: `lectura`, not only `lectum`.
   - `mission`: `missio/missionem`, not only `missum`.
   - `motion`: `motio/motionem`, not only `motum`.
3. Some chains have a direct scholarly pointer to Old French or Anglo-French
   stages (`cession`, `conduct`, `motion`), so LEXIS should support replacing
   generated placeholder stages with named attested/intermediate forms.
4. Promotion needs a per-chain source packet, not a shared batch source.

## Next Actions

1. Add source-custody decisions for the 10 English etymology pointers in this
   proof batch.
2. Generate corrected seed variants for these 10 chains with precise bridge
   forms where the pointer already identifies them.
3. Keep generated Old French/Middle English placeholders as `source_limited`
   until each chain has a source-supported intermediate decision.
