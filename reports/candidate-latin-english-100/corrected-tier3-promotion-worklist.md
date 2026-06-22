# Corrected Tier 3 Promotion Worklist

Artifact root: `artifacts/generated/corrected-latin-english-tier3`

## Summary

- corrected graphs joined: 15
- validation errors total: 223
- validation errors max: 18
- correction actions:
  - `compound_route`: 1
  - `keep_or_replace_target`: 1
  - `replace_base_and_bridge`: 2
  - `replace_bridge`: 10
  - `split_homonym_route`: 1

## Ranked Worklist

| Rank | Chain | Action | Validation errors | Nodes | Edges | Proof source | First blocker |
|---:|---|---|---:|---:|---:|---|---|
| 1 | `068` | `replace_bridge` | 18 | 11 | 11 | `LEXIS-SRCDEC-096-tier3-proof-joy` | generated Old French placeholder must become joie |
| 2 | `074` | `replace_bridge` | 18 | 12 | 11 | `LEXIS-SRCDEC-098-tier3-proof-laud` | generated bridge laudatum should be replaced |
| 3 | `075` | `replace_bridge` | 18 | 11 | 11 | `LEXIS-SRCDEC-099-tier3-proof-relic` | generated bridge lictum is not promotion-ready |
| 4 | `095` | `compound_route` | 18 | 11 | 11 | `LEXIS-SRCDEC-107-tier3-proof-despair` | generated bridge speratum should be replaced |
| 5 | `094` | `split_homonym_route` | 17 | 11 | 10 | `LEXIS-SRCDEC-106-tier3-proof-sound` | homonym routes must be split before promotion |
| 6 | `093` | `replace_bridge` | 16 | 11 | 10 | `LEXIS-SRCDEC-105-tier3-proof-serve` | generated bridge servitum should be replaced |
| 7 | `072` | `replace_bridge` | 15 | 9 | 9 | `LEXIS-SRCDEC-097-tier3-proof-itinerary` | generated bridge itum is not the preferred source route |
| 8 | `082` | `replace_base_and_bridge` | 15 | 10 | 9 | `LEXIS-SRCDEC-101-tier3-proof-novel` | generated base should be replaced or downgraded to related-family context |
| 9 | `099` | `replace_bridge` | 15 | 9 | 9 | `LEXIS-SRCDEC-109-tier3-proof-urgent` | generated bridge ursum should be replaced |
| 10 | `085` | `replace_bridge` | 13 | 9 | 8 | `LEXIS-SRCDEC-102-tier3-proof-placid` | generated bridge placitum should be replaced |
| 11 | `091` | `replace_base_and_bridge` | 13 | 9 | 8 | `LEXIS-SRCDEC-104-tier3-proof-sanitary` | generated base and bridge should be replaced or marked related-family context |
| 12 | `098` | `replace_bridge` | 13 | 9 | 8 | `LEXIS-SRCDEC-108-tier3-proof-timid` | generated bridge timorem should be replaced |
| 13 | `081` | `replace_bridge` | 12 | 7 | 7 | `LEXIS-SRCDEC-100-tier3-proof-noxious` | generated bridge nocitum should be replaced |
| 14 | `090` | `replace_bridge` | 12 | 7 | 7 | `LEXIS-SRCDEC-103-tier3-proof-salient` | generated bridge saltum should be replaced |
| 15 | `025` | `keep_or_replace_target` | 10 | 7 | 6 | `LEXIS-SRCDEC-095-tier3-proof-spectator` | placeholder Old French and Middle English stages are not accepted |

## Promotion Guidance

- Start with rows tied at the maximum validation-error count; these represent the largest corrected graph shapes still blocked by candidate source state.
- For each row, promote only after the proof source is accepted for slice use and the correction blockers are resolved in fixture data.
- Compound and homonym actions should receive explicit edge labels before acceptance, not just source-state updates.
