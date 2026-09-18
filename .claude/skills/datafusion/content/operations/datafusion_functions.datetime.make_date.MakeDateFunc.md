# `datafusion_functions::datetime::make_date::MakeDateFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.make_date.MakeDateFunc.json).

<a id="op-ac5e9e7123b5db5bfc928587"></a>
## MakeDateFunc

`struct` · `datafusion_functions::datetime::make_date::MakeDateFunc` · datafusion-functions 55.1.0

```rust
struct MakeDateFunc
```

Source: `src/datetime/make_date.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-651ef649435f6db18029ef36"></a>
## default

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [79, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/make_date.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25eba23e19da93ab9f0d57a8"></a>
## documentation

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [170, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/make_date.rs:167`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c163adb451c951eb12c9cb43"></a>
## eq

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &MakeDateFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 17], "end": [70, 26], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/make_date.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e5cf0e8aab12a1573b656fef"></a>
## fmt

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 10], "end": [70, 15], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/make_date.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-057e8e4ce2677450bff67c5c"></a>
## hash

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 32], "end": [70, 36], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/make_date.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-60ce8d33fc5a55ad2322a820"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [170, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/make_date.rs:110`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-946e3d9f4a5e5e0c0eebebfa"></a>
## name

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [170, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/make_date.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-53a790ba3f54402d442b9a90"></a>
## new

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [95, 2], "filename": "src/datetime/make_date.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/make_date.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fac6926aaf30961b9f0249fa"></a>
## return_type

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [170, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/make_date.rs:106`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-23a9cb8fb8d6ddd6b497eabe"></a>
## signature

`function` · `datafusion_functions::datetime::make_date::MakeDateFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::make_date::MakeDateFunc", "path": "MakeDateFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [97, 1], "end": [170, 2], "filename": "src/datetime/make_date.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/make_date.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
