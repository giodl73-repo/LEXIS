# LEXIS Role Recommendations

Status: settled.

LEXIS uses the committed `.roles/ROLE.md` panel. This file maps the panel to
VTRACE review lanes.

| VTRACE lane | LEXIS role(s) |
|---|---|
| Systems engineering | Language Historian, Software Assurance Reviewer |
| Traceability | Software Assurance Reviewer, Source Custody Reviewer |
| V&V | Software Assurance Reviewer, Graph Systems Reviewer |
| Domain correctness | Language Historian, Etymology Reviewer, Phonology Reviewer, Script Systems Reviewer |
| Source custody | Source Custody Reviewer |
| Graph/dependency boundary | Graph Systems Reviewer |
| Product/user communication | Product Chronicle Reviewer |
| Configuration/change control | Software Assurance Reviewer |

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Minor: domain correctness should be explicit. | Closed by domain correctness lane. |
| Source Custody Reviewer | Major: source custody needs its own lane. | Closed by source custody lane. |
| Graph Systems Reviewer | Major: RLINE boundary needs its own lane. | Closed by graph/dependency lane. |
| Product Chronicle Reviewer | Minor: communications should have a role lane. | Closed by product/user communication lane. |

## Decision

ROLE_RECOMMENDATIONS is settled for the foundation wave.

