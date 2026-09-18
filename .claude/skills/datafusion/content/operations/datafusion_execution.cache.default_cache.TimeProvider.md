# `datafusion_execution::cache::default_cache::TimeProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.default_cache.TimeProvider.json).

<a id="op-e73cc059c19e9d1c6e09ac52"></a>
## TimeProvider

`trait` · `datafusion_execution::cache::default_cache::TimeProvider` · datafusion-execution 55.1.0

```rust
trait TimeProvider: Send + Sync
```

Source: `src/cache/default_cache.rs:29`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Source of the current time used by a [`DefaultCache`](../operations/datafusion_execution.cache.default_cache.DefaultCache.md#op-0505d2d12e8de5ca44498485) when applying TTLs.

<a id="op-8096fc68f2ed45a4a2f26d6a"></a>
## now

`function` · `datafusion_execution::cache::default_cache::TimeProvider::now` · datafusion-execution 55.1.0

```rust
fn now(&self) -> Instant
```

Source: `src/cache/default_cache.rs:31`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

Return the current instant.
