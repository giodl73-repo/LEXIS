# LEXIS Diagnostic Model

Status: settled.

This diagnostic model defines future error, warning, and finding families before
LEXIS has a CLI or fixtures. It is planning input only.

## Diagnostic Families

| Family | Purpose | Example future condition |
|---|---|---|
| `source_custody` | Block or warn on unreviewed source promotion. | Fixture cites a source family with no custody posture. |
| `scope` | Keep slices bounded. | A `scribere` fixture adds broad Indo-European reconstruction without review. |
| `claim_type` | Require explicit evidence/theory/negative posture. | Edge has no claim type or uncertainty label. |
| `relationship` | Prevent relationship collapse. | Borrowing, descent, and cognate labels are merged. |
| `script` | Preserve script, glyph, transliteration, and sound distinctions. | Script-form edge is used as proof of sound value. |
| `reconstruction` | Surface reconstructed or disputed forms. | Reconstructed root is rendered as attested. |
| `graph_preservation` | Protect LEXIS labels through graph output. | RLINE output drops source posture or edge kind. |
| `chronicle_overclaim` | Block public wording that exceeds evidence. | Chronicle says "proved" for a disputed theory. |

## Allocation Rule

Future validation commands must allocate every failure to one diagnostic family.
Diagnostics that affect source custody, public claims, or graph-label
preservation must be review-blocking until explicitly deferred.

## Decision

DIAGNOSTIC_MODEL is settled as planning input. Future implementation must refine
message text, severity, location, and fixture expectations before CLI release.
