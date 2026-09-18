# `datafusion_physical_plan::execution_plan::CardinalityEffect`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_plan.execution_plan.CardinalityEffect.json).

<a id="op-2c76274ffe796334332a2205"></a>
## CardinalityEffect

`enum` · `datafusion_physical_plan::execution_plan::CardinalityEffect` · datafusion-physical-plan 55.1.0

```rust
enum CardinalityEffect
```

Source: `src/execution_plan.rs:2052`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Indicates the effect an execution plan operator will have on the cardinality
of its input stream

<a id="op-30b6f129f5e980e3cda0c464"></a>
## Equal

`variant` · `datafusion_physical_plan::execution_plan::CardinalityEffect::Equal` · datafusion-physical-plan 55.1.0

```rust
Equal
```

Source: `src/execution_plan.rs:2057`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The operator is guaranteed to produce exactly one row for
each input row

<a id="op-0d14be109e084bd5b14d58c1"></a>
## GreaterEqual

`variant` · `datafusion_physical_plan::execution_plan::CardinalityEffect::GreaterEqual` · datafusion-physical-plan 55.1.0

```rust
GreaterEqual
```

Source: `src/execution_plan.rs:2061`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The operator may produce more output rows than it receives input rows

<a id="op-2a8324aa8fd7072272de4e6c"></a>
## LowerEqual

`variant` · `datafusion_physical_plan::execution_plan::CardinalityEffect::LowerEqual` · datafusion-physical-plan 55.1.0

```rust
LowerEqual
```

Source: `src/execution_plan.rs:2059`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

The operator may produce fewer output rows than it receives input rows

<a id="op-507d355320241a69e5765c17"></a>
## Unknown

`variant` · `datafusion_physical_plan::execution_plan::CardinalityEffect::Unknown` · datafusion-physical-plan 55.1.0

```rust
Unknown
```

Source: `src/execution_plan.rs:2054`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-plan/55.1.0/json).

Unknown effect. This is the default
