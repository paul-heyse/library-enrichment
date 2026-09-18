# `arrow_cast::cast::decimal::DecimalCast`

Full upstream contracts; raw type trees and source locators in [structured records](arrow_cast.cast.decimal.DecimalCast.json).

<a id="op-2def93bd8f5b1e6d2c652264"></a>
## DecimalCast

`trait` · `arrow_cast::cast::decimal::DecimalCast` · arrow-cast 59.3.0

```rust
trait DecimalCast: Sized
```

Source: `src/cast/decimal.rs:22`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

A utility trait that provides checked conversions between
decimal types inspired by [`NumCast`]

Unresolved upstream links (retained, not inferred): ``NumCast``.

<a id="op-c1937957c02954c85277a5b1"></a>
## from_decimal

`function` · `arrow_cast::cast::decimal::DecimalCast::from_decimal` · arrow-cast 59.3.0

```rust
fn from_decimal<T: DecimalCast>(n: T) -> Option<Self>
```

Source: `src/cast/decimal.rs:36`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert a decimal from a decimal

<a id="op-b818e432229961794a1a325f"></a>
## from_f64

`function` · `arrow_cast::cast::decimal::DecimalCast::from_f64` · arrow-cast 59.3.0

```rust
fn from_f64(n: f64) -> Option<Self>
```

Source: `src/cast/decimal.rs:39`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert a decimal from a f64

<a id="op-9db33a990e350bec90b03808"></a>
## to_i128

`function` · `arrow_cast::cast::decimal::DecimalCast::to_i128` · arrow-cast 59.3.0

```rust
fn to_i128(self) -> Option<i128>
```

Source: `src/cast/decimal.rs:30`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert the decimal to an i128

<a id="op-74f6ca31d588996027f8b447"></a>
## to_i256

`function` · `arrow_cast::cast::decimal::DecimalCast::to_i256` · arrow-cast 59.3.0

```rust
fn to_i256(self) -> Option<i256>
```

Source: `src/cast/decimal.rs:33`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert the decimal to an i256

<a id="op-6e5957cd8f975ef8ba203787"></a>
## to_i32

`function` · `arrow_cast::cast::decimal::DecimalCast::to_i32` · arrow-cast 59.3.0

```rust
fn to_i32(self) -> Option<i32>
```

Source: `src/cast/decimal.rs:24`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert the decimal to an i32

<a id="op-6e1d35bd40b6562bb52e8fa6"></a>
## to_i64

`function` · `arrow_cast::cast::decimal::DecimalCast::to_i64` · arrow-cast 59.3.0

```rust
fn to_i64(self) -> Option<i64>
```

Source: `src/cast/decimal.rs:27`. [Exact documentation build](https://docs.rs/crate/arrow-cast/59.3.0/json).

Convert the decimal to an i64
