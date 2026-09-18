# `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.ddl.UserDefinedTypeSqlDefinitionOption.json).

<a id="op-36b0d73054cdd770fc3adf14"></a>
## UserDefinedTypeSqlDefinitionOption

`enum` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption` · sqlparser 0.62.0

```rust
enum UserDefinedTypeSqlDefinitionOption
```

Source: `src/ast/ddl.rs:2641`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Options for PostgreSQL `CREATE TYPE ... (<options>)` statement (base type definition).

Base types are the lowest-level data types in PostgreSQL. To define a new base type,
you must specify functions that convert it to and from text representation, and optionally
binary representation and other properties.

Note: This syntax uses parentheses directly after the type name, without the `AS` keyword.

# PostgreSQL Documentation
See: <https://www.postgresql.org/docs/current/sql-createtype.html>

# Examples
```sql
CREATE TYPE complex (
    INPUT = complex_in,
    OUTPUT = complex_out,
    INTERNALLENGTH = 16,
    ALIGNMENT = double
);
```

<a id="op-169a436e09864e39a26d7fab"></a>
## Alignment

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Alignment` · sqlparser 0.62.0

```rust
Alignment
```

Source: `src/ast/ddl.rs:2663`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Storage alignment requirement (1, 2, 4, or 8 bytes): `ALIGNMENT = alignment`

<a id="op-cbd0fd78cc2f23c11df06bb6"></a>
## Analyze

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Analyze` · sqlparser 0.62.0

```rust
Analyze
```

Source: `src/ast/ddl.rs:2655`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to compute statistics for the data type: `ANALYZE = analyze_function`

<a id="op-e5671ef2a4ebd4e647bb60e2"></a>
## Category

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Category` · sqlparser 0.62.0

```rust
Category
```

Source: `src/ast/ddl.rs:2669`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Type category for implicit casting rules (single char): `CATEGORY = category`

<a id="op-4e318ecef987ef8061feed2f"></a>
## Collatable

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Collatable` · sqlparser 0.62.0

```rust
Collatable
```

Source: `src/ast/ddl.rs:2679`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether the type supports collation: `COLLATABLE = collatable`

<a id="op-53889f65122e654d4252f698"></a>
## Default

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Default` · sqlparser 0.62.0

```rust
Default
```

Source: `src/ast/ddl.rs:2673`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default value for the type: `DEFAULT = default`

<a id="op-a6e40e6a691c1e24de846038"></a>
## Delimiter

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Delimiter` · sqlparser 0.62.0

```rust
Delimiter
```

Source: `src/ast/ddl.rs:2677`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Delimiter character for array value display: `DELIMITER = delimiter`

<a id="op-c270dee2d8c1c123d81edf88"></a>
## Element

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Element` · sqlparser 0.62.0

```rust
Element
```

Source: `src/ast/ddl.rs:2675`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Element type for array types: `ELEMENT = element`

<a id="op-446215f1e99123055ca296ea"></a>
## Input

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Input` · sqlparser 0.62.0

```rust
Input
```

Source: `src/ast/ddl.rs:2643`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert from external text representation to internal: `INPUT = input_function`

<a id="op-03a4f0eb4d1dd5d4d13847d5"></a>
## InternalLength

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::InternalLength` · sqlparser 0.62.0

```rust
InternalLength
```

Source: `src/ast/ddl.rs:2659`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Internal storage size in bytes, or VARIABLE for variable-length: `INTERNALLENGTH = { internallength | VARIABLE }`

<a id="op-c69c59252338a19dd0e759ef"></a>
## Like

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Like` · sqlparser 0.62.0

```rust
Like
```

Source: `src/ast/ddl.rs:2667`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Copy properties from an existing type: `LIKE = like_type`

<a id="op-4ae8b3df1c30abd735e683b3"></a>
## Output

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Output` · sqlparser 0.62.0

```rust
Output
```

Source: `src/ast/ddl.rs:2645`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert from internal to external text representation: `OUTPUT = output_function`

<a id="op-e2e7c5a63766bc9c1d3e2bb3"></a>
## PassedByValue

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::PassedByValue` · sqlparser 0.62.0

```rust
PassedByValue
```

Source: `src/ast/ddl.rs:2661`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Indicates values are passed by value rather than by reference: `PASSEDBYVALUE`

<a id="op-d3ec6d5447100ffaf77ffeec"></a>
## Preferred

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Preferred` · sqlparser 0.62.0

```rust
Preferred
```

Source: `src/ast/ddl.rs:2671`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Whether this type is preferred within its category: `PREFERRED = preferred`

<a id="op-52b1037f106dce74d9cc536a"></a>
## Receive

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Receive` · sqlparser 0.62.0

```rust
Receive
```

Source: `src/ast/ddl.rs:2647`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert from external binary representation to internal: `RECEIVE = receive_function`

<a id="op-2721f221a4bab0b17a77d268"></a>
## Send

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Send` · sqlparser 0.62.0

```rust
Send
```

Source: `src/ast/ddl.rs:2649`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert from internal to external binary representation: `SEND = send_function`

