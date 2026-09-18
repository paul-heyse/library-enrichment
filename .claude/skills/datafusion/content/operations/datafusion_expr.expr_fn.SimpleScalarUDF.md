# `datafusion_expr::expr_fn::SimpleScalarUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.expr_fn.SimpleScalarUDF.json).

<a id="op-8514b6e98d76ad14e6557f29"></a>
## SimpleScalarUDF

`struct` · `datafusion_expr::expr_fn::SimpleScalarUDF` · datafusion-expr 55.1.0

```rust
struct SimpleScalarUDF
```

Source: `src/expr_fn.rs:428`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Implements [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) for functions that have a single signature and
return type.

<a id="op-4c8917d31bf4b99340e2e9ec"></a>
## eq

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &SimpleScalarUDF) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 10], "end": [427, 19], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/expr_fn.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-889a8675851c34d998a52fb2"></a>
## fmt

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [435, 1], "end": [444, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/expr_fn.rs:436`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-49e4f36784e632ed38a3afc7"></a>
## hash

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [427, 25], "end": [427, 29], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/expr_fn.rs:427`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1a7faade956529ff4cd68f1"></a>
## invoke_with_args

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::invoke_with_args` · datafusion-expr 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [497, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/expr_fn.rs:494`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b39dae33ea9f63fc4a011044"></a>
## name

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [497, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/expr_fn.rs:482`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-70a8b1685a43fd6781f6057f"></a>
## new

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::new` · datafusion-expr 55.1.0

```rust
fn new(name: impl Into<String>, input_types: Vec<DataType>, return_type: DataType, volatility: Volatility, fun: ScalarFunctionImplementation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [446, 1], "end": [479, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `SimpleScalarUDF` from a name, input types, return type and
implementation. Implementing [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) allows more flexibility

<a id="op-acfccaba3d43f1e8a2e14c1d"></a>
## new_with_signature

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::new_with_signature` · datafusion-expr 55.1.0

```rust
fn new_with_signature(name: impl Into<String>, signature: Signature, return_type: DataType, fun: ScalarFunctionImplementation) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [446, 1], "end": [479, 2], "filename": "src/expr_fn.rs"}, "trait": null, "trait_path": null}`

Source: `src/expr_fn.rs:466`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Create a new `SimpleScalarUDF` from a name, signature, return type and
implementation. Implementing [`ScalarUDFImpl`](../operations/datafusion_expr.udf.ScalarUDFImpl.md#op-7caee2ff206563b536e983e0) allows more flexibility

<a id="op-acab0a55f70b2f817201094a"></a>
## return_type

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::return_type` · datafusion-expr 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [497, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/expr_fn.rs:490`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-401a415805818d6733c26ce0"></a>
## signature

`function` · `datafusion_expr::expr_fn::SimpleScalarUDF::signature` · datafusion-expr 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::expr_fn::SimpleScalarUDF", "path": "SimpleScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [481, 1], "end": [497, 2], "filename": "src/expr_fn.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/expr_fn.rs:486`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
