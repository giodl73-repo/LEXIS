# Tier 2 Proof Batch 001

Review date: 2026-06-06

Scope: first 10 Tier 2 candidate chains from
`source-review-priority.md`.

Tier 2 chains are reviewed after Tier 1 because they are useful but often need
compound paths, agent-noun bridges, derived noun bridges, or caution around the
generated intermediate placeholders.

## Batch Result

- chains reviewed: 10
- English etymology pointer found: 10
- Latin root/bridge relationship supported by English etymology pointer: 10
- bridge or compound correction required: 8
- ready for accepted graph promotion: 0
- ready for next source-detail pass: 10

## Reviewed Chains

| # | Chain | Source pointer | Finding | Promotion decision |
|---:|---|---|---|---|
| 002 | amare -> amator -> amateur | https://www.etymonline.com/word/amateur and https://www.merriam-webster.com/dictionary/amateur | Supports French `amateur`, Latin `amatorem` / `amator`, and `amare`. | Partial: French stage is supported; generated bridge is plausible but needs exact accusative/nominative handling. |
| 003 | audire -> auditum -> auditor | https://www.etymonline.com/word/audit | Supports Latin `auditus` and `audire`, with `auditor` in the same derivative family. | Hold: source an auditor-specific pointer or model through `audit/auditus` explicitly. |
| 009 | ferre -> transferre -> transfer | https://www.etymonline.com/word/transfer | Supports Old French `transferer` or direct Latin `transferre`, from `trans-` plus `ferre`. | Hold: model compound path and do not treat `ferre` -> `transferre` as a simple descent edge. |
| 010 | finire -> finitum -> finite | https://www.etymonline.com/word/finite | Supports Latin `finitum`, past participle of `finire`. | Hold: likely direct Latin borrowing; generated Old French and Middle English placeholders need omission or attestation decisions. |
| 011 | flos -> florem -> flower | https://www.etymonline.com/word/flower | Supports Old French `flor`, Latin `florem` / `flos`, and also flags a separate Scandinavian/Germanic blossom word. | Hold: model the Romance flower path and keep Germanic homonym/contact caution explicit. |
| 012 | habere -> habitum -> habit | https://www.etymonline.com/word/habit | Supports Old French `habit` / `abit`, Latin `habitus`, and `habere`. | Partial: Old French stage is supported; generated bridge should use `habitus` rather than only `habitum`. |
| 013 | jacere -> projectum -> project | https://www.etymonline.com/word/project | Supports Medieval Latin `proiectum`, Latin `proicere`, and `iacere`. | Hold: model compound `pro-` + `iacere`; generated chain should not imply direct simple descent from `jacere`. |
| 019 | portare -> portatum -> portable | https://www.merriam-webster.com/dictionary/portable | Supports Middle English/French `portable`, Latin `portabilis`, and `portare`. | Hold: generated bridge should use `portabilis`, not `portatum`. |
| 021 | regere -> rectum -> rector | https://www.etymonline.com/word/rector | Supports Latin `rector` from the `rect-` stem of `regere`. | Hold: model agent-noun route through `rector`, not only `rectum`. |
| 026 | spirare -> spiritum -> spirit | https://www.etymonline.com/word/spirit and https://www.merriam-webster.com/dictionary/spirit | Supports Anglo-French or Latin route, Latin `spiritus`, and `spirare`. | Hold: generated bridge should use `spiritus`; semantic/religious translation context should be retained. |

## Requirement Findings

1. Tier 2 immediately exercises graph semantics beyond simple lineage:
   compound formation, agent nouns, plural/number handling, and homonym/contact
   caution.
2. `transfer` and `project` need compound-edge modeling before acceptance.
3. `flower` needs a caution label because the English surface family includes a
   Romance path and a separate Germanic/Scandinavian blossom path.
4. `portable`, `habit`, `rector`, and `spirit` require bridge replacement with
   `portabilis`, `habitus`, `rector`, and `spiritus`.

## Next Actions

1. Add source-custody decisions for the 10 pointers in this proof batch.
2. Generate corrected Tier 2 seed variants with compound and agent-noun bridge
   forms.
3. Keep all ten blocked until the validator can distinguish compound formation
   from inheritance/descent.
