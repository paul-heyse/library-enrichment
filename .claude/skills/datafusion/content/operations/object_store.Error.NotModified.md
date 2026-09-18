# `object_store::Error::NotModified`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.NotModified.json).

<a id="op-ec655b9343b70eaf3a90891d"></a>
## path

`struct_field` · `object_store::Error::NotModified::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2086`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to the file

<a id="op-2505849e3af3c47879222066"></a>
## source

`struct_field` · `object_store::Error::NotModified::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2088`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
