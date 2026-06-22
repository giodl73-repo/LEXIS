# LEXIS Code Rigor

Status: settled.

DESIGN reached fixed point. CODE_RIGOR defines the gates future implementation
must satisfy. It does not create code, fixtures, source records, or validation
commands beyond the current docs-only `git diff --check` gate.

## Rigor principle

LEXIS must fail closed on unsupported language-history claims. Missing evidence,
uncertain reconstruction, unavailable pronunciation, disputed etymology, or
unclear source rights must remain visible and block overconfident output.

## Current validation

Foundation validation now includes the Rust scaffold and first fail-closed
fixture validator:

```powershell
git diff --check
cargo fmt --check
cargo test
cargo run -p lexis-cli -- validate fixtures/planned/source-pointer-scribere/fixture.yaml
```

The fixture validation command is expected to return exit code `1` for
`LEXIS-FIX-001` because the fixture is invalid by design.

Future implementation validation must be added by WORK_PACKAGES before code is
accepted.

## Future fixture discipline

Future fixtures must:

- name a bounded scope,
- carry source and rights posture,
- mark every claim type,
- preserve uncertainty labels,
- include rejected or competing alternatives when known,
- keep unavailable data explicit,
- record role-review state,
- avoid source text redistribution unless source custody allows it.

## Source-custody gate

A future source family cannot be ingested, promoted, or used in chronicles until
the Source Custody Reviewer records:

- source pointer type,
- rights posture,
- redistribution posture,
- citation expectations,
- allowed use in fixtures,
- allowed use in reports,
- allowed use in artifacts,
- review decision and deferred risks.

## Overclaim gate

Future validation and review must flag public wording that:

- says or implies "proved" for reconstruction, inference, disputed theory, or
  source-limited claims,
- hides competing or rejected alternatives,
- treats borrowing as descent or descent as borrowing,
- infers pronunciation, script, or date when unavailable,
- erases source-custody limits.

## RLINE preservation gate

Future RLINE-backed operations must prove they preserve:

- edge kind,
- claim type,
- uncertainty label,
- source links,
- source-custody status,
- competing and rejected alternatives.

Any graph operation that drops those fields is blocked for LEXIS foundation
work.

## Implementation acceptance gates

Future code is accepted only when a work package names:

- the exact files changed,
- the fixture or docs it validates,
- the validation command,
- the role findings considered,
- the non-goals preserved,
- the deferred risks.

## Role-review checkpoint

| Role | Finding | Decision |
|---|---|---|
| Language Historian | Major: unsupported theory must fail closed. | Closed by rigor principle and overclaim gate. |
| Etymology Reviewer | Major: relationship confusion must be a validation/review failure. | Closed by overclaim gate. |
| Phonology Reviewer | Major: unavailable pronunciation must not be inferred. | Closed by fixture discipline and overclaim gate. |
| Script Systems Reviewer | Minor: unavailable script/date data must remain explicit. | Closed by fixture discipline. |
| Source Custody Reviewer | Major: source family intake needs a required review record. | Closed by source-custody gate. |
| Graph Systems Reviewer | Major: RLINE operations must prove label preservation. | Closed by RLINE preservation gate. |
| Product Chronicle Reviewer | Major: public chronicle wording needs an overclaim gate. | Closed by overclaim gate. |
| Software Assurance Reviewer | Major: future code needs per-work-package validation commands. | Closed by implementation acceptance gates. |

## Decision

CODE_RIGOR is settled for the foundation wave. No critical or major actionable
role finding remains. IMPLEMENTATION_PLAN is the next VTRACE stage.
