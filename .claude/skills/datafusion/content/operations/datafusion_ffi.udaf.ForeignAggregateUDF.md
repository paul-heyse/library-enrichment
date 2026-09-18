# `datafusion_ffi::udaf::ForeignAggregateUDF`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_ffi.udaf.ForeignAggregateUDF.json).

<a id="op-b1fbbd6be3a777c6e6a91f42"></a>
## ForeignAggregateUDF

`struct` · `datafusion_ffi::udaf::ForeignAggregateUDF` · datafusion-ffi 55.1.0

```rust
struct ForeignAggregateUDF
```

Source: `src/udaf/mod.rs:432`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

This struct is used to access an UDF provided by a foreign
library across a FFI boundary.

The ForeignAggregateUDF is to be used by the caller of the UDF, so it has
no knowledge or access to the private data. All interaction with the UDF
must occur through the functions defined in FFI_AggregateUDF.

<a id="op-512a9ac90dab99a783be3262"></a>
## accumulator

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::accumulator` · datafusion-ffi 55.1.0

```rust
fn accumulator(&self, acc_args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:502`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9a44d1618df271577bad87b"></a>
## aliases

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::aliases` · datafusion-ffi 55.1.0

```rust
fn aliases(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:573`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-00c1a1a3a43a406d077dcacb"></a>
## coerce_types

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::coerce_types` · datafusion-ffi 55.1.0

```rust
fn coerce_types(&self, arg_types: &[DataType]) -> Result<Vec<DataType>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:617`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a811354c058cdc66438b995"></a>
## create_groups_accumulator

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::create_groups_accumulator` · datafusion-ffi 55.1.0

```rust
fn create_groups_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn GroupsAccumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:561`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7371a60249517f70e8d5bc22"></a>
## create_sliding_accumulator

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::create_sliding_accumulator` · datafusion-ffi 55.1.0

```rust
fn create_sliding_accumulator(&self, args: AccumulatorArgs<'_>) -> Result<Box<dyn Accumulator>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:577`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-64f4f995a75a3fc7a6ceeffb"></a>
## eq

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::eq` · datafusion-ffi 55.1.0

```rust
fn eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [441, 1], "end": [446, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/udaf/mod.rs:442`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-81c1df92d9f317d57809be6a"></a>
## fmt

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::fmt` · datafusion-ffi 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [431, 10], "end": [431, 15], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/udaf/mod.rs:431`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-217a64d5b08ac784c1a124c4"></a>
## groups_accumulator_supported

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::groups_accumulator_supported` · datafusion-ffi 55.1.0

```rust
fn groups_accumulator_supported(&self, args: AccumulatorArgs<'_>) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:549`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7c910579555cb2772fc30ad9"></a>
## hash

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::hash` · datafusion-ffi 55.1.0

```rust
fn hash<H: Hasher>(&self, state: &mut H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [448, 1], "end": [452, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/udaf/mod.rs:449`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0f7a6dbc3a2adfc5c5169158"></a>
## is_nullable

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::is_nullable` · datafusion-ffi 55.1.0

```rust
fn is_nullable(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:498`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-10cc6d2cecfd523ebc9af7e4"></a>
## name

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::name` · datafusion-ffi 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:472`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4bae2233f7e6fb301557f5d9"></a>
## order_sensitivity

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::order_sensitivity` · datafusion-ffi 55.1.0

```rust
fn order_sensitivity(&self) -> AggregateOrderSensitivity
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:605`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-83d5e828bbf4029a137fba2e"></a>
## return_field

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::return_field` · datafusion-ffi 55.1.0

```rust
fn return_field(&self, arg_fields: &[FieldRef]) -> Result<FieldRef>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:484`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0187d677b65ad97d842c9a86"></a>
## return_type

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::return_type` · datafusion-ffi 55.1.0

```rust
fn return_type(&self, _arg_types: &[DataType]) -> Result<DataType>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:480`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f3e85b5fde427df7799babcd"></a>
## signature

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::signature` · datafusion-ffi 55.1.0

```rust
fn signature(&self) -> &Signature
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:476`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d92358cd0c2acbbc845a4db5"></a>
## simplify

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::simplify` · datafusion-ffi 55.1.0

```rust
fn simplify(&self) -> Option<AggregateFunctionSimplification>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:613`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0974bc06322f26332e6d1637"></a>
## state_fields

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::state_fields` · datafusion-ffi 55.1.0

```rust
fn state_fields(&self, args: StateFieldsArgs<'_>) -> Result<Vec<FieldRef>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:510`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a184b3616b6f1d155234ceab"></a>
## supports_null_handling_clause

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::supports_null_handling_clause` · datafusion-ffi 55.1.0

```rust
fn supports_null_handling_clause(&self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:609`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-73f8176a8b467d1463bf5aaa"></a>
## with_beneficial_ordering

`function` · `datafusion_ffi::udaf::ForeignAggregateUDF::with_beneficial_ordering` · datafusion-ffi 55.1.0

```rust
fn with_beneficial_ordering(Arc<self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_ffi::udaf::ForeignAggregateUDF", "path": "ForeignAggregateUDF"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [471, 1], "end": [625, 2], "filename": "src/udaf/mod.rs"}, "trait": {"args": null, "id": "datafusion_expr::udaf::AggregateUDFImpl", "path": "AggregateUDFImpl"}, "trait_path": "datafusion_expr::udaf::AggregateUDFImpl"}`

Source: `src/udaf/mod.rs:588`. [Exact documentation build](https://docs.rs/crate/datafusion-ffi/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
