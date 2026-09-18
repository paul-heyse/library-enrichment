# `datafusion_functions_aggregate::first_last::LastValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.first_last.LastValue.json).

<a id="op-11dd90b02b5881e3af6f9ff5"></a>
## LastValue

`struct` · `datafusion_functions_aggregate::first_last::LastValue` · datafusion-functions-aggregate 55.1.0

```rust
struct LastValue
```

Source: `src/first_last.rs:1030`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d9f75ccc3c210a6dfd2497c2"></a>
## accumulator

`function` · `datafusion_functions_aggregate::first_last::LastValue::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1075`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7d4dc84955df1f7a7acb5c18"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::first_last::LastValue::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1147`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b294c5850dba5302d77a51ae"></a>
## default

`function` · `datafusion_functions_aggregate::first_last::LastValue::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1035, 1], "end": [1039, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/first_last.rs:1036`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f143438d0a3170bd1bf32cd3"></a>
## documentation

`function` · `datafusion_functions_aggregate::first_last::LastValue::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1139`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98e9328aa19aeb1d9282bfd1"></a>
## eq

`function` · `datafusion_functions_aggregate::first_last::LastValue::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &LastValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1029, 10], "end": [1029, 19], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/first_last.rs:1029`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a4186f30eee7fdfd60ebcd3d"></a>
## fmt

`function` · `datafusion_functions_aggregate::first_last::LastValue::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1029, 31], "end": [1029, 36], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/first_last.rs:1029`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b2a24cd794fa3602d87cb0a"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::first_last::LastValue::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1143`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97650af8c2a88c7fa7d86614"></a>
## hash

`function` · `datafusion_functions_aggregate::first_last::LastValue::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1029, 25], "end": [1029, 29], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/first_last.rs:1029`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1599ed86da23684796a096d9"></a>
## name

`function` · `datafusion_functions_aggregate::first_last::LastValue::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1051`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2ee9f14cd3bbd51b68062c93"></a>
## new

`function` · `datafusion_functions_aggregate::first_last::LastValue::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1041, 1], "end": [1048, 2], "filename": "src/first_last.rs"}, "trait": null, "trait_path": null}`

Source: `src/first_last.rs:1042`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a86b2faf00a51a9a0797786"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::first_last::LastValue::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b54b77d45a00db776c3c5eba"></a>
## return_field

`function` · `datafusion_functions_aggregate::first_last::LastValue::return_field` · datafusion-functions-aggregate 55.1.0

```rust
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1063`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0ff230c76f88f3ee9e94c3d"></a>
## return_type

`function` · `datafusion_functions_aggregate::first_last::LastValue::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1059`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7afe809d70810afa93992ad6"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::first_last::LastValue::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c57e2daac3ef85e4b4e83af3"></a>
## signature

`function` · `datafusion_functions_aggregate::first_last::LastValue::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1055`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-998cd5059571f19804d524d3"></a>
## state_fields

`function` · `datafusion_functions_aggregate::first_last::LastValue::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1096`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8fb3ed79ba5e43548fb91dc7"></a>
## supports_null_handling_clause

`function` · `datafusion_functions_aggregate::first_last::LastValue::supports_null_handling_clause` · datafusion-functions-aggregate 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1135`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4b62af461e9f14b733e6d1df"></a>
## with_beneficial_ordering

`function` · `datafusion_functions_aggregate::first_last::LastValue::with_beneficial_ordering` · datafusion-functions-aggregate 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::first_last::LastValue", "path": "LastValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [1050, 1], "end": [1153, 2], "filename": "src/first_last.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/first_last.rs:1117`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
