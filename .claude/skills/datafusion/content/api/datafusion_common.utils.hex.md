# `datafusion_common::utils::hex`

Crate `datafusion-common` · 6 public items · structured records in [`model/datafusion_common.utils.hex.json`](../model/datafusion_common.utils.hex.json)

## HexCase

`enum` · `datafusion_common::utils::hex::HexCase`

```rust
enum HexCase
```

**Variants**: `Lower`, `Upper`

**Derives**: Clone, Copy, Debug, Eq, PartialEq, StructuralPartialEq

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.HexCase.md).


Case of the emitted hex digits.

---

## encode_bytes

`function` · `datafusion_common::utils::hex::encode_bytes`

```rust
fn encode_bytes(bytes: &[u8], case: HexCase) -> String
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.encode_bytes.md).


Returns the hex encoding of `bytes` as an owned `String`.

# Example

```
use datafusion_common::utils::hex::{HexCase, encode_bytes};

assert_eq!(encode_bytes(&[0xde, 0xad, 0xbe, 0xef], HexCase::Lower), "deadbeef");
assert_eq!(encode_bytes(&[0xde, 0xad, 0xbe, 0xef], HexCase::Upper), "DEADBEEF");
```

---

## encode_bytes_into

`function` · `datafusion_common::utils::hex::encode_bytes_into`

```rust
fn encode_bytes_into(bytes: &[u8], case: HexCase, out: &mut Vec<u8>)
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.encode_bytes_into.md).


Appends the hex encoding of `bytes` to `out`.

Allocates only through `out`'s own growth. Callers that must bound or guard
that growth should reserve capacity in `out` before calling.

---

## encode_bytes_to_slice

`function` · `datafusion_common::utils::hex::encode_bytes_to_slice`

```rust
fn encode_bytes_to_slice(bytes: &[u8], case: HexCase, out: &mut [u8]) -> Result<()>
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.encode_bytes_to_slice.md).


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

---

## encode_u64

`function` · `datafusion_common::utils::hex::encode_u64`

```rust
fn encode_u64(v: u64, case: HexCase, buf: &mut [u8; 16]) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.encode_u64.md).


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

---

## ToHex

`trait` · `datafusion_common::utils::hex::ToHex`

```rust
trait ToHex: ArrowNativeType
```

**Methods** (1)

```rust
fn write_hex(self, case: HexCase, buf: &mut [u8; 16]) -> &[u8]
```

[Full member, field, variant and typed contracts](../operations/datafusion_common.utils.hex.ToHex.md).


Trait for converting integer types to hexadecimal in a buffer

---
