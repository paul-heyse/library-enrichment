# `datafusion_functions::math::factorial::FactorialFunc`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_functions.math.factorial.FactorialFunc.json).

<a id="op-f39c2437707337ed57611fda"></a>
## FactorialFunc

`struct` · `datafusion_functions::math::factorial::FactorialFunc` · datafusion-functions 55.1.0

```rust
struct FactorialFunc
```

Source: `src/math/factorial.rs:48`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ce537831d2a17e7d66329217"></a>
## default

`function` · `datafusion_functions::math::factorial::FactorialFunc::default` · datafusion-functions 55.1.0

```rust
fn default() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [52, 1], "end": [56, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "core::default::Default", "path": "Default"}, "trait_path": "core::default::Default"}`

Source: `src/math/factorial.rs:53`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3086254f414df9c0d3b7ddcf"></a>
## documentation

`function` · `datafusion_functions::math::factorial::FactorialFunc::documentation` · datafusion-functions 55.1.0

```rust
fn documentation(&self) -> Option<&Documentation>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:119`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7ae79db7ab3adfcff7b286a3"></a>
## eq

`function` · `datafusion_functions::math::factorial::FactorialFunc::eq` · datafusion-functions 55.1.0

```rust
fn eq(&self, other: &FactorialFunc) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 17], "end": [47, 26], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/math/factorial.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fce34d1bd4d12a47985c7582"></a>
## fmt

`function` · `datafusion_functions::math::factorial::FactorialFunc::fmt` · datafusion-functions 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 10], "end": [47, 15], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/math/factorial.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b0ad76cf4b60bc24e052d65b"></a>
## hash

`function` · `datafusion_functions::math::factorial::FactorialFunc::hash` · datafusion-functions 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [47, 32], "end": [47, 36], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/math/factorial.rs:47`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eca7d047e4ad613624b09375"></a>
## invoke_with_args

`function` · `datafusion_functions::math::factorial::FactorialFunc::invoke_with_args` · datafusion-functions 55.1.0

```rust
fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:83`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba4e59863b683a5720d6ff3b"></a>
## is_strict

`function` · `datafusion_functions::math::factorial::FactorialFunc::is_strict` · datafusion-functions 55.1.0

```rust
fn is_strict(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:79`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1bc89a0bb834cbefa76fa775"></a>
## name

`function` · `datafusion_functions::math::factorial::FactorialFunc::name` · datafusion-functions 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:67`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bb773a39050f0e029ad059fb"></a>
## new

`function` · `datafusion_functions::math::factorial::FactorialFunc::new` · datafusion-functions 55.1.0

```rust
fn new() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [58, 1], "end": [64, 2], "filename": "src/math/factorial.rs"}, "trait": null, "trait_path": null}`

Source: `src/math/factorial.rs:59`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2643f8d7b996d253ae1b07c3"></a>
## return_type

`function` · `datafusion_functions::math::factorial::FactorialFunc::return_type` · datafusion-functions 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:75`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d14fcdadc692b4100adac751"></a>
## signature

`function` · `datafusion_functions::math::factorial::FactorialFunc::signature` · datafusion-functions 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_functions::math::factorial::FactorialFunc", "path": "FactorialFunc"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [66, 1], "end": [122, 2], "filename": "src/math/factorial.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/math/factorial.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-functions/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
