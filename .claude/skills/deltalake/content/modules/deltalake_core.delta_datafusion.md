# `deltalake_core::delta_datafusion`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_core.delta_datafusion.json).

<a id="op-519a8b219525627b80750e8f"></a>
## delta_datafusion

`module` · `deltalake_core::delta_datafusion` · deltalake-core 1.0.0+58f07cd6

Access: **public**. Canonical source location is not automatically a valid import path.

```rust
mod delta_datafusion
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/core/src/delta_datafusion/mod.rs#L1).

Source: `crates/core/src/delta_datafusion/mod.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

Datafusion integration for Delta Table

Example:

```rust
use std::sync::Arc;
use datafusion::execution::context::SessionContext;

async {
  let mut ctx = SessionContext::new();
  let table = deltalake_core::open_table_with_storage_options(
      url::Url::parse("memory://").unwrap(),
      std::collections::HashMap::new()
  )
      .await
      .unwrap();
  ctx.register_table("demo", table.table_provider().await.unwrap()).unwrap();

  let batches = ctx
      .sql("SELECT * FROM demo").await.unwrap()
      .collect()
      .await.unwrap();
};
```
