# `datafusion_expr::expr_fn::SimpleWindowUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.SimpleWindowUDF.json).

<a id="op-f1534acdb8d7c9ef98d83080"></a>
## SimpleWindowUDF

`struct` · `datafusion_expr::expr_fn::SimpleWindowUDF` · datafusion-expr 55.1.0

```rust
struct SimpleWindowUDF
```

Source: `src/expr_fn.rs:641`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Implements [`WindowUDFImpl`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e2b205ded2fa04ff480d51c9) for functions that have a single signature and
return type.

<a id="op-8a7a4f05e0c3f7d8cbcb0328"></a>
## eq

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SimpleWindowUDF) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 10], "end": [640, 19], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr_fn.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-69faa1ac09780d5a2f573bbd"></a>
## field

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::field` · datafusion-expr 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 1], "end": [707, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/expr_fn.rs:696`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f1f7113cf5554547024babcb"></a>
## fmt

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [648, 1], "end": [657, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr_fn.rs:649`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-12fa083fbb80387b42da6817"></a>
## hash

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [640, 25], "end": [640, 29], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr_fn.rs:640`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79e76959e369eaeed769b7bb"></a>
## limit_effect

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::limit_effect` · datafusion-expr 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 1], "end": [707, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/expr_fn.rs:704`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-631169cc480ff86087e8f9fe"></a>
## name

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 1], "end": [707, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/expr_fn.rs:681`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-82662d7c5269dfd786fb6151"></a>
## new

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::new` · datafusion-expr 55.1.0

```rust
fn new(name: impl Into<String>, input_type: DataType, return_type: DataType, volatility: Volatility, partition_evaluator_factory: PartitionEvaluatorFactory) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [659, 1], "end": [678, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:662`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `SimpleWindowUDF` from a name, input types, return type and
implementation. Implementing [`WindowUDFImpl`](../operations/datafusion_expr.udwf.WindowUDFImpl.md#op-e2b205ded2fa04ff480d51c9) allows more flexibility

<a id="op-1bccf0ac9973e8a39a6f9c6b"></a>
## partition_evaluator

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::partition_evaluator` · datafusion-expr 55.1.0

```rust
fn partition_evaluator(&self, _partition_evaluator_args: PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 1], "end": [707, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/expr_fn.rs:689`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df20d4cee941a074fee847a8"></a>
## signature

`function` · `datafusion_expr::expr_fn::SimpleWindowUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleWindowUDF", "path": "SimpleWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [680, 1], "end": [707, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/expr_fn.rs:685`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
