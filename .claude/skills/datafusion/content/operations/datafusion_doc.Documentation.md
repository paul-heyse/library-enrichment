# `datafusion_doc::Documentation`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_doc.Documentation.json).

<a id="op-0d9b5cd247c1b7b1819374a8"></a>
## Documentation

`struct` · `datafusion_doc::Documentation` · datafusion-doc 55.1.0

```rust
struct Documentation
```

Source: `src/lib.rs:50`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Documentation for use by `ScalarUDFImpl`, `AggregateUDFImpl` and `WindowUDFImpl` functions.

See the [`DocumentationBuilder`](../operations/datafusion_doc.DocumentationBuilder.md#op-3c8aad68c8ea77587c22b5df) to create a new [`Documentation`](../operations/datafusion_doc.Documentation.md#op-0d9b5cd247c1b7b1819374a8) struct.

The DataFusion [SQL function documentation] is automatically  generated from these structs.
The name of the udf will be pulled from the `ScalarUDFImpl::name`,
`AggregateUDFImpl::name` or `WindowUDFImpl::name`
function as appropriate.

All strings in the documentation are required to be
in [markdown format](https://www.markdownguide.org/basic-syntax/).

Currently, documentation only supports a single language
thus all text should be in English.

[SQL function documentation]: https://datafusion.apache.org/user-guide/sql/index.html

<a id="op-dc91c853587df7d375b8e42c"></a>
## alternative_syntax

`struct_field` · `datafusion_doc::Documentation::alternative_syntax` · datafusion-doc 55.1.0

```rust
alternative_syntax: Option<Vec<String>>
```

Source: `src/lib.rs:66`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

A list of alternative syntax examples for a function

<a id="op-674505ef31df19f864b1026e"></a>
## arguments

`struct_field` · `datafusion_doc::Documentation::arguments` · datafusion-doc 55.1.0

```rust
arguments: Option<Vec<(String, String)>>
```

Source: `src/lib.rs:64`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Arguments for the UDF which will be displayed in array order.
Left member of a pair is the argument name, right is a
description for the argument

<a id="op-7c00fdcab9b15f6af1dd5d02"></a>
## builder

`function` · `datafusion_doc::Documentation::builder` · datafusion-doc 55.1.0

```rust
fn builder(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> DocumentationBuilder
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [166, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:76`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Returns a new [`DocumentationBuilder`](../operations/datafusion_doc.DocumentationBuilder.md#op-3c8aad68c8ea77587c22b5df) with no options set.

<a id="op-648cdeddd402a5e04b2a6a74"></a>
## clone

`function` · `datafusion_doc::Documentation::clone` · datafusion-doc 55.1.0

```rust
fn clone(&self) -> Documentation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 17], "end": [49, 22], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/lib.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-95151c4df1725e1ed8d8139c"></a>
## description

`struct_field` · `datafusion_doc::Documentation::description` · datafusion-doc 55.1.0

```rust
description: String
```

Source: `src/lib.rs:54`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

The description for the UDF

<a id="op-c129cf4586eb201a90b7520e"></a>
## doc_section

`struct_field` · `datafusion_doc::Documentation::doc_section` · datafusion-doc 55.1.0

```rust
doc_section: DocSection
```

Source: `src/lib.rs:52`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

The section in the documentation where the UDF will be documented

<a id="op-805bf0244ab04a6424ec3f71"></a>
## eq

`function` · `datafusion_doc::Documentation::eq` · datafusion-doc 55.1.0

```rust
fn eq(&self, other: &Documentation) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 24], "end": [49, 33], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/lib.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f77617f0344d5e3314c80491"></a>
## fmt

`function` · `datafusion_doc::Documentation::fmt` · datafusion-doc 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 10], "end": [49, 15], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/lib.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5f786b1a358e34343d7d57d6"></a>
## hash

`function` · `datafusion_doc::Documentation::hash` · datafusion-doc 55.1.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [49, 39], "end": [49, 43], "filename": "src/lib.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/lib.rs:49`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9ef72a7cf136e6fa8fc4990d"></a>
## related_udfs

`struct_field` · `datafusion_doc::Documentation::related_udfs` · datafusion-doc 55.1.0

```rust
related_udfs: Option<Vec<String>>
```

Source: `src/lib.rs:71`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Related functions if any. Values should match the related
udf's name exactly. Related udf's must be of the same
UDF type (scalar, aggregate or window) for proper linking to
occur

<a id="op-612cbe8b1e8e51c93c5018b4"></a>
## sql_example

`struct_field` · `datafusion_doc::Documentation::sql_example` · datafusion-doc 55.1.0

```rust
sql_example: Option<String>
```

Source: `src/lib.rs:60`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

A sql example for the UDF, usually in the form of a sql prompt
query and output. It is strongly recommended to provide an
example for anything but the most basic UDF's

<a id="op-e99811ca83337722751ad50f"></a>
## syntax_example

`struct_field` · `datafusion_doc::Documentation::syntax_example` · datafusion-doc 55.1.0

```rust
syntax_example: String
```

Source: `src/lib.rs:56`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

A brief example of the syntax. For example "ascii(str)"

<a id="op-80df943e8a987fd4125dd2cd"></a>
## to_doc_attribute

`function` · `datafusion_doc::Documentation::to_doc_attribute` · datafusion-doc 55.1.0

```rust
fn to_doc_attribute(&self) -> String
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::Documentation", "path": "Documentation"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [74, 1], "end": [166, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:87`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Output the `Documentation` struct in form of custom Rust documentation attributes
It is useful to semi automate during tmigration of UDF documentation
generation from code based to attribute based and can be safely removed after
