# `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.to_unixtime.ToUnixtimeFunc.json).

<a id="op-da984a047929a2ab75c78aa0"></a>
## ToUnixtimeFunc

`struct` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc` · datafusion-functions 55.1.0

```rust
struct ToUnixtimeFunc
```

Source: `src/datetime/to_unixtime.rs:63`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-411d90707781646f2c4923c8"></a>
## default

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [71, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/to_unixtime.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-366cec7e237a8be051252ad9"></a>
## documentation

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [149, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_unixtime.rs:146`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37e4a73f3a87d4c05f17eb73"></a>
## eq

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ToUnixtimeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 17], "end": [62, 26], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/to_unixtime.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83d48571ca65ece12255746f"></a>
## fmt

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 10], "end": [62, 15], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/to_unixtime.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a760ee325eb2b67d2b587196"></a>
## hash

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [62, 32], "end": [62, 36], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/to_unixtime.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8b8fa76baab8f1d6e5e94b95"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [149, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_unixtime.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3db41bbe964f59e1363c8489"></a>
## name

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [149, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_unixtime.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-aecba071e9e4ec03fd8ab53d"></a>
## new

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [79, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/to_unixtime.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-96e6bb7232108b0da360086a"></a>
## return_type

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [149, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_unixtime.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-52204a0c247e7d0f063dc9cb"></a>
## signature

`function` · `datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::to_unixtime::ToUnixtimeFunc", "path": "ToUnixtimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [149, 2], "filename": "src/datetime/to_unixtime.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/to_unixtime.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