<a id="op-53a50bf193e6dff43cb6e9bc"></a>
## Storage

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Storage` · sqlparser 0.62.0

```rust
Storage
```

Source: `src/ast/ddl.rs:2665`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Storage strategy for varlena types: `STORAGE = storage`

<a id="op-3f990480076ac060bc678292"></a>
## Subscript

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::Subscript` · sqlparser 0.62.0

```rust
Subscript
```

Source: `src/ast/ddl.rs:2657`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to handle subscripting operations: `SUBSCRIPT = subscript_function`

<a id="op-01d2effecaa7f848ec193649"></a>
## TypmodIn

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::TypmodIn` · sqlparser 0.62.0

```rust
TypmodIn
```

Source: `src/ast/ddl.rs:2651`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert type modifiers from text array to internal form: `TYPMOD_IN = type_modifier_input_function`

<a id="op-c35696bc5951f89e26818469"></a>
## TypmodOut

`variant` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::TypmodOut` · sqlparser 0.62.0

```rust
TypmodOut
```

Source: `src/ast/ddl.rs:2653`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Function to convert type modifiers from internal to text form: `TYPMOD_OUT = type_modifier_output_function`

<a id="op-02d9950ae7c88dbbec30136f"></a>
## clone

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::clone` · sqlparser 0.62.0

```rust
fn clone(&self) -> UserDefinedTypeSqlDefinitionOption
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 17], "end": [2638, 22], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::clone::Clone", "path": "Clone"}, "trait_path": "core::clone::Clone"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-301e06fbdff18daec026b625"></a>
## cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::cmp` · sqlparser 0.62.0

```rust
fn cmp(&self, other: &UserDefinedTypeSqlDefinitionOption) -> cmp::Ordering
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 51], "end": [2638, 54], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::Ord", "path": "Ord"}, "trait_path": "core::cmp::Ord"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-51ee250c00cbb5bcfdd88ead"></a>
## deserialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::deserialize` · sqlparser 0.62.0

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private228::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'de"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [2639, 49], "end": [2639, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": {"angle_bracketed": {"args": [{"lifetime": "'de"}], "constraints": []}}, "id": "serde_core::de::Deserialize", "path": "Deserialize"}, "trait_path": "serde_core::de::Deserialize"}`

Source: `src/ast/ddl.rs:2639`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-add8570ce1b9f121139258c2"></a>
## eq

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::eq` · sqlparser 0.62.0

```rust
fn eq(&self, other: &UserDefinedTypeSqlDefinitionOption) -> bool
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 24], "end": [2638, 33], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialEq", "path": "PartialEq"}, "trait_path": "core::cmp::PartialEq"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-09789d19a075b752621a2ca6"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2682, 1], "end": [2718, 2], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Display", "path": "Display"}, "trait_path": "core::fmt::Display"}`

Source: `src/ast/ddl.rs:2683`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-22bd973ab108c8eb1b1fc906"></a>
## fmt

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::fmt` · sqlparser 0.62.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 10], "end": [2638, 15], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-c39a1a158d05d7eda09f2065"></a>
## hash

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::hash` · sqlparser 0.62.0

```rust
fn hash<__H: hash::Hasher>(&self, state: &mut __H)
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 56], "end": [2638, 60], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::hash::Hash", "path": "Hash"}, "trait_path": "core::hash::Hash"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-479b4d3c4e8ed9e8f97e1d02"></a>
## partial_cmp

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::partial_cmp` · sqlparser 0.62.0

```rust
fn partial_cmp(&self, other: &UserDefinedTypeSqlDefinitionOption) -> option::Option<cmp::Ordering>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2638, 35], "end": [2638, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "core::cmp::PartialOrd", "path": "PartialOrd"}, "trait_path": "core::cmp::PartialOrd"}`

Source: `src/ast/ddl.rs:2638`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-9a92fe09371edd04b2938281"></a>
## serialize

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::serialize` · sqlparser 0.62.0

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private228::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2639, 38], "end": [2639, 47], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "serde_core::ser::Serialize", "path": "Serialize"}, "trait_path": "serde_core::ser::Serialize"}`

Source: `src/ast/ddl.rs:2639`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-d3f4ae29d15a24516870510a"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::VisitorMut>(&mut self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2640, 47], "end": [2640, 55], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::VisitMut", "path": "VisitMut"}, "trait_path": "sqlparser::ast::visitor::VisitMut"}`

Source: `src/ast/ddl.rs:2640`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e8bf22984a878e643dad7b62"></a>
## visit

`function` · `sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption::visit` · sqlparser 0.62.0

```rust
fn visit<V: sqlparser::ast::Visitor>(&self, visitor: &mut V) -> ::std::ops::ControlFlow<V::Break>
```

Implementation context: `{"for": {"resolved_path": {"args": null, "id": "sqlparser::ast::ddl::UserDefinedTypeSqlDefinitionOption", "path": "UserDefinedTypeSqlDefinitionOption"}}, "generics": {"params": [], "where_predicates": []}, "is_negative": false, "span": {"begin": [2640, 40], "end": [2640, 45], "filename": "src/ast/ddl.rs"}, "trait": {"args": null, "id": "sqlparser::ast::visitor::Visit", "path": "Visit"}, "trait_path": "sqlparser::ast::visitor::Visit"}`

Source: `src/ast/ddl.rs:2640`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

No upstream documentation on this item; consult its owner/trait contract.
