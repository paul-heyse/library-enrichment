# `parquet::schema::types::ColumnPath`

Full upstream contracts; raw type trees and source locators in [structured records](parquet.schema.types.ColumnPath.json).

<a id="op-af9bcd597f4e17b094dee0f4"></a>
## ColumnPath

`struct` · `parquet::schema::types::ColumnPath` · parquet 59.3.0

```rust
struct ColumnPath
```

Source: `src/schema/types.rs:760`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Represents the location of a column in a Parquet schema

# Example: refer to column named `'my_column'`
```
# use parquet::schema::types::ColumnPath;
let column_path = ColumnPath::from("my_column");
```

# Example: refer to column named `c` in a nested struct `{a: {b: {c: ...}}}`
```
# use parquet::schema::types::ColumnPath;
// form path 'a.b.c'
let column_path = ColumnPath::from(vec![
  String::from("a"),
  String::from("b"),
  String::from("c")
]);
```

<a id="op-0c2b1949e30f6593b9a4bbdb"></a>
## append

`function` · `parquet::schema::types::ColumnPath::append` · parquet 59.3.0

```rust
fn append(&mut self, tail: Vec<String>)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 1], "end": [806, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:798`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Appends more components to end of column path.
```rust
use parquet::schema::types::ColumnPath;

let mut path = ColumnPath::new(vec!["a".to_string(), "b".to_string(), "c"
.to_string()]);
assert_eq!(&path.string(), "a.b.c");

path.append(vec!["d".to_string(), "e".to_string()]);
assert_eq!(&path.string(), "a.b.c.d.e");
```

<a id="op-4c9aa787182b2da0ba20e082"></a>
## as_ref

`function` · `parquet::schema::types::ColumnPath::as_ref` · parquet 59.3.0

```rust
fn as_ref(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [834, 1], "end": [838, 2], "filename": "src/schema/types.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"slice": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}}], "constraints": []}}, "id": "core::convert::AsRef", "path": "AsRef"}, "trait_path": "core::convert::AsRef"}`

Source: `src/schema/types.rs:835`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-57edd5f833db8f53046b718f"></a>
## clone

`function` · `parquet::schema::types::ColumnPath::clone` · parquet 59.3.0

```rust
fn clone(&self) -> ColumnPath
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [759, 10], "end": [759, 15], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/schema/types.rs:759`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-2a7fa9606b0975396095d55a"></a>
## eq

`function` · `parquet::schema::types::ColumnPath::eq` · parquet 59.3.0

```rust
fn eq(&self, other: &ColumnPath) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [759, 17], "end": [759, 26], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/schema/types.rs:759`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-8adfd4b5f03f9969f06e3b9d"></a>
## fmt

`function` · `parquet::schema::types::ColumnPath::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [808, 1], "end": [812, 2], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/schema/types.rs:809`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e10dd7926ed6259bf817d566"></a>
## fmt

`function` · `parquet::schema::types::ColumnPath::fmt` · parquet 59.3.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [759, 28], "end": [759, 33], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/schema/types.rs:759`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d46c629298ad388a3a9e544f"></a>
## from

`function` · `parquet::schema::types::ColumnPath::from` · parquet 59.3.0

```rust
fn from(parts: Vec<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [814, 1], "end": [818, 2], "filename": "src/schema/types.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "alloc::vec::Vec", "path": "Vec"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema/types.rs:815`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e69e39abd865e4ee4620f1cb"></a>
## from

`function` · `parquet::schema::types::ColumnPath::from` · parquet 59.3.0

```rust
fn from(single_path: &str) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [820, 1], "end": [825, 2], "filename": "src/schema/types.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"borrowed_ref": {"is_mutable": false, "lifetime": null, "type": {"primitive": "str"}}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema/types.rs:821`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ee24af99b736d59ee38a595f"></a>
## from

`function` · `parquet::schema::types::ColumnPath::from` · parquet 59.3.0

```rust
fn from(single_path: String) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [827, 1], "end": [832, 2], "filename": "src/schema/types.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"type": {"resolved_path": {"args": null, "id": "alloc::string::String", "path": "String"}}}], "constraints": []}}, "id": "core::convert::From", "path": "From"}, "trait_path": "core::convert::From"}`

Source: `src/schema/types.rs:828`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f78c20a5a36c0b915796d2a1"></a>
## hash

`function` · `parquet::schema::types::ColumnPath::hash` · parquet 59.3.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [759, 39], "end": [759, 43], "filename": "src/schema/types.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/schema/types.rs:759`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-37f29321dbddbacd671f700a"></a>
## new

`function` · `parquet::schema::types::ColumnPath::new` · parquet 59.3.0

```rust
fn new(parts: Vec<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 1], "end": [806, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:772`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Creates new column path from vector of field names.

<a id="op-777609b4aa2e9bd96aef0df3"></a>
## parts

`function` · `parquet::schema::types::ColumnPath::parts` · parquet 59.3.0

```rust
fn parts(&self) -> &[String]
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 1], "end": [806, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:803`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns a slice of path components.

<a id="op-84321766d3b8186b68522aa8"></a>
## string

`function` · `parquet::schema::types::ColumnPath::string` · parquet 59.3.0

```rust
fn string(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "parquet::schema::types::ColumnPath", "path": "ColumnPath"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [770, 1], "end": [806, 2], "filename": "src/schema/types.rs"}, "trait": null, "trait_path": null}`

Source: `src/schema/types.rs:783`. [Exact documentation build](https://docs.rs/crate/parquet/59.3.0/json).

Returns string representation of this column path.
```rust
use parquet::schema::types::ColumnPath;

let path = ColumnPath::new(vec!["a".to_string(), "b".to_string(), "c".to_string()]);
assert_eq!(&path.string(), "a.b.c");
```
