# `datafusion_functions::datetime::to_char::ToCharFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.to_char.ToCharFunc.json).

<a id="op-1e7701c010963dc726ac7b9d"></a>
## ToCharFunc

`struct` · `datafusion_functions::datetime::to_char::ToCharFunc` · datafusion-functions 55.1.0

```rust
struct ToCharFunc
```

Source: `src/datetime/to_char.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-112a3bda54c68823d49f2754"></a>
## aliases

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::aliases` · datafusion-functions 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:149`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88c673c96ed3f370df970b52"></a>
## default

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [68, 1], "end": [72, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/to_char.rs:69`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dfee4165b626059dfa62636a"></a>
## documentation

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:153`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b3e4a50ea3a2818d08d2d69"></a>
## eq

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ToCharFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 26], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/to_char.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bec2c749b9d121ce74231c7"></a>
## fmt

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/to_char.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b6201d339ab83cab8ccbb360"></a>
## hash

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 32], "end": [62, 36], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/to_char.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-11427fe2205b5cbf23cabe26"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-88a197d577aece8e70692e22"></a>
## name

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e869a08d9f4d1627acd151f"></a>
## new

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [115, 2], "filename": "src/datetime/to_char.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/to_char.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3c7cb867a4867d71696ff688"></a>
## return_type

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b4168d5ff8cca55a8ee5f4a9"></a>
## signature

`function` · `datafusion_functions::datetime::to_char::ToCharFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_char::ToCharFunc", "path": "ToCharFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [156, 2], "filename": "src/datetime/to_char.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_char.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
