# `arrow_json::reader::tape::TapeElement`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_json.reader.tape.TapeElement.json).

<a id="op-ce1f8ae0600b580b812744b2"></a>
## TapeElement

`enum` · `arrow_json::reader::tape::TapeElement` · arrow-json 59.3.0

```rust
enum TapeElement
```

Source: `src/reader/tape.rs:34`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

We decode JSON to a flattened tape representation,
allowing for efficient traversal of the JSON data

This approach is inspired by [simdjson]

Uses `u32` for offsets to ensure `TapeElement` is 64-bits. A future
iteration may increase this to a custom `u56` type.

[simdjson]: https://github.com/simdjson/simdjson/blob/master/doc/tape.md

<a id="op-a035ce000b14111abfdf62d0"></a>
## EndList

`variant` · `arrow_json::reader::tape::TapeElement::EndList` · arrow-json 59.3.0

```rust
EndList
```

Source: `src/reader/tape.rs:50`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The end of a list , i.e. `]`

Contains the offset of the corresponding [`Self::StartList`]

<a id="op-9397330fbc3e1e216e5b9bb9"></a>
## EndObject

`variant` · `arrow_json::reader::tape::TapeElement::EndObject` · arrow-json 59.3.0

```rust
EndObject
```

Source: `src/reader/tape.rs:42`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The end of an object, i.e. `}`

Contains the offset of the corresponding [`Self::StartObject`]

<a id="op-8b43fe1890589b1f229cd607"></a>
## F32

`variant` · `arrow_json::reader::tape::TapeElement::F32` · arrow-json 59.3.0

```rust
F32
```

Source: `src/reader/tape.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A 32-bit float or the low-bits of a 64-bit float if preceded by [`Self::F64`]

<a id="op-4d3d54b543dd875e20cba62d"></a>
## F64

`variant` · `arrow_json::reader::tape::TapeElement::F64` · arrow-json 59.3.0

```rust
F64
```

Source: `src/reader/tape.rs:73`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The high bits of a 64-bit float

Followed by [`Self::F32`] containing the low bits

<a id="op-ae97bb4cb8e4bb0869e3f73a"></a>
## False

`variant` · `arrow_json::reader::tape::TapeElement::False` · arrow-json 59.3.0

```rust
False
```

Source: `src/reader/tape.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A false literal

<a id="op-b8d8cd4219f726873b2f1d56"></a>
## I32

`variant` · `arrow_json::reader::tape::TapeElement::I32` · arrow-json 59.3.0

```rust
I32
```

Source: `src/reader/tape.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A 32-bit signed integer

May be preceded by [`Self::I64`] containing high bits

<a id="op-c2465cd1058e5a027c7dd3af"></a>
## I64

`variant` · `arrow_json::reader::tape::TapeElement::I64` · arrow-json 59.3.0

```rust
I64
```

Source: `src/reader/tape.rs:63`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The high bits of a i64

Followed by [`Self::I32`] containing the low bits

<a id="op-b5bc158b5e044d63ee0784a6"></a>
## Null

`variant` · `arrow_json::reader::tape::TapeElement::Null` · arrow-json 59.3.0

```rust
Null
```

Source: `src/reader/tape.rs:83`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A null literal

<a id="op-489468993ee3a81cfe90f60f"></a>
## Number

`variant` · `arrow_json::reader::tape::TapeElement::Number` · arrow-json 59.3.0

```rust
Number
```

Source: `src/reader/tape.rs:58`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A numeric value

Contains the offset into the [`Tape`] string data

<a id="op-67c8419735dd3edf19def253"></a>
## StartList

`variant` · `arrow_json::reader::tape::TapeElement::StartList` · arrow-json 59.3.0

```rust
StartList
```

Source: `src/reader/tape.rs:46`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The start of a list , i.e. `[`

Contains the offset of the corresponding [`Self::EndList`]

<a id="op-ad7702bd0447f2706ca66991"></a>
## StartObject

`variant` · `arrow_json::reader::tape::TapeElement::StartObject` · arrow-json 59.3.0

```rust
StartObject
```

Source: `src/reader/tape.rs:38`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

The start of an object, i.e. `{`

Contains the offset of the corresponding [`Self::EndObject`]

<a id="op-13fe6418499a3a35232a9195"></a>
## String

`variant` · `arrow_json::reader::tape::TapeElement::String` · arrow-json 59.3.0

```rust
String
```

Source: `src/reader/tape.rs:54`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A string value

Contains the offset into the [`Tape`] string data

<a id="op-e6502ec896361f0b2d311853"></a>
## True

`variant` · `arrow_json::reader::tape::TapeElement::True` · arrow-json 59.3.0

```rust
True
```

Source: `src/reader/tape.rs:79`. [Exact documentation build](https://docs.rs/crate/arrow-json/59.3.0/json).

A true literal
