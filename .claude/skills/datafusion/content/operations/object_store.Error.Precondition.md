# `object_store::Error::Precondition`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.Precondition.json).

<a id="op-91496a60d443187601080e14"></a>
## path

`struct_field` · `object_store::Error::Precondition::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2077`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to the file

<a id="op-26941a4ea1b3bb5e2fb08149"></a>
## source

`struct_field` · `object_store::Error::Precondition::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2079`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
