# `ast_grep_config::transform::string_case`

Crate `ast-grep-config` · 2 public items · structured records in [`model/ast_grep_config.transform.string_case.json`](../model/ast_grep_config.transform.string_case.json)

## Separator

`enum` · `ast_grep_config::transform::string_case::Separator`

```rust
enum Separator
```

**Variants**: `CaseChange`, `Dash`, `Dot`, `Slash`, `Space`, `Underscore`

Separator to split string. e.g. `user_accountName` -> `user`, `accountName`
It will be rejoin according to `StringCase`.

---

## StringCase

`enum` · `ast_grep_config::transform::string_case::StringCase`

```rust
enum StringCase
```

**Variants**: `LowerCase`, `UpperCase`, `Capitalize`, `CamelCase`, `SnakeCase`, `KebabCase`, `PascalCase`

An enumeration representing different cases for strings.

---
