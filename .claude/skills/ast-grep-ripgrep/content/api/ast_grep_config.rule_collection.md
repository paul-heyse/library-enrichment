# `ast_grep_config::rule_collection`

Crate `ast-grep-config` · 1 public items · structured records in [`model/ast_grep_config.rule_collection.json`](../model/ast_grep_config.rule_collection.json)

## RuleCollection

`struct` · `ast_grep_config::rule_collection::RuleCollection`

Also reachable as `ast_grep_config::RuleCollection`

```rust
struct RuleCollection<L: Language + Eq>
```

**Derives**: Default

**Methods** (6)

```rust
fn for_each_rule(&self, f: impl FnMut(&RuleConfig<L>))
fn for_path<P: AsRef<Path>>(&self, path: P) -> Vec<&RuleConfig<L>>
fn get_rule(&self, id: &str) -> Option<&RuleConfig<L>>
fn get_rule_from_lang(&self, path: &Path, lang: L) -> Vec<&RuleConfig<L>>
fn total_rule_count(&self) -> usize
fn try_new(configs: Vec<RuleConfig<L>>) -> Result<Self, globset::Error>
```

A collection of rules to run one round of scanning.
Rules will be grouped together based on their language, path globbing and pattern rule.

---
