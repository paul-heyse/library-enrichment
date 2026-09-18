# `datafusion_functions_aggregate::first_last::FirstValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.FirstValue.json).

<a id="op-a117c22d6064c7fc2d59cfab"></a>
## FirstValue

`struct` · `datafusion_functions_aggregate::first_last::FirstValue` · datafusion-functions-aggregate 55.1.0

```rust
struct FirstValue
```

Source: `src/first_last.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1e41b79c3967caa5c61cac6"></a>
## accumulator

`function` · `datafusion_functions_aggregate::first_last::FirstValue::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:299`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6bad05ac8374bc494a6c7c0"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::first_last::FirstValue::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:345`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ba98fbc446ede0e0ab5ede7"></a>
## default

`function` · `datafusion_functions_aggregate::first_last::FirstValue::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [259, 1], "end": [263, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/first_last.rs:260`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31afe3ed09d679c14f4982f8"></a>
## documentation

`function` · `datafusion_functions_aggregate::first_last::FirstValue::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5fffdffe122e5fa523ed7ea7"></a>
## eq

`function` · `datafusion_functions_aggregate::first_last::FirstValue::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &FirstValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 10], "end": [253, 19], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/first_last.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8bb00692c493cde71f0c00fb"></a>
## fmt

`function` · `datafusion_functions_aggregate::first_last::FirstValue::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 31], "end": [253, 36], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/first_last.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6ebabdc9fff249e1be0769d"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::first_last::FirstValue::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-069a2d7a1981def663bd176a"></a>
## hash

`function` · `datafusion_functions_aggregate::first_last::FirstValue::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [253, 25], "end": [253, 29], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/first_last.rs:253`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b2d5e8cab6fd6d4c863a2e75"></a>
## name

`function` · `datafusion_functions_aggregate::first_last::FirstValue::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:275`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f65621e4122375bae8c6f58"></a>
## new

`function` · `datafusion_functions_aggregate::first_last::FirstValue::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 1], "end": [272, 2], "filename": "src/first_last.rs"}, "trait": null, "trait_path": null}`

Source: `src/first_last.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e75bcf25e6dd6c2a2d7e9bff"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::first_last::FirstValue::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1d23a5c1fa01dae27af8a190"></a>
## return_field

`function` · `datafusion_functions_aggregate::first_last::FirstValue::return_field` · datafusion-functions-aggregate 55.1.0

```rust
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cc0714fff9768576bb703a3a"></a>
## return_type

`function` · `datafusion_functions_aggregate::first_last::FirstValue::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:283`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c71b1cbc054399282c2303b6"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::first_last::FirstValue::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-59f0c3d3947ef106e2ac7e8f"></a>
## signature

`function` · `datafusion_functions_aggregate::first_last::FirstValue::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:279`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e16f9e29c746e8b46cc07d6"></a>
## state_fields

`function` · `datafusion_functions_aggregate::first_last::FirstValue::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:320`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6ef2dd29afc37da2dad3b88c"></a>
## supports_null_handling_clause

`function` · `datafusion_functions_aggregate::first_last::FirstValue::supports_null_handling_clause` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:370`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8383f5883be40750c49860b1"></a>
## with_beneficial_ordering

`function` · `datafusion_functions_aggregate::first_last::FirstValue::with_beneficial_ordering` · datafusion-functions-aggregate 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::FirstValue", "path": "FirstValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [274, 1], "end": [377, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
