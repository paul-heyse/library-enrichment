# `datafusion_functions_aggregate::bool_and_or::BoolAnd`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.bool_and_or.BoolAnd.json).

<a id="op-49c68019371be8e954ba66e3"></a>
## BoolAnd

`struct` · `datafusion_functions_aggregate::bool_and_or::BoolAnd` · datafusion-functions-aggregate 55.1.0

```rust
struct BoolAnd
```

Source: `src/bool_and_or.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

BOOL_AND aggregate expression

<a id="op-142742b5da48470d41f15a31"></a>
## accumulator

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2c3185890e5b87863f27e67d"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39ad6d5e9abc031cb550b32b"></a>
## default

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [121, 1], "end": [125, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/bool_and_or.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cdb585e0b3284fe3e592dd38"></a>
## documentation

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-99f139fe1b0cac063f9c33a9"></a>
## eq

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &BoolAnd) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 17], "end": [108, 26], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/bool_and_or.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5c0279c1fd2ef94494f73819"></a>
## fmt

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 10], "end": [108, 15], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/bool_and_or.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1109a6577cce09641783cc3d"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:155`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0b8214fd7eb2e243b525f19b"></a>
## hash

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [108, 32], "end": [108, 36], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/bool_and_or.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb403b3a48f453c1ccfe3c32"></a>
## name

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e3a5121c58daa7f42ed21c3"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c19c100ff526683d09df496d"></a>
## return_type

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:136`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9948beb9206c32e21f6504b6"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:179`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3030b4d7baaec8a9f8fa6baa"></a>
## signature

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb94ee951b569ce3c70409d5"></a>
## state_fields

`function` · `datafusion_functions_aggregate::bool_and_or::BoolAnd::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolAnd", "path": "BoolAnd"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [127, 1], "end": [186, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:144`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
