# `ast_grep_core::tree_sitter`

Crate `ast-grep-core` · 6 public items · structured records in [`model/ast_grep_core.tree_sitter.json`](../model/ast_grep_core.tree_sitter.json)

## TSParseError

`enum` · `ast_grep_core::tree_sitter::TSParseError`

```rust
enum TSParseError
```

**Variants**: `Language`, `TreeUnavailable`

**Implements**: `core::convert::From`, `core::error::Error`, `core::fmt::Display`

**Derives**: Debug

**via `core::convert::From`**

```rust
fn from(source: LanguageError) -> Self
```

**via `core::error::Error`**

```rust
fn source(&self) -> ::core::option::Option<&dyn ::thiserror::__private20::Error + 'static>
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, __formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result
```

Represents tree-sitter related error

---

## perform_edit

`function` · `ast_grep_core::tree_sitter::perform_edit`

```rust
fn perform_edit<S: ContentExt>(tree: &mut tree_sitter::Tree, input: &mut S, edit: &source::Edit<S>) -> tree_sitter::InputEdit
```

---

## DisplayContext

`struct` · `ast_grep_core::tree_sitter::DisplayContext`

```rust
struct DisplayContext<'r>
```

**Fields**: `matched`, `leading`, `trailing`, `start_line`

---

## StrDoc

`struct` · `ast_grep_core::tree_sitter::StrDoc`

```rust
struct StrDoc<L: LanguageExt>
```

**Fields**: `src`, `lang`, `tree`

**Implements**: `ast_grep_core::source::Doc`

**Derives**: Clone

**Methods** (2)

```rust
fn new(src: &str, lang: L) -> Self
fn try_new(src: &str, lang: L) -> Result<Self, String>
```

**via `ast_grep_core::source::Doc`**

```rust
fn do_edit(&mut self, edit: &Edit<Self::Source>) -> Result<(), String>
fn get_lang(&self) -> &Self::Lang
fn get_node_text<'a>(&'a self, node: &Self::Node<'a>) -> Cow<'a, str>
fn get_source(&self) -> &Self::Source
fn root_node(&self) -> Node<'_>
```

---

## ContentExt

`trait` · `ast_grep_core::tree_sitter::ContentExt`

```rust
trait ContentExt: Content
```

**Implementors** (1)

- `alloc::string::String`

**Methods** (1)

```rust
fn accept_edit(&mut self, edit: &Edit<Self>) -> InputEdit
```

---

## LanguageExt

`trait` · `ast_grep_core::tree_sitter::LanguageExt`

Also reachable as `ast_grep_language::LanguageExt`

```rust
trait LanguageExt: Language
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

**Methods** (4)

```rust
fn ast_grep<S: AsRef<str>>(&self, source: S) -> AstGrep<StrDoc<Self>>
fn extract_injections<L: LanguageExt>(&self, _root: Node<'_, StrDoc<L>>) -> Vec<(String, Vec<TSRange>)>
fn get_ts_language(&self) -> TSLanguage
fn injectable_languages(&self) -> Option<&'static [&'static str]>
```

tree-sitter specific language trait

---
