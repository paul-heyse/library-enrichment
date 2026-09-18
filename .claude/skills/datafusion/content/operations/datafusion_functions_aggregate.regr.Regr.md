# `datafusion_functions_aggregate::regr::Regr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.regr.Regr.json).

<a id="op-c9cc5c71ff99e01269ea9e75"></a>
## Regr

`struct` · `datafusion_functions_aggregate::regr::Regr` · datafusion-functions-aggregate 55.1.0

```rust
struct Regr
```

Source: `src/regr.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-943e44b83d37959005de5732"></a>
## accumulator

`function` · `datafusion_functions_aggregate::regr::Regr::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39706bd0233ad82b10894817"></a>
## default_value

`function` · `datafusion_functions_aggregate::regr::Regr::default_value` · datafusion-functions-aggregate 55.1.0

```rust
fn default_value(&self, _data_type: &DataType) -> Result<ScalarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa08185e9b492af1b7cdfb06"></a>
## documentation

`function` · `datafusion_functions_aggregate::regr::Regr::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:514`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c174dae315f9a5766346f835"></a>
## eq

`function` · `datafusion_functions_aggregate::regr::Regr::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Regr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 10], "end": [52, 19], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/regr.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9606d9882ba6a65051f9d839"></a>
## fmt

`function` · `datafusion_functions_aggregate::regr::Regr::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 31], "end": [52, 36], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/regr.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47f42a39fb34cb4eeb6420f9"></a>
## hash

`function` · `datafusion_functions_aggregate::regr::Regr::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 25], "end": [52, 29], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/regr.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5779404aab25e282235d36ee"></a>
## is_nullable

`function` · `datafusion_functions_aggregate::regr::Regr::is_nullable` · datafusion-functions-aggregate 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:468`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8ce590dd9d7787fe21ccb9aa"></a>
## name

`function` · `datafusion_functions_aggregate::regr::Regr::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e457f98593dec0c78114c542"></a>
## new

`function` · `datafusion_functions_aggregate::regr::Regr::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new(regr_type: RegrType, func_name: &'static str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [70, 2], "filename": "src/regr.rs"}, "trait": null, "trait_path": null}`

Source: `src/regr.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a54156dff76088054deda3b"></a>
## return_type

`function` · `datafusion_functions_aggregate::regr::Regr::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3ed62efd165d50cc31f391a"></a>
## signature

`function` · `datafusion_functions_aggregate::regr::Regr::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:448`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2791499a1282d4269ce44dba"></a>
## state_fields

`function` · `datafusion_functions_aggregate::regr::Regr::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::regr::Regr", "path": "Regr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [443, 1], "end": [517, 2], "filename": "src/regr.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/regr.rs:476`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
