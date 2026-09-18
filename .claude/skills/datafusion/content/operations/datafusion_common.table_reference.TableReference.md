# `datafusion_common::table_reference::TableReference`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_common.table_reference.TableReference.json).

<a id="op-dafce6f1cf123e142b4fcab0"></a>
## TableReference

`enum` · `datafusion_common::table_reference::TableReference` · datafusion-common 55.1.0

```rust
enum TableReference
```

Source: `src/table_reference.rs:78`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A multi part identifier (path) to a table that may require further
resolution (e.g. `foo.bar`).

[`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0)s are cheap to `clone()` as they are implemented with
`Arc`.

See [`ResolvedTableReference`](../operations/datafusion_common.table_reference.ResolvedTableReference.md#op-8e17918e7e89b471267a2243) for a fully resolved table reference.

# Creating [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0)

When converting strings to [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0)s, the string is parsed as
though it were a SQL identifier, normalizing (convert to lowercase) any
unquoted identifiers.  [`TableReference::bare`](../operations/datafusion_common.table_reference.TableReference.md#op-5b493fc994310c5087d44220) creates references without
applying normalization semantics.

# Examples
```
# use datafusion_common::TableReference;
// Get a table reference to 'mytable'
let table_reference = TableReference::from("mytable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'mytable' (note the capitalization)
let table_reference = TableReference::from("MyTable");
assert_eq!(table_reference, TableReference::bare("mytable"));

// Get a table reference to 'MyTable' (note the capitalization) using double quotes
// (programmatically it is better to use `TableReference::bare` for this)
let table_reference = TableReference::from(r#""MyTable""#);
assert_eq!(table_reference, TableReference::bare("MyTable"));

// Get a table reference to 'myschema.mytable' (note the capitalization)
let table_reference = TableReference::from("MySchema.MyTable");
assert_eq!(
    table_reference,
    TableReference::partial("myschema", "mytable")
);
```

<a id="op-29dea43e4c5ca5fe5b2952a7"></a>
## Bare

`variant` · `datafusion_common::table_reference::TableReference::Bare` · datafusion-common 55.1.0

```rust
Bare
```

Source: `src/table_reference.rs:80`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

An unqualified table reference, e.g. "table"

<a id="op-c640dd34bc387be1b071f7aa"></a>
## Full

`variant` · `datafusion_common::table_reference::TableReference::Full` · datafusion-common 55.1.0

```rust
Full
```

Source: `src/table_reference.rs:92`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A fully resolved table reference, e.g. "catalog.schema.table"

<a id="op-444721033041bed44f65d905"></a>
## Partial

`variant` · `datafusion_common::table_reference::TableReference::Partial` · datafusion-common 55.1.0

```rust
Partial
```

Source: `src/table_reference.rs:85`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

A partially resolved table reference, e.g. "schema.table"

<a id="op-5b493fc994310c5087d44220"></a>
## bare

`function` · `datafusion_common::table_reference::TableReference::bare` · datafusion-common 55.1.0

```rust
fn bare(table: impl Into<Arc<str>>) -> TableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:129`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience method for creating a [`TableReference::Bare`](../operations/datafusion_common.table_reference.TableReference.md#op-29dea43e4c5ca5fe5b2952a7)

As described on [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) this does *NO* normalization at
all, so "Foo.Bar" stays as a reference to the table named
"Foo.Bar" (rather than "foo"."bar")

<a id="op-a0fb61146e9e79c2a584f770"></a>
## catalog

`function` · `datafusion_common::table_reference::TableReference::catalog` · datafusion-common 55.1.0

```rust
fn catalog(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:183`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieve the catalog name if  [`Self::Full`](../operations/datafusion_common.table_reference.TableReference.md#op-c640dd34bc387be1b071f7aa), `None` otherwise.

<a id="op-2f095171b27b376ddd723c15"></a>
## clone

`function` · `datafusion_common::table_reference::TableReference::clone` · datafusion-common 55.1.0

```rust
fn clone(&self) -> TableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 17], "end": [77, 22], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-414cc460a6c29f9ecde15757"></a>
## cmp

`function` · `datafusion_common::table_reference::TableReference::cmp` · datafusion-common 55.1.0

```rust
fn cmp(&self, other: &TableReference) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 57], "end": [77, 60], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5a26b137f5ecb1b5dc4c4bfc"></a>
## eq

`function` · `datafusion_common::table_reference::TableReference::eq` · datafusion-common 55.1.0

```rust
fn eq(&self, other: &TableReference) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 24], "end": [77, 33], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a7aba49796dd491b78bfc041"></a>
## fmt

`function` · `datafusion_common::table_reference::TableReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 10], "end": [77, 15], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d783a1f41145c247de9b5c5f"></a>
## fmt

`function` · `datafusion_common::table_reference::TableReference::fmt` · datafusion-common 55.1.0

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [102, 1], "end": [116, 2], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/table_reference.rs:103`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-0e20dc4c7fa9250c95724081"></a>
## from

`function` · `datafusion_common::table_reference::TableReference::from` · datafusion-common 55.1.0

```rust
fn from(s: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [351, 1], "end": [355, 2], "filename": "src/table_reference.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_reference.rs:352`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-529d5c77cdfd35b219187993"></a>
## from

`function` · `datafusion_common::table_reference::TableReference::from` · datafusion-common 55.1.0

```rust
fn from(s: &'a str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [339, 1], "end": [343, 2], "filename": "src/table_reference.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_reference.rs:340`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6cb6184b80ef9a20e24d4ed2"></a>
## from

`function` · `datafusion_common::table_reference::TableReference::from` · datafusion-common 55.1.0

```rust
fn from(resolved: ResolvedTableReference) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [357, 1], "end": [365, 2], "filename": "src/table_reference.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::ResolvedTableReference", "path": "ResolvedTableReference"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_reference.rs:358`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-757ccddcdc5dde0792001727"></a>
## from

`function` · `datafusion_common::table_reference::TableReference::from` · datafusion-common 55.1.0

```rust
fn from(s: &'a String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [345, 1], "end": [349, 2], "filename": "src/table_reference.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": "'a", "type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/table_reference.rs:346`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca4710b49ab0973627a5597e"></a>
## full

`function` · `datafusion_common::table_reference::TableReference::full` · datafusion-common 55.1.0

```rust
fn full(catalog: impl Into<Arc<str>>, schema: impl Into<Arc<str>>, table: impl Into<Arc<str>>) -> TableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:152`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience method for creating a [`TableReference::Full`](../operations/datafusion_common.table_reference.TableReference.md#op-c640dd34bc387be1b071f7aa)

Note: *NO* normalization is applied to the catalog, schema or table
name.

<a id="op-f893a408889fdc421da4f11b"></a>
## hash

`function` · `datafusion_common::table_reference::TableReference::hash` · datafusion-common 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 39], "end": [77, 43], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ba327ea8336f63be974fd4ac"></a>
## heap_size

`function` · `datafusion_common::table_reference::TableReference::heap_size` · datafusion-common 55.1.0

```rust
fn heap_size(&self, ctx: &mut DFHeapSizeCtx) -> usize
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "crate::TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [93, 1], "end": [107, 2], "filename": "src/heap_size.rs"}, "trait": {"args": null, "id": "datafusion_common::heap_size::DFHeapSize", "path": "DFHeapSize"}, "trait_path": "datafusion_common::heap_size::DFHeapSize"}`

Source: `src/heap_size.rs:94`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-df9c299807afcd200b6556c8"></a>
## none

`function` · `datafusion_common::table_reference::TableReference::none` · datafusion-common 55.1.0

```rust
fn none() -> Option<TableReference>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:120`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience method for creating a typed none `None`

<a id="op-8f98a3a3b1a7e4bac0732593"></a>
## parse_str

`function` · `datafusion_common::table_reference::TableReference::parse_str` · datafusion-common 55.1.0

```rust
fn parse_str(s: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:280`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Forms a [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) by parsing `s` as a multipart SQL
identifier, normalizing `s` to lowercase.
See docs on [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) for more details.

<a id="op-7c527f644795b8a2f38a2b71"></a>
## parse_str_normalized

`function` · `datafusion_common::table_reference::TableReference::parse_str_normalized` · datafusion-common 55.1.0

```rust
fn parse_str_normalized(s: &str, ignore_case: bool) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:287`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Forms a [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) by parsing `s` as a multipart SQL
identifier, normalizing `s` to lowercase if `ignore_case` is `false`.
See docs on [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) for more details.

<a id="op-52cf1d1bbeb29d33393166ad"></a>
## partial

`function` · `datafusion_common::table_reference::TableReference::partial` · datafusion-common 55.1.0

```rust
fn partial(schema: impl Into<Arc<str>>, table: impl Into<Arc<str>>) -> TableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:138`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Convenience method for creating a [`TableReference::Partial`](../operations/datafusion_common.table_reference.TableReference.md#op-444721033041bed44f65d905).

Note: *NO* normalization is applied to the schema or table name.

<a id="op-c8e43e12171bb743f452f6f8"></a>
## partial_cmp

`function` · `datafusion_common::table_reference::TableReference::partial_cmp` · datafusion-common 55.1.0

```rust
fn partial_cmp(&self, other: &TableReference) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [77, 45], "end": [77, 55], "filename": "src/table_reference.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/table_reference.rs:77`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e9a52606f55c19fdb69792c8"></a>
## resolve

`function` · `datafusion_common::table_reference::TableReference::resolve` · datafusion-common 55.1.0

```rust
fn resolve(self, default_catalog: &str, default_schema: &str) -> ResolvedTableReference
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Given a default catalog and schema, ensure this table reference is fully
resolved

<a id="op-c230b753228f2395d5f57373"></a>
## resolved_eq

`function` · `datafusion_common::table_reference::TableReference::resolved_eq` · datafusion-common 55.1.0

```rust
fn resolved_eq(&self, other: &Self) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:196`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Compare with another [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) as if both are resolved.
This allows comparing across variants. If a field is not present
in both variants being compared then it is ignored in the comparison.

e.g. this allows a [`TableReference::Bare`](../operations/datafusion_common.table_reference.TableReference.md#op-29dea43e4c5ca5fe5b2952a7) to be considered equal to a
fully qualified [`TableReference::Full`](../operations/datafusion_common.table_reference.TableReference.md#op-c640dd34bc387be1b071f7aa) if the table names match.

<a id="op-4b81f5b3cdcc0b0d167fc852"></a>
## schema

`function` · `datafusion_common::table_reference::TableReference::schema` · datafusion-common 55.1.0

```rust
fn schema(&self) -> Option<&str>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:175`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieve the schema name if [`Self::Partial]` or [`Self::`Full`],
`None` otherwise.

<a id="op-52da8f327b3e3325d8f6fcac"></a>
## table

`function` · `datafusion_common::table_reference::TableReference::table` · datafusion-common 55.1.0

```rust
fn table(&self) -> &str
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:165`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Retrieve the table name, regardless of qualification.

<a id="op-55cf8751b0aa2a59ef5cf457"></a>
## to_quoted_string

`function` · `datafusion_common::table_reference::TableReference::to_quoted_string` · datafusion-common 55.1.0

```rust
fn to_quoted_string(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:258`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Forms a string where the identifiers are quoted

# Example
```
# use datafusion_common::TableReference;
let table_reference = TableReference::partial("myschema", "mytable");
assert_eq!(table_reference.to_quoted_string(), "myschema.mytable");

let table_reference = TableReference::partial("MySchema", "MyTable");
assert_eq!(
    table_reference.to_quoted_string(),
    r#""MySchema"."MyTable""#
);
```

<a id="op-090dd7eee216c52290bc9333"></a>
## to_vec

`function` · `datafusion_common::table_reference::TableReference::to_vec` · datafusion-common 55.1.0

```rust
fn to_vec(&self) -> Vec<String>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_common::table_reference::TableReference", "path": "TableReference"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [118, 1], "end": [334, 2], "filename": "src/table_reference.rs"}, "trait": null, "trait_path": null}`

Source: `src/table_reference.rs:321`. [Exact documentation build](https://docs.rs/crate/datafusion-common/55.1.0/json).

Decompose a [`TableReference`](../operations/datafusion_common.table_reference.TableReference.md#op-dafce6f1cf123e142b4fcab0) to separate parts. The result vector contains
at most three elements in the following sequence:
```no_rust
[<catalog>, <schema>, table]
```
