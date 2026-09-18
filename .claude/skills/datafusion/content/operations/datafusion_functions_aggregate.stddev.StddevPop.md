# `datafusion_functions_aggregate::stddev::StddevPop`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.stddev.StddevPop.json).

<a id="op-67a17772ae6f7518d7c2275a"></a>
## StddevPop

`struct` · `datafusion_functions_aggregate::stddev::StddevPop` · datafusion-functions-aggregate 55.1.0

```rust
struct StddevPop
```

Source: `src/stddev.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

STDDEV_POP population aggregate expression

<a id="op-dc05bc586144561a69576983"></a>
## accumulator

`function` · `datafusion_functions_aggregate::stddev::StddevPop::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e1cc3685f2e0cd75c382f843"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::stddev::StddevPop::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:231`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8c478dc4f06b990ddc8345a9"></a>
## default

`function` · `datafusion_functions_aggregate::stddev::StddevPop::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [173, 1], "end": [177, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stddev.rs:174`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-45c8ad29aceefde3f20ab1e7"></a>
## documentation

`function` · `datafusion_functions_aggregate::stddev::StddevPop::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:240`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8179144c91a0109065c270f"></a>
## eq

`function` · `datafusion_functions_aggregate::stddev::StddevPop::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &StddevPop) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 10], "end": [168, 19], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stddev.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b293ff1c839cf03ca543839e"></a>
## fmt

`function` · `datafusion_functions_aggregate::stddev::StddevPop::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 31], "end": [168, 36], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stddev.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b586d2a97caa395f9c07efd9"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::stddev::StddevPop::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:227`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e628b014173a27c547bbb81"></a>
## hash

`function` · `datafusion_functions_aggregate::stddev::StddevPop::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [168, 25], "end": [168, 29], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/stddev.rs:168`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44a90a411265cd36d1bad56f"></a>
## name

`function` · `datafusion_functions_aggregate::stddev::StddevPop::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:189`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-97494e51747f3bdf046f648d"></a>
## new

`function` · `datafusion_functions_aggregate::stddev::StddevPop::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [179, 1], "end": [186, 2], "filename": "src/stddev.rs"}, "trait": null, "trait_path": null}`

Source: `src/stddev.rs:181`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new STDDEV_POP aggregate function

<a id="op-c32b765863f87f2c5f493fa0"></a>
## return_type

`function` · `datafusion_functions_aggregate::stddev::StddevPop::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-20c66bc46e70bb07a8fb0d6a"></a>
## signature

`function` · `datafusion_functions_aggregate::stddev::StddevPop::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:193`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0d6552b567456019f93c6244"></a>
## state_fields

`function` · `datafusion_functions_aggregate::stddev::StddevPop::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::StddevPop", "path": "StddevPop"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [188, 1], "end": [243, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:197`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
