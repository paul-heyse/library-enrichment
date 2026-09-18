# `object_store::Error::PermissionDenied`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.PermissionDenied.json).

<a id="op-d398ce55ed5bac977bcd516b"></a>
## path

`struct_field` · `object_store::Error::PermissionDenied::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2113`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to the file

<a id="op-c35871845d186a5dc490cdf2"></a>
## source

`struct_field` · `object_store::Error::PermissionDenied::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2115`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
