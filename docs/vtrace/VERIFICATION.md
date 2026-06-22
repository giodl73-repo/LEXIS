# LEXIS Verification

Status: settled.

WORK_PACKAGES reached fixed point. Verification defines how future tests prove
implementation meets requirements. It does not implement tests or fixtures.

## Current verification

Foundation docs are verified with:

```powershell
git diff --check
```

## Future verification matrix

| Area | Required future checks |
|---|---|
| Scope | Valid fixture has bounded scope; negative fixture missing scope fails. |
| Claim posture | Attested/reconstructed/inferred/competing/rejected/unknown claims validate distinctly. |
| Source custody | Unreviewed sources and disallowed redistribution fail. |
| Relationship kinds | Descent, cognacy, borrowing, calque, sound shift, meaning shift, script variant, support, and dispute edges remain distinct. |
| Uncertainty | Required uncertainty labels are present and unavailable data is explicit. |
| Graph output | Graph slice preserves edge kind, claim type, uncertainty, source links, custody status, and alternatives. |
| RLINE integration | Any RLINE-backed operation passes the same preservation checks as local graph output. |
| Chronicle output | Required sections are present and overclaim wording is flagged. |
| Work packages | Each package names changed files, validation commands, role findings, and deferred risks. |
| Spec model | Future implementation specs satisfy required shape or remain draft planning input. |
| Contract boundaries | Future durable interfaces report affected boundary class and closeout fields. |
| Scenario model | Future scenarios include positive path, negative paths, evidence, fixture candidates, and findings. |
| Stage execution | S0-S6 board accurately marks pass, pass-with-risk, or blocked state. |
| Evidence ledger | Evidence rows resolve to docs-only artifacts and do not claim implementation proof. |
| Review checklists | Required checklist rows are pass, pass-with-risk, or blocked with rationale. |
| Role recommendations | VTRACE lanes map to LEXIS `.roles` reviewers. |
| Language profiles | Active and planned profiles name L0/L1/L2 expectations. |
| Change control | DCRs map to accepted or proposed future work. |

## Verification gate

Future implementation may not be marked complete unless positive and negative
checks both pass for the touched surface.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: verification must test evidence posture, not only syntax. | Closed by claim posture and uncertainty checks. |
| Etymology Reviewer | Major: relationship collapse needs negative tests. | Closed by relationship-kind checks. |
| Phonology Reviewer | Minor: unavailable data checks should cover sound data. | Closed by uncertainty checks. |
| Script Systems Reviewer | Minor: unavailable data checks should cover script data. | Closed by uncertainty checks. |
| Source Custody Reviewer | Major: source-custody negatives must block promotion. | Closed by source-custody checks. |
| Graph Systems Reviewer | Major: RLINE integration must be checked like local graph output. | Closed by RLINE integration checks. |
| Product Chronicle Reviewer | Major: chronicle overclaim checks must be required. | Closed by chronicle output checks. |
| Software Assurance Reviewer | Major: positive-only testing is insufficient. | Closed by verification gate. |

## Decision

VERIFICATION is settled for the foundation wave. No critical or major
actionable role finding remains. VALIDATION is the next VTRACE stage.
