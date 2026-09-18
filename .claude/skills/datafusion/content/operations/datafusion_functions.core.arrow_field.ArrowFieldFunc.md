# `datafusion_functions::core::arrow_field::ArrowFieldFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.core.arrow_field.ArrowFieldFunc.json).

<a id="op-f1cc043e3bcd5afd6912d64f"></a>
## ArrowFieldFunc

`struct` · `datafusion_functions::core::arrow_field::ArrowFieldFunc` · datafusion-functions 55.1.0

```rust
struct ArrowFieldFunc
```

Source: `src/core/arrow_field.rs:55`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-02e84328597ea94f6599a675"></a>
## clone

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::clone` · datafusion-functions 55.1.0

```rust
fn clone(&self) -> ArrowFieldFunc
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 10], "end": [54, 15], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/core/arrow_field.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7092bd7c00ee8a7ee036dd1e"></a>
## default

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 1], "end": [63, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/core/arrow_field.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cf13f8e2ceec4664aeb79696"></a>
## documentation

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [162, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_field.rs:159`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-243150e843d11ee23641c40e"></a>
## eq

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ArrowFieldFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 24], "end": [54, 33], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/core/arrow_field.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a557cf0aceacf3a3567916d"></a>
## fmt

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 17], "end": [54, 22], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/core/arrow_field.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1770378ae1f70116324d238c"></a>
## hash

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [54, 39], "end": [54, 43], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/core/arrow_field.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6c97407f9c32277139eba0b1"></a>
## invoke_with_args

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [162, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_field.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c7978394a94e63ab51327bda"></a>
## name

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [162, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_field.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-24ee8dbe3bd3ee20d437271a"></a>
## new

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [94, 2], "filename": "src/core/arrow_field.rs"}, "trait": null, "trait_path": null}`

Source: `src/core/arrow_field.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dee5ee8d8d1ebe8fb4ced49"></a>
## return_type

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [162, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_field.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9b719cab9fd0d1fda4aa0bb"></a>
## signature

`function` · `datafusion_functions::core::arrow_field::ArrowFieldFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::core::arrow_field::ArrowFieldFunc", "path": "ArrowFieldFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [96, 1], "end": [162, 2], "filename": "src/core/arrow_field.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/core/arrow_field.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
