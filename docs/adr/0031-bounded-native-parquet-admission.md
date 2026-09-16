---
id: ADR-0031
title: Isolate native Parquet admission allocations
status: superseded
date: 2026-09-14
deciders: [paul-heyse]
level: decision
principles: [DM-07, DM-27, DM-35, DM-39, DM-56]
design: [§6.3, §8.2, §10]
review: docs/design_review/reviews/design_review_native-architecture-completion_2026-09-14.md
evidence: Implemented
supersedes: []
superseded-by: ADR-0043
revisit: The pinned native decoder exposes verified preallocation limits for footer lists, page decompression and dictionary expansion.
verification: hostile_footer_allocation_is_confined_to_native_worker; native_decoder_isolates_row_groups_before_arrow_batching; dropped_admission_future_cancels_and_reaps_running_decoder
---

# ADR-0031: Isolate native Parquet admission allocations

## Context

Plan 11 W3/W7 require bounds before untrusted input can exhaust the daemon. Parquet 59.3.0
allocates from footer list counts and uncompressed page lengths before returning a decoded
Arrow batch. A row-count limit, a serialized-file limit and `get_array_memory_size()` after
construction cannot establish a preallocation ceiling. The earlier eight-row admission
heuristic was insufficient. Native DataFusion pool accounting does not cover these allocations.

## Scope

Add one private native admission executable for evidence and catalog Parquet. It changes the
allocation boundary in design §6.3, §8.2 and §10, preserving the single schema/provider model.
The initially supported resource enforcement is Linux with a 64-bit pointer width. A missing
worker or unsupported limit implementation is an explicit admission failure, never an in-process
fallback. This is static parsing of data; it does not execute studied library code or grant a
build/runtime profile.

## Drivers

Keep malformed native metadata outside the daemon's allocation domain, preserve exact bytes
and typed admission, and avoid maintaining a second Parquet parser or custom storage engine.

## Options

- Smaller batches alone: rejected; footer, dictionary and decompression allocations precede them.
- A custom page-header parser or forked Parquet allocator: rejected; the service would own a
  second format implementation and additional semantics without a native public hook.
- A private bounded process using the pinned native decoder: selected; normal queries still
  use the admitted DataFusion provider and require no worker on warm cache hits.

## Decision

Before creating a provider for changed/cold evidence or catalog files, a dedicated sibling
`library-enrichment-native-worker` executable applies effective hard/soft `RLIMIT_AS` of at
most 1 GiB, CPU of at most 30 seconds and disabled core dumps. Tighter inherited limits remain
in force. Setup/readback failure rejects before parsing. These are virtual-address and CPU
limits, not a claim that the entire service or all concurrent queries have a 1 GiB RSS ceiling.
Trusted executable loading occurs before the worker installs limits.

The daemon passes at most 16 KiB of typed control input, with exact physical file, digest, size,
relation and configured bounds. The worker opens regular non-symlink bytes, checks their digest,
loads native metadata once and decodes one selected row group at a time. It checks the canonical
schema/metadata, row counts, per-record bounds, decoded batches and decoded group budget. Footer
and decompression attacks remain inside the bounded process. Only a complete successful exit
with a bounded request-bound summary admits a file. The parent rechecks file witnesses before
publishing providers; source bytes are never replaced by a worker's inferred summary.

One coordinator limits actual decoder processes, including cancelled callers. The parent caps
output while draining, applies wall deadlines, kills/reaps on failure and propagates dropped
async calls to running/queued validation. The executable path is derived from the running binary,
not the request, shell or PATH. It inherits no environment and uses `/` as its working directory.
There is no public SQL, shell, alternate engine or bypass switch.

### Consequences

Installation and test builds must include this additional Rust binary. Cold validation incurs
process startup and an exact hash; warm admitted reads retain existing cache behavior. The 1 GiB
worker address-space allowance includes executable mappings and decoder/runtime allocations.
Valid inputs may be rejected under tighter inherited limits. W7 must measure actual RSS, cold
latency and concurrency; this record does not certify those performance budgets.

### Compensating controls

Exact pinned source evidence, mandatory process supervision, request/response bounds, no fallback,
single native schema registry, scoped build/installation support, hostile-footer and actual
process cancellation tests. Native SQL still checks cross-batch keys and foreign references.

## Evidence

Verified 2026-09-14 with the required upstream-verifier and direct installed-source inspection.

| Claim | Source | Retrieved | Quote |
|---|---|---|---|
| Footer allocation precedes list-element validation | [Parquet 59.3 source](https://docs.rs/crate/parquet/59.3.0/source/src/parquet_thrift.rs) | 2026-09-14 | `Vec::with_capacity(list_ident.size as usize)` |
| Page allocation uses advertised uncompressed size | [Native page decoder](https://docs.rs/crate/parquet/59.3.0/source/src/file/serialized_reader.rs) | 2026-09-14 | `Vec::with_capacity(uncompressed_page_size)` |
| Native row-group selection is public | [Arrow reader builder](https://docs.rs/parquet/59.3.0/parquet/arrow/arrow_reader/struct.ArrowReaderBuilder.html) | 2026-09-14 | `with_row_groups` |
| Address-space and CPU limits are distinct | [Linux getrlimit](https://man7.org/linux/man-pages/man2/getrlimit.2.html) | 2026-09-14 | “maximum size of the process's virtual memory (address space)” |
| Allocator failure may abort, without unwinding | [Rust allocation handler](https://doc.rust-lang.org/std/alloc/fn.handle_alloc_error.html) | 2026-09-14 | “abort the process” |

## Verification

`hostile_footer_allocation_is_confined_to_native_worker` constructs a tiny malicious compact
footer declaring 2^31−1 schema elements. It requires the actual worker's allocation abort, then
successfully validates an ordinary file using a fresh worker. The parent never parses the attack.
`native_decoder_isolates_row_groups_before_arrow_batching` proves that two groups which would
otherwise share an over-budget native batch pass independently within the chosen byte bound.
`decoder_supervision_rejects_overflow_abnormal_exit_and_reaps_timeout` attacks output/exit/time
handling. `dropped_admission_future_cancels_and_reaps_running_decoder` requires actual process
absence after cancellation. Logs and remaining qualification are in the execution ledger.

## Boundaries preserved

§B1–§B13 remain intact: all identity, schema, admission and query authority stays in Rust; the
worker uses the same Arrow/Parquet crates and validators, writes no evidence and publishes
nothing. Python remains the thin adapter and extraction worker. Studied-library execution still
requires its existing qualified capsule. No migration, TTL or optional legacy reader is added.

## More information

[Plan 11](../plans/11-arrow-datafusion-completion-and-legacy-removal.md),
[execution ledger](../plans/11-arrow-datafusion-execution-ledger.md),
[measurement protocol](../architecture/arrow-measurement-protocol.md).

## Status history

- 2026-09-14 — proposed; implementation and focused native-process tests exist; scoped review,
  complete build/distribution checks and fixed-budget measurements remain required.

- 2026-09-14 — accepted after scoped native architecture review; remaining actual producer,
  client and final performance qualification stays in Plan 12.
- 2026-09-15 — superseded by ADR-0043.
