# LEXIS Language Profiles

Status: settled.

| Profile ID | Surface | Applicability | L0 | L1 | L2 | Review Lanes |
|---|---|---|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE docs, README, wave records, roles, skills | current | `git diff --check` | role/trace inspection | portfolio review | L-1..L-8 |
| PROFILE-RUST-001 | future Rust workspace | planned | future `cargo fmt --check` | future tests and fixture validation | scenario proof and role review | Software Assurance, Graph Systems |
| PROFILE-FIXTURE-001 | future fixtures/scenarios | planned | future parse/validate command | negative tests | source-custody and validation review | Source Custody, Etymology, Graph Systems |
| PROFILE-REPORT-001 | future chronicle/report output | planned | future required-section check | overclaim validation | reader/downstream validation | Product Chronicle, Language Historian |

## Decision

LANGUAGE_PROFILES is settled for the foundation wave. Only the docs profile is
currently active.

