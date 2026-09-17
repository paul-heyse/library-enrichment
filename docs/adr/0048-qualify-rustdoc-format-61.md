---
id: ADR-0048
title: Use one exact format-61 model for facts and rendering
status: accepted
date: 2026-09-16
deciders: [paul-heyse]
level: decision
principles: [DM-02, DM-06, DM-16, DM-42]
design: [§4.1, §17]
review: not-required: Exact producer-model pin and narrow renderer correction within the accepted native producer boundary.
evidence: Interface-checked
supersedes: []
superseded-by: null
revisit: A published public-api release uses the exact qualified rustdoc-types model without the local renderer patch.
verification: cargo test -p enrichment-core producer::rustdoc and cargo test -p enrichment-store rust_normalize::tests
---

# ADR-0048: Use one exact format-61 model for facts and rendering

## Context

Plan 17 FP07 identifies a parser that advertises format 61 while using rustdoc-types 0.59.0.
Stable-level encoding changed in format 61; default-body instability was added in format 60.
The current fixtures contain only unstable stability records and cannot establish field fidelity.

## Scope

The current Rust producer format contract, signature renderer dependency, and typed facts.
Historical internal epochs and historical source-format adapters are removed. Additional current
hosted formats require their own explicit field-preservation qualification.

## Drivers

Preserve exact upstream facts, hosted-first acquisition, a single qualified parser model, and
bounded pure rendering inside the existing producer process.

## Options

- Retain the old parser: rejected because it can reject or drop authoritative fields.
- Upgrade the service parser alone: rejected because public-api reparses with its own dependency.
- Replace the complete renderer: unnecessary semantic duplication and qualification cost.
- Pin a narrow public-api source patch with the same format-61 dependency: selected.

## Decision

Use exactly rustdoc-types 0.61.0 in both the fact worker and public-api renderer. Retain the
0.52.2 renderer source with a documented dependency/pattern patch and a distinct producer identity.
Do not strip or relabel source JSON. Refuse formats outside the qualified current source contract
before full decoding. Preserve ordinary stability, const stability and default-body instability
separately, including absence and default presence. Expose these through generated native records.

### Consequences

The service owns a small upstream patch until a qualified release replaces it. Input remains
parsed twice; worker process limits bound renderer materialization. Successful parsing does not
certify extracted fields or runtime execution capability.

### Compensating controls

The lockfile and dependency assertion enforce one model. A real dated-nightly fixture covers
stable/unstable records and unstable defaults on functions, associated constants and types.
Malformed and unsupported documents refuse. R04 remains required.

## Evidence

Verified by the upstream-verifier on 2026-09-16; source details and fixture inventory are in
[the compatibility matrix](../architecture/compatibility-matrix.md#plan-17-fp07-rustdoc-format-61--verified-2026-09-16).

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Current renderer requires model 59 | [public-api dependencies](https://crates.io/api/v1/crates/public-api/0.52.2/dependencies) | 2026-09-16 | `"req":"^0.59.0"` |
| Exact target model exists | [rustdoc-types source](https://github.com/rust-lang/rustdoc-types/blob/2e63bdc44b94e86fe12d6e0034d1ed9c2ac6e9dc/src/lib.rs) | 2026-09-16 | `pub const FORMAT_VERSION: u32 = 61;` |
| Renderer has one affected exhaustive associated-type pattern | [renderer source](https://github.com/cargo-public-api/cargo-public-api/blob/56b483933ceb741978a9f59228e48cae2013e74d/public-api/src/render.rs#L158) | 2026-09-16 | `ItemEnum::AssocType` |

## Verification

`cargo test -p enrichment-core producer::rustdoc` checks refusal and independently expected typed
stability/default facts and signature renderings from the real fixture. `cargo test -p
enrichment-store rust_normalize::tests` checks the native consumer. Dependency and full
installed-worker qualification remain separate required Plan 17 receipts.

## Boundaries preserved

§B1 Rust core ownership, §B2 thin Python, §B4 rust-analyzer semantics, §B6 hosted rustdoc first,
§B7 native evidence storage and retrieval, and §B13 execution containment remain in force.
A renderer is syntax evidence and never substitutes for semantic or runtime observations.

## More information

[Plan 17 FP07](../plans/17-schema-governed-unified-runtime-hard-pivot.md#fp07--finish-acquisition-facts-producer-fidelity-and-normalization).

## Status history

- 2026-09-16 — accepted; interface checked, implementation qualification remains open.
