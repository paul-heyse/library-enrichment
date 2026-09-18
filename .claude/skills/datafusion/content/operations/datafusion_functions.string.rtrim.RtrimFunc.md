# `datafusion_functions::string::rtrim::RtrimFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.rtrim.RtrimFunc.json).

<a id="op-9cfdfbf06d9397d2538c9514"></a>
## RtrimFunc

`struct` · `datafusion_functions::string::rtrim::RtrimFunc` · datafusion-functions 55.1.0

```rust
struct RtrimFunc
```

Source: `src/string/rtrim.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8a0dcf9bd4c942c7dc50143e"></a>
## default

`function` · `datafusion_functions::string::rtrim::RtrimFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [89, 1], "end": [93, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/rtrim.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b5770a51f3dd4afe2fe7e26b"></a>
## documentation

`function` · `datafusion_functions::string::rtrim::RtrimFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [137, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/rtrim.rs:134`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1fbba0742423e481a4c8b977"></a>
## eq

`function` · `datafusion_functions::string::rtrim::RtrimFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &RtrimFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 17], "end": [84, 26], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/rtrim.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-36e8450c6ddf53c5c833d4fc"></a>
## fmt

`function` · `datafusion_functions::string::rtrim::RtrimFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 10], "end": [84, 15], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/rtrim.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-343f237f4ef61db92e7a72a4"></a>
## hash

`function` · `datafusion_functions::string::rtrim::RtrimFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 32], "end": [84, 36], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/rtrim.rs:84`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b40c11906cd854e171b195fc"></a>
## invoke_with_args

`function` · `datafusion_functions::string::rtrim::RtrimFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [137, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/rtrim.rs:130`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-98cbb0f00302f2913e2a22f1"></a>
## name

`function` · `datafusion_functions::string::rtrim::RtrimFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [137, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/rtrim.rs:118`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-026333a83e7fa13149310807"></a>
## new

`function` · `datafusion_functions::string::rtrim::RtrimFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [95, 1], "end": [115, 2], "filename": "src/string/rtrim.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/rtrim.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f2947fa6831f7e3539d9f6a"></a>
## return_type

`function` · `datafusion_functions::string::rtrim::RtrimFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [137, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/rtrim.rs:126`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f9b646963adcc18249a6481"></a>
## signature

`function` · `datafusion_functions::string::rtrim::RtrimFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::rtrim::RtrimFunc", "path": "RtrimFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [117, 1], "end": [137, 2], "filename": "src/string/rtrim.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/rtrim.rs:122`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
