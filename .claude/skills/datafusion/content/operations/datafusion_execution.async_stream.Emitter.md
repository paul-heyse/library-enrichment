# `datafusion_execution::async_stream::Emitter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.async_stream.Emitter.json).

<a id="op-8937711056e5aea84f9c72af"></a>
## Emitter

`struct` · `datafusion_execution::async_stream::Emitter` · datafusion-execution 55.1.0

```rust
struct Emitter<T>
```

Source: `src/async_stream.rs:145`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

A handle for emitting values from an [`async_stream`](../operations/datafusion_execution.async_stream.async_stream.md#op-134777deff59feac15c8b28e) generator.

The generator closure receives an `Emitter<T>` as its argument.

<a id="op-7a624ab206922908766e399b"></a>
## emit

`function` · `datafusion_execution::async_stream::Emitter::emit` · datafusion-execution 55.1.0

```rust
fn emit(&mut self, value: T) -> impl FusedFuture<Output = ()>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"generic": "T"}}], "constraints": []}}, "id": "datafusion_execution::async_stream::Emitter", "path": "Emitter"}}, "generics": {"params": [{"kind": {"type": {"bounds": [], "default": null, "is_synthetic": false}}, "name": "T"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [160, 1], "end": [189, 2], "filename": "src/async_stream.rs"}, "trait": null, "trait_path": null}`

Source: `src/async_stream.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Returns a `Future` that emits `value` as the next stream item.

The returned future **must be awaited immediately**. On its first poll it
yields `Poll::Pending`, handing control back to the stream consumer so it
can observe the emitted value. On the next poll (triggered by the
consumer calling `poll_next` again) it completes with `Poll::Ready(())`,
resuming the generator.

# Panics

Panics if `emit` is called a second time before the previous future has
been awaited, because doing so would silently overwrite the unconsumed
value.
