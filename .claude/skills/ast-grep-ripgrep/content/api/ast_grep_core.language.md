# `ast_grep_core::language`

Crate `ast-grep-core` · 1 public items · structured records in [`model/ast_grep_core.language.json`](../model/ast_grep_core.language.json)

## Language

`trait` · `ast_grep_core::language::Language`

Also reachable as `ast_grep_core::Language`, `ast_grep_language::Language`

```rust
trait Language: Clone + 'static
```

**Implementors** (30)

- `ast_grep_dynamic::DynamicLang`
- `ast_grep_language::Bash`
- `ast_grep_language::C`
- `ast_grep_language::CSharp`
- `ast_grep_language::Cpp`
- `ast_grep_language::Css`
- `ast_grep_language::Dart`
- `ast_grep_language::Elixir`
- `ast_grep_language::Go`
- `ast_grep_language::Haskell`
- `ast_grep_language::Hcl`
- `ast_grep_language::Java`
- `ast_grep_language::JavaScript`
- `ast_grep_language::Json`
- `ast_grep_language::Kotlin`
- `ast_grep_language::Lua`
- `ast_grep_language::Markdown`
- `ast_grep_language::Nix`
- `ast_grep_language::Php`
- `ast_grep_language::Python`
- `ast_grep_language::Ruby`
- `ast_grep_language::Rust`
- `ast_grep_language::Scala`
- `ast_grep_language::Solidity`
- `ast_grep_language::SupportLang`
- `ast_grep_language::Swift`
- `ast_grep_language::Tsx`
- `ast_grep_language::TypeScript`
- `ast_grep_language::Yaml`
- `ast_grep_language::html::Html`

**Methods** (8)

```rust
fn build_pattern(&self, builder: &PatternBuilder<'_>) -> Result<Pattern, PatternError>
fn expando_char(&self) -> char
fn extract_meta_var(&self, source: &str) -> Option<MetaVariable>
fn field_to_id(&self, field: &str) -> Option<u16>
fn from_path<P: AsRef<Path>>(_path: P) -> Option<Self>
fn kind_to_id(&self, kind: &str) -> u16
fn meta_var_char(&self) -> char
fn pre_process_pattern<'q>(&self, query: &'q str) -> Cow<'q, str>
```

Trait to abstract ts-language usage in ast-grep, which includes:
* which character is used for meta variable.
* if we need to use other char in meta var for parser at runtime
* pre process the Pattern code.

---
