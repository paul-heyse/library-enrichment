# `ast_grep_config::rule::parameterized_util`

Crate `ast-grep-config` · 2 public items · structured records in [`model/ast_grep_config.rule.parameterized_util.json`](../model/ast_grep_config.rule.parameterized_util.json)

## ParameterizedUtilError

`enum` · `ast_grep_config::rule::parameterized_util::ParameterizedUtilError`

Also reachable as `ast_grep_config::ParameterizedUtilError`

```rust
enum ParameterizedUtilError
```

**Variants**: `InvalidUtilityId`, `InvalidUtilityArgument`, `DuplicateUtilityArgument`, `InvalidUtilityCall`, `MissingUtilityArguments`, `UnexpectedUtilityArguments`, `UtilityParameterCalled`, `MissingUtilityArgument`, `UnknownUtilityArgument`

**Implements**: `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

---

## SerializableUtilityCall

`struct` · `ast_grep_config::rule::parameterized_util::SerializableUtilityCall`

```rust
struct SerializableUtilityCall
```

---
