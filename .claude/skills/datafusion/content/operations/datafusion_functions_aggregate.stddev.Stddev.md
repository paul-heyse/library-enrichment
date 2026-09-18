# `datafusion_functions_aggregate::stddev::Stddev`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_aggregate.stddev.Stddev.json).

<a id="op-da528fc691480495a9e33606"></a>
## Stddev

`struct` · `datafusion_functions_aggregate::stddev::Stddev` · datafusion-functions-aggregate 55.1.0

```rust
struct Stddev
```

Source: `src/stddev.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

STDDEV and STDDEV_SAMP (standard deviation) aggregate expression

<a id="op-972a1577cbf12f55111485f3"></a>
## accumulator

`function` · `datafusion_functions_aggregate::stddev::Stddev::accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68f6463a93b5a850aec53dc7"></a>
## aliases

`function` · `datafusion_functions_aggregate::stddev::Stddev::aliases` · datafusion-functions-aggregate 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:125`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25bb1e3c12858140db448cde"></a>
## create_groups_accumulator

`function` · `datafusion_functions_aggregate::stddev::Stddev::create_groups_accumulator` · datafusion-functions-aggregate 55.1.0

```rust
fn create_groups_accumulator(&self, _args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:133`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e6eb368ccef6d4377e42691"></a>
## default

`function` · `datafusion_functions_aggregate::stddev::Stddev::default` · datafusion-functions-aggregate 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [74, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/stddev.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ddc6ba7b1e7fa98027b300f8"></a>
## documentation

`function` · `datafusion_functions_aggregate::stddev::Stddev::documentation` · datafusion-functions-aggregate 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:140`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-744cab6bb49ef0a65111dac2"></a>
## eq

`function` · `datafusion_functions_aggregate::stddev::Stddev::eq` · datafusion-functions-aggregate 55.1.0

```rust
fn eq(&self, other: &Stddev) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 19], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/stddev.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9bea25ba0d58f5ec7b0cde72"></a>
## fmt

`function` · `datafusion_functions_aggregate::stddev::Stddev::fmt` · datafusion-functions-aggregate 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 31], "end": [64, 36], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/stddev.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-77455ef99f01be9af0c4e81a"></a>
## groups_accumulator_supported

`function` · `datafusion_functions_aggregate::stddev::Stddev::groups_accumulator_supported` · datafusion-functions-aggregate 55.1.0

```rust
fn groups_accumulator_supported(&self, acc_args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ec15662d0cc594d6efff910c"></a>
## hash

`function` · `datafusion_functions_aggregate::stddev::Stddev::hash` · datafusion-functions-aggregate 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 25], "end": [64, 29], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/stddev.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-978a57affca0fb09fbea0bf5"></a>
## name

`function` · `datafusion_functions_aggregate::stddev::Stddev::name` · datafusion-functions-aggregate 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09907a18f3bdeea8e4e81ea6"></a>
## new

`function` · `datafusion_functions_aggregate::stddev::Stddev::new` · datafusion-functions-aggregate 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [76, 1], "end": [84, 2], "filename": "src/stddev.rs"}, "trait": null, "trait_path": null}`

Source: `src/stddev.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

Create a new STDDEV aggregate function

<a id="op-b00900124a79b2bed6b63138"></a>
## return_type

`function` · `datafusion_functions_aggregate::stddev::Stddev::return_type` · datafusion-functions-aggregate 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:95`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-136069fb6ea7b25c2497ea97"></a>
## signature

`function` · `datafusion_functions_aggregate::stddev::Stddev::signature` · datafusion-functions-aggregate 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-478cc979c55fc18e3cd6fa01"></a>
## state_fields

`function` · `datafusion_functions_aggregate::stddev::Stddev::state_fields` · datafusion-functions-aggregate 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_aggregate::stddev::Stddev", "path": "Stddev"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [143, 2], "filename": "src/stddev.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/stddev.rs:99`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-aggregate/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
