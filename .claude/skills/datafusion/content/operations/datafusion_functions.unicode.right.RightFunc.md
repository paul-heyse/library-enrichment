# `datafusion_functions::unicode::right::RightFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.right.RightFunc.json).

<a id="op-b03d83ff96c19ee1f569ce64"></a>
## RightFunc

`struct` · `datafusion_functions::unicode::right::RightFunc` · datafusion-functions 55.1.0

```rust
struct RightFunc
```

Source: `src/unicode/right.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51a6d0d1607966cd1981a95d"></a>
## default

`function` · `datafusion_functions::unicode::right::RightFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unicode/right.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d4148170f6a7b03771da6b36"></a>
## documentation

`function` · `datafusion_functions::unicode::right::RightFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/right.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-13218aea8d9a3c841bd28dde"></a>
## eq

`function` · `datafusion_functions::unicode::right::RightFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &RightFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 26], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unicode/right.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c875323e1e8784899bac01dd"></a>
## fmt

`function` · `datafusion_functions::unicode::right::RightFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unicode/right.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3fad9883e0da1d6b790ab51a"></a>
## hash

`function` · `datafusion_functions::unicode::right::RightFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 32], "end": [46, 36], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unicode/right.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1c73e59cd867b085b56d91e2"></a>
## invoke_with_args

`function` · `datafusion_functions::unicode::right::RightFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/right.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns right n characters in the string, or when n is negative, returns all but first |n| characters.
right('abcde', 2) = 'de'
right('abcde', -2) = 'cde'
The implementation uses UTF-8 code points as characters

<a id="op-f420346a311f2ebac16baa42"></a>
## name

`function` · `datafusion_functions::unicode::right::RightFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/right.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-779afd0ffb07618adc5c7f9b"></a>
## new

`function` · `datafusion_functions::unicode::right::RightFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [71, 2], "filename": "src/unicode/right.rs"}, "trait": null, "trait_path": null}`

Source: `src/unicode/right.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e678caab2085aa0df0eea0c"></a>
## return_type

`function` · `datafusion_functions::unicode::right::RightFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/right.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c8c636774efbd8bad0aacb1a"></a>
## signature

`function` · `datafusion_functions::unicode::right::RightFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::right::RightFunc", "path": "RightFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/right.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/right.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
