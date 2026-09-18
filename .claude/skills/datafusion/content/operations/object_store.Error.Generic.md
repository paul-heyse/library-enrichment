# `object_store::Error::Generic`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.Generic.json).

<a id="op-85a21fd83fb67dc3b83db55d"></a>
## source

`struct_field` · `object_store::Error::Generic::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2028`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error

<a id="op-665706ef183d80c71233d183"></a>
## store

`struct_field` · `object_store::Error::Generic::store` · object_store 0.13.2

```rust
store: &'static str
```

Source: `src/lib.rs:2026`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The store this error originated from
