# `datafusion_common::utils::hex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.json).

<a id="op-c369abad4fb7ce3b5800b94c"></a>
## hex

`module` · `datafusion_common::utils::hex` · datafusion-common 55.1.0

```rust
mod hex
```

Source: `src/utils/hex.rs:18`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Hex encoding of bytes and integers.

[`encode_bytes`](../operations/datafusion_common.utils.hex.encode_bytes.md#op-0197579924575dfd1749c944) and [`encode_bytes_into`](../operations/datafusion_common.utils.hex.encode_bytes_into.md#op-a7b9a0cc56bf8eeec2d5f1ac) encode a byte slice into an
owned `String` or an appended `Vec<u8>`, respectively; [`encode_bytes_to_slice`](../operations/datafusion_common.utils.hex.encode_bytes_to_slice.md#op-b0457388849a130997225de7)
writes into a caller-provided, pre-sized buffer. [`encode_u64`](../operations/datafusion_common.utils.hex.encode_u64.md#op-c667b3936cb54bde7b564ee0) encodes an
integer, trimming leading zeros. All four take a [`HexCase`](../operations/datafusion_common.utils.hex.HexCase.md#op-e9733982d2c2f0976780952d) to choose
between lowercase and uppercase digits.
