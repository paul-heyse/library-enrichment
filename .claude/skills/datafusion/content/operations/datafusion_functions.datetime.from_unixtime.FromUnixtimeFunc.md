# `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.from_unixtime.FromUnixtimeFunc.json).

<a id="op-d7a437f25e8e646f2002a128"></a>
## FromUnixtimeFunc

`struct` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc` · datafusion-functions 55.1.0

```rust
struct FromUnixtimeFunc
```

Source: `src/datetime/from_unixtime.rs:51`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d68c893f4099035614b8528f"></a>
## default

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [59, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/from_unixtime.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee7afe5a1a1c448f15649bdd"></a>
## documentation

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7286577bdeaad673fd265512"></a>
## eq

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &FromUnixtimeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 17], "end": [50, 26], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/from_unixtime.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1b6120d4c82a178ed1ea10f6"></a>
## fmt

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 10], "end": [50, 15], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/from_unixtime.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf0855fd7425a56f1c1468fe"></a>
## hash

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [50, 32], "end": [50, 36], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/from_unixtime.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f93422daa5316ac3938696ec"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-987d853ee691ce6305b39390"></a>
## name

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aced381391821018ab4df9a3"></a>
## new

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [70, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/from_unixtime.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86b7416f21b0b0db212a5b01"></a>
## output_ordering

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::output_ordering` · datafusion-functions 55.1.0

```rust
fn output_ordering(&self, inputs: &[ExprProperties]) -> Result<SortProperties>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:151`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b0e2c4d4c53089bd2862bda"></a>
## preserves_lex_ordering

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::preserves_lex_ordering` · datafusion-functions 55.1.0

```rust
fn preserves_lex_ordering(&self, _inputs: &[ExprProperties]) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:158`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35746a743085cab677bf1048"></a>
## return_field_from_args

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:81`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee6b6d51d1485ff7204651dd"></a>
## return_type

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:114`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a43a682c068c738702704890"></a>
## signature

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dccd8b7b2f0bccbba24dacc7"></a>
## strictly_order_preserving

`function` · `datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc::strictly_order_preserving` · datafusion-functions 55.1.0

```rust
fn strictly_order_preserving(&self, _inputs: &[ExprProperties]) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::from_unixtime::FromUnixtimeFunc", "path": "FromUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [172, 2], "filename": "src/datetime/from_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/from_unixtime.rs:162`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
