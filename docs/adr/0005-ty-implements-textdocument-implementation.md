---
id: ADR-0005
title: ty implements `textDocument/implementation`; gate P08 is superseded
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: [DM-43, DM-59]
design: [§B4, §5.4, §13]
review: not-required: backfilled; recorded before the design-review process existed (ADR-0009)
evidence: Tested
supersedes: []
superseded-by: null
revisit: A ty release changes `textDocument/implementation` support, or another [S18] claim is relied on
verification: Gates P08a and P08b in `tests/gates.toml`; the `initialize` capability probe recorded in the ProducerRun

---

# ADR-0005: ty implements `textDocument/implementation`; gate P08 is superseded

## Context

Blueprint §5.4 states:

> "The current ty feature table supports definitions, hover, signatures, references, workspace
> symbols, call/type hierarchy, and diagnostics, but lists `textDocument/implementation` as
> unsupported. Probe capabilities at runtime and preserve unsupported outcomes rather than
> inventing answers."

Acceptance gate P08 turns that into a test:

> **P08** — ty implementation method unsupported → `UNSUPPORTED_CAPABILITY`, not an empty
> implementation list.

That premise is now false. ty implemented the method in **0.0.64**, released 2026-07-27, and
the pinned `ty 0.0.80` serves it.

The published capability table [S18] is **stale, not merely conservative**: it still prints
"`textDocument/implementation` | ❌ Not supported | #3514" and links issue #3514, which GitHub
reports as `closed`/`completed` on 2026-07-26 by merged PR `astral-sh/ruff#25410`. The page
footer reads "September 9, 2026" — republished six weeks after the feature shipped, with the
table uncorrected.

The blueprint's own instruction is what catches this: *probe capabilities at runtime*. Doing so
contradicted the sentence that precedes it.

## Decision

1. **Gate P08 is superseded.** Its ID is retired in place, never renumbered or deleted, per the
   frozen-contract rule. `tests/ACCEPTANCE_PLAN.md` is manifested and unchanged;
   `tests/gates.toml` carries `superseded` with a pointer to this ADR.
2. **The Python semantic adapter must implement the call** and map real locations. No
   `UNSUPPORTED_CAPABILITY` branch may be tested against this method.
3. **Two replacement gates**, registered as `P08a` and `P08b`:
   - **P08a** — implementation lookup on a *nominal* base class or `@abstractmethod` returns
     the mapped subclass locations.
   - **P08b** — implementation lookup on a `typing.Protocol` returns only the protocol's own
     declaration; the result is recorded as **incomplete evidence**, never as "no implementors".
4. **[S18] is unreliable as a whole and must not be used to derive any capability assertion.**
   Probe `initialize` at runtime and record the negotiated result in the `ProducerRun`. The
   general requirement to handle `UNSUPPORTED_CAPABILITY` stands — it is simply not exercised
   by *this* method on *this* version.

## Evidence

| Claim | Source | Retrieved | Evidence |
|---|---|---|---|
| The docs table still says "Not supported" | https://docs.astral.sh/ty/features/language-server/ | 2026-09-13 | "`textDocument/implementation` \| ❌ Not supported \| #3514"; footer "September 9, 2026" |
| Issue #3514 is closed as completed | https://github.com/astral-sh/ty/issues/3514 | 2026-09-13 | `state: closed`, `state_reason: completed`, `closed_at: 2026-07-26T23:42:53Z`, closed by merged `astral-sh/ruff#25410` |
| The feature shipped in 0.0.64 | https://raw.githubusercontent.com/astral-sh/ty/main/CHANGELOG.md | 2026-09-13 | `## 0.0.64` "Released on 2026-07-27." → "Implement LSP `textDocument/implementation` request ([#25410])" |
| ty 0.0.80 advertises the capability | executed: `.venv/bin/ty server` | 2026-09-13 | `initialize` → `serverInfo {"name":"ty","version":"0.0.80"}`, `implementationProvider: true` — advertised regardless of what the client declares |
| ty 0.0.80 **answers** the request | executed: LSP `textDocument/implementation` | 2026-09-13 | Nominal base class → 3 locations (base + both subclasses); `@abstractmethod` → abstract def + both overrides. Not an error, not an empty list. |
| Protocol conformance is **not** resolved | executed | 2026-09-13 | `class P(Protocol)` and `P.ping` each returned only themselves; the structural implementor was absent, with no error signal |
| Negotiated position encoding | executed | 2026-09-13 | `positionEncoding: "utf-16"` — confirms blueprint §9.2: do not assume byte offsets match LSP positions |

Verified twice independently: by direct LSP probe during this session, and by an
`upstream-verifier` run that additionally located the changelog entry and the closed issue.

## Verification

`P08a` and `P08b` in `tests/gates.toml`, once the ty LSP client exists in Phase 4. Until then
both are `not_run`, and `P08` is reported as superseded rather than passing or failing.

## Consequences

The Protocol finding is the more dangerous half. ty returns a **plausible, non-empty, silently
incomplete** answer for structural conformance: no error, no warning, just a short list. An
evidence path that treats an implementation result as complete will under-report protocol
implementors. Per §6.2 this is an evidence gap and must be recorded as one — which is exactly
the distinction between "searched scope" and "whole-library absence" that gate C01 protects.

More generally: a published capability table was wrong in the *permissive* direction. Runtime
probing is not belt-and-braces here; it is the only reliable source.

## Boundaries preserved

ty remains the Python semantic engine. The epistemic classes are unchanged — this evidence is
`typechecker_observed`. Nothing about core ownership, execution policy, or the repository
boundary moves.

## Status history

- 2026-09-13 — accepted.
