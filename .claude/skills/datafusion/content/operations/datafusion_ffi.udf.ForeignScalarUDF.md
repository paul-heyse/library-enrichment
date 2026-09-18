# `datafusion_ffi::udf::ForeignScalarUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udf.ForeignScalarUDF.json).

<a id="op-673b7043db9b397c0e396af5"></a>
## ForeignScalarUDF

`struct` · `datafusion_ffi::udf::ForeignScalarUDF` · datafusion-ffi 55.1.0

```rust
struct ForeignScalarUDF
```

Source: `src/udf/mod.rs:341`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignScalarUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_ScalarUDF.

<a id="op-c04a8677749ecc2907b7288a"></a>
## aliases

`function` · `datafusion_ffi::udf::ForeignScalarUDF::aliases` · datafusion-ffi 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:497`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00c8ccd48b75041f2183539a"></a>
## coerce_types

`function` · `datafusion_ffi::udf::ForeignScalarUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:505`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-efe7b3db4c2e309e38fac3e0"></a>
## eq

`function` · `datafusion_ffi::udf::ForeignScalarUDF::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [366, 1], "end": [379, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udf/mod.rs:367`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-04c8f4bf1f4b1c3b2d73653a"></a>
## fmt

`function` · `datafusion_ffi::udf::ForeignScalarUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [340, 10], "end": [340, 15], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udf/mod.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-72a57ae49ccfd26bcefbad6f"></a>
## hash

`function` · `datafusion_ffi::udf::ForeignScalarUDF::hash` · datafusion-ffi 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [382, 1], "end": [395, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udf/mod.rs:383`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-205ea349a99c52afb1138ac2"></a>
## invoke_with_args

`function` · `datafusion_ffi::udf::ForeignScalarUDF::invoke_with_args` · datafusion-ffi 55.1.0

```rust
fn invoke_with_args(&self, invoke_args: ScalarFunctionArgs) -> Result<ColumnarValue>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:444`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4ffa489a24ac6e41334a95e6"></a>
## name

`function` · `datafusion_ffi::udf::ForeignScalarUDF::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:418`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-479fad2ef0e43f05908ae08c"></a>
## placement

`function` · `datafusion_ffi::udf::ForeignScalarUDF::placement` · datafusion-ffi 55.1.0

```rust
fn placement(&self, args: &[ExpressionPlacement]) -> ExpressionPlacement
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:513`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e54147944d21831727107f1e"></a>
## preserves_lex_ordering

`function` · `datafusion_ffi::udf::ForeignScalarUDF::preserves_lex_ordering` · datafusion-ffi 55.1.0

```rust
fn preserves_lex_ordering(&self, inputs: &[ExprProperties]) -> Result<bool>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:524`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-534fe964d381bff1e91e148b"></a>
## return_field_from_args

`function` · `datafusion_ffi::udf::ForeignScalarUDF::return_field_from_args` · datafusion-ffi 55.1.0

```rust
fn return_field_from_args(&self, args: ReturnFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:430`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-86b175adb09260daae938ee1"></a>
## return_type

`function` · `datafusion_ffi::udf::ForeignScalarUDF::return_type` · datafusion-ffi 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:426`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-eb0657c324f294feada1f657"></a>
## short_circuits

`function` · `datafusion_ffi::udf::ForeignScalarUDF::short_circuits` · datafusion-ffi 55.1.0

```rust
fn short_circuits(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:501`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a1c542a5486287f297680133"></a>
## signature

`function` · `datafusion_ffi::udf::ForeignScalarUDF::signature` · datafusion-ffi 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:422`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2f33ec856b879fd8c09ec270"></a>
## with_updated_config

`function` · `datafusion_ffi::udf::ForeignScalarUDF::with_updated_config` · datafusion-ffi 55.1.0

```rust
fn with_updated_config(&self, config: &ConfigOptions) -> Option<ScalarUDF>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udf::ForeignScalarUDF", "path": "ForeignScalarUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [417, 1], "end": [551, 2], "filename": "src/udf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udf::ScalarUDFImpl", "path": "ScalarUDFImpl"}, "trait_path": "datafusion_expr::udf::ScalarUDFImpl"}`

Source: `src/udf/mod.rs:536`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
