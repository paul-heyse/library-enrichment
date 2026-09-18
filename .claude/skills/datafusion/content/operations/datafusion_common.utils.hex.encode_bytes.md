# `datafusion_common::utils::hex::encode_bytes`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.encode_bytes.json).

<a id="op-0197579924575dfd1749c944"></a>
## encode_bytes

`function` · `datafusion_common::utils::hex::encode_bytes` · datafusion-common 55.1.0

```rust
fn encode_bytes(bytes: &[u8], case: HexCase) -> String
```

Source: `src/utils/hex.rs:173`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Returns the hex encoding of `bytes` as an owned `String`.

# Example

```
use datafusion_common::utils::hex::{HexCase, encode_bytes};

assert_eq!(encode_bytes(&[0xde, 0xad, 0xbe, 0xef], HexCase::Lower), "deadbeef");
assert_eq!(encode_bytes(&[0xde, 0xad, 0xbe, 0xef], HexCase::Upper), "DEADBEEF");
```
