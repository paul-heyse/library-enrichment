# `object_store::client::backoff::BackoffConfig`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.client.backoff.BackoffConfig.json).

<a id="op-34b26a88c595bed8eca98825"></a>
## BackoffConfig

`struct` · `object_store::client::backoff::BackoffConfig` · object_store 0.13.2

```rust
struct BackoffConfig
```

Source: `src/client/backoff.rs:31`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Exponential backoff with decorrelated jitter algorithm

The first backoff will always be `init_backoff`.

Subsequent backoffs will pick a random value between `init_backoff` and
`base * previous` where `previous` is the duration of the previous backoff

See <https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/>

<a id="op-a1a4578bfc0edd4c44d6f2e8"></a>
## base

`struct_field` · `object_store::client::backoff::BackoffConfig::base` · object_store 0.13.2

```rust
base: f64
```

Source: `src/client/backoff.rs:37`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The multiplier to use for the next backoff duration

<a id="op-0e5dc6b4a83c240b9d58d95b"></a>
## clone

`function` · `object_store::client::backoff::BackoffConfig::clone` · object_store 0.13.2

```rust
fn clone(&self) -> BackoffConfig
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 17], "end": [30, 22], "filename": "src/client/backoff.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/client/backoff.rs:30`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8642821a3275788d6aeb63d"></a>
## default

`function` · `object_store::client::backoff::BackoffConfig::default` · object_store 0.13.2

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [40, 1], "end": [48, 2], "filename": "src/client/backoff.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/client/backoff.rs:41`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-16af7eed492faf6449d0fdee"></a>
## fmt

`function` · `object_store::client::backoff::BackoffConfig::fmt` · object_store 0.13.2

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "object_store::client::backoff::BackoffConfig", "path": "BackoffConfig"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [30, 10], "end": [30, 15], "filename": "src/client/backoff.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/client/backoff.rs:30`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5e08392070700d6b12e9191"></a>
## init_backoff

`struct_field` · `object_store::client::backoff::BackoffConfig::init_backoff` · object_store 0.13.2

```rust
init_backoff: std::time::Duration
```

Source: `src/client/backoff.rs:33`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The initial backoff duration

<a id="op-787465497f6b03e4eef66042"></a>
## max_backoff

`struct_field` · `object_store::client::backoff::BackoffConfig::max_backoff` · object_store 0.13.2

```rust
max_backoff: std::time::Duration
```

Source: `src/client/backoff.rs:35`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The maximum backoff duration
