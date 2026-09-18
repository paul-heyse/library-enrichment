# `datafusion_expr::logical_plan::plan::Union`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.plan.Union.json).

<a id="op-6c1434630ac324035f3338f2"></a>
## Union

`struct` · `datafusion_expr::logical_plan::plan::Union` · datafusion-expr 55.1.0

```rust
struct Union
```

Source: `src/logical_plan/plan.rs:3176`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Union multiple inputs

<a id="op-5fc675e399dae4f23ea43f56"></a>
## clone

`function` · `datafusion_expr::logical_plan::plan::Union::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> Union
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 17], "end": [3175, 22], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/plan.rs:3175`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4baa1d0649131f72266cdca"></a>
## eq

`function` · `datafusion_expr::logical_plan::plan::Union::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Union) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 24], "end": [3175, 33], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/plan.rs:3175`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35493dba7da9d5b361280a17"></a>
## fmt

`function` · `datafusion_expr::logical_plan::plan::Union::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 10], "end": [3175, 15], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/plan.rs:3175`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7638fdfe5bb8f666c3e9345"></a>
## hash

`function` · `datafusion_expr::logical_plan::plan::Union::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3175, 39], "end": [3175, 43], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/plan.rs:3175`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62021b2c9fce2ce73e61e033"></a>
## inputs

`struct_field` · `datafusion_expr::logical_plan::plan::Union::inputs` · datafusion-expr 55.1.0

```rust
inputs: Vec<std::sync::Arc<LogicalPlan>>
```

Source: `src/logical_plan/plan.rs:3178`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Inputs to merge

<a id="op-552b41a3a720b951eef08de6"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::plan::Union::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3419, 1], "end": [3426, 2], "filename": "src/logical_plan/plan.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/plan.rs:3420`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c89c55200bd05dd0b5e0bee"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::plan::Union::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/plan.rs:3180`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Union schema. Should be the same for all inputs.

<a id="op-ed28622937ed8e691fe9f73a"></a>
## try_new

`function` · `datafusion_expr::logical_plan::plan::Union::try_new` · datafusion-expr 55.1.0

```rust
fn try_new(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3183, 1], "end": [3416, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3186`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Constructs new Union instance deriving schema from inputs.
Schema data types must match exactly.

<a id="op-1dc312ba9b7232a9e0b4be58"></a>
## try_new_by_name

`function` · `datafusion_expr::logical_plan::plan::Union::try_new_by_name` · datafusion-expr 55.1.0

```rust
fn try_new_by_name(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3183, 1], "end": [3416, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3203`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Constructs a new Union instance that combines rows from different tables by name,
instead of by position. This means that the specified inputs need not have schemas
that are all the same width.

<a id="op-d4d794d8fcc234e18d386c67"></a>
## try_new_with_loose_types

`function` · `datafusion_expr::logical_plan::plan::Union::try_new_with_loose_types` · datafusion-expr 55.1.0

```rust
fn try_new_with_loose_types(inputs: Vec<Arc<LogicalPlan>>) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::plan::Union", "path": "Union"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [3183, 1], "end": [3416, 2], "filename": "src/logical_plan/plan.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/plan.rs:3195`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Constructs new Union instance deriving schema from inputs.
Inputs do not have to have matching types and produced schema will
take type from the first input.
