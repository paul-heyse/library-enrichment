# `ast_grep_config`

Crate `ast-grep-config` · 2 public items · structured records in [`model/ast_grep_config.json`](../model/ast_grep_config.json)

## from_str

`function` · `ast_grep_config::from_str`

```rust
fn from_str<'de, T: Deserialize<'de>>(s: &'de str) -> Result<T, serde_yaml::Error>
```

---

## from_yaml_string

`function` · `ast_grep_config::from_yaml_string`

```rust
fn from_yaml_string<'a, L: Language + Deserialize<'a>>(yamls: &'a str, registration: &GlobalRules) -> Result<Vec<RuleConfig<L>>, RuleConfigError>
```

---
