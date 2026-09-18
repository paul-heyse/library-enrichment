# `object_store::Error::AlreadyExists`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.AlreadyExists.json).

<a id="op-f9bbb70876055a4f64716081"></a>
## path

`struct_field` · `object_store::Error::AlreadyExists::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2068`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to the

<a id="op-2235044ab693076dc963cd78"></a>
## source

`struct_field` · `object_store::Error::AlreadyExists::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2070`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
