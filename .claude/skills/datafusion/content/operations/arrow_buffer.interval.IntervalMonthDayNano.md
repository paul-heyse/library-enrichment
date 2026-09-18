# `arrow_buffer::interval::IntervalMonthDayNano`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_buffer.interval.IntervalMonthDayNano.json).

<a id="op-124a76e3b87e96892e283f4a"></a>
## IntervalMonthDayNano

`struct` · `arrow_buffer::interval::IntervalMonthDayNano` · arrow-buffer 59.3.0

```rust
struct IntervalMonthDayNano
```

Source: `src/interval.rs:70`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

 Value of an IntervalMonthDayNano array

  ## Representation

 This type is stored as a single 128 bit integer, interpreted as three
 different signed integral fields:

 1. The number of months (32 bits)
 2. The number days (32 bits)
 2. The number of nanoseconds (64 bits).

 Nanoseconds does not allow for leap seconds.

 Each field is independent (e.g. there is no constraint that the quantity of
 nanoseconds represents less than a day's worth of time).

 ```text
 ┌───────────────┬─────────────┬─────────────────────────────┐
 │     Months    │     Days    │            Nanos            │
 │   (32 bits)   │  (32 bits)  │          (64 bits)          │
 └───────────────┴─────────────┴─────────────────────────────┘
 0            32             64                           128 bit offset
 ```
 Please see the [Arrow Spec](https://github.com/apache/arrow/blob/081b4022fe6f659d8765efc82b3f4787c5039e3c/format/Schema.fbs#L409-L415) for more details

## Note on Comparing and Ordering for Calendar Types

 Values of `IntervalMonthDayNano` are compared using their binary
 representation, which can lead to surprising results.

 Spans of time measured in calendar units are not fixed in absolute size (e.g.
 number of seconds) which makes defining comparisons and ordering non trivial.
 For example `1 month` is 28 days for February but `1 month` is 31 days
 in December.

 This makes the seemingly simple operation of comparing two intervals
 complicated in practice. For example is `1 month` more or less than `30
 days`? The answer depends on what month you are talking about.

 This crate defines comparisons for calendar types using their binary
 representation which is fast and efficient, but leads
 to potentially surprising results.

 For example a
 `IntervalMonthDayNano` of `1 month` will compare as **greater** than a
 `IntervalMonthDayNano` of `100 days` because the binary representation of `1 month`
 is larger than the binary representation of 100 days.

<a id="op-6a0df0a182bc5015c233fcf9"></a>
## MAX

`assoc_const` · `arrow_buffer::interval::IntervalMonthDayNano::MAX` · arrow-buffer 59.3.0

```rust
MAX
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:90`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The maximum value that can be represented

<a id="op-4e6ec94692441785f714a81c"></a>
## MIN

`assoc_const` · `arrow_buffer::interval::IntervalMonthDayNano::MIN` · arrow-buffer 59.3.0

```rust
MIN
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:93`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The minimum value that can be represented

<a id="op-d05458a5234852aafd07c14f"></a>
## MINUS_ONE

`assoc_const` · `arrow_buffer::interval::IntervalMonthDayNano::MINUS_ONE` · arrow-buffer 59.3.0

```rust
MINUS_ONE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:87`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The multiplicative inverse, i.e. `-1`.

<a id="op-3bea2819201845c78e988d05"></a>
## ONE

`assoc_const` · `arrow_buffer::interval::IntervalMonthDayNano::ONE` · arrow-buffer 59.3.0

```rust
ONE
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:84`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The multiplicative identity, i.e. `1`.

<a id="op-0053b443a62eabdf33b46202"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00f3938e179d329fe8bc1d66"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eee44bbffb9b01f5a2134f3"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [275, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/interval.rs:264`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4521e12a08d9dde58f970d93"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63f44e56b16462d9a16a9348"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [303, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-78868d0f93e7b847607583eb"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [303, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86dbc849827366479795ebcc"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [312, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bda2d8c8f3249dde1b43b2f7"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [285, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d14d44855f5e9c007742a3d4"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [312, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f96188fcd4cf25c8197543f7"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [285, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9c9eda480e1876324434e57"></a>
## Output

`assoc_type` · `arrow_buffer::interval::IntervalMonthDayNano::Output` · arrow-buffer 59.3.0

```rust
Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e681cc1c000191d8da90ac4e"></a>
## ZERO

`assoc_const` · `arrow_buffer::interval::IntervalMonthDayNano::ZERO` · arrow-buffer 59.3.0

```rust
ZERO
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:81`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

The additive identity i.e. `0`.

<a id="op-1ab17a96d2f89611476c5f66"></a>
## add

`function` · `arrow_buffer::interval::IntervalMonthDayNano::add` · arrow-buffer 59.3.0

```rust
fn add(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [285, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ef75ea3095160c315791b2c3"></a>
## add

`function` · `arrow_buffer::interval::IntervalMonthDayNano::add` · arrow-buffer 59.3.0

```rust
fn add(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [285, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Add", "path": "Add"}, "trait_path": "core::ops::arith::Add"}`

Source: `src/interval.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2cd7df18d2902c4fecff6175"></a>
## add_assign

`function` · `arrow_buffer::interval::IntervalMonthDayNano::add_assign` · arrow-buffer 59.3.0

```rust
fn add_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [277, 1], "end": [285, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::AddAssign", "path": "AddAssign"}, "trait_path": "core::ops::arith::AddAssign"}`

Source: `src/interval.rs:277`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c2185be9333be77d22710e7"></a>
## as_usize

`function` · `arrow_buffer::interval::IntervalMonthDayNano::as_usize` · arrow-buffer 59.3.0

```rust
fn as_usize(self) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:219`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-183af128a5a51959ef9d0c86"></a>
## checked_abs

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_abs` · arrow-buffer 59.3.0

```rust
fn checked_abs(self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:117`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the absolute value

<a id="op-188ba3e228a018e2aef7bae6"></a>
## checked_add

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_add` · arrow-buffer 59.3.0

```rust
fn checked_add(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:157`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked addition

<a id="op-14593d4ced4b3dbb90d689e3"></a>
## checked_div

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_div` · arrow-buffer 59.3.0

```rust
fn checked_div(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:215`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked division

<a id="op-f592314e5c04730af660ca6c"></a>
## checked_mul

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_mul` · arrow-buffer 59.3.0

```rust
fn checked_mul(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:196`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked multiplication

<a id="op-462f5a3ade476c468059b31b"></a>
## checked_neg

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_neg` · arrow-buffer 59.3.0

```rust
fn checked_neg(self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:137`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Negates the value

<a id="op-e4b8c9fcfda20fdb2bbda905"></a>
## checked_pow

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_pow` · arrow-buffer 59.3.0

```rust
fn checked_pow(self, exp: u32) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:254`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked exponentiation

<a id="op-f3db4366daa01de6ac02f205"></a>
## checked_rem

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_rem` · arrow-buffer 59.3.0

```rust
fn checked_rem(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:234`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked remainder

<a id="op-eb9149a1156f996fcace40df"></a>
## checked_sub

`function` · `arrow_buffer::interval::IntervalMonthDayNano::checked_sub` · arrow-buffer 59.3.0

```rust
fn checked_sub(self, other: Self) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:177`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs checked subtraction

<a id="op-395b519ad4d164726203fd2b"></a>
## clone

`function` · `arrow_buffer::interval::IntervalMonthDayNano::clone` · arrow-buffer 59.3.0

```rust
fn clone(&self) -> IntervalMonthDayNano
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 32], "end": [68, 37], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-63e4095faddea9a90494a545"></a>
## cmp

`function` · `arrow_buffer::interval::IntervalMonthDayNano::cmp` · arrow-buffer 59.3.0

```rust
fn cmp(&self, other: &IntervalMonthDayNano) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 60], "end": [68, 63], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-723cdb31bdffe867c619a656"></a>
## days

`struct_field` · `arrow_buffer::interval::IntervalMonthDayNano::days` · arrow-buffer 59.3.0

```rust
days: i32
```

Source: `src/interval.rs:74`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Number of days

<a id="op-c6c27f2e8a340fd713a16a8e"></a>
## default

`function` · `arrow_buffer::interval::IntervalMonthDayNano::default` · arrow-buffer 59.3.0

```rust
fn default() -> IntervalMonthDayNano
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 17], "end": [68, 24], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1e2336988b7017561f376451"></a>
## div

`function` · `arrow_buffer::interval::IntervalMonthDayNano::div` · arrow-buffer 59.3.0

```rust
fn div(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [312, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1f1481a571cfa5d22ff278d"></a>
## div

`function` · `arrow_buffer::interval::IntervalMonthDayNano::div` · arrow-buffer 59.3.0

```rust
fn div(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [312, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Div", "path": "Div"}, "trait_path": "core::ops::arith::Div"}`

Source: `src/interval.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c000372817c432bbfe6b26a0"></a>
## div_assign

`function` · `arrow_buffer::interval::IntervalMonthDayNano::div_assign` · arrow-buffer 59.3.0

```rust
fn div_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [304, 1], "end": [312, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::DivAssign", "path": "DivAssign"}, "trait_path": "core::ops::arith::DivAssign"}`

Source: `src/interval.rs:304`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-af459d4f671ca058fb38f747"></a>
## eq

`function` · `arrow_buffer::interval::IntervalMonthDayNano::eq` · arrow-buffer 59.3.0

```rust
fn eq(&self, other: &IntervalMonthDayNano) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 43], "end": [68, 52], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7853fbc8052c8e7b39e9bda2"></a>
## fmt

`function` · `arrow_buffer::interval::IntervalMonthDayNano::fmt` · arrow-buffer 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 10], "end": [68, 15], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32eadf0f3c8cf0a94ace3b87"></a>
## from_usize

`function` · `arrow_buffer::interval::IntervalMonthDayNano::from_usize` · arrow-buffer 59.3.0

```rust
fn from_usize(_: usize) -> Option<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:215`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad45de2a025326d6139a176e"></a>
## hash

`function` · `arrow_buffer::interval::IntervalMonthDayNano::hash` · arrow-buffer 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 54], "end": [68, 58], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-202d2fd43d960dbaf3cbbffb"></a>
## months

`struct_field` · `arrow_buffer::interval::IntervalMonthDayNano::months` · arrow-buffer 59.3.0

```rust
months: i32
```

Source: `src/interval.rs:72`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Number of months

<a id="op-4fd24d2fc67c12c329bd4fb4"></a>
## mul

`function` · `arrow_buffer::interval::IntervalMonthDayNano::mul` · arrow-buffer 59.3.0

```rust
fn mul(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [303, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-959068a61431a463250fdbe7"></a>
## mul

`function` · `arrow_buffer::interval::IntervalMonthDayNano::mul` · arrow-buffer 59.3.0

```rust
fn mul(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [303, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Mul", "path": "Mul"}, "trait_path": "core::ops::arith::Mul"}`

Source: `src/interval.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-faa27dfcfd7576e480517805"></a>
## mul_assign

`function` · `arrow_buffer::interval::IntervalMonthDayNano::mul_assign` · arrow-buffer 59.3.0

```rust
fn mul_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [295, 1], "end": [303, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::MulAssign", "path": "MulAssign"}, "trait_path": "core::ops::arith::MulAssign"}`

Source: `src/interval.rs:295`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-171756cbc036fa7aee5a0adc"></a>
## nanoseconds

`struct_field` · `arrow_buffer::interval::IntervalMonthDayNano::nanoseconds` · arrow-buffer 59.3.0

```rust
nanoseconds: i64
```

Source: `src/interval.rs:76`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Number of nanoseconds

<a id="op-654f3d3840d0e317e0de9ef2"></a>
## neg

`function` · `arrow_buffer::interval::IntervalMonthDayNano::neg` · arrow-buffer 59.3.0

```rust
fn neg(self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [263, 1], "end": [275, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Neg", "path": "Neg"}, "trait_path": "core::ops::arith::Neg"}`

Source: `src/interval.rs:267`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-56d04ca932cfd0c8f8b9a1e6"></a>
## new

`function` · `arrow_buffer::interval::IntervalMonthDayNano::new` · arrow-buffer 59.3.0

```rust
const fn new(months: i32, days: i32, nanoseconds: i64) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:97`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Create a new [`IntervalMonthDayNano`](../operations/arrow_buffer.interval.IntervalMonthDayNano.md#op-124a76e3b87e96892e283f4a)

<a id="op-a478a4a510c5d1df0bc9dee7"></a>
## partial_cmp

`function` · `arrow_buffer::interval::IntervalMonthDayNano::partial_cmp` · arrow-buffer 59.3.0

```rust
fn partial_cmp(&self, other: &IntervalMonthDayNano) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 65], "end": [68, 75], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/interval.rs:68`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-868b4037e14f33d923d4e042"></a>
## rem

`function` · `arrow_buffer::interval::IntervalMonthDayNano::rem` · arrow-buffer 59.3.0

```rust
fn rem(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95ffd14559a17eb195c8e35c"></a>
## rem

`function` · `arrow_buffer::interval::IntervalMonthDayNano::rem` · arrow-buffer 59.3.0

```rust
fn rem(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Rem", "path": "Rem"}, "trait_path": "core::ops::arith::Rem"}`

Source: `src/interval.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-878519fbecc7249b7332feca"></a>
## rem_assign

`function` · `arrow_buffer::interval::IntervalMonthDayNano::rem_assign` · arrow-buffer 59.3.0

```rust
fn rem_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [313, 1], "end": [321, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::RemAssign", "path": "RemAssign"}, "trait_path": "core::ops::arith::RemAssign"}`

Source: `src/interval.rs:313`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-14955aff74e256dd2ec8889e"></a>
## sub

`function` · `arrow_buffer::interval::IntervalMonthDayNano::sub` · arrow-buffer 59.3.0

```rust
fn sub(self, rhs: Self) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-675d8b0cdb7b1146b4e9f45b"></a>
## sub

`function` · `arrow_buffer::interval::IntervalMonthDayNano::sub` · arrow-buffer 59.3.0

```rust
fn sub(self, rhs: &'a IntervalMonthDayNano) -> Self::Output
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/interval.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}}}}], "constraints": []}}, "id": "core::ops::arith::Sub", "path": "Sub"}, "trait_path": "core::ops::arith::Sub"}`

Source: `src/interval.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32fe2260310559b161337d98"></a>
## sub_assign

`function` · `arrow_buffer::interval::IntervalMonthDayNano::sub_assign` · arrow-buffer 59.3.0

```rust
fn sub_assign(&mut self, rhs: Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [286, 1], "end": [294, 2], "filename": "src/interval.rs"}, "trait": {"args": null, "id": "core::ops::arith::SubAssign", "path": "SubAssign"}, "trait_path": "core::ops::arith::SubAssign"}`

Source: `src/interval.rs:286`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-610e5de53d6a239b8cddf6bf"></a>
## to_i64

`function` · `arrow_buffer::interval::IntervalMonthDayNano::to_i64` · arrow-buffer 59.3.0

```rust
fn to_i64(self) -> Option<i64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:235`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47934416d44133c1c70efb42"></a>
## to_isize

`function` · `arrow_buffer::interval::IntervalMonthDayNano::to_isize` · arrow-buffer 59.3.0

```rust
fn to_isize(self) -> Option<isize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:231`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c541f375fdf766d9c92667a"></a>
## to_usize

`function` · `arrow_buffer::interval::IntervalMonthDayNano::to_usize` · arrow-buffer 59.3.0

```rust
fn to_usize(self) -> Option<usize>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:227`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d2bb809b35411d0c9419041"></a>
## usize_as

`function` · `arrow_buffer::interval::IntervalMonthDayNano::usize_as` · arrow-buffer 59.3.0

```rust
fn usize_as(i: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "crate::IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [214, 1], "end": [238, 2], "filename": "src/native.rs"}, "trait": {"args": null, "id": "arrow_buffer::native::ArrowNativeType", "path": "ArrowNativeType"}, "trait_path": "arrow_buffer::native::ArrowNativeType"}`

Source: `src/native.rs:223`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2773ea50e9651189aab26a67"></a>
## wrapping_abs

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_abs` · arrow-buffer 59.3.0

```rust
fn wrapping_abs(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:107`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Computes the absolute value

<a id="op-1b3a6f0f70d52796326c2373"></a>
## wrapping_add

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_add` · arrow-buffer 59.3.0

```rust
fn wrapping_add(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:147`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping addition

<a id="op-24ce02340600bd95831fad49"></a>
## wrapping_div

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_div` · arrow-buffer 59.3.0

```rust
fn wrapping_div(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:206`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping division

<a id="op-96f179e6eb9658f4dfac93b6"></a>
## wrapping_mul

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_mul` · arrow-buffer 59.3.0

```rust
fn wrapping_mul(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:187`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping multiplication

<a id="op-e505cd4bea2a1005d869d2cf"></a>
## wrapping_neg

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_neg` · arrow-buffer 59.3.0

```rust
fn wrapping_neg(self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:127`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Negates the value

<a id="op-e2f3025ead96cdeb23765635"></a>
## wrapping_pow

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_pow` · arrow-buffer 59.3.0

```rust
fn wrapping_pow(self, exp: u32) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:244`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping exponentiation

<a id="op-fcc8ac4f99a0e011a686e694"></a>
## wrapping_rem

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_rem` · arrow-buffer 59.3.0

```rust
fn wrapping_rem(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:225`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping remainder

<a id="op-954944f3c682845d56bf35b5"></a>
## wrapping_sub

`function` · `arrow_buffer::interval::IntervalMonthDayNano::wrapping_sub` · arrow-buffer 59.3.0

```rust
fn wrapping_sub(self, other: Self) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "arrow_buffer::interval::IntervalMonthDayNano", "path": "IntervalMonthDayNano"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [261, 2], "filename": "src/interval.rs"}, "trait": null, "trait_path": null}`

Source: `src/interval.rs:167`. [Exact documentation build](https://docs.rs/crate/arrow-buffer/59.3.0/json).

Performs wrapping subtraction
