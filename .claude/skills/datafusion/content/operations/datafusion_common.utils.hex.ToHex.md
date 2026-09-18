# `datafusion_common::utils::hex::ToHex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.utils.hex.ToHex.json).

<a id="op-d9111f82f54e974124fcc586"></a>
## ToHex

`trait` · `datafusion_common::utils::hex::ToHex` · datafusion-common 55.1.0

```rust
trait ToHex: ArrowNativeType
```

Source: `src/utils/hex.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Trait for converting integer types to hexadecimal in a buffer

<a id="op-9d69f1510684d4ffb3340f5f"></a>
## write_hex

`function` · `datafusion_common::utils::hex::ToHex::write_hex` · datafusion-common 55.1.0

```rust
fn write_hex(self, case: HexCase, buf: &mut [u8; 16]) -> &[u8]
```

Source: `src/utils/hex.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Writes the hex representation into `buf` and returns the written
subslice. Digits are right-aligned with leading zeros trimmed.
