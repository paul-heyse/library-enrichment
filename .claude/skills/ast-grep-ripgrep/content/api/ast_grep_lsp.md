# `ast_grep_lsp`

Crate `ast-grep-lsp` · 2 public items · structured records in [`model/ast_grep_lsp.json`](../model/ast_grep_lsp.json)

## Backend

`struct` · `ast_grep_lsp::Backend`

```rust
struct Backend<L: LSPLang>
```

**Implements**: `tower_lsp_server::server::LanguageServer`

**Methods** (1)

```rust
fn new<F>(client: Client, base: PathBuf, rule_finder: F) -> Self where F: Fn() -> anyhow::Result<RuleCollection<L>> + Send + Sync + 'static
```

**via `tower_lsp_server::server::LanguageServer`**

```rust
async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>>
async fn did_change(&self, params: DidChangeTextDocumentParams)
async fn did_change_configuration(&self, _: DidChangeConfigurationParams)
async fn did_change_watched_files(&self, _params: DidChangeWatchedFilesParams)
async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams)
async fn did_close(&self, params: DidCloseTextDocumentParams)
async fn did_open(&self, params: DidOpenTextDocumentParams)
async fn did_save(&self, _: DidSaveTextDocumentParams)
async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>>
async fn hover(&self, params: HoverParams) -> Result<Option<Hover>>
async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult>
async fn initialized(&self, _: InitializedParams)
async fn shutdown(&self) -> Result<()>
```

---

## LSPLang

`trait` · `ast_grep_lsp::LSPLang`

```rust
trait LSPLang: LanguageExt + Eq + Send + Sync + FromStr + 'static
```

---
