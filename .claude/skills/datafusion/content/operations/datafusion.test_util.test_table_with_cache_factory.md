# `datafusion::test_util::test_table_with_cache_factory`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion.test_util.test_table_with_cache_factory.json).

<a id="op-54d8002ac9f843308dfb3e0c"></a>
## test_table_with_cache_factory

`function` · `datafusion::test_util::test_table_with_cache_factory` · datafusion 55.1.0

```rust
async fn test_table_with_cache_factory() -> error::Result<dataframe::DataFrame>
```

Source: `src/test_util/mod.rs:344`. [Exact documentation build](https://docs.rs/crate/datafusion/55.1.0/json).

Create a test table registered to a session context with an associated cache factory
