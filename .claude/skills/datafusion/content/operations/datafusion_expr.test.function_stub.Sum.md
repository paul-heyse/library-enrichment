# `datafusion_expr::test::function_stub::Sum`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.test.function_stub.Sum.json).

<a id="op-f262d2a7ac099344d9c6cc29"></a>
## Sum

`struct` · `datafusion_expr::test::function_stub::Sum` · datafusion-expr 55.1.0

```rust
struct Sum
```

Source: `src/test/function_stub.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Stub `sum` used for optimizer testing

<a id="op-1bbadc9459af8081c7780f66"></a>
## accumulator

`function` · `datafusion_expr::test::function_stub::Sum::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-994f80a5f311c3a740677823"></a>
## coerce_types

`function` · `datafusion_expr::test::function_stub::Sum::coerce_types` · datafusion-expr 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68ae4b0c2ce0f5b7e30288bb"></a>
## create_groups_accumulator

`function` · `datafusion_expr::test::function_stub::Sum::create_groups_accumulator` · datafusion-expr 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:195`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4995b9e89d018ba40a4a2ed"></a>
## default

`function` · `datafusion_expr::test::function_stub::Sum::default` · datafusion-expr 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 1], "end": [112, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/test/function_stub.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f444138fee3c0203ff80c565"></a>
## eq

`function` · `datafusion_expr::test::function_stub::Sum::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &Sum) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 17], "end": [95, 26], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/test/function_stub.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-599cd92bf45e0da6df2688bf"></a>
## fmt

`function` · `datafusion_expr::test::function_stub::Sum::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 10], "end": [95, 15], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/test/function_stub.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fa8431df0bcdc51f54fa425"></a>
## groups_accumulator_supported

`function` · `datafusion_expr::test::function_stub::Sum::groups_accumulator_supported` · datafusion-expr 55.1.0

```rust
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:191`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5e3d122916b79aa9a4a250d4"></a>
## hash

`function` · `datafusion_expr::test::function_stub::Sum::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 32], "end": [95, 36], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/test/function_stub.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-42e81d882de87f8533303234"></a>
## name

`function` · `datafusion_expr::test::function_stub::Sum::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:115`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c9e5630dc45c05ee74fc07dc"></a>
## new

`function` · `datafusion_expr::test::function_stub::Sum::new` · datafusion-expr 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 1], "end": [106, 2], "filename": "src/test/function_stub.rs"}, "trait": null, "trait_path": null}`

Source: `src/test/function_stub.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f28b01381c112b5f396c2f00"></a>
## order_sensitivity

`function` · `datafusion_expr::test::function_stub::Sum::order_sensitivity` · datafusion-expr 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:206`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64c96adcd79d7069ffd6ce20"></a>
## return_type

`function` · `datafusion_expr::test::function_stub::Sum::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:148`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ad8d0a8f9cd78ae898f74337"></a>
## reverse_expr

`function` · `datafusion_expr::test::function_stub::Sum::reverse_expr` · datafusion-expr 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9021ec671c5c450ee85e233"></a>
## signature

`function` · `datafusion_expr::test::function_stub::Sum::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c1b51d5d43ce7b88c1a23d87"></a>
## state_fields

`function` · `datafusion_expr::test::function_stub::Sum::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::test::function_stub::Sum", "path": "Sum"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [114, 1], "end": [209, 2], "filename": "src/test/function_stub.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/test/function_stub.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
