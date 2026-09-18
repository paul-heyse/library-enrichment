# `datafusion_expr::expr_fn::SimpleAggregateUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.SimpleAggregateUDF.json).

<a id="op-ffc6e256701bcb4004eb4d15"></a>
## SimpleAggregateUDF

`struct` · `datafusion_expr::expr_fn::SimpleAggregateUDF` · datafusion-expr 55.1.0

```rust
struct SimpleAggregateUDF
```

Source: `src/expr_fn.rs:530`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Implements [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) for functions that have a single signature and
return type.

<a id="op-a57525ad58652bec56d5c4d5"></a>
## accumulator

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::accumulator` · datafusion-expr 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [614, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/expr_fn.rs:604`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3922bc25946f59442b044b05"></a>
## eq

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SimpleAggregateUDF) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 10], "end": [529, 19], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr_fn.rs:529`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-39e750668fb26067de8e256f"></a>
## fmt

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [538, 1], "end": [547, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr_fn.rs:539`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0a8cec17ebedbe1aff811fb"></a>
## hash

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [529, 25], "end": [529, 29], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr_fn.rs:529`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-560be25b9970837623a7f723"></a>
## name

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [614, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/expr_fn.rs:592`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-822a3354f176eaae3d77e8f7"></a>
## new

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::new` · datafusion-expr 55.1.0

```rust
fn new(name: impl Into<String>, input_type: Vec<DataType>, return_type: DataType, volatility: Volatility, accumulator: AccumulatorFactoryFunction, state_fields: Vec<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [589, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:552`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `SimpleAggregateUDF` from a name, input types, return type, state type and
implementation. Implementing [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) allows more flexibility

<a id="op-4e0bbeded243c77088b4b87c"></a>
## new_with_signature

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::new_with_signature` · datafusion-expr 55.1.0

```rust
fn new_with_signature(name: impl Into<String>, signature: Signature, return_type: DataType, accumulator: AccumulatorFactoryFunction, state_fields: Vec<FieldRef>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [549, 1], "end": [589, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `SimpleAggregateUDF` from a name, signature, return type, state type and
implementation. Implementing [`AggregateUDFImpl`](../operations/datafusion_expr.udaf.AggregateUDFImpl.md#op-9f175a3c2e0fb1ef9e53572c) allows more flexibility

<a id="op-01b0a064b0641adf64f70722"></a>
## return_type

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [614, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/expr_fn.rs:600`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-164a335272225d341392adba"></a>
## signature

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [614, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/expr_fn.rs:596`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5228bb5d48a6df4e2f84f5fa"></a>
## state_fields

`function` · `datafusion_expr::expr_fn::SimpleAggregateUDF::state_fields` · datafusion-expr 55.1.0

```rust
fn state_fields(&self, _args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleAggregateUDF", "path": "SimpleAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [591, 1], "end": [614, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/expr_fn.rs:611`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
