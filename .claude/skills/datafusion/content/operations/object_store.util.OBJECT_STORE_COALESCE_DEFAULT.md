# `object_store::util::OBJECT_STORE_COALESCE_DEFAULT`

Full upstream contracts; raw type trees and source locators in [structured records](object_store.util.OBJECT_STORE_COALESCE_DEFAULT.json).

<a id="op-893435d02142513417767dc6"></a>
## OBJECT_STORE_COALESCE_DEFAULT

`constant` · `object_store::util::OBJECT_STORE_COALESCE_DEFAULT` · object_store 0.13.2

```rust
const OBJECT_STORE_COALESCE_DEFAULT: u64 = _
```

Source: `src/util.rs:92`. [Exact documentation build](https://docs.rs/crate/object_store/0.13.2/json).

Range requests with a gap less than or equal to this,
will be coalesced into a single request by [`coalesce_ranges`](../operations/object_store.util.coalesce_ranges.md#op-c80863c21948960e5fcda945)
