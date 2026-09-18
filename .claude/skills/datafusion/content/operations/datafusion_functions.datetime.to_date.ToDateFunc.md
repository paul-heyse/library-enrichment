# `datafusion_functions::datetime::to_date::ToDateFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.to_date.ToDateFunc.json).

<a id="op-0ff5edded5c7241946d069af"></a>
## ToDateFunc

`struct` · `datafusion_functions::datetime::to_date::ToDateFunc` · datafusion-functions 55.1.0

```rust
struct ToDateFunc
```

Source: `src/datetime/to_date.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d6ca86a07f4f845130ab7669"></a>
## default

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [72, 1], "end": [76, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/to_date.rs:73`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f30c7a43198f6c14183ed2f"></a>
## documentation

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [190, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_date.rs:187`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee6e113841f7a775ab7457e8"></a>
## eq

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ToDateFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 17], "end": [67, 26], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/to_date.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b8573dc2acfbaec72f770ad8"></a>
## fmt

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 10], "end": [67, 15], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/to_date.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c5d297d5371ed9834a4f2b8a"></a>
## hash

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 32], "end": [67, 36], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/to_date.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d321b28c7d9a777c3570aa09"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [190, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_date.rs:132`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cef4efcd48d2114256aebc97"></a>
## name

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [190, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_date.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d31dba4794095b0b62b823f"></a>
## new

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [78, 1], "end": [117, 2], "filename": "src/datetime/to_date.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/to_date.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8e9ea3cf93fcf1522c6d8f6a"></a>
## return_type

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [190, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_date.rs:128`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ff8a8875eff78fdaa36f189b"></a>
## signature

`function` · `datafusion_functions::datetime::to_date::ToDateFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_date::ToDateFunc", "path": "ToDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [119, 1], "end": [190, 2], "filename": "src/datetime/to_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_date.rs:124`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
