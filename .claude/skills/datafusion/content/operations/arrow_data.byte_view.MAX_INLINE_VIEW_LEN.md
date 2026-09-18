# `arrow_data::byte_view::MAX_INLINE_VIEW_LEN`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_data.byte_view.MAX_INLINE_VIEW_LEN.json).

<a id="op-cc5752925eeccf8fdeefe7ed"></a>
## MAX_INLINE_VIEW_LEN

`constant` · `arrow_data::byte_view::MAX_INLINE_VIEW_LEN` · arrow-data 59.3.0

```rust
const MAX_INLINE_VIEW_LEN: u32 = 12
```

Source: `src/byte_view.rs:27`. [Exact documentation build](https://docs.rs/crate/arrow-data/59.3.0/json).

The maximum number of bytes that can be stored inline in a byte view.

See [`ByteView`](../operations/arrow_data.byte_view.ByteView.md#op-f50179c7bea821afbdfdb4ae) and [`GenericByteViewArray`] for more information on the
layout of the views.

[`GenericByteViewArray`]: https://docs.rs/arrow/latest/arrow/array/struct.GenericByteViewArray.html
