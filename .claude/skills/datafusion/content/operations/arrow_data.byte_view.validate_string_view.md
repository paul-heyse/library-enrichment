# `arrow_data::byte_view::validate_string_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.byte_view.validate_string_view.json).

<a id="op-e3c4b9595ac60d42def55c44"></a>
## validate_string_view

`function` · `arrow_data::byte_view::validate_string_view` · arrow-data 59.3.0

```rust
fn validate_string_view(views: &[u128], buffers: &[arrow_buffer::Buffer]) -> Result<(), arrow_schema::ArrowError>
```

Source: `src/byte_view.rs:153`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

Validates the combination of `views` and `buffers` is a valid StringView
