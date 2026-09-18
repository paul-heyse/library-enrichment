# `arrow_buffer::interval::IntervalDayTime`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.interval.IntervalDayTime.json).

<a id="op-e9ed0fecb92cb97c67e9f81d"></a>
## IntervalDayTime

`struct` · `arrow_buffer::interval::IntervalDayTime` · arrow-buffer 59.3.0

```rust
struct IntervalDayTime
```

Source: `src/interval.rs:350`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Value of an IntervalDayTime array

## Representation

This type is stored as a single 64 bit integer, interpreted as two i32
fields:

1. the number of elapsed days
2. The number of milliseconds (no leap seconds),

```text
┌──────────────┬──────────────┐
│     Days     │ Milliseconds │
│  (32 bits)   │  (32 bits)   │
└──────────────┴──────────────┘
0              31            63 bit offset
```

Please see the [Arrow Spec](https://github.com/apache/arrow/blob/081b4022fe6f659d8765efc82b3f4787c5039e3c/format/Schema.fbs#L406-L408) for more details

## Note on Comparing and Ordering for Calendar Types

Values of `IntervalDayTime` are compared using their binary representation,
which can lead to surprising results. Please see the description of ordering on
[`IntervalMonthDayNano`](../operations/arrow_buffer.interval.IntervalMonthDayNano.md#op-124a76e3b87e96892e283f4a) for more details

<a id="op-05af7980a6d07b61e8087d0c"></a>
## MAX

`assoc_const` · `arrow_buffer::interval::IntervalDayTime::MAX` · arrow-buffer 59.3.0

```rust
MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:368`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The maximum value that can be represented

<a id="op-96953436d60ca2e3daefe5f2"></a>
## MIN

`assoc_const` · `arrow_buffer::interval::IntervalDayTime::MIN` · arrow-buffer 59.3.0

```rust
MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:371`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The minimum value that can be represented

<a id="op-c4a8a1b398c5ba432e804afe"></a>
## MINUS_ONE

`assoc_const` · `arrow_buffer::interval::IntervalDayTime::MINUS_ONE` · arrow-buffer 59.3.0

```rust
MINUS_ONE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:365`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The multiplicative inverse, i.e. `-1`.

<a id="op-18f1680383b1f26079a7b767"></a>
## ONE

`assoc_const` · `arrow_buffer::interval::IntervalDayTime::ONE` · arrow-buffer 59.3.0

```rust
ONE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:362`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The multiplicative identity, i.e. `1`.

<a id="op-201d349e0f959bc03c535178"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [561, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7b2f16b8cd25ff57af1551"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [552, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-367a7b434f1123403b971942"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [570, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e4b58c094002a415a9f4c91"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [521, 1], "end": [533, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/interval.rs:522`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-493ff3eacc1c14a9f35c2920"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [570, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49fd5b26f683e962353cab91"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [543, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69c3c4fb8d47982690b1eb20"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [579, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bee0449066c0ffba1959d27f"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [552, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c899086625b4c30d4b397470"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [579, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3cb56deb3425ba71ebba9b2"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [561, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6eb7aceda6d1f2a03dee7c5"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalDayTime::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [543, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a5039e794f82721457be7ed"></a>
## ZERO

`assoc_const` · `arrow_buffer::interval::IntervalDayTime::ZERO` · arrow-buffer 59.3.0

```rust
ZERO
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:359`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The additive identity i.e. `0`.

<a id="op-4fd0423c7ff603208207e9e1"></a>
## add

`function` · `arrow_buffer::interval::IntervalDayTime::add` · arrow-buffer 59.3.0

```rust
fn add(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [543, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a67e82e82b742d86785495e"></a>
## add

`function` · `arrow_buffer::interval::IntervalDayTime::add` · arrow-buffer 59.3.0

```rust
fn add(self, rhs: &'a IntervalDayTime) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [543, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e404ed00e01007b65207f610"></a>
## add_assign

`function` · `arrow_buffer::interval::IntervalDayTime::add_assign` · arrow-buffer 59.3.0

```rust
fn add_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [535, 1], "end": [543, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::AddAssign", "path": "AddAssign"}, "trait_path": "core::ops::arith::AddAssign"}`

Source: `src/interval.rs:535`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3da37b48a752f1abfd32302a"></a>
## as_usize

`function` · `arrow_buffer::interval::IntervalDayTime::as_usize` · arrow-buffer 59.3.0

```rust
fn as_usize(self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:246`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-898d5db5850175d3869fa856"></a>
## checked_abs

`function` · `arrow_buffer::interval::IntervalDayTime::checked_abs` · arrow-buffer 59.3.0

```rust
fn checked_abs(self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:390`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the absolute value

<a id="op-a49ab433b532c7ed6cbb5733"></a>
## checked_add

`function` · `arrow_buffer::interval::IntervalDayTime::checked_add` · arrow-buffer 59.3.0

```rust
fn checked_add(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:426`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked addition

<a id="op-a62e0f6f7ccf8e6e1bc4ba22"></a>
## checked_div

`function` · `arrow_buffer::interval::IntervalDayTime::checked_div` · arrow-buffer 59.3.0

```rust
fn checked_div(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:478`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked division

<a id="op-91cf241f131f9b348e97cfcc"></a>
## checked_mul

`function` · `arrow_buffer::interval::IntervalDayTime::checked_mul` · arrow-buffer 59.3.0

```rust
fn checked_mul(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:461`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked multiplication

<a id="op-29af26a0a23c173277461cab"></a>
## checked_neg

`function` · `arrow_buffer::interval::IntervalDayTime::checked_neg` · arrow-buffer 59.3.0

```rust
fn checked_neg(self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:408`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Negates the value

<a id="op-f749a72d2151a49a1bc87547"></a>
## checked_pow

`function` · `arrow_buffer::interval::IntervalDayTime::checked_pow` · arrow-buffer 59.3.0

```rust
fn checked_pow(self, exp: u32) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:513`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked exponentiation

<a id="op-449b69b25e746d8e27e5c776"></a>
## checked_rem

`function` · `arrow_buffer::interval::IntervalDayTime::checked_rem` · arrow-buffer 59.3.0

```rust
fn checked_rem(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:495`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked remainder

<a id="op-2535e0ea097cd3894d425a32"></a>
## checked_sub

`function` · `arrow_buffer::interval::IntervalDayTime::checked_sub` · arrow-buffer 59.3.0

```rust
fn checked_sub(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:444`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked subtraction

<a id="op-a4c940c237928e150010ec59"></a>
## clone

`function` · `arrow_buffer::interval::IntervalDayTime::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> IntervalDayTime
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 32], "end": [348, 37], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e7d6c9b8a0823a6a216a750f"></a>
## cmp

`function` · `arrow_buffer::interval::IntervalDayTime::cmp` · arrow-buffer 59.3.0

```rust
fn cmp(&self, other: &IntervalDayTime) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 60], "end": [348, 63], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-58b2b0e0b4fa36568894decc"></a>
## days

`struct_field` · `arrow_buffer::interval::IntervalDayTime::days` · arrow-buffer 59.3.0

```rust
days: i32
```

Source: `src/interval.rs:352`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Number of days

<a id="op-4c3bfc746b657bd354e678ba"></a>
## default

`function` · `arrow_buffer::interval::IntervalDayTime::default` · arrow-buffer 59.3.0

```rust
fn default() -> IntervalDayTime
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 17], "end": [348, 24], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3655c5bb95fb3d153e52815d"></a>
## div

`function` · `arrow_buffer::interval::IntervalDayTime::div` · arrow-buffer 59.3.0

```rust
fn div(self, rhs: &'a IntervalDayTime) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [570, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f462c8c237448934c708253e"></a>
## div

`function` · `arrow_buffer::interval::IntervalDayTime::div` · arrow-buffer 59.3.0

```rust
fn div(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [570, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-236c4826213c737b7b2caa4c"></a>
## div_assign

`function` · `arrow_buffer::interval::IntervalDayTime::div_assign` · arrow-buffer 59.3.0

```rust
fn div_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [562, 1], "end": [570, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::DivAssign", "path": "DivAssign"}, "trait_path": "core::ops::arith::DivAssign"}`

Source: `src/interval.rs:562`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-667accafa0dc828d42a8b898"></a>
## eq

`function` · `arrow_buffer::interval::IntervalDayTime::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &IntervalDayTime) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 43], "end": [348, 52], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-05ca8fd5f2d12a4a22250903"></a>
## fmt

`function` · `arrow_buffer::interval::IntervalDayTime::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 10], "end": [348, 15], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3a4f2e99cb808de1b7c608da"></a>
## from_usize

`function` · `arrow_buffer::interval::IntervalDayTime::from_usize` · arrow-buffer 59.3.0

```rust
fn from_usize(_: usize) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:242`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6002e2c17909214f40dc6dd"></a>
## hash

`function` · `arrow_buffer::interval::IntervalDayTime::hash` · arrow-buffer 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 54], "end": [348, 58], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7bcfa3402b6826c59b1131a9"></a>
## milliseconds

`struct_field` · `arrow_buffer::interval::IntervalDayTime::milliseconds` · arrow-buffer 59.3.0

```rust
milliseconds: i32
```

Source: `src/interval.rs:354`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Number of milliseconds

<a id="op-83aa7c850e6b89d79dd762ea"></a>
## mul

`function` · `arrow_buffer::interval::IntervalDayTime::mul` · arrow-buffer 59.3.0

```rust
fn mul(self, rhs: &'a IntervalDayTime) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [561, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a507e703fb53e93d455ca688"></a>
## mul

`function` · `arrow_buffer::interval::IntervalDayTime::mul` · arrow-buffer 59.3.0

```rust
fn mul(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [561, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d05ad2a01f58df9e52c8579b"></a>
## mul_assign

`function` · `arrow_buffer::interval::IntervalDayTime::mul_assign` · arrow-buffer 59.3.0

```rust
fn mul_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [553, 1], "end": [561, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::MulAssign", "path": "MulAssign"}, "trait_path": "core::ops::arith::MulAssign"}`

Source: `src/interval.rs:553`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eaf7e0727033bdeafacd0bee"></a>
## neg

`function` · `arrow_buffer::interval::IntervalDayTime::neg` · arrow-buffer 59.3.0

```rust
fn neg(self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [521, 1], "end": [533, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/interval.rs:525`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d92a2fb6b2d1467ca0c49a2"></a>
## new

`function` · `arrow_buffer::interval::IntervalDayTime::new` · arrow-buffer 59.3.0

```rust
const fn new(days: i32, milliseconds: i32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:375`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`IntervalDayTime`](../operations/arrow_buffer.interval.IntervalDayTime.md#op-e9ed0fecb92cb97c67e9f81d)

<a id="op-a92c4609e3c8a52762c16587"></a>
## partial_cmp

`function` · `arrow_buffer::interval::IntervalDayTime::partial_cmp` · arrow-buffer 59.3.0

```rust
fn partial_cmp(&self, other: &IntervalDayTime) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [348, 65], "end": [348, 75], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/interval.rs:348`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1540a19e5a1fb8cb43b5ebb2"></a>
## rem

`function` · `arrow_buffer::interval::IntervalDayTime::rem` · arrow-buffer 59.3.0

```rust
fn rem(self, rhs: &'a IntervalDayTime) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [579, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e65094266cb3f0732f917bd"></a>
## rem

`function` · `arrow_buffer::interval::IntervalDayTime::rem` · arrow-buffer 59.3.0

```rust
fn rem(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [579, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10948e007aee82f558cb59f5"></a>
## rem_assign

`function` · `arrow_buffer::interval::IntervalDayTime::rem_assign` · arrow-buffer 59.3.0

```rust
fn rem_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [571, 1], "end": [579, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::RemAssign", "path": "RemAssign"}, "trait_path": "core::ops::arith::RemAssign"}`

Source: `src/interval.rs:571`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22e915d18511b319caea4447"></a>
## sub

`function` · `arrow_buffer::interval::IntervalDayTime::sub` · arrow-buffer 59.3.0

```rust
fn sub(self, rhs: &'a IntervalDayTime) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [552, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}}}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f7f74273df2d4dc59c929ca4"></a>
## sub

`function` · `arrow_buffer::interval::IntervalDayTime::sub` · arrow-buffer 59.3.0

```rust
fn sub(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [552, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78682dfa02963d14c2a5f8f3"></a>
## sub_assign

`function` · `arrow_buffer::interval::IntervalDayTime::sub_assign` · arrow-buffer 59.3.0

```rust
fn sub_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [544, 1], "end": [552, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::SubAssign", "path": "SubAssign"}, "trait_path": "core::ops::arith::SubAssign"}`

Source: `src/interval.rs:544`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b5c9dc60441dc2f458cee2a"></a>
## to_i64

`function` · `arrow_buffer::interval::IntervalDayTime::to_i64` · arrow-buffer 59.3.0

```rust
fn to_i64(self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:262`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf18bf13e9d95792aae6ddf2"></a>
## to_isize

`function` · `arrow_buffer::interval::IntervalDayTime::to_isize` · arrow-buffer 59.3.0

```rust
fn to_isize(self) -> Option<isize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:258`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-de534085da5efe672168dd11"></a>
## to_usize

`function` · `arrow_buffer::interval::IntervalDayTime::to_usize` · arrow-buffer 59.3.0

```rust
fn to_usize(self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3c297770e1f5a4dde080327"></a>
## usize_as

`function` · `arrow_buffer::interval::IntervalDayTime::usize_as` · arrow-buffer 59.3.0

```rust
fn usize_as(i: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "crate::IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [241, 1], "end": [265, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:250`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6bb3573e2db316075d0da711"></a>
## wrapping_abs

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_abs` · arrow-buffer 59.3.0

```rust
fn wrapping_abs(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:381`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the absolute value

<a id="op-bd863a906ff58df9d7001fd0"></a>
## wrapping_add

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_add` · arrow-buffer 59.3.0

```rust
fn wrapping_add(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:417`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping addition

<a id="op-72390c4d84797b8c400ad87e"></a>
## wrapping_div

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_div` · arrow-buffer 59.3.0

```rust
fn wrapping_div(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:470`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping division

<a id="op-599157bb21fc94f38fed7389"></a>
## wrapping_mul

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_mul` · arrow-buffer 59.3.0

```rust
fn wrapping_mul(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:453`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping multiplication

<a id="op-aa71dbe6b48c3637afe00d64"></a>
## wrapping_neg

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_neg` · arrow-buffer 59.3.0

```rust
fn wrapping_neg(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:399`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Negates the value

<a id="op-30d3915ae30a50f08181c768"></a>
## wrapping_pow

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_pow` · arrow-buffer 59.3.0

```rust
fn wrapping_pow(self, exp: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:504`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping exponentiation

<a id="op-fefaa689278a86a8f9829002"></a>
## wrapping_rem

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_rem` · arrow-buffer 59.3.0

```rust
fn wrapping_rem(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:487`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping remainder

<a id="op-06ed9e6b46ebcd20b76c2027"></a>
## wrapping_sub

`function` · `arrow_buffer::interval::IntervalDayTime::wrapping_sub` · arrow-buffer 59.3.0

```rust
fn wrapping_sub(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalDayTime", "path": "IntervalDayTime"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [519, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:435`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping subtraction
