# `arrow_buffer::native::ArrowNativeType`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.native.ArrowNativeType.json).

<a id="op-5f78ae59d708cee9753802c9"></a>
## ArrowNativeType

`trait` · `arrow_buffer::native::ArrowNativeType` · arrow-buffer 59.3.0

```rust
trait ArrowNativeType: std::fmt::Debug + Send + Sync + Copy + PartialOrd + Default + private::Sealed + 'static
```

Source: `src/native.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Trait expressing a Rust type that has the same in-memory representation as
Arrow.

This includes `i16`, `f32`, but excludes `bool` (which in arrow is
represented in bits).

In little endian machines, types that implement [`ArrowNativeType`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) can be
memcopied to arrow buffers as is.

# Transmute Safety

A type T implementing this trait means that any arbitrary slice of bytes of length and
alignment `size_of::<T>()` can be safely interpreted as a value of that type without
being unsound, i.e. potentially resulting in undefined behaviour.

Note: in the case of floating point numbers this transmutation can result in a signalling
NaN, which, whilst sound, can be unwieldy. In general, whilst it is perfectly sound to
reinterpret bytes as different types using this trait, it is likely unwise. For more information
see [f32::from_bits] and [f64::from_bits].

Note: `bool` is restricted to `0` or `1`, and so `bool: !ArrowNativeType`

# Sealed

Due to the above restrictions, this trait is sealed to prevent accidental misuse

Unresolved upstream links (retained, not inferred): `f32::from_bits`, `f64::from_bits`.

<a id="op-1489d0e4d6037abb95f78c7b"></a>
## as_usize

`function` · `arrow_buffer::native::ArrowNativeType::as_usize` · arrow-buffer 59.3.0

```rust
fn as_usize(self) -> usize
```

Source: `src/native.rs:67`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert to usize according to the [`as`] operator

[`as`]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast

<a id="op-674306174389f9c336c6ac9f"></a>
## from_usize

`function` · `arrow_buffer::native::ArrowNativeType::from_usize` · arrow-buffer 59.3.0

```rust
fn from_usize(_: usize) -> Option<Self>
```

Source: `src/native.rs:62`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert native integer type from usize

Returns `None` if [`Self`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) is not an integer or conversion would result
in truncation/overflow

<a id="op-4945ea5f3e4e5fe60419b99f"></a>
## get_byte_width

`function` · `arrow_buffer::native::ArrowNativeType::get_byte_width` · arrow-buffer 59.3.0

```rust
fn get_byte_width() -> usize
```

Source: `src/native.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Returns the byte width of this native type.

<a id="op-67b689bcc7ee60ba6b968141"></a>
## to_i64

`function` · `arrow_buffer::native::ArrowNativeType::to_i64` · arrow-buffer 59.3.0

```rust
fn to_i64(self) -> Option<i64>
```

Source: `src/native.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert native type to i64.

Returns `None` if [`Self`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) is not an integer or conversion would result
in truncation/overflow

<a id="op-98aff91df0752e3dfeb0898d"></a>
## to_isize

`function` · `arrow_buffer::native::ArrowNativeType::to_isize` · arrow-buffer 59.3.0

```rust
fn to_isize(self) -> Option<isize>
```

Source: `src/native.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert native type to isize.

Returns `None` if [`Self`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) is not an integer or conversion would result
in truncation/overflow

<a id="op-9bb6f0cf26fc527ea7eb5f10"></a>
## to_usize

`function` · `arrow_buffer::native::ArrowNativeType::to_usize` · arrow-buffer 59.3.0

```rust
fn to_usize(self) -> Option<usize>
```

Source: `src/native.rs:78`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert native type to usize.

Returns `None` if [`Self`](../operations/arrow_buffer.native.ArrowNativeType.md#op-5f78ae59d708cee9753802c9) is not an integer or conversion would result
in truncation/overflow

<a id="op-effd1e90f73c93ac8bdb643d"></a>
## usize_as

`function` · `arrow_buffer::native::ArrowNativeType::usize_as` · arrow-buffer 59.3.0

```rust
fn usize_as(i: usize) -> Self
```

Source: `src/native.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Convert from usize according to the [`as`] operator

[`as`]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast
