# `datafusion_datasource::morsel`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_datasource.morsel.json).

<a id="op-cc8d4bf7f77d94da9a897a77"></a>
## morsel

`module` · `datafusion_datasource::morsel` · datafusion-datasource 55.1.0

```rust
mod morsel
```

Source: `src/morsel/mod.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-datasource/55.1.0/json).

Structures for Morsel Driven IO.

NOTE: As of DataFusion 54.0.0, these are experimental APIs that may change
substantially.

Morsel Driven IO is a technique for parallelizing the reading of large files
by dividing them into smaller "morsels" that are processed independently.

It is inspired by the paper [Morsel-Driven Parallelism: A NUMA-Aware Query
Evaluation Framework for the Many-Core Age](https://db.in.tum.de/~leis/papers/morsels.pdf).
