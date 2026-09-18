# `datafusion_ffi::physical_expr::ForeignPhysicalExpr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.physical_expr.ForeignPhysicalExpr.json).

<a id="op-2134bfaadf5b6df794872c38"></a>
## ForeignPhysicalExpr

`struct` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr` · datafusion-ffi 55.1.0

```rust
struct ForeignPhysicalExpr
```

Source: `src/physical_expr/mod.rs:507`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This wrapper struct exists on the receiver side of the FFI interface, so it has
no guarantees about being able to access the data in `private_data`. Any functions
defined on this struct must only use the stable functions provided in
FFI_PhysicalExpr to interact with the expression.

<a id="op-76bdc23b24f415213320e9ec"></a>
## children

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::children` · datafusion-ffi 55.1.0

```rust
fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:590`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6b340b8d83192d0bd07003f"></a>
## data_type

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::data_type` · datafusion-ffi 55.1.0

```rust
fn data_type(&self, input_schema: &Schema) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:542`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9010cd29022ac6ef78562dd3"></a>
## eq

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [734, 1], "end": [739, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/physical_expr/mod.rs:735`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7c9bddb160d91b30da1c584"></a>
## evaluate

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::evaluate` · datafusion-ffi 55.1.0

```rust
fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:557`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a84016654d91467f49203382"></a>
## evaluate_bounds

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::evaluate_bounds` · datafusion-ffi 55.1.0

```rust
fn evaluate_bounds(&self, children: &[&Interval]) -> Result<Interval>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:610`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-54e48d420c0ca999b8c8ab16"></a>
## evaluate_selection

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::evaluate_selection` · datafusion-ffi 55.1.0

```rust
fn evaluate_selection(&self, batch: &RecordBatch, selection: &BooleanArray) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-895a3defc5bcc8af49f77162"></a>
## evaluate_statistics

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::evaluate_statistics` · datafusion-ffi 55.1.0

```rust
fn evaluate_statistics(&self, children: &[&Distribution]) -> Result<Distribution>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e936a88dff7dfca202f58d29"></a>
## expression_id

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::expression_id` · datafusion-ffi 55.1.0

```rust
fn expression_id(&self) -> Option<u64>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:728`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-40c0ccf2d22d7f424195248f"></a>
## fmt

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [506, 10], "end": [506, 15], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/physical_expr/mod.rs:506`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69318b2598d934edcf5ef875"></a>
## fmt

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [747, 1], "end": [752, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/physical_expr/mod.rs:748`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7a1c2f24c8f96f201375224c"></a>
## fmt_sql

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::fmt_sql` · datafusion-ffi 55.1.0

```rust
fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:702`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-929f2b54d9c9de38f25fc0f5"></a>
## get_properties

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::get_properties` · datafusion-ffi 55.1.0

```rust
fn get_properties(&self, children: &[ExprProperties]) -> Result<ExprProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:691`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44ca715bc35577cbec6f0f03"></a>
## hash

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::hash` · datafusion-ffi 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [740, 1], "end": [745, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/physical_expr/mod.rs:741`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebb649121f02e1d162a7bc51"></a>
## is_volatile_node

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::is_volatile_node` · datafusion-ffi 55.1.0

```rust
fn is_volatile_node(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:724`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-410c4d044caeed757f1def46"></a>
## nullable

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::nullable` · datafusion-ffi 55.1.0

```rust
fn nullable(&self, input_schema: &Schema) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:550`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6d39c78f360e83b0bdc98e3"></a>
## propagate_constraints

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::propagate_constraints` · datafusion-ffi 55.1.0

```rust
fn propagate_constraints(&self, interval: &Interval, children: &[&Interval]) -> Result<Option<Vec<Interval>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:621`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bb5e1c4ccde642fda178166"></a>
## propagate_statistics

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::propagate_statistics` · datafusion-ffi 55.1.0

```rust
fn propagate_statistics(&self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:663`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e561190335492e16a8e3c65"></a>
## return_field

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::return_field` · datafusion-ffi 55.1.0

```rust
fn return_field(&self, input_schema: &Schema) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:565`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-29a20d1e8a3d3ff925754c3f"></a>
## snapshot

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::snapshot` · datafusion-ffi 55.1.0

```rust
fn snapshot(&self) -> Result<Option<Arc<dyn PhysicalExpr>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:711`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f20c2efe3658ac798a8fbf1"></a>
## snapshot_generation

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::snapshot_generation` · datafusion-ffi 55.1.0

```rust
fn snapshot_generation(&self) -> u64
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:720`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebb64e57a978aff29bf77702"></a>
## with_new_children

`function` · `datafusion_ffi::physical_expr::ForeignPhysicalExpr::with_new_children` · datafusion-ffi 55.1.0

```rust
fn with_new_children(Arc<self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::physical_expr::ForeignPhysicalExpr", "path": "ForeignPhysicalExpr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [541, 1], "end": [731, 2], "filename": "src/physical_expr/mod.rs"}, "trait": {"args": null, "id": "datafusion_physical_expr_common::physical_expr::PhysicalExpr", "path": "PhysicalExpr"}, "trait_path": "datafusion_physical_expr_common::physical_expr::PhysicalExpr"}`

Source: `src/physical_expr/mod.rs:594`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
