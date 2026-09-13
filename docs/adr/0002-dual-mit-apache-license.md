---
id: ADR-0002
title: Dual MIT / Apache-2.0 license
status: accepted
date: 2026-09-13
deciders: [paul-heyse]
level: decision
principles: []
design: [§12.1]
review: not-required: licensing; it governs no design surface
evidence: Implemented
supersedes: []
superseded-by: null
revisit: A dependency's license makes `MIT OR Apache-2.0` unpublishable, or a consumer needs a different grant
verification: `just deps-policy` — `cargo deny check licenses`

---

# ADR-0002: Dual MIT / Apache-2.0 license

## Context

The repository is published publicly. A public repository with no `LICENSE` is "all rights
reserved" by default: readable, but not legally usable or contributable. Several of the
operator's other public repositories carry no license, so this was a deliberate choice rather
than a default.

## Decision

Dual-license under `MIT OR Apache-2.0`, the Rust ecosystem convention. `LICENSE-MIT` and
`LICENSE-APACHE` at the repository root; `license = "MIT OR Apache-2.0"` in `Cargo.toml` and
`pyproject.toml`.

Apache-2.0 contributes an explicit patent grant; MIT maximises downstream compatibility.
Offering both lets a consumer pick whichever fits their own licensing.

## Evidence

The Apache-2.0 text was fetched verbatim from `https://www.apache.org/licenses/LICENSE-2.0.txt`
on 2026-09-13 rather than reproduced from memory.

## Verification

`cargo deny check licenses` enforces the allowed-license set for dependencies. It does not check
our own license declaration; that is verified by review.

## Consequences

Captured upstream fixture artifacts (rustdoc JSON, PyPI metadata, documentation inventories)
carry their own upstream licenses and are **not** covered by ours. Where redistribution is
permitted, record source and digest attribution alongside the fixture, per blueprint §14.1. A
fixture whose license does not permit redistribution must be fetched at test time instead of
committed.

## Boundaries preserved

All of them.

## Status history

- 2026-09-13 — accepted.
