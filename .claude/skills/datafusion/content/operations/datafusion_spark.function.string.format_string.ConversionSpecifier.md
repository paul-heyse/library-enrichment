# `datafusion_spark::function::string::format_string::ConversionSpecifier`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.format_string.ConversionSpecifier.json).

<a id="op-8c3e74258f040669b91e3f16"></a>
## ConversionSpecifier

`struct` · `datafusion_spark::function::string::format_string::ConversionSpecifier` · datafusion-spark 55.1.0

```rust
struct ConversionSpecifier
```

Source: `src/function/string/format_string.rs:394`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Parsed printf conversion specifier

<a id="op-8b14c41bc07adc27f96882d1"></a>
## alt_form

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::alt_form` · datafusion-spark 55.1.0

```rust
alt_form: bool
```

Source: `src/function/string/format_string.rs:397`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `#`: use `0x`, etc?

<a id="op-88c4f27a266d059f439e0bd1"></a>
## argument_index

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::argument_index` · datafusion-spark 55.1.0

```rust
argument_index: usize
```

Source: `src/function/string/format_string.rs:395`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b11a220101ab76dae6ac2122"></a>
## clone

`function` · `datafusion_spark::function::string::format_string::ConversionSpecifier::clone` · datafusion-spark 55.1.0

```rust
fn clone(&self) -> ConversionSpecifier
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::ConversionSpecifier", "path": "ConversionSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 17], "end": [393, 22], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/function/string/format_string.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b98057d616a9f526f79874fc"></a>
## conversion_type

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::conversion_type` · datafusion-spark 55.1.0

```rust
conversion_type: ConversionType
```

Source: `src/function/string/format_string.rs:415`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

data type

<a id="op-090cc2591824084b4bdd6e8c"></a>
## eq

`function` · `datafusion_spark::function::string::format_string::ConversionSpecifier::eq` · datafusion-spark 55.1.0

```rust
fn eq(&self, other: &ConversionSpecifier) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::ConversionSpecifier", "path": "ConversionSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 30], "end": [393, 39], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/function/string/format_string.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b0311ebec7dc28a52110518"></a>
## fmt

`function` · `datafusion_spark::function::string::format_string::ConversionSpecifier::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::ConversionSpecifier", "path": "ConversionSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [393, 10], "end": [393, 15], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/format_string.rs:393`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cb45a4e35adb168e09bbe64e"></a>
## force_sign

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::force_sign` · datafusion-spark 55.1.0

```rust
force_sign: bool
```

Source: `src/function/string/format_string.rs:405`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `+`: Always show sign? (for signed numbers)

<a id="op-1858b1a00722c511c8c827bd"></a>
## format

`function` · `datafusion_spark::function::string::format_string::ConversionSpecifier::format` · datafusion-spark 55.1.0

```rust
fn format(&self, string: &mut String, value: &ScalarValue) -> Result<()>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_spark::function::string::format_string::ConversionSpecifier", "path": "ConversionSpecifier"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [973, 1], "end": [2255, 2], "filename": "src/function/string/format_string.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/format_string.rs:996`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4d703a666db727fb5d21cbe6"></a>
## grouping_separator

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::grouping_separator` · datafusion-spark 55.1.0

```rust
grouping_separator: bool
```

Source: `src/function/string/format_string.rs:407`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `,`: include locale-specific grouping separators

<a id="op-b3e7a60399b59579e176e9b2"></a>
## left_adj

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::left_adj` · datafusion-spark 55.1.0

```rust
left_adj: bool
```

Source: `src/function/string/format_string.rs:401`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `-`: left-adjust (pad with spaces on the right)

<a id="op-1779bbbd1fbd87ae226fb239"></a>
## negative_in_parentheses

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::negative_in_parentheses` · datafusion-spark 55.1.0

```rust
negative_in_parentheses: bool
```

Source: `src/function/string/format_string.rs:409`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `(`: enclose negative numbers in parentheses

<a id="op-159a677c2e85fa41ed0fb236"></a>
## precision

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::precision` · datafusion-spark 55.1.0

```rust
precision: NumericParam
```

Source: `src/function/string/format_string.rs:413`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

floating point field precision

<a id="op-6a09a23238b060ad1aacbce1"></a>
## space_sign

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::space_sign` · datafusion-spark 55.1.0

```rust
space_sign: bool
```

Source: `src/function/string/format_string.rs:403`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `' '` (space): indicate sign with a space?

<a id="op-7aeedf647295eea7e15d9cd2"></a>
## width

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::width` · datafusion-spark 55.1.0

```rust
width: NumericParam
```

Source: `src/function/string/format_string.rs:411`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

field width

<a id="op-74abcd03a062d4ffdc2b11f8"></a>
## zero_pad

`struct_field` · `datafusion_spark::function::string::format_string::ConversionSpecifier::zero_pad` · datafusion-spark 55.1.0

```rust
zero_pad: bool
```

Source: `src/function/string/format_string.rs:399`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

flag `0`: left-pad with zeros?
