# `datafusion_functions_aggregate::bool_and_or::BoolOr`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.bool_and_or.BoolOr.json).

<a id="op-9b78202e0b746a4efaae528a"></a>
## BoolOr

`struct` · `datafusion_functions_aggregate::bool_and_or::BoolOr` · datafusion-functions-aggregate 55.1.0

```rust
struct BoolOr
```

Source: `src/bool_and_or.rs:238`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

BOOL_OR aggregate expression

<a id="op-daef14a1f8cf7a45c9c931a8"></a>
## accumulator

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, _: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:269`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a3154544b27106fd91339d05"></a>
## clone

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::clone` · datafusion-functions-aggregate 55.1.0

```rust
fn clone(&self) -> BoolOr
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 17], "end": [237, 22], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/bool_and_or.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3941c73ae576c0988244a746"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6885f9fbc183f6e5d0298cb9"></a>
## default

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [254, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/bool_and_or.rs:251`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82240971513b2cde32ce8e90"></a>
## documentation

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:313`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3e8d80bec976c5904e5e8391"></a>
## eq

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &BoolOr) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 24], "end": [237, 33], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/bool_and_or.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ea07728c910cfb89103f8ecc"></a>
## fmt

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 10], "end": [237, 15], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/bool_and_or.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73754ebd03586676d735b74b"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, _args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8f177a7f7bc5a52fd1004b3"></a>
## hash

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [237, 39], "end": [237, 43], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/bool_and_or.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3aed790a0bcbf877e85718d"></a>
## name

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:257`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35a0f8fe45e6a92e92e4e37b"></a>
## order_sensitivity

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::order_sensitivity` · datafusion-functions-aggregate 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:305`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1a40168e6cbb9c81f6c0f6c3"></a>
## return_type

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88f4e5a46e6341592ed72ec5"></a>
## reverse_expr

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::reverse_expr` · datafusion-functions-aggregate 55.1.0

```rust
fn reverse_expr(&self) -> ReversedUDAF
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:309`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06b0ba6501ccd53d96704f91"></a>
## signature

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:261`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db1835a7884f083b25072b86"></a>
## state_fields

`function` · `datafusion_functions_aggregate::bool_and_or::BoolOr::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::bool_and_or::BoolOr", "path": "BoolOr"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [256, 1], "end": [316, 2], "filename": "src/bool_and_or.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/bool_and_or.rs:273`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
