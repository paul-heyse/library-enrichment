# `datafusion_expr::logical_plan::ddl::CreateIndex`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_expr.logical_plan.ddl.CreateIndex.json).

<a id="op-cdc7c8dab1ab347036033191"></a>
## CreateIndex

`struct` · `datafusion_expr::logical_plan::ddl::CreateIndex` · datafusion-expr 55.1.0

```rust
struct CreateIndex
```

Source: `src/logical_plan/ddl.rs:777`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-50ac058dcc42a5e55cff76d6"></a>
## clone

`function` · `datafusion_expr::logical_plan::ddl::CreateIndex::clone` · datafusion-expr 55.1.0

```rust
fn clone(&self) -> CreateIndex
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 10], "end": [776, 15], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/logical_plan/ddl.rs:776`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-7e61e6b0dc1a70697a946b9f"></a>
## columns

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::columns` · datafusion-expr 55.1.0

```rust
columns: Vec<SortExpr>
```

Source: `src/logical_plan/ddl.rs:781`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8458657b2b7a455886125c26"></a>
## eq

`function` · `datafusion_expr::logical_plan::ddl::CreateIndex::eq` · datafusion-expr 55.1.0

```rust
fn eq(&self, other: &CreateIndex) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 17], "end": [776, 26], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/logical_plan/ddl.rs:776`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-06d79afd55d3d6d106b001c6"></a>
## fmt

`function` · `datafusion_expr::logical_plan::ddl::CreateIndex::fmt` · datafusion-expr 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 38], "end": [776, 43], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/logical_plan/ddl.rs:776`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-db2063ee2d82343e0ecdbcbd"></a>
## hash

`function` · `datafusion_expr::logical_plan::ddl::CreateIndex::hash` · datafusion-expr 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [776, 32], "end": [776, 36], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/logical_plan/ddl.rs:776`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f63e3ad655ffec4a8d16160"></a>
## if_not_exists

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::if_not_exists` · datafusion-expr 55.1.0

```rust
if_not_exists: bool
```

Source: `src/logical_plan/ddl.rs:783`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a2ae19ba1c05b99d649a162"></a>
## name

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::name` · datafusion-expr 55.1.0

```rust
name: Option<String>
```

Source: `src/logical_plan/ddl.rs:778`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f4efc05c24cdc0a77b00edc4"></a>
## partial_cmp

`function` · `datafusion_expr::logical_plan::ddl::CreateIndex::partial_cmp` · datafusion-expr 55.1.0

```rust
fn partial_cmp(&self, other: &Self) -> Option<Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_expr::logical_plan::ddl::CreateIndex", "path": "CreateIndex"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [788, 1], "end": [820, 2], "filename": "src/logical_plan/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/logical_plan/ddl.rs:789`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-25065d2dd26bb8958672d00d"></a>
## schema

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::schema` · datafusion-expr 55.1.0

```rust
schema: datafusion_common::DFSchemaRef
```

Source: `src/logical_plan/ddl.rs:784`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-bfd0a3220d751913a1f398aa"></a>
## table

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::table` · datafusion-expr 55.1.0

```rust
table: datafusion_common::TableReference
```

Source: `src/logical_plan/ddl.rs:779`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-451a0e23a8751c6e940bbdc8"></a>
## unique

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::unique` · datafusion-expr 55.1.0

```rust
unique: bool
```

Source: `src/logical_plan/ddl.rs:782`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f22f4a40415e80eb95ff0212"></a>
## using

`struct_field` · `datafusion_expr::logical_plan::ddl::CreateIndex::using` · datafusion-expr 55.1.0

```rust
using: Option<String>
```

Source: `src/logical_plan/ddl.rs:780`. [Exact documentation build](https://docs.rs/crate/datafusion-expr/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
