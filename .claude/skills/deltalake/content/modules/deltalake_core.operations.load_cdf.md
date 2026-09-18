# `deltalake_core::operations::load_cdf`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.operations.load_cdf.json).

<a id="op-ede9fcbf8190c2446a56a500"></a>
## load_cdf

`module` · `deltalake_core::operations::load_cdf` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod load_cdf
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/operations/load_cdf.rs#L1).

Source: `crates/core/src/operations/load_cdf.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Module for reading the change datafeed of delta tables

# Example
```rust ignore
let table = open_table(Url::from_directory_path("/abs/path/to/table").unwrap())?;
let builder = CdfLoadBuilder::new(table.log_store(), table.snapshot())
    .with_starting_version(3);

let ctx = SessionContext::new();
let provider = DeltaCdfTableProvider::try_new(builder)?;
let df = ctx.read_table(provider).await?;
