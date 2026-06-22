# Source Review Priority - Candidate Latin-English 100

Review date: 2026-06-06

This review sorts the generated 100 multi-stage chains into source-lookup
priority. It does not accept source custody or relationship claims. The goal is
to decide which slices should be researched first and what each review must
prove before promotion.

## Current Corpus Shape

- candidate chains: 100
- stages per chain: 5
- total generated nodes: 1100
- total generated edges: 1100
- current validation: 0 valid, 100 invalid by design

Every chain currently has:

```text
Latin base -> Latin bridge -> Old French candidate -> Middle English candidate -> Modern English
```

The Old French and Middle English stages are placeholders. A source review must
replace each placeholder with attested or source-limited historical forms before
the slice can be accepted.

## Promotion Rule

A candidate chain can move from `candidate_review` to `accepted_for_slice` only
after review records:

1. a source pointer for the Latin base/bridge,
2. a source pointer for the English derivative,
3. whether Old French and Middle English stages are attested, inferred, absent,
   or replaced by another intermediate language,
4. edge decisions for each transition,
5. uncertainty labels for every stage and edge.

## Tier 1 - Review First

These are high-value, high-plausibility chains that should become the first
accepted multi-stage corpus candidates.

| # | Chain | Why first |
|---:|---|---|
| 001 | agere -> actum -> act | Clean act-/actus family; good baseline for root/bridge/English derivative. |
| 004 | capere -> captum -> capture | Strong capture/capt- family and common derivative cluster. |
| 005 | cedere -> cessum -> cession | Good ced-/cess- alternation case. |
| 006 | credere -> creditum -> credit | Strong credit-/cred- family with clear graph value. |
| 007 | ducere -> ductum -> conduct | Strong duct/conduct chain with compounds. |
| 008 | facere -> factum -> factor | Strong fac-/fact- family. |
| 014 | jungere -> junctum -> junction | Clean junct- family. |
| 015 | legere -> lectum -> lecture | Useful leg-/lect- semantic and form contrast. |
| 016 | mittere -> missum -> mission | Strong miss-/mission family. |
| 017 | movere -> motum -> motion | Strong mot-/motion family. |
| 018 | ponere -> positum -> position | Strong posit-/position family. |
| 020 | premere -> pressum -> pressure | Strong press-/pressure family. |
| 022 | rumpere -> ruptum -> rupture | Strong rupt- family. |
| 023 | scribere -> scriptum -> script | Already aligns with accepted first-slice work. |
| 024 | sentire -> sensum -> sense | Strong sens-/sense family. |
| 028 | tangere -> tactum -> contact | Strong tact-/contact family. |
| 030 | videre -> visum -> vision | Strong vis-/vision family. |
| 031 | vocare -> vocatum -> vocation | Strong voc-/vocation family. |
| 036 | vertere -> versum -> version | Strong vers-/version family. |
| 039 | secare -> sectum -> section | Strong sect-/section family. |
| 041 | solvere -> solutum -> solution | Strong solut-/solution family. |
| 042 | struere -> structum -> structure | Strong struct-/structure family. |
| 046 | dicere -> dictum -> dictate | Strong dict-/dictate family. |
| 048 | dare -> datum -> data | Strong datum/data family. |
| 052 | claudere -> clausum -> clause | Strong claus-/clause family. |
| 057 | delere -> deletum -> delete | Strong delet-/delete family. |

Tier 1 target: promote 25 chains after source lookup. This would create a first
usable multi-stage accepted corpus.

## Tier 2 - Review After Tier 1

These are likely useful but need more careful intermediate-path or bridge
handling.

