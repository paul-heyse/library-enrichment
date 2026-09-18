# `datafusion_datasource::boundary_stream`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.boundary_stream.json).

<a id="op-8afcf97c2496890e192a7f18"></a>
## boundary_stream

`module` · `datafusion_datasource::boundary_stream` · datafusion-datasource 55.1.0

```rust
mod boundary_stream
```

Source: `src/boundary_stream.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Streaming boundary-aligned wrapper for newline-delimited JSON and CSV range reads.

[`AlignedBoundaryStream`](../operations/datafusion_datasource.boundary_stream.AlignedBoundaryStream.md#op-07f52935072c17fd47be5e48) wraps a raw byte stream and lazily aligns to
record (newline) boundaries, avoiding the need for separate `get_opts`
calls to locate boundary positions.
