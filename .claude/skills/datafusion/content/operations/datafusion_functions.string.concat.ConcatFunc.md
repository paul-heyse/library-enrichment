# `datafusion_functions::string::concat::ConcatFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.concat.ConcatFunc.json).

<a id="op-b2a60fda30740ae30dc81516"></a>
## ConcatFunc

`struct` · `datafusion_functions::string::concat::ConcatFunc` · datafusion-functions 55.1.0

```rust
struct ConcatFunc
```

Source: `src/string/concat.rs:57`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cda8c7218230dd448e9a84f9"></a>
## coerce_types

`function` · `datafusion_functions::string::concat::ConcatFunc::coerce_types` · datafusion-functions 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Coerce all arguments to the widest type within the binary / string family

<a id="op-cbc8bca548a6c943c41483f9"></a>
## default

`function` · `datafusion_functions::string::concat::ConcatFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [61, 1], "end": [65, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/concat.rs:62`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-455e84259fbefbc818ec3021"></a>
## documentation

`function` · `datafusion_functions::string::concat::ConcatFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f5d61c16567abd3cc90fa5c5"></a>
## eq

`function` · `datafusion_functions::string::concat::ConcatFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ConcatFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 17], "end": [56, 26], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/concat.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-424aa7d8d22b4512ca201dcc"></a>
## fmt

`function` · `datafusion_functions::string::concat::ConcatFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 10], "end": [56, 15], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/concat.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-318988fc8981bbff21215dc1"></a>
## hash

`function` · `datafusion_functions::string::concat::ConcatFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [56, 32], "end": [56, 36], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/concat.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8346211218c52abde3ce82f1"></a>
## invoke_with_args

`function` · `datafusion_functions::string::concat::ConcatFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Concatenates the text representations of all the arguments. NULL arguments are ignored.
concat('abcde', 2, NULL, 22) = 'abcde222'

<a id="op-cf309185dcc0044dad09a3e7"></a>
## name

`function` · `datafusion_functions::string::concat::ConcatFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:82`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2b65a22cd67d25de29471d55"></a>
## new

`function` · `datafusion_functions::string::concat::ConcatFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [67, 1], "end": [76, 2], "filename": "src/string/concat.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/concat.rs:68`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a6332e9633a8a798dec6131e"></a>
## return_type

`function` · `datafusion_functions::string::concat::ConcatFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:102`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

mixed inputs, prefer Utf8View; prefer LargeUtf8 over Utf8 to avoid
potential overflow on LargeUtf8 input.
For binaries, use the similar hierarchy

<a id="op-42a3ed44c8d762ac91b3b631"></a>
## signature

`function` · `datafusion_functions::string::concat::ConcatFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:86`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2e05088ea6519b473a0199a2"></a>
## simplify

`function` · `datafusion_functions::string::concat::ConcatFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat::ConcatFunc", "path": "ConcatFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [81, 1], "end": [255, 2], "filename": "src/string/concat.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Simplify the `concat` function by
1. filtering out all `null` literals
2. concatenating contiguous literal arguments

For example:
`concat(col(a), 'hello ', 'world', col(b), null)`
will be optimized to
`concat(col(a), 'hello world', col(b))`
