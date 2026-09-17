# `ast_grep_config::rule::referent_rule`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.rule.referent_rule.json`](../model/ast_grep_config.rule.referent_rule.json)

## ReferentRuleError

`enum` · `ast_grep_config::rule::referent_rule::ReferentRuleError`

```rust
enum ReferentRuleError
```

**Variants**: `UndefinedUtil`, `DuplicateRule`, `CyclicRule`

---

## GlobalRules

`struct` · `ast_grep_config::rule::referent_rule::GlobalRules`

Also reachable as `ast_grep_config::GlobalRules`

```rust
struct GlobalRules
```

**Derives**: Clone, Default

**Methods** (1)

```rust
fn insert(&self, id: &str, rule: RuleCore, params: Option<Vec<String>>) -> Result<(), ReferentRuleError>
```

---

## ReferentRule

`struct` · `ast_grep_config::rule::referent_rule::ReferentRule`

```rust
struct ReferentRule
```

---

## RuleRegistration

`struct` · `ast_grep_config::rule::referent_rule::RuleRegistration`

```rust
struct RuleRegistration
```

---
