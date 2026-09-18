# `datafusion::execution::context::DataFilePaths`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.execution.context.DataFilePaths.json).

<a id="op-16a51b2bb79452404d0c1294"></a>
## DataFilePaths

`trait` · `datafusion::execution::context::DataFilePaths` · datafusion 55.1.0

```rust
trait DataFilePaths
```

Source: `src/execution/context/mod.rs:121`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

DataFilePaths adds a method to convert strings and vector of strings to vector of [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) URLs.
This allows methods such [`SessionContext::read_csv`](../operations/datafusion.execution.context.SessionContext.md#op-b76fcba763d91dfabf32a973) and [`SessionContext::read_avro`](../operations/datafusion.execution.context.SessionContext.md#op-62bc23af61d323278c5825d1)
to take either a single file or multiple files.

<a id="op-bd6c076f0afa04fbcf51d423"></a>
## to_urls

`function` · `datafusion::execution::context::DataFilePaths::to_urls` · datafusion 55.1.0

```rust
fn to_urls(self) -> Result<Vec<ListingTableUrl>>
```

Source: `src/execution/context/mod.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Parse to a vector of [`ListingTableUrl`](../operations/datafusion_datasource.url.ListingTableUrl.md#op-1b5851ef006daad7359f3a88) URLs.
