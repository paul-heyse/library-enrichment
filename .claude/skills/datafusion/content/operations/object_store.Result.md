# `object_store::Result`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.Result.json).

<a id="op-bf99a05eed23ad684e0979d8"></a>
## Result

`type_alias` · `object_store::Result` · object_store 0.13.2

```rust
type Result<T, E = Error> = std::result::Result<T, E>
```

Source: `src/lib.rs:2016`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

A specialized `Result` for object store-related errors
