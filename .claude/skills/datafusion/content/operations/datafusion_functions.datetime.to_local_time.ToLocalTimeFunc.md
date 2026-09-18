# `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.to_local_time.ToLocalTimeFunc.json).

<a id="op-3db2a6dcfbecf3fa121e0b78"></a>
## ToLocalTimeFunc

`struct` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc` · datafusion-functions 55.1.0

```rust
struct ToLocalTimeFunc
```

Source: `src/datetime/to_local_time.rs:101`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

A UDF function that converts a timezone-aware timestamp to local time (with no offset or
timezone information). In other words, this function strips off the timezone from the timestamp,
while keep the display value of the timestamp the same.

<a id="op-e9b0ebc2e1a1be9af32f8304"></a>
## default

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [105, 1], "end": [109, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/to_local_time.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f96341d891bc7d622619b46c"></a>
## documentation

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [149, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_local_time.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-89b67e2cfa596d5098e2d4d6"></a>
## eq

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ToLocalTimeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 17], "end": [100, 26], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/to_local_time.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d603b61dfa870af847b159a5"></a>
## fmt

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 10], "end": [100, 15], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/to_local_time.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-33b6e14bbf91bdf17eaa1a8d"></a>
## hash

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [100, 32], "end": [100, 36], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/to_local_time.rs:100`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d7028c6f1a3156bda369d93e"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [149, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_local_time.rs:141`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc9bc6d1e7655dfdb379f53c"></a>
## name

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [149, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_local_time.rs:123`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-44d38bf6f4ebb5571a50e0d0"></a>
## new

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [111, 1], "end": [120, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/to_local_time.rs:112`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-769fd7d54f0c2258052897d0"></a>
## return_type

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [149, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_local_time.rs:131`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a70b86e8401bc86c90386c18"></a>
## signature

`function` · `datafusion_functions::datetime::to_local_time::ToLocalTimeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_local_time::ToLocalTimeFunc", "path": "ToLocalTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [122, 1], "end": [149, 2], "filename": "src/datetime/to_local_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_local_time.rs:127`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
