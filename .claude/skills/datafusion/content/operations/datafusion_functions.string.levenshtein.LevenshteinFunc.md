# `datafusion_functions::string::levenshtein::LevenshteinFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.string.levenshtein.LevenshteinFunc.json).

<a id="op-0d91823927b0f8e216231f05"></a>
## LevenshteinFunc

`struct` · `datafusion_functions::string::levenshtein::LevenshteinFunc` · datafusion-functions 55.1.0

```rust
struct LevenshteinFunc
```

Source: `src/string/levenshtein.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f6feab93bf32096e1e2ff1c6"></a>
## default

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [64, 1], "end": [68, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/string/levenshtein.rs:65`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-28f92e7f5ad0448fb7e017bd"></a>
## documentation

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [122, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/levenshtein.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1aca003c84e140d72c4636a7"></a>
## eq

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &LevenshteinFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 17], "end": [59, 26], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/string/levenshtein.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b590ac9a9e82a5bacaf7140d"></a>
## fmt

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 10], "end": [59, 15], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/string/levenshtein.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-71c796d4775123d7fe17868f"></a>
## hash

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [59, 32], "end": [59, 36], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/string/levenshtein.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ebf0d89cc4bf0e720bb2b265"></a>
## invoke_with_args

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [122, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/levenshtein.rs:105`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ba6a3bdf18cf159605be776"></a>
## name

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [122, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/levenshtein.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4337a9dd55b11ba13a9f9f99"></a>
## new

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [70, 1], "end": [82, 2], "filename": "src/string/levenshtein.rs"}, "trait": null, "trait_path": null}`

Source: `src/string/levenshtein.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d9256affbe145ae94680eb0"></a>
## return_type

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [122, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/levenshtein.rs:93`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a2528591296c705ab78612d7"></a>
## signature

`function` · `datafusion_functions::string::levenshtein::LevenshteinFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::string::levenshtein::LevenshteinFunc", "path": "LevenshteinFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [84, 1], "end": [122, 2], "filename": "src/string/levenshtein.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/string/levenshtein.rs:89`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
