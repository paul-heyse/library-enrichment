# `datafusion_common::utils::hex::encode_bytes_to_slice`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.encode_bytes_to_slice.json).

<a id="op-b0457388849a130997225de7"></a>
## encode_bytes_to_slice

`function` · `datafusion_common::utils::hex::encode_bytes_to_slice` · datafusion-common 55.1.0

```rust
fn encode_bytes_to_slice(bytes: &[u8], case: HexCase, out: &mut [u8]) -> Result<()>
```

Source: `src/utils/hex.rs:147`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Writes the hex encoding of `bytes` into `out`.

This is for callers that already own a pre-sized buffer (for example a
slice of a larger, pre-allocated output array) and want to write directly
into it rather than appending to a `Vec`.

Returns an internal error if `out` is not exactly `2 * bytes.len()` bytes
long, without filling any of the `out` buffer.

# Example

```
use datafusion_common::utils::hex::{HexCase, encode_bytes_to_slice};

let mut out = [0u8; 8];
encode_bytes_to_slice(&[0xde, 0xad, 0xbe, 0xef], HexCase::Lower, &mut out)?;
assert_eq!(&out, b"deadbeef");
# Ok::<(), datafusion_common::DataFusionError>(())
```
