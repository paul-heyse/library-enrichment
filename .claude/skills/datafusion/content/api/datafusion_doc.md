# `datafusion_doc`

Crate `datafusion-doc` · 3 public items · structured records in [`model/datafusion_doc.json`](../model/datafusion_doc.json)

## DocSection

`struct` · `datafusion_doc::DocSection`

Also reachable as `datafusion::logical_expr::DocSection`, `datafusion_expr::DocSection`

```rust
struct DocSection
```

**Fields**: `include`, `label`, `description`

**Derives**: Clone, Debug, Default, Eq, Hash, PartialEq, StructuralPartialEq

---

## Documentation

`struct` · `datafusion_doc::Documentation`

Also reachable as `datafusion::logical_expr::Documentation`, `datafusion_expr::Documentation`

```rust
struct Documentation
```

**Fields**: `doc_section`, `description`, `syntax_example`, `sql_example`, `arguments`, `alternative_syntax`, `related_udfs`

**Derives**: Clone, Debug, Eq, Hash, PartialEq, StructuralPartialEq

**Methods** (2)

```rust
fn builder(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> DocumentationBuilder
fn to_doc_attribute(&self) -> String
```

Documentation for use by `ScalarUDFImpl`, `AggregateUDFImpl` and `WindowUDFImpl` functions.

See the [`DocumentationBuilder`] to create a new [`Documentation`] struct.

The DataFusion [SQL function documentation] is automatically  generated from these structs.
The name of the udf will be pulled from the `ScalarUDFImpl::name`,
`AggregateUDFImpl::name` or `WindowUDFImpl::name`
function as appropriate.

All strings in the documentation are required to be
in [markdown format](https://www.markdownguide.org/basic-syntax/).

Currently, documentation only supports a single language
thus all text should be in English.

[SQL function documentation]: https://datafusion.apache.org/user-guide/sql/index.html

---

## DocumentationBuilder

`struct` · `datafusion_doc::DocumentationBuilder`

Also reachable as `datafusion::logical_expr::DocumentationBuilder`, `datafusion_expr::DocumentationBuilder`

```rust
struct DocumentationBuilder
```

**Fields**: `doc_section`, `description`, `syntax_example`, `sql_example`, `arguments`, `alternative_syntax`, `related_udfs`

**Methods** (10)

```rust
fn build(self) -> Documentation
fn new_with_details(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> Self
fn with_alternative_syntax(self, syntax_name: impl Into<String>) -> Self
fn with_argument(self, arg_name: impl Into<String>, arg_description: impl Into<String>) -> Self
fn with_description(self, description: impl Into<String>) -> Self
fn with_doc_section(self, doc_section: DocSection) -> Self
fn with_related_udf(self, related_udf: impl Into<String>) -> Self
fn with_sql_example(self, sql_example: impl Into<String>) -> Self
fn with_standard_argument(self, arg_name: impl Into<String>, expression_type: Option<&str>) -> Self
fn with_syntax_example(self, syntax_example: impl Into<String>) -> Self
```

A builder for [`Documentation`]'s.

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

---
