# `datafusion_expr::logical_plan::dml::WriteOp`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.dml.WriteOp.json).

<a id="op-18e21f9cfeca26c2ee4449fa"></a>
## WriteOp

`enum` · `datafusion_expr::logical_plan::dml::WriteOp` · datafusion-expr 55.1.0

```rust
enum WriteOp
```

Source: `src/logical_plan/dml.rs:235`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

The type of DML operation to perform.

See [`DmlStatement`](../operations/datafusion_expr.logical_plan.dml.DmlStatement.md#op-2ac6db89e384424dd62ef114) for more details.

Marked `#[non_exhaustive]` so adding new variants in future releases is
not a SemVer break for downstream matchers.

<a id="op-dd87d202ac66f47eaff3cfb3"></a>
## Ctas

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::Ctas` · datafusion-expr 55.1.0

```rust
Ctas
```

Source: `src/logical_plan/dml.rs:243`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`CREATE TABLE AS SELECT` operation

<a id="op-16134f60fabb1a5031bb9ebf"></a>
## Delete

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::Delete` · datafusion-expr 55.1.0

```rust
Delete
```

Source: `src/logical_plan/dml.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`DELETE` operation

<a id="op-9b96b949244deada741ae137"></a>
## Insert

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::Insert` · datafusion-expr 55.1.0

```rust
Insert
```

Source: `src/logical_plan/dml.rs:237`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`INSERT INTO` operation

<a id="op-72d7f3e0aaea66ff74df7fe0"></a>
## MergeInto

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::MergeInto` · datafusion-expr 55.1.0

```rust
MergeInto
```

Source: `src/logical_plan/dml.rs:247`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`MERGE INTO` operation

<a id="op-22a558ecb37c2be48f4974ff"></a>
## Truncate

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::Truncate` · datafusion-expr 55.1.0

```rust
Truncate
```

Source: `src/logical_plan/dml.rs:245`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`TRUNCATE` operation

<a id="op-d39fcbbd1c364343aed12fdd"></a>
## Update

`variant` · `datafusion_expr::logical_plan::dml::WriteOp::Update` · datafusion-expr 55.1.0

```rust
Update
```

Source: `src/logical_plan/dml.rs:241`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

`UPDATE` operation

<a id="op-a9c8542fb5fcaa054a440fb7"></a>
## clone

`function` · `datafusion_expr::logical_plan::dml::WriteOp::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> WriteOp
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 17], "end": [233, 22], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/dml.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4cc7ef019f597cf4203de83b"></a>
## eq

`function` · `datafusion_expr::logical_plan::dml::WriteOp::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &WriteOp) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 24], "end": [233, 33], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/dml.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-856dd6c104afba447dfd8724"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::WriteOp::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [264, 1], "end": [268, 2], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/logical_plan/dml.rs:265`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f19a1d68dd43cb7a0b3aae92"></a>
## fmt

`function` · `datafusion_expr::logical_plan::dml::WriteOp::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 10], "end": [233, 15], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/dml.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-445b8164e316ce743aaf7e9b"></a>
## hash

`function` · `datafusion_expr::logical_plan::dml::WriteOp::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 51], "end": [233, 55], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/dml.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-1eeea5c69472e85b6ee66ef1"></a>
## name

`function` · `datafusion_expr::logical_plan::dml::WriteOp::name` · datafusion-expr 55.1.0

```rust
fn name(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [250, 1], "end": [262, 2], "filename": "src/logical_plan/dml.rs"}, "trait": null, "trait_path": null}`

Source: `src/logical_plan/dml.rs:252`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

Return a descriptive name of this [`WriteOp`](../operations/datafusion_expr.logical_plan.dml.WriteOp.md#op-18e21f9cfeca26c2ee4449fa)

<a id="op-58038cd534942bf0b0810738"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::dml::WriteOp::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &WriteOp) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::dml::WriteOp", "path": "WriteOp"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [233, 39], "end": [233, 49], "filename": "src/logical_plan/dml.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/dml.rs:233`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
