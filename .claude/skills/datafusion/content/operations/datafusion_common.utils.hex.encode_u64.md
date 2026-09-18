# `datafusion_common::utils::hex::encode_u64`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.encode_u64.json).

<a id="op-c667b3936cb54bde7b564ee0"></a>
## encode_u64

`function` · `datafusion_common::utils::hex::encode_u64` · datafusion-common 55.1.0

```rust
fn encode_u64(v: u64, case: HexCase, buf: &mut [u8; 16]) -> &[u8]
```

Source: `src/utils/hex.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Writes `v` as hex into `buf` and returns the written subslice.

Digits are written right-aligned with leading zeros trimmed, so the result
borrows the tail of `buf`. Zero encodes as `"0"`.

Signed values should be cast with `as u64`, which yields the two's
complement representation that both `to_hex` and Spark's `hex` produce for
negative input.

# Example

The caller owns the buffer and can reuse it across calls; each call
returns a fresh subslice of it, borrowed for as long as `buf` is:

```
use datafusion_common::utils::hex::{HexCase, encode_u64};

let mut buf = [0u8; 16];
assert_eq!(encode_u64(0xAB, HexCase::Lower, &mut buf), b"ab");
assert_eq!(encode_u64(0, HexCase::Lower, &mut buf), b"0");
```
