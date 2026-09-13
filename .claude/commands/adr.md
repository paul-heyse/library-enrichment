---
description: Record an architecture decision, including any deviation from the blueprint.
argument-hint: <short-slug>
allowed-tools: Bash, Read, Grep, Glob, Write, Edit
---

Create an ADR for: $1

`AGENT_HANDOFF.md` requires an ADR with evidence and tests for every necessary deviation, so
this must be low-friction enough that it actually happens.

1. Find the next number: `ls docs/adr/`.
2. Write `docs/adr/NNNN-$1.md` from the template in `docs/adr/0000-template.md`.
3. Fill in every section. An ADR without evidence is an opinion.

The sections that matter most here:

- **Binding boundary touched** — which entry in the blueprint's §1.1 table this affects, and
  whether it is preserved or changed. If a binding decision changes, say so explicitly rather
  than letting it happen implicitly.
- **Evidence** — primary-source URLs with retrieval dates and exact quotes. Not recollection,
  not a plausible inference. If you have not read it this session, it is not evidence. Use the
  `upstream-verifier` subagent if the claim concerns an external tool.
- **Tests that prove it** — the specific test or gate that will fail if this decision is
  violated later. A decision with no executable consequence will silently erode.

Then add the ADR to the index in `docs/adr/README.md`.
