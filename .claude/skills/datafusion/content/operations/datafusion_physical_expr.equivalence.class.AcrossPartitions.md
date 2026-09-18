# `datafusion_physical_expr::equivalence::class::AcrossPartitions`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_physical_expr.equivalence.class.AcrossPartitions.json).

<a id="op-9ebbba6924a72a1deaa44c10"></a>
## AcrossPartitions

`enum` · `datafusion_physical_expr::equivalence::class::AcrossPartitions` · datafusion-physical-expr 55.1.0

```rust
enum AcrossPartitions
```

Source: `src/equivalence/class.rs:43`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

Represents whether a constant expression's value is uniform or varies across
partitions. Has two variants:
- `Heterogeneous`: The constant expression may have different values for
  different partitions.
- `Uniform(Option<ScalarValue>)`: The constant expression has the same value
  across all partitions, or is `None` if the value is unknown.

<a id="op-27f3c711a98a95c1138610ba"></a>
## Heterogeneous

`variant` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::Heterogeneous` · datafusion-physical-expr 55.1.0

```rust
Heterogeneous
```

Source: `src/equivalence/class.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8563a29bdd461e9ea5622e5d"></a>
## Uniform

`variant` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::Uniform` · datafusion-physical-expr 55.1.0

```rust
Uniform
```

Source: `src/equivalence/class.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1dcc10d6dea2ca4621e3452"></a>
## clone

`function` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::clone` · datafusion-physical-expr 55.1.0

```rust
fn clone(&self) -> AcrossPartitions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::AcrossPartitions", "path": "AcrossPartitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 10], "end": [42, 15], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/equivalence/class.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc84a926ef1f7e246b3d91d5"></a>
## default

`function` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::default` · datafusion-physical-expr 55.1.0

```rust
fn default() -> AcrossPartitions
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::AcrossPartitions", "path": "AcrossPartitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 24], "end": [42, 31], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/equivalence/class.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f46820ad4af9e87f312011e2"></a>
## eq

`function` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::eq` · datafusion-physical-expr 55.1.0

```rust
fn eq(&self, other: &AcrossPartitions) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::AcrossPartitions", "path": "AcrossPartitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 37], "end": [42, 46], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/equivalence/class.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6737f28c90e142369e090d28"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::AcrossPartitions", "path": "AcrossPartitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [42, 17], "end": [42, 22], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/equivalence/class.rs:42`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfb1cbfa4e3b0bc104efe67f"></a>
## fmt

`function` · `datafusion_physical_expr::equivalence::class::AcrossPartitions::fmt` · datafusion-physical-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_physical_expr::equivalence::class::AcrossPartitions", "path": "AcrossPartitions"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [62, 2], "filename": "src/equivalence/class.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/equivalence/class.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-physical-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
