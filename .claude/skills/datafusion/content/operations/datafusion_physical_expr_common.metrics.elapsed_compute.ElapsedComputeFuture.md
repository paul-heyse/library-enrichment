# `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr_common.metrics.elapsed_compute.ElapsedComputeFuture.json).

<a id="op-35519ba29206359f0bacd223"></a>
## ElapsedComputeFuture

`struct` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture` · datafusion-physical-expr-common 55.1.0

```rust
struct ElapsedComputeFuture<T>
```

Source: `src/metrics/elapsed_compute.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

Wraps any [`Future`] and accumulates the wall-clock time spent inside
each [`Future::poll`] call into `elapsed_compute`. Everything that
executes synchronously within a `poll()` scope is measured — including
CPU-bound work, memory copies, and any blocking the future performs
before returning. Time between polls (when the runtime has suspended the
future waiting for I/O, a channel, or a waker) is not measured.

For futures that mix synchronous CPU work with async I/O this gives a
good approximation of CPU time: async I/O causes the future to yield
(`Poll::Pending`), so the I/O latency is excluded automatically.

Note: uses `pin-project` rather than `pin-project-lite` in order to
support `PinnedDrop`, which ensures accumulated time is flushed even
if the future is cancelled (dropped before completion).

Unresolved upstream links (retained, not inferred): ``Future``, ``Future::poll``.

<a id="op-df0a619b6eb966e81ff60d4e"></a>
## Output

`assoc_type` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture::Output` · datafusion-physical-expr-common 55.1.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture", "path": "ElapsedComputeFuture"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "O"}}}, "name": "Output"}]}}, "id": "core::future::future::Future", "path": "Future"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [83, 2], "filename": "src/metrics/elapsed_compute.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/metrics/elapsed_compute.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0bd3b3e8224a9b3208dbcc3d"></a>
## drop

`function` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture::drop` · datafusion-physical-expr-common 55.1.0

```rust
fn drop(&mut self)
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture", "path": "ElapsedComputeFuture"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 15], "end": [42, 25], "filename": "src/metrics/elapsed_compute.rs"}, "trait": {"args": null, "id": "core::ops::drop::Drop", "path": "Drop"}, "trait_path": "core::ops::drop::Drop"}`

Source: `src/metrics/elapsed_compute.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d7f20561dce75dfd028e6cd"></a>
## poll

`function` · `datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture::poll` · datafusion-physical-expr-common 55.1.0

```rust
fn poll(Pin<&mut self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "F"}}], "constraints": []}}, "id": "datafusion_physical_expr_common::metrics::elapsed_compute::ElapsedComputeFuture", "path": "ElapsedComputeFuture"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "O"}, {"kind": {"type": {"bounds": [{"trait_bound": {"generic_params": [], "modifier": "none", "trait": {"args": {"angle_bracketed": {"args": [], "constraints": [{"args": null, "binding": {"equality": {"type": {"generic": "O"}}}, "name": "Output"}]}}, "id": "core::future::future::Future", "path": "Future"}}}], "default": null, "is_synthetic": false}}, "name": "F"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [83, 2], "filename": "src/metrics/elapsed_compute.rs"}, "trait": {"args": null, "id": "core::future::future::Future", "path": "Future"}, "trait_path": "core::future::future::Future"}`

Source: `src/metrics/elapsed_compute.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
