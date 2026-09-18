# `datafusion_execution::cache::default_cache::SystemTimeProvider`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_execution.cache.default_cache.SystemTimeProvider.json).

<a id="op-101e4ffbce3f0c545b798398"></a>
## SystemTimeProvider

`struct` · `datafusion_execution::cache::default_cache::SystemTimeProvider` · datafusion-execution 55.1.0

```rust
struct SystemTimeProvider
```

Source: `src/cache/default_cache.rs:38`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

[`TimeProvider`](../operations/datafusion_execution.cache.default_cache.TimeProvider.md#op-e73cc059c19e9d1c6e09ac52) backed by [`Instant::now`].

This is the default time source used by [`DefaultCache`](../operations/datafusion_execution.cache.default_cache.DefaultCache.md#op-0505d2d12e8de5ca44498485)

Unresolved upstream links (retained, not inferred): ``Instant::now``.

<a id="op-3ce0f400b9cdf2678fbe8f6c"></a>
## default

`function` · `datafusion_execution::cache::default_cache::SystemTimeProvider::default` · datafusion-execution 55.1.0

```rust
fn default() -> SystemTimeProvider
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::default_cache::SystemTimeProvider", "path": "SystemTimeProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 17], "end": [37, 24], "filename": "src/cache/default_cache.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/cache/default_cache.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39cb750c94003270d28609bf"></a>
## fmt

`function` · `datafusion_execution::cache::default_cache::SystemTimeProvider::fmt` · datafusion-execution 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::default_cache::SystemTimeProvider", "path": "SystemTimeProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [37, 10], "end": [37, 15], "filename": "src/cache/default_cache.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/cache/default_cache.rs:37`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3536a84b7f3a703e1b9a155a"></a>
## now

`function` · `datafusion_execution::cache::default_cache::SystemTimeProvider::now` · datafusion-execution 55.1.0

```rust
fn now(&self) -> Instant
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_execution::cache::default_cache::SystemTimeProvider", "path": "SystemTimeProvider"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [44, 2], "filename": "src/cache/default_cache.rs"}, "trait": {"args": null, "id": "datafusion_execution::cache::default_cache::TimeProvider", "path": "TimeProvider"}, "trait_path": "datafusion_execution::cache::default_cache::TimeProvider"}`

Source: `src/cache/default_cache.rs:41`. [Exact documentation build](https://docs.rs/crate/datafusion-execution/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
