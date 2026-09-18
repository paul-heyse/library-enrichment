# `datafusion_functions::string::uuid::UuidFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.uuid.UuidFunc.json).

<a id="op-f5f92f6840fdfdc2762ebce5"></a>
## UuidFunc

`struct` · `datafusion_functions::string::uuid::UuidFunc` · datafusion-functions 55.1.0

```rust
struct UuidFunc
```

Source: `src/string/uuid.rs:45`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0ae3550b229c21b4ca921db1"></a>
## default

`function` · `datafusion_functions::string::uuid::UuidFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 1], "end": [53, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/uuid.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d2718c5c288e1c552dd08b5e"></a>
## documentation

`function` · `datafusion_functions::string::uuid::UuidFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [110, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/uuid.rs:107`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aae55f0d281d3d9685ea35b8"></a>
## eq

`function` · `datafusion_functions::string::uuid::UuidFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &UuidFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 17], "end": [44, 26], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/uuid.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7e5424aa3669da29d66ed28"></a>
## fmt

`function` · `datafusion_functions::string::uuid::UuidFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 10], "end": [44, 15], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/uuid.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9240d526cd6830af1b427604"></a>
## hash

`function` · `datafusion_functions::string::uuid::UuidFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [44, 32], "end": [44, 36], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/uuid.rs:44`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-673ae17f0cdf5a423386e8c0"></a>
## invoke_with_args

`function` · `datafusion_functions::string::uuid::UuidFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [110, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/uuid.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Prints random (v4) uuid values per row
uuid() = 'a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11'

<a id="op-e9f5a5cf6ba890a0db987d20"></a>
## name

`function` · `datafusion_functions::string::uuid::UuidFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [110, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/uuid.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-79b13f7705cedb407569cf45"></a>
## new

`function` · `datafusion_functions::string::uuid::UuidFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [55, 1], "end": [61, 2], "filename": "src/string/uuid.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/uuid.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d8a6aed8a130852c82e1079a"></a>
## return_type

`function` · `datafusion_functions::string::uuid::UuidFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [110, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/uuid.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6206b5e6a0a6281e8488626c"></a>
## signature

`function` · `datafusion_functions::string::uuid::UuidFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::uuid::UuidFunc", "path": "UuidFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [63, 1], "end": [110, 2], "filename": "src/string/uuid.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/uuid.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
