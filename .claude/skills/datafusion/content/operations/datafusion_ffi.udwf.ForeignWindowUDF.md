# `datafusion_ffi::udwf::ForeignWindowUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udwf.ForeignWindowUDF.json).

<a id="op-cfc1c6fd5bd041227c3a9172"></a>
## ForeignWindowUDF

`struct` · `datafusion_ffi::udwf::ForeignWindowUDF` · datafusion-ffi 55.1.0

```rust
struct ForeignWindowUDF
```

Source: `src/udwf/mod.rs:266`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignWindowUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_WindowUDF.

<a id="op-5f84f76bf2643efd8b12d11e"></a>
## aliases

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::aliases` · datafusion-ffi 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:318`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc0a4d1a4dc93ad14e15c728"></a>
## coerce_types

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:322`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-87b605f5bb0beb50b5b231e0"></a>
## eq

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [276, 1], "end": [281, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udwf/mod.rs:277`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3aed5d293cc4cc4b6dff666b"></a>
## field

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::field` · datafusion-ffi 55.1.0

```rust
fn field(&self, field_args: WindowUDFFieldArgs<'_>) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:342`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-fc370ca9f770fa80486b3ce9"></a>
## fmt

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [265, 10], "end": [265, 15], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udwf/mod.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d20423a16a637ff83312da7a"></a>
## hash

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::hash` · datafusion-ffi 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [283, 1], "end": [287, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udwf/mod.rs:284`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-68f2c6707b2d2de922c9b965"></a>
## limit_effect

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::limit_effect` · datafusion-ffi 55.1.0

```rust
fn limit_effect(&self, _args: &[Arc<dyn PhysicalExpr>]) -> LimitEffect
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:366`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7b8ca9b19d8644aef79f44de"></a>
## name

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:310`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9cd4516e16f80bb3cca27011"></a>
## partition_evaluator

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::partition_evaluator` · datafusion-ffi 55.1.0

```rust
fn partition_evaluator(&self, args: datafusion_expr::function::PartitionEvaluatorArgs<'_>) -> Result<Box<dyn PartitionEvaluator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:330`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cd236d173c0b3a943fa30d3d"></a>
## signature

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::signature` · datafusion-ffi 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:314`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e3513bd5faefbb1b88054a00"></a>
## sort_options

`function` · `datafusion_ffi::udwf::ForeignWindowUDF::sort_options` · datafusion-ffi 55.1.0

```rust
fn sort_options(&self) -> Option<SortOptions>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udwf::ForeignWindowUDF", "path": "ForeignWindowUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [309, 1], "end": [369, 2], "filename": "src/udwf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udwf::WindowUDFImpl", "path": "WindowUDFImpl"}, "trait_path": "datafusion_expr::udwf::WindowUDFImpl"}`

Source: `src/udwf/mod.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
