# `datafusion_spark::function::string::format_string::NumericParam`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.format_string.NumericParam.json).

<a id="op-4de1615c1a7404e2b39f9df5"></a>
## NumericParam

`enum` · `datafusion_spark::function::string::format_string::NumericParam` · datafusion-spark 55.1.0

```rust
enum NumericParam
```

Source: `src/function/string/format_string.rs:420`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Width / precision parameter

<a id="op-ddbe25d2e93b941db50e410e"></a>
## FromArgument

`variant` · `datafusion_spark::function::string::format_string::NumericParam::FromArgument` · datafusion-spark 55.1.0

```rust
FromArgument
```

Source: `src/function/string/format_string.rs:424`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Get the width from the previous argument

<a id="op-469f6d3becfef259450b6c83"></a>
## Literal

`variant` · `datafusion_spark::function::string::format_string::NumericParam::Literal` · datafusion-spark 55.1.0

```rust
Literal
```

Source: `src/function/string/format_string.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

The literal width

<a id="op-86c16ff6f38115cda78e5b1e"></a>
## clone

`function` · `datafusion_spark::function::string::format_string::NumericParam::clone` · datafusion-spark 55.1.0

```rust
fn clone(&self) -> NumericParam
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::NumericParam", "path": "NumericParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 17], "end": [419, 22], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/function/string/format_string.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-578fa865fe464bb9ddaa5300"></a>
## eq

`function` · `datafusion_spark::function::string::format_string::NumericParam::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &NumericParam) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::NumericParam", "path": "NumericParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 30], "end": [419, 39], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/format_string.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9d3ca5ec90aff368bbdb1a01"></a>
## fmt

`function` · `datafusion_spark::function::string::format_string::NumericParam::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::NumericParam", "path": "NumericParam"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [419, 10], "end": [419, 15], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/format_string.rs:419`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
