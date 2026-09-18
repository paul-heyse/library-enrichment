# `datafusion_functions::unicode::left::LeftFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.unicode.left.LeftFunc.json).

<a id="op-346646dd3973bb7fe073d493"></a>
## LeftFunc

`struct` · `datafusion_functions::unicode::left::LeftFunc` · datafusion-functions 55.1.0

```rust
struct LeftFunc
```

Source: `src/unicode/left.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6417e215ed893bfb9777d536"></a>
## default

`function` · `datafusion_functions::unicode::left::LeftFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [51, 1], "end": [55, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/unicode/left.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d64a7fa1167d5dd27cae5287"></a>
## documentation

`function` · `datafusion_functions::unicode::left::LeftFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/left.rs:104`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1da3113f4cbc8a662f07d968"></a>
## eq

`function` · `datafusion_functions::unicode::left::LeftFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &LeftFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 17], "end": [46, 26], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/unicode/left.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-35ed85b2644cf23da82ec734"></a>
## fmt

`function` · `datafusion_functions::unicode::left::LeftFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 10], "end": [46, 15], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/unicode/left.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-869c3c516c6643cf8de24d48"></a>
## hash

`function` · `datafusion_functions::unicode::left::LeftFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [46, 32], "end": [46, 36], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/unicode/left.rs:46`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-90ba01e543db61bef2ff268c"></a>
## invoke_with_args

`function` · `datafusion_functions::unicode::left::LeftFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/left.rs:90`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Returns first n characters in the string, or when n is negative, returns all but last |n| characters.
left('abcde', 2) = 'ab'
left('abcde', -2) = 'abc'
The implementation uses UTF-8 code points as characters

<a id="op-85e6c81099be104b0098a787"></a>
## name

`function` · `datafusion_functions::unicode::left::LeftFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/left.rs:74`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6e6193d4fc6c5393f2d9d785"></a>
## new

`function` · `datafusion_functions::unicode::left::LeftFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [57, 1], "end": [71, 2], "filename": "src/unicode/left.rs"}, "trait": null, "trait_path": null}`

Source: `src/unicode/left.rs:58`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e93261881e216723506b2e5"></a>
## return_type

`function` · `datafusion_functions::unicode::left::LeftFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/left.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9cd85be0eb889abfee9fdd5"></a>
## signature

`function` · `datafusion_functions::unicode::left::LeftFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::unicode::left::LeftFunc", "path": "LeftFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [73, 1], "end": [107, 2], "filename": "src/unicode/left.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/unicode/left.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
