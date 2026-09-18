# `datafusion_functions_aggregate::any_value::AnyValue`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.any_value.AnyValue.json).

<a id="op-c1c2099a4d385bffede9dcea"></a>
## AnyValue

`struct` · `datafusion_functions_aggregate::any_value::AnyValue` · datafusion-functions-aggregate 55.1.0

```rust
struct AnyValue
```

Source: `src/any_value.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0c4ed69bc0c0e08583bffc10"></a>
## accumulator

`function` · `datafusion_functions_aggregate::any_value::AnyValue::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fdb7f5f2e8c74c1cd6aa2980"></a>
## default

`function` · `datafusion_functions_aggregate::any_value::AnyValue::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 1], "end": [66, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/any_value.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9649c3f75fa6afb6151366a0"></a>
## documentation

`function` · `datafusion_functions_aggregate::any_value::AnyValue::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bcf6c4634be3bd9fcc963c4"></a>
## eq

`function` · `datafusion_functions_aggregate::any_value::AnyValue::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &AnyValue) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 10], "end": [57, 19], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/any_value.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-32b079ce1cd6a848bac78f13"></a>
## fmt

`function` · `datafusion_functions_aggregate::any_value::AnyValue::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 31], "end": [57, 36], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/any_value.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f82b057ef8e7b7c8478d00c8"></a>
## hash

`function` · `datafusion_functions_aggregate::any_value::AnyValue::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 25], "end": [57, 29], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/any_value.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bde3b0c66dd685407cc6f86c"></a>
## name

`function` · `datafusion_functions_aggregate::any_value::AnyValue::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f8c018b54794f6626fc60c20"></a>
## new

`function` · `datafusion_functions_aggregate::any_value::AnyValue::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [74, 2], "filename": "src/any_value.rs"}, "trait": null, "trait_path": null}`

Source: `src/any_value.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50b4af12028a4e2b56a6a8ca"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::any_value::AnyValue::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-01c86ff0e13e39fa18ff1188"></a>
## return_field

`function` · `datafusion_functions_aggregate::any_value::AnyValue::return_field` · datafusion-functions-aggregate 55.1.0

```rust
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06cb81756c19431843974a79"></a>
## return_type

`function` · `datafusion_functions_aggregate::any_value::AnyValue::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d39a4873b5dbdc349b432697"></a>
## signature

`function` · `datafusion_functions_aggregate::any_value::AnyValue::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4d654833c8ec1e8660218a4"></a>
## state_fields

`function` · `datafusion_functions_aggregate::any_value::AnyValue::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::any_value::AnyValue", "path": "AnyValue"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [125, 2], "filename": "src/any_value.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/any_value.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
