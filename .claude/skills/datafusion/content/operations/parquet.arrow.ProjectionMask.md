# `parquet::arrow::ProjectionMask`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.arrow.ProjectionMask.json).

<a id="op-28a259558ea082dc20ece1ec"></a>
## ProjectionMask

`struct` · `parquet::arrow::ProjectionMask` · parquet 59.3.0

```rust
struct ProjectionMask
```

Source: `src/arrow/mod.rs:255`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

A [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) identifies a set of columns within a potentially nested schema to project

In particular, a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) can be constructed from a list of leaf column indices
or root column indices where:

* Root columns are the direct children of the root schema, enumerated in order
* Leaf columns are the child-less leaves of the schema as enumerated by a depth-first search

For example, the schema

```ignore
message schema {
  REQUIRED boolean         leaf_1;
  REQUIRED GROUP group {
    OPTIONAL int32 leaf_2;
    OPTIONAL int64 leaf_3;
  }
}
```

Has roots `["leaf_1", "group"]` and leaves `["leaf_1", "leaf_2", "leaf_3"]`

For non-nested schemas, i.e. those containing only primitive columns, the root
and leaves are the same


<a id="op-7806731ea3be24118305c4d2"></a>
## all

`function` · `parquet::arrow::ProjectionMask::all` · parquet 59.3.0

```rust
fn all() -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:280`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) which selects all columns

<a id="op-bf1b6ccc5af83ee76041e7c5"></a>
## clone

`function` · `parquet::arrow::ProjectionMask::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ProjectionMask
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 17], "end": [254, 22], "filename": "src/arrow/mod.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/arrow/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8cef2a585fc120fc4fa958ce"></a>
## columns

`function` · `parquet::arrow::ProjectionMask::columns` · parquet 59.3.0

```rust
fn columns<'a>(schema: &SchemaDescriptor, names: impl IntoIterator<Item = &'a str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:356`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) which selects only the named columns

All leaf columns that fall below a given name will be selected. For example, given
the schema
```ignore
message schema {
  OPTIONAL group a (MAP) {
    REPEATED group key_value {
      REQUIRED BYTE_ARRAY key (UTF8);  // leaf index 0
      OPTIONAL group value (MAP) {
        REPEATED group key_value {
          REQUIRED INT32 key;          // leaf index 1
          REQUIRED BOOLEAN value;      // leaf index 2
        }
      }
    }
  }
  REQUIRED INT32 b;                    // leaf index 3
  REQUIRED DOUBLE c;                   // leaf index 4
}
```
`["a.key_value.value", "c"]` would return leaf columns 1, 2, and 4. `["a"]` would return
columns 0, 1, and 2.

Note: repeated or out of order indices will not impact the final mask.

i.e. `["b", "c"]` will construct the same mask as `["c", "b", "c"]`.

Also, this will not produce the desired results if a column contains a '.' in its name.
Use [`Self::leaves`](../operations/parquet.arrow.ProjectionMask.md#op-8d5b244b6085c18a89f848d0) or [`Self::roots`](../operations/parquet.arrow.ProjectionMask.md#op-538ef2a8481545dde6ab67ea) in that case.

<a id="op-95289108cb225f7014535136"></a>
## eq

`function` · `parquet::arrow::ProjectionMask::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ProjectionMask) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 24], "end": [254, 33], "filename": "src/arrow/mod.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/arrow/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca830a1bd85771693c1faca3"></a>
## fmt

`function` · `parquet::arrow::ProjectionMask::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [254, 10], "end": [254, 15], "filename": "src/arrow/mod.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/arrow/mod.rs:254`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6b5e75b24756acad6a4c71d0"></a>
## intersect

`function` · `parquet::arrow::ProjectionMask::intersect` · parquet 59.3.0

```rust
fn intersect(&mut self, other: &Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:411`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Intersect two projection masks

Example:
```text
mask1 = [true, false, true]
mask2 = [false, true, true]
intersect(mask1, mask2) = [false, false, true]
```

<a id="op-f2bf329f9092cc9366b51a79"></a>
## leaf_included

`function` · `parquet::arrow::ProjectionMask::leaf_included` · parquet 59.3.0

```rust
fn leaf_included(&self, leaf_idx: usize) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:380`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns true if the leaf column `leaf_idx` is included by the mask

<a id="op-8d5b244b6085c18a89f848d0"></a>
## leaves

`function` · `parquet::arrow::ProjectionMask::leaves` · parquet 59.3.0

```rust
fn leaves(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:296`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) which selects only the specified leaf columns

Note: repeated or out of order indices will not impact the final mask

i.e. `[0, 1, 2]` will construct the same mask as `[1, 0, 0, 2]`

<a id="op-9c7e390f1659dc6877494bf7"></a>
## none

`function` · `parquet::arrow::ProjectionMask::none` · parquet 59.3.0

```rust
fn none(len: usize) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:285`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) which selects no columns

<a id="op-538ef2a8481545dde6ab67ea"></a>
## roots

`function` · `parquet::arrow::ProjectionMask::roots` · parquet 59.3.0

```rust
fn roots(schema: &SchemaDescriptor, indices: impl IntoIterator<Item = usize>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:309`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Create a [`ProjectionMask`](../operations/parquet.arrow.ProjectionMask.md#op-28a259558ea082dc20ece1ec) which selects only the specified root columns

Note: repeated or out of order indices will not impact the final mask

i.e. `[0, 1, 2]` will construct the same mask as `[1, 0, 0, 2]`

<a id="op-5aa39e7c7a8dd5cb9fdbc22b"></a>
## union

`function` · `parquet::arrow::ProjectionMask::union` · parquet 59.3.0

```rust
fn union(&mut self, other: &Self)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::arrow::ProjectionMask", "path": "ProjectionMask"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [278, 1], "end": [457, 2], "filename": "src/arrow/mod.rs"}, "trait": null, "trait_path": null}`

Source: `src/arrow/mod.rs:392`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Union two projection masks

Example:
```text
mask1 = [true, false, true]
mask2 = [false, true, true]
union(mask1, mask2) = [true, true, true]
```
