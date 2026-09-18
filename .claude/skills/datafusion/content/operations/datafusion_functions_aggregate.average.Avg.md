# `datafusion_functions_aggregate::average::Avg`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.average.Avg.json).

<a id="op-2a6cac8a66e8bd00030a03e6"></a>
## Avg

`struct` · `datafusion_functions_aggregate::average::Avg` · datafusion-functions-aggregate 55.1.0

```rust
struct Avg
```

Source: `src/average.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d0c89b2c49ad80398ccc312"></a>
## accumulator

`function` · `datafusion_functions_aggregate::average::Avg::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b7662f1e127a12480046fcd3"></a>
## aliases

`function` · `datafusion_functions_aggregate::average::Avg::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:452`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fb94e681551a64554bb26934"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::average::Avg::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:374`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e0f01ad9ab52c6a7e9bf96"></a>
## default

`function` · `datafusion_functions_aggregate::average::Avg::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [125, 1], "end": [129, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/average.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e744f32c53c167fda805b536"></a>
## documentation

`function` · `datafusion_functions_aggregate::average::Avg::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:460`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e6cae80a28ded49eed12924d"></a>
## eq

`function` · `datafusion_functions_aggregate::average::Avg::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Avg) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 17], "end": [93, 26], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/average.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fd82bbedf261e30e26d62853"></a>
## fmt

`function` · `datafusion_functions_aggregate::average::Avg::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 10], "end": [93, 15], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/average.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98e6a12a1536e3e071e45fb6"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::average::Avg::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:362`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3bf83864d576fb7ca66a4f37"></a>
## hash

`function` · `datafusion_functions_aggregate::average::Avg::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 32], "end": [93, 36], "filename": "src/average.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/average.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9e6ddf99867c79219703dce8"></a>
## name

`function` · `datafusion_functions_aggregate::average::Avg::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-547b5dae772a84c59ba66265"></a>
## new

`function` · `datafusion_functions_aggregate::average::Avg::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [99, 1], "end": [123, 2], "filename": "src/average.rs"}, "trait": null, "trait_path": null}`

Source: `src/average.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6c6fc163986fa4664d07e01"></a>
## return_type

`function` · `datafusion_functions_aggregate::average::Avg::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:219`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ec46f36fdc052832c4eed5d"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::average::Avg::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:456`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b03c4302f7d620c9bb64648"></a>
## signature

`function` · `datafusion_functions_aggregate::average::Avg::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0fb1a1383d9945f670121c50"></a>
## state_fields

`function` · `datafusion_functions_aggregate::average::Avg::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::average::Avg", "path": "Avg"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [210, 1], "end": [463, 2], "filename": "src/average.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/average.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
