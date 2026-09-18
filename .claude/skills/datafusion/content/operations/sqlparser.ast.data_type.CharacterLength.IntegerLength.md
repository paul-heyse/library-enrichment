# `sqlparser::ast::data_type::CharacterLength::IntegerLength`

Full upstream contracts; raw type trees and source locators in [structured records](sqlparser.ast.data_type.CharacterLength.IntegerLength.json).

<a id="op-24ef7f643b2158e0f2749402"></a>
## length

`struct_field` · `sqlparser::ast::data_type::CharacterLength::IntegerLength::length` · sqlparser 0.62.0

```rust
length: u64
```

Source: `src/ast/data_type.rs:1052`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Default (if VARYING) or maximum (if not VARYING) length

<a id="op-c1d48109bc4b76826c907635"></a>
## unit

`struct_field` · `sqlparser::ast::data_type::CharacterLength::IntegerLength::unit` · sqlparser 0.62.0

```rust
unit: Option<CharLengthUnits>
```

Source: `src/ast/data_type.rs:1054`. [Exact documentation build](https://docs.rs/crate/sqlparser/0.62.0/json).

Optional unit. If not informed, the ANSI handles it as CHARACTERS implicitly
