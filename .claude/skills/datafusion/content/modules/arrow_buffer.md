# `arrow_buffer`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.json).

<a id="op-e147f1254e2292b9855350ca"></a>
## arrow_buffer

`module` · `arrow_buffer` · arrow-buffer 59.3.0

```rust
mod arrow_buffer
```

Source: `src/lib.rs:18`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Low-level buffer abstractions for [Apache Arrow Rust](https://docs.rs/arrow)

# Byte Storage abstractions
- [`MutableBuffer`](../operations/arrow_buffer.buffer.mutable.MutableBuffer.md#op-c6098a137f542fbc611c6673): Raw memory buffer that can be mutated and grown
- [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b): Immutable buffer that is shared across threads

# Typed Abstractions

There are also several wrappers over [`Buffer`](../operations/arrow_buffer.buffer.immutable.Buffer.md#op-f54755e677e7b3529b3edb8b) with methods for
easier manipulation:

- [`BooleanBuffer`][]: Bitmasks (buffer of packed bits)
- [`NullBuffer`][]: Arrow null (validity) bitmaps ([`BooleanBuffer`](../operations/arrow_buffer.buffer.boolean.BooleanBuffer.md#op-3a838cdae998215374fcb4b5) with extra utilities)
- [`ScalarBuffer<T>`][]: Typed buffer for primitive types (e.g., `i32`, `f64`)
- [`OffsetBuffer<O>`][]: Offsets used in variable-length types (e.g., strings, lists)
- [`RunEndBuffer<E>`][]: Run-ends used in run-encoded encoded data
