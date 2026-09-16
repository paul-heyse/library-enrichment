# `ruff_notebook::schema`

Crate `ruff_notebook` · 12 public items · structured records in [`model/ruff_notebook.schema.json`](../model/ruff_notebook.schema.json)

## Cell

`enum` · `ruff_notebook::schema::Cell`

Also reachable as `ruff_notebook::Cell`

```rust
enum Cell
```

**Variants**: `Code`, `Markdown`, `Raw`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**Methods** (3)

```rust
fn is_code_cell(&self) -> bool
fn metadata(&self) -> &CellMetadata
fn source(&self) -> &SourceValue
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

String identifying the type of cell.

---

## SourceValue

`enum` · `ruff_notebook::schema::SourceValue`

Also reachable as `ruff_notebook::SourceValue`

```rust
enum SourceValue
```

**Variants**: `String`, `StringArray`

**Implements**: `core::fmt::Display`, `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

mimetype output (e.g. text/plain), represented as either an array of strings or a
string.

Contents of the cell, represented as an array of lines.

The stream's text output, represented as an array of strings.

---

## CellMetadata

`struct` · `ruff_notebook::schema::CellMetadata`

Also reachable as `ruff_notebook::CellMetadata`

```rust
struct CellMetadata
```

**Fields**: `vscode`, `extra`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Cell-level metadata.

---

## CodeCell

`struct` · `ruff_notebook::schema::CodeCell`

Also reachable as `ruff_notebook::CodeCell`

```rust
struct CodeCell
```

**Fields**: `execution_count`, `id`, `metadata`, `outputs`, `source`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Notebook code cell.

---

## CodeCellMetadataVSCode

`struct` · `ruff_notebook::schema::CodeCellMetadataVSCode`

Also reachable as `ruff_notebook::CodeCellMetadataVSCode`

```rust
struct CodeCellMetadataVSCode
```

**Fields**: `language_id`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

VS Code specific cell metadata.
<https://github.com/microsoft/vscode/blob/e6c009a3d4ee60f352212b978934f52c4689fbd9/extensions/ipynb/src/serializers.ts#L104-L107>

---

## Kernelspec

`struct` · `ruff_notebook::schema::Kernelspec`

Also reachable as `ruff_notebook::Kernelspec`

```rust
struct Kernelspec
```

**Fields**: `language`, `extra`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Kernel information.

---

## LanguageInfo

`struct` · `ruff_notebook::schema::LanguageInfo`

Also reachable as `ruff_notebook::LanguageInfo`

```rust
struct LanguageInfo
```

**Fields**: `codemirror_mode`, `file_extension`, `mimetype`, `name`, `pygments_lexer`, `extra`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Language information.

---

## MarkdownCell

`struct` · `ruff_notebook::schema::MarkdownCell`

Also reachable as `ruff_notebook::MarkdownCell`

```rust
struct MarkdownCell
```

**Fields**: `attachments`, `id`, `metadata`, `source`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Notebook markdown cell.

---

## RawCell

`struct` · `ruff_notebook::schema::RawCell`

Also reachable as `ruff_notebook::RawCell`

```rust
struct RawCell
```

**Fields**: `attachments`, `id`, `metadata`, `source`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Notebook raw nbconvert cell.

---

## RawNotebook

`struct` · `ruff_notebook::schema::RawNotebook`

Also reachable as `ruff_notebook::RawNotebook`

```rust
struct RawNotebook
```

**Fields**: `cells`, `metadata`, `nbformat`, `nbformat_minor`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

The root of the JSON of a Jupyter Notebook

Generated by <https://app.quicktype.io/> from
<https://github.com/jupyter/nbformat/blob/16b53251aabf472ad9406ddb1f78b0421c014eeb/nbformat/v4/nbformat.v4.schema.json>
Jupyter Notebook v4.5 JSON schema.

---

## RawNotebookMetadata

`struct` · `ruff_notebook::schema::RawNotebookMetadata`

Also reachable as `ruff_notebook::RawNotebookMetadata`

```rust
struct RawNotebookMetadata
```

**Fields**: `authors`, `kernelspec`, `language_info`, `orig_nbformat`, `title`, `extra`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Default, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Notebook root-level metadata.

---

## SortAlphabetically

`struct` · `ruff_notebook::schema::SortAlphabetically`

Also reachable as `ruff_notebook::SortAlphabetically`

```rust
struct SortAlphabetically<T: Serialize>
```

**Implements**: `serde_core::ser::Serialize`

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

This is used to serialize any value implementing [`Serialize`] alphabetically.

The reason for this is to maintain consistency in the generated JSON string,
which is useful for diffing. The default serializer keeps the order of the
fields as they are defined in the struct, which will not be consistent when
there are `extra` fields.

# Example

```
use std::collections::BTreeMap;

use serde::Serialize;

use ruff_notebook::SortAlphabetically;

#[derive(Serialize)]
struct MyStruct {
   a: String,
   #[serde(flatten)]
   extra: BTreeMap<String, String>,
   b: String,
}

let my_struct = MyStruct {
    a: "a".to_string(),
    extra: BTreeMap::from([
        ("d".to_string(), "d".to_string()),
        ("c".to_string(), "c".to_string()),
    ]),
    b: "b".to_string(),
};

let serialized = serde_json::to_string_pretty(&SortAlphabetically(&my_struct)).unwrap();
assert_eq!(
    serialized,
r#"{
  "a": "a",
  "b": "b",
  "c": "c",
  "d": "d"
}"#
);
```

---
