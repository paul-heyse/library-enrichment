# `deltalake_opendal::sorted`

Full upstream contracts; raw type trees and source locators in [structured records](deltalake_opendal.sorted.json).

<a id="op-b3b1365d5fd79a7a43358184"></a>
## sorted

`module` · `deltalake_opendal::sorted` · deltalake-opendal 1.0.0+58f07cd6

Access: **internal_module**. Canonical source location is not automatically a valid import path.

```rust
mod sorted
```

[Exact source](https://github.com/delta-io/delta-rs/blob/58f07cd62bfbce3649a7e1c87c696288068ae184/crates/opendal/src/sorted.rs#L1).

Source: `crates/opendal/src/sorted.rs:1`. [Exact documentation build](https://github.com/delta-io/delta-rs/tree/58f07cd62bfbce3649a7e1c87c696288068ae184).

A wrapper that restores `object_store`'s lexicographic listing guarantee.

`OpendalStore` streams list results in whatever order the underlying OpenDAL
service returns them (e.g. filesystem readdir order for `fs`), but the
`object_store` contract — and delta-kernel's log replay — require listings
sorted by path. This wrapper buffers and sorts each listing.
