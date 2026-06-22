# Findings: LEXIS-SC-002 Borrowing vs Descent

Status: planned, not executed.

## Expected Findings

| Finding | Class | Disposition |
|---|---|---|
| Baseline has no separate coincidence edge kind. | major | Represent through `unknown`, `rejected_alternative`, or `disputes_claim` until a DCR proves a new edge kind is needed. |
| Relationship collapse must fail validation before graph output. | major | Defer executable proof to `LEXIS-WP-005` and `LEXIS-WP-006`. |
| Chronicle wording must translate edge distinctions without simplifying them away. | minor | Defer wording proof to `LEXIS-WP-008`. |

## Review Note

This scenario gives the relationship specs a concrete negative path, but it is
not validation evidence until negative fixtures and diagnostics exist.

