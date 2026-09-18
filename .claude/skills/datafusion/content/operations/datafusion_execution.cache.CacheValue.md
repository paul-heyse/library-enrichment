# `datafusion_execution::cache::CacheValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.CacheValue.json).

<a id="op-d587824c0cfad36c6b9f18d1"></a>
## CacheValue

`trait` · `datafusion_execution::cache::CacheValue` · datafusion-execution 55.1.0

```rust
trait CacheValue: Clone + Send + Sync
```

Source: `src/cache/mod.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Value type for entries stored in a [`Cache`](../operations/datafusion_execution.cache.Cache.md#op-6f236f740169b7945f9787f8).

<a id="op-27033ec5bb650cbc7857b3cc"></a>
## size

`function` · `datafusion_execution::cache::CacheValue::size` · datafusion-execution 55.1.0

```rust
fn size(&self) -> usize
```

Source: `src/cache/mod.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Size of the value in bytes used for cache memory accounting.
