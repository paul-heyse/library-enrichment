# `datafusion_expr::expr::WindowFunctionDefinition`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr.WindowFunctionDefinition.json).

<a id="op-a8ce566547457ddc39dd883e"></a>
## WindowFunctionDefinition

`enum` · `datafusion_expr::expr::WindowFunctionDefinition` · datafusion-expr 55.1.0

```rust
enum WindowFunctionDefinition
```

Source: `src/expr.rs:1161`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A function used as a SQL window function

In SQL, you can use:
- Actual window functions ([`WindowUDF`](../operations/datafusion_expr.udwf.WindowUDF.md#op-42e8216e20a1c833692a7e65))
- Normal aggregate functions ([`AggregateUDF`](../operations/datafusion_expr.udaf.AggregateUDF.md#op-d90e5a97479981a539718379))

<a id="op-d3edd0f992db7a49ae3124e2"></a>
## AggregateUDF

`variant` · `datafusion_expr::expr::WindowFunctionDefinition::AggregateUDF` · datafusion-expr 55.1.0

```rust
AggregateUDF
```

Source: `src/expr.rs:1163`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A user defined aggregate function

<a id="op-506e0843caab856e712168c7"></a>
## WindowUDF

`variant` · `datafusion_expr::expr::WindowFunctionDefinition::WindowUDF` · datafusion-expr 55.1.0

```rust
WindowUDF
```

Source: `src/expr.rs:1165`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

A user defined window function

<a id="op-678cceaeb0d2b93ade6cd9d5"></a>
## clone

`function` · `datafusion_expr::expr::WindowFunctionDefinition::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WindowFunctionDefinition
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1160, 17], "end": [1160, 22], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/expr.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b3ffa02140a0a2a20b01cf0"></a>
## eq

`function` · `datafusion_expr::expr::WindowFunctionDefinition::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WindowFunctionDefinition) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1160, 24], "end": [1160, 33], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04b6004cd3f9305ff4225184"></a>
## fmt

`function` · `datafusion_expr::expr::WindowFunctionDefinition::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1160, 10], "end": [1160, 15], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-734542ba2ab23322fb7a227f"></a>
## fmt

`function` · `datafusion_expr::expr::WindowFunctionDefinition::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1212, 1], "end": [1219, 2], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/expr.rs:1213`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79c288bdb0a5df68e0efa630"></a>
## from

`function` · `datafusion_expr::expr::WindowFunctionDefinition::from` · datafusion-expr 55.1.0

```rust
fn from(value: Arc<WindowUDF>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1227, 1], "end": [1231, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::udwf::WindowUDF", "path": "WindowUDF"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:1228`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fde81c17cdc8588746a0ef86"></a>
## from

`function` · `datafusion_expr::expr::WindowFunctionDefinition::from` · datafusion-expr 55.1.0

```rust
fn from(value: Arc<AggregateUDF>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1221, 1], "end": [1225, 2], "filename": "src/expr.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_expr::udaf::AggregateUDF", "path": "AggregateUDF"}}}], "constraints": []}}, "id": "alloc::sync::Arc", "path": "Arc"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/expr.rs:1222`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d453c73a3af352838aeb0a9d"></a>
## hash

`function` · `datafusion_expr::expr::WindowFunctionDefinition::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1160, 51], "end": [1160, 55], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-577782c98c23bfe45faf94b2"></a>
## name

`function` · `datafusion_expr::expr::WindowFunctionDefinition::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1210, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1194`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Function's name for display

<a id="op-ff2a49490dbc642a1e1ebaa1"></a>
## partial_cmp

`function` · `datafusion_expr::expr::WindowFunctionDefinition::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WindowFunctionDefinition) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1160, 39], "end": [1160, 49], "filename": "src/expr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/expr.rs:1160`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9563807069ac51f4c97d885"></a>
## return_field

`function` · `datafusion_expr::expr::WindowFunctionDefinition::return_field` · datafusion-expr 55.1.0

```rust
fn return_field(&self, input_expr_fields: &[FieldRef], display_name: &str) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1210, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1170`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns the datatype of the window function

<a id="op-ed47db377e4fea03a07eab86"></a>
## signature

`function` · `datafusion_expr::expr::WindowFunctionDefinition::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1210, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1186`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The signatures supported by the function `fun`.

<a id="op-443a824fc130ad27dd924d68"></a>
## simplify

`function` · `datafusion_expr::expr::WindowFunctionDefinition::simplify` · datafusion-expr 55.1.0

```rust
fn simplify(&self) -> Option<WindowFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr::WindowFunctionDefinition", "path": "WindowFunctionDefinition"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1168, 1], "end": [1210, 2], "filename": "src/expr.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr.rs:1204`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Returns this window function's simplification hook, if any.

See [`WindowFunctionSimplification`](../operations/datafusion_expr.function.WindowFunctionSimplification.md#op-4e3825e9eb717b05a23f2e0d) for more information
