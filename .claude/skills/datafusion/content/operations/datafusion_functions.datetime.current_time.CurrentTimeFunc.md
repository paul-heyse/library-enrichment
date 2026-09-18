# `datafusion_functions::datetime::current_time::CurrentTimeFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.datetime.current_time.CurrentTimeFunc.json).

<a id="op-0f1378cc19d2befb0afa7b22"></a>
## CurrentTimeFunc

`struct` · `datafusion_functions::datetime::current_time::CurrentTimeFunc` · datafusion-functions 55.1.0

```rust
struct CurrentTimeFunc
```

Source: `src/datetime/current_time.rs:61`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a8094fe4cff52a640616ed01"></a>
## default

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [65, 1], "end": [69, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/datetime/current_time.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8d9a2637abaee3c7d49949ce"></a>
## documentation

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1f15bd25f9c45308389bf366"></a>
## eq

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &CurrentTimeFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 17], "end": [60, 26], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/datetime/current_time.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ed3aea8bf0dfc4c51363d7ec"></a>
## fmt

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 10], "end": [60, 15], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/datetime/current_time.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6090eeb5d74f55a5e5105534"></a>
## hash

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [60, 32], "end": [60, 36], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/datetime/current_time.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-da861abb7626a96153560fc0"></a>
## invoke_with_args

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:98`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ae7831633ac0d514981a8308"></a>
## name

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2dae6a7d96fcb76b270deb95"></a>
## new

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [71, 1], "end": [77, 2], "filename": "src/datetime/current_time.rs"}, "trait": null, "trait_path": null}`

Source: `src/datetime/current_time.rs:72`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8dd688e45641e62a529fa98"></a>
## return_type

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7fc61f7bb3711a5f3443b9b0"></a>
## signature

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd9765a83eb78dcef55a73f2"></a>
## simplify

`function` · `datafusion_functions::datetime::current_time::CurrentTimeFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::datetime::current_time::CurrentTimeFunc", "path": "CurrentTimeFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [85, 1], "end": [137, 2], "filename": "src/datetime/current_time.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/datetime/current_time.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
