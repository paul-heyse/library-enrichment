# `datafusion_doc::DocumentationBuilder`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_doc.DocumentationBuilder.json).

<a id="op-3c8aad68c8ea77587c22b5df"></a>
## DocumentationBuilder

`struct` · `datafusion_doc::DocumentationBuilder` · datafusion-doc 55.1.0

```rust
struct DocumentationBuilder
```

Source: `src/lib.rs:211`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

A builder for [`Documentation`](../operations/datafusion_doc.Documentation.md#op-0d9b5cd247c1b7b1819374a8)'s.

Example:

```rust

# fn main() {
    use datafusion_doc::{DocSection, Documentation};
    let doc_section = DocSection {
        include: true,
        label: "Display Label",
        description: None,
    };

    let documentation = Documentation::builder(doc_section, "Add one to an int32".to_owned(), "add_one(2)".to_owned())
          .with_argument("arg_1", "The int32 number to add one to")
          .build();
# }

<a id="op-2c28488dacc7bc4e1111c8a0"></a>
## alternative_syntax

`struct_field` · `datafusion_doc::DocumentationBuilder::alternative_syntax` · datafusion-doc 55.1.0

```rust
alternative_syntax: Option<Vec<String>>
```

Source: `src/lib.rs:217`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-dc54579e5c70a715ba74889c"></a>
## arguments

`struct_field` · `datafusion_doc::DocumentationBuilder::arguments` · datafusion-doc 55.1.0

```rust
arguments: Option<Vec<(String, String)>>
```

Source: `src/lib.rs:216`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4a83e85a6e027d386ceab260"></a>
## build

`function` · `datafusion_doc::DocumentationBuilder::build` · datafusion-doc 55.1.0

```rust
fn build(self) -> Documentation
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:317`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Build the documentation from provided components

Panics if `doc_section`, `description` or `syntax_example` is not set

<a id="op-26518138bf176091ad306f19"></a>
## description

`struct_field` · `datafusion_doc::DocumentationBuilder::description` · datafusion-doc 55.1.0

```rust
description: String
```

Source: `src/lib.rs:213`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-b44c86baecf7e2451bcba286"></a>
## doc_section

`struct_field` · `datafusion_doc::DocumentationBuilder::doc_section` · datafusion-doc 55.1.0

```rust
doc_section: DocSection
```

Source: `src/lib.rs:212`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d0306fc02851f927e7852bb7"></a>
## new_with_details

`function` · `datafusion_doc::DocumentationBuilder::new_with_details` · datafusion-doc 55.1.0

```rust
fn new_with_details(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:223`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Creates a new [`DocumentationBuilder`](../operations/datafusion_doc.DocumentationBuilder.md#op-3c8aad68c8ea77587c22b5df) with all required fields

<a id="op-18be4f43b626ef9e45f46f50"></a>
## related_udfs

`struct_field` · `datafusion_doc::DocumentationBuilder::related_udfs` · datafusion-doc 55.1.0

```rust
related_udfs: Option<Vec<String>>
```

Source: `src/lib.rs:218`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-03053b15c778d6c18414883f"></a>
## sql_example

`struct_field` · `datafusion_doc::DocumentationBuilder::sql_example` · datafusion-doc 55.1.0

```rust
sql_example: Option<String>
```

Source: `src/lib.rs:215`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-3cab86c09ec3e07c14e05961"></a>
## syntax_example

`struct_field` · `datafusion_doc::DocumentationBuilder::syntax_example` · datafusion-doc 55.1.0

```rust
syntax_example: String
```

Source: `src/lib.rs:214`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-a19741d38265cd477513c8f6"></a>
## with_alternative_syntax

`function` · `datafusion_doc::DocumentationBuilder::with_alternative_syntax` · datafusion-doc 55.1.0

```rust
fn with_alternative_syntax(self, syntax_name: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:300`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfdbfcfc3340ee1d0a66f4ff"></a>
## with_argument

`function` · `datafusion_doc::DocumentationBuilder::with_argument` · datafusion-doc 55.1.0

```rust
fn with_argument(self, arg_name: impl Into<String>, arg_description: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:262`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Adds documentation for a specific argument to the documentation.

Arguments are displayed in the order they are added.

<a id="op-4bdc51c8ffdfbe51ef84dc14"></a>
## with_description

`function` · `datafusion_doc::DocumentationBuilder::with_description` · datafusion-doc 55.1.0

```rust
fn with_description(self, description: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:244`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4e0b7a0f745e9554ee8ce64e"></a>
## with_doc_section

`function` · `datafusion_doc::DocumentationBuilder::with_doc_section` · datafusion-doc 55.1.0

```rust
fn with_doc_section(self, doc_section: DocSection) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:239`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-ca3c486c3af19babd662d4d2"></a>
## with_related_udf

`function` · `datafusion_doc::DocumentationBuilder::with_related_udf` · datafusion-doc 55.1.0

```rust
fn with_related_udf(self, related_udf: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:307`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-6388cca4476e1deadb4fa061"></a>
## with_sql_example

`function` · `datafusion_doc::DocumentationBuilder::with_sql_example` · datafusion-doc 55.1.0

```rust
fn with_sql_example(self, sql_example: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:254`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-cfda65e395f720efa2afe315"></a>
## with_standard_argument

`function` · `datafusion_doc::DocumentationBuilder::with_standard_argument` · datafusion-doc 55.1.0

```rust
fn with_standard_argument(self, arg_name: impl Into<String>, expression_type: Option<&str>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:288`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

Add a standard "expression" argument to the documentation

The argument is rendered like below if Some() is passed through:

```text
<arg_name>:
  <expression_type> expression to operate on. Can be a constant, column, or function, and any combination of operators.
```

The argument is rendered like below if None is passed through:

```text
<arg_name>:
  The expression to operate on. Can be a constant, column, or function, and any combination of operators.
```

<a id="op-daec3306933a9145acb39fad"></a>
## with_syntax_example

`function` · `datafusion_doc::DocumentationBuilder::with_syntax_example` · datafusion-doc 55.1.0

```rust
fn with_syntax_example(self, syntax_example: impl Into<String>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "datafusion_doc::DocumentationBuilder", "path": "DocumentationBuilder"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [221, 1], "end": [338, 2], "filename": "src/lib.rs"}, "trait": null, "trait_path": null}`

Source: `src/lib.rs:249`. [Exact documentation build](https://docs.rs/crate/datafusion-doc/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.
