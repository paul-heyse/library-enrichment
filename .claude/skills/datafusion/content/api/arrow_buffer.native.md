# `arrow_buffer::native`

Crate `arrow-buffer` · 2 public items · structured records in [`model/arrow_buffer.native.json`](../model/arrow_buffer.native.json)

## ArrowNativeType

`trait` · `arrow_buffer::native::ArrowNativeType`

Also reachable as `arrow::datatypes::ArrowNativeType`

```rust
trait ArrowNativeType: std::fmt::Debug + Send + Sync + Copy + PartialOrd + Default + private::Sealed + 'static
```

**Implementors** (4)

- `arrow_buffer::bigint::i256`
- `arrow_buffer::interval::IntervalDayTime`
- `arrow_buffer::interval::IntervalMonthDayNano`
- `half::binary16::f16`

**Methods** (7)

```rust
fn as_usize(self) -> usize
fn from_usize(_: usize) -> Option<Self>
fn get_byte_width() -> usize
fn to_i64(self) -> Option<i64>
fn to_isize(self) -> Option<isize>
fn to_usize(self) -> Option<usize>
fn usize_as(i: usize) -> Self
```

Trait expressing a Rust type that has the same in-memory representation as
Arrow.

This includes `i16`, `f32`, but excludes `bool` (which in arrow is
represented in bits).

In little endian machines, types that implement [`ArrowNativeType`] can be
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

---

## ToByteSlice

`trait` · `arrow_buffer::native::ToByteSlice`

Also reachable as `arrow::datatypes::ToByteSlice`

```rust
trait ToByteSlice
```

**Methods** (1)

```rust
fn to_byte_slice(&self) -> &[u8]
```

Allows conversion from supported Arrow types to a byte slice.

---
