# `arrow_buffer::util::bit_iterator::try_for_each_valid_idx`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.util.bit_iterator.try_for_each_valid_idx.json).

<a id="op-ae5f9121e2bd1495402f58c5"></a>
## try_for_each_valid_idx

`function` · `arrow_buffer::util::bit_iterator::try_for_each_valid_idx` · arrow-buffer 59.3.0

```rust
fn try_for_each_valid_idx<E, F: FnMut(usize) -> Result<(), E>>(len: usize, offset: usize, null_count: usize, nulls: Option<&[u8]>, f: F) -> Result<(), E>
```

Source: `src/util/bit_iterator.rs:392`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Calls the provided closure for each index in the provided null mask that is set,
using an adaptive strategy based on the null count

Ideally this would be encapsulated in an [`Iterator`] that would determine the optimal
strategy up front, and then yield indexes based on this.

Unfortunately, external iteration based on the resulting [`Iterator`] would match the strategy
variant on each call to [`Iterator::next`], and LLVM generally cannot eliminate this.

One solution to this might be internal iteration, e.g. [`Iterator::try_fold`], however,
it is currently [not possible] to override this for custom iterators in stable Rust.

As such this is the next best option

[not possible]: https://github.com/rust-lang/rust/issues/69595

Unresolved upstream links (retained, not inferred): ``Iterator``, ``Iterator::next``, ``Iterator::try_fold``.
