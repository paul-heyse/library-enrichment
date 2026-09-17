# `ast_grep_config::combined`

Crate `ast-grep-config` · 4 public items · structured records in [`model/ast_grep_config.combined.json`](../model/ast_grep_config.combined.json)

## NO_SUPPRESS_ALL_ID

`constant` · `ast_grep_config::combined::NO_SUPPRESS_ALL_ID`

Also reachable as `ast_grep_config::NO_SUPPRESS_ALL_ID`

```rust
const NO_SUPPRESS_ALL_ID: &str = "no-suppress-all"
```

---

## UNUSED_SUPPRESSION_ID

`constant` · `ast_grep_config::combined::UNUSED_SUPPRESSION_ID`

Also reachable as `ast_grep_config::UNUSED_SUPPRESSION_ID`

```rust
const UNUSED_SUPPRESSION_ID: &str = "unused-suppression"
```

---

## CombinedScan

`struct` · `ast_grep_config::combined::CombinedScan`

Also reachable as `ast_grep_config::CombinedScan`

```rust
struct CombinedScan<'r, L: Language>
```

**Methods** (7)

```rust
fn get_rule(&self, idx: usize) -> &'r RuleConfig<L>
fn new(rules: Vec<&'r RuleConfig<L>>) -> Self
fn no_suppress_all_config(severity: Severity, lang: L) -> RuleConfig<L>
fn scan<'a, D>(&self, root: &'a AstGrep<D>, separate_fix: bool) -> ScanResult<'a, '_, D, L> where D: Doc<Lang = L>
fn set_no_suppress_all_rule(&mut self, rule: &'r RuleConfig<L>)
fn set_unused_suppression_rule(&mut self, rule: &'r RuleConfig<L>)
fn unused_config(severity: Severity, lang: L) -> RuleConfig<L>
```

A struct to group all rules according to their potential kinds.
This can greatly reduce traversal times and skip unmatchable rules.
Rules are referenced by their index in the rules vector.

---

## ScanResult

`struct` · `ast_grep_config::combined::ScanResult`

```rust
struct ScanResult<'t, 'r, D: Doc, L: Language>
```

**Fields**: `diffs`, `matches`

---
