# `datafusion_execution::async_stream::TryEmitter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.async_stream.TryEmitter.json).

<a id="op-a4a27f9f5c8ee8d9be3ec4b0"></a>
## TryEmitter

`struct` · `datafusion_execution::async_stream::TryEmitter` · datafusion-execution 55.1.0

```rust
struct TryEmitter<T, E>
```

Source: `src/async_stream.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A handle for emitting values from an [`async_try_stream`](../operations/datafusion_execution.async_stream.async_try_stream.md#op-d258c5b02971e9e9225dd32c) generator.

The generator closure receives a `TryEmitter<T, E>` as its argument.

<a id="op-2cb158b391402c9f82d96bae"></a>
## emit

`function` · `datafusion_execution::async_stream::TryEmitter::emit` · datafusion-execution 55.1.0

```rust
fn emit(&mut self, value: T) -> impl FusedFuture<Output = ()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}, {"type": {"generic": "E"}}], "constraints": []}}, "id": "datafusion_execution::async_stream::TryEmitter", "path": "TryEmitter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}, {"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "E"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [191, 1], "end": [210, 2], "filename": "src/async_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_stream.rs:201`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Emits `Ok(value)` as the next stream item and suspends the generator.

Behaves identically to [`Emitter::emit`](../operations/datafusion_execution.async_stream.Emitter.md#op-7a624ab206922908766e399b): the returned future must be
awaited immediately and yields `Poll::Pending` on its first poll to
transfer control to the stream consumer.

# Panics

Panics if called before the previous emit future has been awaited.
