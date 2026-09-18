# `arrow_buffer::native::ToByteSlice`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.native.ToByteSlice.json).

<a id="op-1b9325701b0db97cf08b60e6"></a>
## ToByteSlice

`trait` · `arrow_buffer::native::ToByteSlice` · arrow-buffer 59.3.0

```rust
trait ToByteSlice
```

Source: `src/native.rs:268`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Allows conversion from supported Arrow types to a byte slice.

<a id="op-23d52d55c5b117419b3ec0fd"></a>
## to_byte_slice

`function` · `arrow_buffer::native::ToByteSlice::to_byte_slice` · arrow-buffer 59.3.0

```rust
fn to_byte_slice(&self) -> &[u8]
```

Source: `src/native.rs:270`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Converts this instance into a byte slice
