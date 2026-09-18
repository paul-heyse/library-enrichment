# `datafusion_functions_nested::dimension::ArrayDims`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions_nested.dimension.ArrayDims.json).

<a id="op-aeeb6c1d73eb84a6fc17ce55"></a>
## ArrayDims

`struct` · `datafusion_functions_nested::dimension::ArrayDims` · datafusion-functions-nested 55.1.0

```rust
struct ArrayDims
```

Source: `src/dimension.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b82bd11a6e6ac0531ab4b1c"></a>
## aliases

`function` · `datafusion_functions_nested::dimension::ArrayDims::aliases` · datafusion-functions-nested 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6d5d341e1b1da49c6342015c"></a>
## default

`function` · `datafusion_functions_nested::dimension::ArrayDims::default` · datafusion-functions-nested 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [77, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/dimension.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ee294933ec6ad5d26d41fa0"></a>
## documentation

`function` · `datafusion_functions_nested::dimension::ArrayDims::documentation` · datafusion-functions-nested 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:109`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-19c7e185236045bad896b476"></a>
## eq

`function` · `datafusion_functions_nested::dimension::ArrayDims::eq` · datafusion-functions-nested 55.1.0

```rust
fn eq(&self, other: &ArrayDims) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 26], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/dimension.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f84ddefedaa31e2011819bbe"></a>
## fmt

`function` · `datafusion_functions_nested::dimension::ArrayDims::fmt` · datafusion-functions-nested 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/dimension.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e96cf9322f56c4a55f79cba7"></a>
## hash

`function` · `datafusion_functions_nested::dimension::ArrayDims::hash` · datafusion-functions-nested 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 32], "end": [67, 36], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/dimension.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5bcc3f91ee8827c15a6f62a0"></a>
## invoke_with_args

`function` · `datafusion_functions_nested::dimension::ArrayDims::invoke_with_args` · datafusion-functions-nested 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b9f45ab438eca4e16d2b6d8b"></a>
## name

`function` · `datafusion_functions_nested::dimension::ArrayDims::name` · datafusion-functions-nested 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-31b716521606f898ea3458eb"></a>
## new

`function` · `datafusion_functions_nested::dimension::ArrayDims::new` · datafusion-functions-nested 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [79, 1], "end": [86, 2], "filename": "src/dimension.rs"}, "trait": null, "trait_path": null}`

Source: `src/dimension.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-66e409da73bb735d32ef1b03"></a>
## return_type

`function` · `datafusion_functions_nested::dimension::ArrayDims::return_type` · datafusion-functions-nested 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:97`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c3589b5274397b3cd0d92371"></a>
## signature

`function` · `datafusion_functions_nested::dimension::ArrayDims::signature` · datafusion-functions-nested 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions_nested::dimension::ArrayDims", "path": "ArrayDims"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [88, 1], "end": [112, 2], "filename": "src/dimension.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/dimension.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions-nested/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
