# `datafusion_functions::string::concat_ws::ConcatWsFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.concat_ws.ConcatWsFunc.json).

<a id="op-a0951ffd98c6c3efb8011743"></a>
## ConcatWsFunc

`struct` · `datafusion_functions::string::concat_ws::ConcatWsFunc` · datafusion-functions 55.1.0

```rust
struct ConcatWsFunc
```

Source: `src/string/concat_ws.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1cf2cf28f455eb33b4fde655"></a>
## coerce_types

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::coerce_types` · datafusion-functions 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:96`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Coerce all arguments to the widest type within the binary / string family

<a id="op-f90e7227438057a3ee207dcd"></a>
## default

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [69, 1], "end": [73, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/concat_ws.rs:70`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9b0845a1f375677971421a44"></a>
## documentation

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2fe2e292405156910112b962"></a>
## eq

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &ConcatWsFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 17], "end": [64, 26], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/concat_ws.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-62ae0e937869d86bcd831e35"></a>
## fmt

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 10], "end": [64, 15], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/concat_ws.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7e8eb7fc55db512df433dd0"></a>
## hash

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 32], "end": [64, 36], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/concat_ws.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f0ca483c92c1b90655a17f01"></a>
## invoke_with_args

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:116`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Concatenates all but the first argument, with separators. The first
argument is used as the separator string, and should not be NULL. Other
NULL arguments are ignored.
concat_ws(',', 'abcde', 2, NULL, 22) = 'abcde,2,22'

<a id="op-2b586592817d0f0c3e416a93"></a>
## name

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8f603bbb6e046e40364e2f28"></a>
## new

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [75, 1], "end": [84, 2], "filename": "src/string/concat_ws.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/concat_ws.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bd0b1beccfc1ad348440f5e1"></a>
## return_type

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:108`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Match the return type to the input types. Delegates to `concat` implementation.

<a id="op-20964c3ca96671340120a9f6"></a>
## signature

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:91`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fa7f9debd058729a00475e1f"></a>
## simplify

`function` · `datafusion_functions::string::concat_ws::ConcatWsFunc::simplify` · datafusion-functions 55.1.0

```rust
fn simplify(&self, args: Vec<Expr>, _info: &SimplifyContext) -> Result<ExprSimplifyResult>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::concat_ws::ConcatWsFunc", "path": "ConcatWsFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [86, 1], "end": [325, 2], "filename": "src/string/concat_ws.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/concat_ws.rs:311`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

Simply the `concat_ws` function by
1. folding to `null` if the delimiter is null
2. filtering out `null` arguments
3. using `concat` to replace `concat_ws` if the delimiter is an empty string
4. concatenating contiguous literals if the delimiter is a literal.
