# `datafusion_functions_window_common::partition::PartitionEvaluatorArgs`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_window_common.partition.PartitionEvaluatorArgs.json).

<a id="op-4c6717207cad2fbeefaec16a"></a>
## PartitionEvaluatorArgs

`struct` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs` · datafusion-functions-window-common 55.1.0

```rust
struct PartitionEvaluatorArgs<'a>
```

Source: `src/partition.rs:25`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Arguments passed to created user-defined window function state
during physical execution.

<a id="op-9db29be26f4fd0e49bb74316"></a>
## default

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::default` · datafusion-functions-window-common 55.1.0

```rust
fn default() -> PartitionEvaluatorArgs<'a>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 17], "end": [24, 24], "filename": "src/partition.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/partition.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6919104db96caf0765afcd8"></a>
## fmt

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::fmt` · datafusion-functions-window-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [24, 10], "end": [24, 15], "filename": "src/partition.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/partition.rs:24`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6a9860d28f03a128e0b5695d"></a>
## ignore_nulls

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::ignore_nulls` · datafusion-functions-window-common 55.1.0

```rust
fn ignore_nulls(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [88, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns `true` when `IGNORE NULLS` is specified, otherwise
returns `false`.

<a id="op-8118173f69c5d1e2ee2532b2"></a>
## input_exprs

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::input_exprs` · datafusion-functions-window-common 55.1.0

```rust
fn input_exprs(&self) -> &'a [Arc<dyn PhysicalExpr>]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [88, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the expressions passed as arguments to the user-defined
window function.

<a id="op-0abb124e504919a1df99ef41"></a>
## input_fields

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::input_fields` · datafusion-functions-window-common 55.1.0

```rust
fn input_fields(&self) -> &'a [FieldRef]
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [88, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns the [`FieldRef`](../operations/arrow_schema.field.FieldRef.md#op-a0fdaf7a91a3563923566542)s corresponding to the input expressions
to the user-defined window function.

<a id="op-db226e2bfc6d5106d400aa5b"></a>
## is_reversed

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::is_reversed` · datafusion-functions-window-common 55.1.0

```rust
fn is_reversed(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [88, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Returns `true` when the user-defined window function is
reversed, otherwise returns `false`.

<a id="op-efeebadda2613a245e5bbc0b"></a>
## new

`function` · `datafusion_functions_window_common::partition::PartitionEvaluatorArgs::new` · datafusion-functions-window-common 55.1.0

```rust
fn new(input_exprs: &'a [Arc<dyn PhysicalExpr>], input_fields: &'a [FieldRef], is_reversed: bool, ignore_nulls: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_functions_window_common::partition::PartitionEvaluatorArgs", "path": "PartitionEvaluatorArgs"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [38, 1], "end": [88, 2], "filename": "src/partition.rs"}, "trait": null, "trait_path": null}`

Source: `src/partition.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-window-common/55.1.0/json).

Create an instance of [`PartitionEvaluatorArgs`](../operations/datafusion_functions_window_common.partition.PartitionEvaluatorArgs.md#op-4c6717207cad2fbeefaec16a).

# Arguments

* `input_exprs` - The expressions passed as arguments
  to the user-defined window function.
* `input_fields` - The fields corresponding to the
  arguments to the user-defined window function.
* `is_reversed` - Set to `true` if and only if the user-defined
  window function is reversible and is reversed.
* `ignore_nulls` - Set to `true` when `IGNORE NULLS` is
  specified.
