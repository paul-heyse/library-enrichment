# `datafusion_common::utils::adjust_offsets_for_slice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.adjust_offsets_for_slice.json).

<a id="op-8443ee3fd14581f1634f4931"></a>
## adjust_offsets_for_slice

`function` · `datafusion_common::utils::adjust_offsets_for_slice` · datafusion-common 55.1.0

```rust
fn adjust_offsets_for_slice<O: OffsetSizeTrait>(list: &arrow::array::GenericListArray<O>) -> arrow::buffer::OffsetBuffer<O>
```

Source: `src/utils/mod.rs:1235`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

If `list` is sliced, returns an adjusted offset buffer so that
it points to the sliced portion of the list values, and not the whole list values
