# `datafusion_common::utils::hex::encode_bytes_into`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.encode_bytes_into.json).

<a id="op-a7b9a0cc56bf8eeec2d5f1ac"></a>
## encode_bytes_into

`function` · `datafusion_common::utils::hex::encode_bytes_into` · datafusion-common 55.1.0

```rust
fn encode_bytes_into(bytes: &[u8], case: HexCase, out: &mut Vec<u8>)
```

Source: `src/utils/hex.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Appends the hex encoding of `bytes` to `out`.

Allocates only through `out`'s own growth. Callers that must bound or guard
that growth should reserve capacity in `out` before calling.