| # | Chain | Review concern |
|---:|---|---|
| 002 | amare -> amator -> amateur | Verify French intermediary and semantic route. |
| 003 | audire -> auditum -> auditor | English form may route through Latin agent noun directly. |
| 009 | ferre -> transferre -> transfer | Compound path, not simple bridge. |
| 010 | finire -> finitum -> finite | Verify finite route and bridge form. |
| 011 | flos -> florem -> flower | Likely Romance/Germanic contact complexity; verify exact path. |
| 012 | habere -> habitum -> habit | Needs habitus/habit route review. |
| 013 | jacere -> projectum -> project | Compound path; useful but needs careful edge labels. |
| 019 | portare -> portatum -> portable | Verify portable through portabilis/portable. |
| 021 | regere -> rectum -> rector | Agent noun route needs review. |
| 026 | spirare -> spiritum -> spirit | Spiritus path; strong but bridge needs exact form. |
| 027 | stare -> statum -> state | Status/state route needs review. |
| 029 | tenere -> tentum -> tenant | Tenant route likely through tenens/tenant; bridge correction may be needed. |
| 032 | volvere -> volutum -> volume | Volume likely through volumen; bridge correction likely. |
| 033 | currere -> cursum -> course | Course/cursus route useful; verify Romance stage. |
| 034 | pellere -> pulsum -> pulse | Pulse path useful; verify exact semantic route. |
| 035 | tendere -> tensum -> tension | Strong but route through tensio needs review. |
| 037 | venire -> ventum -> advent | Compound adventus path. |
| 038 | vincere -> victum -> victory | Victoria path likely needs bridge correction. |
| 040 | sedere -> sessum -> session | Sessio/session route likely; verify bridge. |
| 043 | trahere -> tractum -> tractor | Tractor route is agentive and later technical. |
| 044 | valere -> validum -> valid | Validus route needs bridge exactness. |
| 045 | velle -> volitio -> volition | Corrected but still source-sensitive. |
| 047 | docere -> doctum -> doctor | Doctor agent noun route. |
| 049 | cavere -> cautum -> caution | Cautio/caution route likely. |
| 050 | celare -> celatum -> conceal | Prefix path requires compound review. |
| 051 | clamare -> clamatum -> claim | Claim route through Old French needs careful verification. |
| 053 | colere -> cultum -> culture | Cultura/culture route likely needs bridge change. |
| 054 | componere -> compositum -> compose | Compose route through componere/compositus and French. |
| 055 | condicere -> condicio -> condition | Corrected; condition path needs source confirmation. |
| 056 | cupere -> cupitum -> cupidity | Cupiditas route likely needs bridge correction. |
| 058 | dolere -> dolorem -> dolor | Strong but not a common modern English everyday chain. |
| 059 | eximere -> exemptum -> exempt | Corrected compound path. |
| 060 | errare -> erratum -> error | Error route likely strong; verify bridge. |
| 061 | esse -> essentia -> essence | Philosophical/abstract route; source-sensitive. |
| 062 | fallere -> falsum -> false | Falsus/false route likely. |
| 063 | fateri -> fassum -> confess | Compound confiteri/confess path. |
| 064 | flectere -> flexum -> flex | Strong flex- route. |
| 065 | fluere -> fluxum -> flux | Strong flux route. |
| 066 | frangere -> fractum -> fracture | Strong fract- route. |
| 067 | fundere -> fusum -> fusion | Strong fus-/fusion route. |
| 069 | gerere -> gestum -> gesture | Gesture route likely through gestura. |
| 070 | gradi -> gressum -> progress | Compound progress path. |
| 071 | haerere -> haesum -> adhesion | Compound adhesion path. |
| 073 | laedere -> laesum -> lesion | Lesio/lesion route likely. |
| 076 | loqui -> locutum -> locution | Locutio route likely. |
| 077 | lucere -> lucem -> lucid | Lucidus bridge likely. |
| 078 | manere -> mansum -> mansion | Mansio/mansion route likely. |
| 079 | monere -> monitum -> monitor | Monitor agent noun route. |
| 080 | nasci -> natum -> nation | Natio/nation route likely. |
| 083 | orare -> oratum -> orator | Orator agent noun route. |
| 084 | pati -> passum -> passion | Passio/passion route likely. |
| 086 | plicare -> plicatum -> complicate | Compound complicare route. |
| 087 | quaerere -> quaesitum -> question | Quaestio/question route likely. |
| 088 | rapere -> raptum -> rapture | Raptus/rapture route likely. |
| 089 | ridere -> risum -> risible | Strong but less central. |
| 092 | scire -> scientia -> science | Scientia/science route strong. |
| 096 | stringere -> strictum -> strict | Strictus/strict route likely. |
| 097 | sumere -> sumptum -> assume | Compound assumere path. |
| 100 | uti -> usum -> use | Very useful but source path may be complex. |

## Tier 3 - Review Last Or Replace If Needed

These may still be useful, but they are more likely to need replacement,
alternate bridge forms, or non-Latin chain decisions.

| # | Chain | Concern |
|---:|---|---|
| 025 | spectare -> spectator -> spectator | Redundant target; should become `spectacle`, `inspect`, or keep as agent-noun study. |
| 068 | gaudere -> gaudium -> joy | English joy path is Romance/French and may not be cleanly represented by current bridge. |
| 072 | ire -> itum -> itinerary | Itinerary route likely through iter/itinerarium. |
| 074 | laudare -> laudatum -> laud | Valid-looking but lower graph value. |
| 075 | linquere -> lictum -> relic | Should route through relinquere/relictum/relic. |
| 081 | nocere -> nocitum -> noxious | Noxius bridge likely. |
| 082 | novare -> novatum -> novel | Novus/novel route may be better than novare. |
| 085 | placere -> placitum -> placid | Placidus bridge likely. |
| 090 | salire -> saltum -> salient | Saliens/salient route likely. |
| 091 | sanare -> sanatum -> sanitary | Sanitas/sanitary route likely. |
| 093 | servire -> servitum -> serve | Better than servare, but servus/servire route needs care. |
| 094 | sonare -> sonitum -> sound | English sound has multiple etymological paths; high ambiguity. |
| 095 | sperare -> speratum -> despair | Compound desperare path. |
| 098 | timere -> timorem -> timid | Timidus bridge likely. |
| 099 | urgere -> ursum -> urgent | Urgens/urgent route likely; current bridge is weak. |

## Next Review Work

1. Source-review Tier 1 with two source families: Latin lexicographic pointer and
   English etymology pointer.
2. Replace generated Old French/Middle English placeholder labels with actual
   sourced forms where available.
3. Promote only the stages that are source-supported; leave absent or inferred
   intermediate stages as `source_limited`.
4. Re-run `batch validate` and expect Tier 1 accepted fixtures to move toward
   `validated` while Tier 2 and Tier 3 remain blocked.
