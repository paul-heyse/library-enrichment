# `object_store::Error::Unauthenticated`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Error.Unauthenticated.json).

<a id="op-24b2337b5637aac6a800bb89"></a>
## path

`struct_field` · `object_store::Error::Unauthenticated::path` · object_store 0.13.2

```rust
path: String
```

Source: `src/lib.rs:2126`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The path to the file

<a id="op-0c4f584ddefe49078bb78f8d"></a>
## source

`struct_field` · `object_store::Error::Unauthenticated::source` · object_store 0.13.2

```rust
source: Box<dyn std::error::Error + Send + Sync + 'static>
```

Source: `src/lib.rs:2128`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

The wrapped error
