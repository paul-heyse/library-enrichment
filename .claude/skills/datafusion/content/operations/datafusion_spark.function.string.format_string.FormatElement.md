# `datafusion_spark::function::string::format_string::FormatElement`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.format_string.FormatElement.json).

<a id="op-e8640ca64613d1c2f053e17e"></a>
## FormatElement

`enum` · `datafusion_spark::function::string::format_string::FormatElement` · datafusion-spark 55.1.0

```rust
enum FormatElement<'a>
```

Source: `src/function/string/format_string.rs:385`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-67eb74f21afa957b269caa8a"></a>
## Format

`variant` · `datafusion_spark::function::string::format_string::FormatElement::Format` · datafusion-spark 55.1.0

```rust
Format
```

Source: `src/function/string/format_string.rs:389`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

A format specifier

<a id="op-858785cf1e4598c49b5946bd"></a>
## Verbatim

`variant` · `datafusion_spark::function::string::format_string::FormatElement::Verbatim` · datafusion-spark 55.1.0

```rust
Verbatim
```

Source: `src/function/string/format_string.rs:387`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Some characters that are copied to the output as-is

<a id="op-dca427322c87e95aebaa7b09"></a>
## fmt

`function` · `datafusion_spark::function::string::format_string::FormatElement::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_spark::function::string::format_string::FormatElement", "path": "FormatElement"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [384, 10], "end": [384, 15], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/format_string.rs:384`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
