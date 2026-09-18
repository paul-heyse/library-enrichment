# `arrow_data::byte_view::validate_binary_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.byte_view.validate_binary_view.json).

<a id="op-80cb33b4c74239e085b8c230"></a>
## validate_binary_view

`function` · `arrow_data::byte_view::validate_binary_view` · arrow-data 59.3.0

```rust
fn validate_binary_view(views: &[u128], buffers: &[arrow_buffer::Buffer]) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/byte_view.rs:148`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates the combination of `views` and `buffers` is a valid BinaryView
