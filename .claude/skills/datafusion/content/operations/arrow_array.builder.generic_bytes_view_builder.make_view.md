# `arrow_array::builder::generic_bytes_view_builder::make_view`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_array.builder.generic_bytes_view_builder.make_view.json).

<a id="op-fba67198295fcc13b7f656ac"></a>
## make_view

`function` · `arrow_array::builder::generic_bytes_view_builder::make_view` · arrow-array 59.3.0

```rust
fn make_view(data: &[u8], block_id: u32, offset: u32) -> u128
```

Source: `src/builder/generic_bytes_view_builder.rs:676`. [Exact documentation build](https://docs.rs/crate/arrow-array/59.3.0/json).

Create a view based on the given data, block id and offset.

Note that the code below is carefully examined with x86_64 assembly code: <https://godbolt.org/z/685YPsd5G>
The goal is to avoid calling into `ptr::copy_non_interleave`, which makes function call (i.e., not inlined),
which slows down things.
