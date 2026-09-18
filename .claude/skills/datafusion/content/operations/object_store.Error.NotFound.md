# `object_store::Error::NotFound`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.NotFound.json).

<a id="op-bc0d97e9d3198c804a367fa7"></a>
## path

`struct_field` · `object_store::Error::NotFound::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2035`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to file

<a id="op-4f9c3f221780f6e64a701c4d"></a>
## source

`struct_field` · `object_store::Error::NotFound::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2037`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
