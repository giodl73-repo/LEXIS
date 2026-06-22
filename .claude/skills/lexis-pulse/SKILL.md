---
name: lexis-pulse
description: Execute one small LEXIS pulse with role review and validation.
allowed-tools:
  - Read
  - Write
  - Glob
  - Grep
---

# LEXIS Pulse

Use this skill to execute a single LEXIS pulse.

## Pulse protocol

1. Read `README.md`, `PRODUCT_PLAN.md`, `context/waves/PHASES.md`, and the
   active wave.
2. Read the target pulse file.
3. If the pulse is VTRACE work, edit only the active stage deliverable.
4. Run the LEXIS role panel from `.roles/ROLE.md`.
5. Close critical and major actionable findings before moving forward.
6. Record deferred findings with a named later stage or work package.
7. Run validation commands named by the pulse.

