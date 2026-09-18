# `datafusion_functions::datetime::date_part::DatePartFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.date_part.DatePartFunc.json).

<a id="op-f4ffb65114fc357342431dc6"></a>
## DatePartFunc

`struct` · `datafusion_functions::datetime::date_part::DatePartFunc` · datafusion-functions 55.1.0

```rust
struct DatePartFunc
```

Source: `src/datetime/date_part.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-293bf6ea7523e03c000c3ec7"></a>
## aliases

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:324`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fab8022deb212ff2c4f07dc1"></a>
## default

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [112, 1], "end": [116, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/date_part.rs:113`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5cedcaa14e1b2d010b8822f5"></a>
## documentation

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:328`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f01897c1fcc878160db78fe5"></a>
## eq

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &DatePartFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 17], "end": [106, 26], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/date_part.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88dc0def4ac5cb34092fe6a5"></a>
## fmt

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 10], "end": [106, 15], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/date_part.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90fff24e8ccb47c6c8bdfc76"></a>
## hash

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [106, 32], "end": [106, 36], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/date_part.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b1d424ec4ba2acd0ddbe8cda"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee0e35f966959e2d3045a825"></a>
## name

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:157`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4c8bfdb415d8d55e0d8a4176"></a>
## new

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [154, 2], "filename": "src/datetime/date_part.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/date_part.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-271bb206138a6322209b03c2"></a>
## preimage

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::preimage` · datafusion-functions 55.1.0

```rust
fn preimage(&self, args: &[Expr], lit_expr: &Expr, info: &SimplifyContext) -> Result<PreimageResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:263`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e021b131ae14e6348f38e0cc"></a>
## return_field_from_args

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::return_field_from_args` · datafusion-functions 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:169`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b985d05e3e031abe2ff245dd"></a>
## return_type

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-43774b60d3e10ae6ddd6c0f3"></a>
## signature

`function` · `datafusion_functions::datetime::date_part::DatePartFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::date_part::DatePartFunc", "path": "DatePartFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [156, 1], "end": [331, 2], "filename": "src/datetime/date_part.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/date_part.rs:161`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
