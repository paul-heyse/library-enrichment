# `ra_ap_ide`

Crate `ra_ap_ide` · 4 public items · structured records in [`model/ra_ap_ide.json`](../model/ra_ap_ide.json)

## Analysis

`struct` · `ra_ap_ide::Analysis`

```rust
struct Analysis
```

**Derives**: Debug

**Methods** (76)

```rust
fn annotations(&self, config: &AnnotationConfig<'_>, file_id: FileId) -> Cancellable<Vec<Annotation>>
fn assists_with_fixes(&self, assist_config: &AssistConfig, diagnostics_config: &DiagnosticsConfig, resolve: AssistResolveStrategy, frange: FileRange) -> Cancellable<Vec<Assist>>
fn call_hierarchy(&self, position: FilePosition, config: &CallHierarchyConfig<'_>) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>>
fn child_modules(&self, position: FilePosition) -> Cancellable<Vec<NavigationTarget>>
fn completions(&self, config: &CompletionConfig<'_>, position: FilePosition, trigger_character: Option<char>) -> Cancellable<Option<Vec<CompletionItem>>>
fn crate_edition(&self, crate_id: Crate) -> Cancellable<Edition>
fn crate_root(&self, crate_id: Crate) -> Cancellable<FileId>
fn crates_for(&self, file_id: FileId) -> Cancellable<Vec<Crate>>
fn discover_test_roots(&self) -> Cancellable<Vec<TestItem>>
fn discover_tests_in_crate(&self, crate_id: Crate) -> Cancellable<Vec<TestItem>>
fn discover_tests_in_crate_by_test_id(&self, crate_id: &str) -> Cancellable<Vec<TestItem>>
fn discover_tests_in_file(&self, file_id: FileId) -> Cancellable<Vec<TestItem>>
fn editioned_file_id_to_vfs(&self, file_id: hir::EditionedFileId) -> FileId
fn evaluate_predicate(&self, text: String, position: FilePosition) -> Cancellable<PredicateEvaluationResult>
fn expand_macro(&self, position: FilePosition) -> Cancellable<Option<ExpandedMacro>>
fn extend_selection(&self, frange: FileRange) -> Cancellable<TextRange>
fn external_docs(&self, position: FilePosition, target_dir: Option<&str>, sysroot: Option<&str>) -> Cancellable<doc_links::DocumentationLinks>
fn fetch_crates(&self) -> Cancellable<FxIndexSet<CrateInfo>>
fn file_line_index(&self, file_id: FileId) -> Cancellable<Arc<LineIndex>>
fn file_structure(&self, config: &FileStructureConfig, file_id: FileId) -> Cancellable<Vec<StructureNode>>
fn file_text(&self, file_id: FileId) -> Cancellable<Arc<str>>
fn find_all_refs(&self, position: FilePosition, config: &FindAllRefsConfig<'_>) -> Cancellable<Option<Vec<ReferenceSearchResult>>>
fn folding_ranges(&self, file_id: FileId, collapsed_text: bool) -> Cancellable<Vec<Fold>>
fn from_single_file(text: String, proc_macro_cwd: Arc<AbsPathBuf>) -> (Analysis, FileId)
fn full_diagnostics(&self, config: &DiagnosticsConfig, resolve: AssistResolveStrategy, file_id: FileId) -> Cancellable<Vec<Diagnostic>>
fn get_failed_obligations(&self, offset: TextSize, file_id: FileId) -> Cancellable<String>
fn get_recursive_memory_layout(&self, position: FilePosition) -> Cancellable<Option<RecursiveMemoryLayout>>
fn goto_declaration(&self, position: FilePosition, config: &GotoDefinitionConfig<'_>) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>>
fn goto_definition(&self, position: FilePosition, config: &GotoDefinitionConfig<'_>) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>>
fn goto_implementation(&self, config: &GotoImplementationConfig, position: FilePosition) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>>
fn goto_type_definition(&self, position: FilePosition) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>>
fn highlight(&self, highlight_config: HighlightConfig<'_>, file_id: FileId) -> Cancellable<Vec<HlRange>>
fn highlight_as_html(&self, file_id: FileId, rainbow: bool) -> Cancellable<String>
fn highlight_as_html_with_config(&self, config: HighlightConfig<'_>, file_id: FileId, rainbow: bool) -> Cancellable<String>
fn highlight_range(&self, highlight_config: HighlightConfig<'_>, frange: FileRange) -> Cancellable<Vec<HlRange>>
fn highlight_related(&self, config: HighlightRelatedConfig, position: FilePosition) -> Cancellable<Option<Vec<HighlightedRange>>>
fn hover(&self, config: &HoverConfig<'_>, range: FileRange) -> Cancellable<Option<RangeInfo<HoverResult>>>
fn incoming_calls(&self, config: &CallHierarchyConfig<'_>, position: FilePosition) -> Cancellable<Option<Vec<CallItem>>>
fn inlay_hints(&self, config: &InlayHintsConfig<'_>, file_id: FileId, range: Option<TextRange>) -> Cancellable<Vec<InlayHint>>
fn inlay_hints_resolve(&self, config: &InlayHintsConfig<'_>, file_id: FileId, resolve_range: TextRange, hash: u64, hasher: impl Fn(&InlayHint) -> u64 + Send + UnwindSafe) -> Cancellable<Option<InlayHint>>
fn interpret_function(&self, position: FilePosition) -> Cancellable<String>
fn is_crate_no_std(&self, crate_id: Crate) -> Cancellable<bool>
fn is_library_file(&self, file_id: FileId) -> Cancellable<bool>
fn is_local_source_root(&self, source_root_id: SourceRootId) -> Cancellable<bool>
fn is_proc_macro_crate(&self, crate_id: Crate) -> Cancellable<bool>
fn join_lines(&self, config: &JoinLinesConfig, frange: FileRange) -> Cancellable<TextEdit>
fn matching_brace(&self, position: FilePosition) -> Cancellable<Option<TextSize>>
fn moniker(&self, position: FilePosition) -> Cancellable<Option<RangeInfo<Vec<moniker::MonikerResult>>>>
fn move_item(&self, range: FileRange, direction: Direction) -> Cancellable<Option<TextEdit>>
fn on_char_typed(&self, position: FilePosition, char_typed: char) -> Cancellable<Option<SourceChange>>
fn on_enter(&self, position: FilePosition) -> Cancellable<Option<TextEdit>>
fn outgoing_calls(&self, config: &CallHierarchyConfig<'_>, position: FilePosition) -> Cancellable<Option<Vec<CallItem>>>
fn parallel_prime_caches<F>(&self, scope: &[Crate], num_worker_threads: usize, cb: F) -> Cancellable<()> where F: Fn(ParallelPrimeCachesProgress) + Sync + std::panic::UnwindSafe
fn parent_module(&self, position: FilePosition) -> Cancellable<Vec<NavigationTarget>>
fn parse(&self, file_id: FileId) -> Cancellable<SourceFile>
fn prepare_rename(&self, position: FilePosition) -> Cancellable<Result<RangeInfo<()>, RenameError>>
fn related_tests(&self, position: FilePosition, search_scope: Option<SearchScope>) -> Cancellable<Vec<Runnable>>
fn relevant_crates_for(&self, file_id: FileId) -> Cancellable<Vec<Crate>>
fn rename(&self, position: FilePosition, new_name: &str, config: &RenameConfig) -> Cancellable<Result<SourceChange, RenameError>>
fn resolve_annotation(&self, config: &AnnotationConfig<'_>, annotation: Annotation) -> Cancellable<Annotation>
fn resolve_completion_edits(&self, config: &CompletionConfig<'_>, position: FilePosition, imports: impl IntoIterator<Item = CompletionItemImport> + std::panic::UnwindSafe) -> Cancellable<Vec<TextEdit>>
fn runnables(&self, file_id: FileId) -> Cancellable<Vec<Runnable>>
fn semantic_diagnostics(&self, config: &DiagnosticsConfig, resolve: AssistResolveStrategy, file_id: FileId) -> Cancellable<Vec<Diagnostic>>
fn signature_help(&self, position: FilePosition) -> Cancellable<Option<SignatureHelp>>
fn source_root_id(&self, file_id: FileId) -> Cancellable<SourceRootId>
fn status(&self, file_id: Option<FileId>) -> Cancellable<String>
fn structural_search_replace(&self, query: &str, parse_only: bool, resolve_context: FilePosition, selections: Vec<FileRange>) -> Cancellable<Result<SourceChange, SsrError>>
fn symbol_search(&self, query: Query, limit: usize) -> Cancellable<Vec<NavigationTarget>>
fn syntax_diagnostics(&self, config: &DiagnosticsConfig, file_id: FileId) -> Cancellable<Vec<Diagnostic>>
fn transitive_rev_deps(&self, crate_id: Crate) -> Cancellable<Vec<Crate>>
fn view_crate_graph(&self, full: bool) -> Cancellable<String>
fn view_hir(&self, position: FilePosition) -> Cancellable<String>
fn view_item_tree(&self, file_id: FileId) -> Cancellable<String>
fn view_mir(&self, position: FilePosition) -> Cancellable<String>
fn view_syntax_tree(&self, file_id: FileId) -> Cancellable<String>
fn will_rename_file(&self, file_id: FileId, new_name_stem: &str, config: &RenameConfig) -> Cancellable<Option<SourceChange>>
```

Analysis is a snapshot of a world state at a moment in time. It is the main
entry point for asking semantic information about the world. When the world
state is advanced using `AnalysisHost::apply_change` method, all existing
`Analysis` are canceled (most method return `Err(Canceled)`).

---

## AnalysisHost

`struct` · `ra_ap_ide::AnalysisHost`

```rust
struct AnalysisHost
```

**Derives**: Debug, Default

**Methods** (11)

```rust
fn analysis(&self) -> Analysis
fn apply_change(&mut self, change: ChangeWithProcMacros) -> Duration
fn new(lru_capacity: Option<u16>) -> AnalysisHost
fn per_query_memory_usage(&mut self) -> Vec<(String, profile::Bytes, usize)>
fn raw_database(&self) -> &RootDatabase
fn raw_database_mut(&mut self) -> &mut RootDatabase
fn trigger_cancellation(&mut self)
fn trigger_garbage_collection(&mut self)
fn update_lru_capacities(&mut self, lru_capacities: &FxHashMap<Box<str>, u16>)
fn update_lru_capacity(&mut self, lru_capacity: Option<u16>)
fn with_database(db: RootDatabase) -> AnalysisHost
```

`AnalysisHost` stores the current state of the world.

---

## RangeInfo

`struct` · `ra_ap_ide::RangeInfo`

```rust
struct RangeInfo<T>
```

**Fields**: `range`, `info`

**Implements**: `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`

**Derives**: Debug

**Methods** (1)

```rust
fn new(range: TextRange, info: T) -> RangeInfo<T>
```

**via `ra_ap_ide_db::ra_fixture::UpmapFromRaFixture`**

```rust
fn upmap_from_ra_fixture(self, __analysis: &::ide_db::ra_fixture::RaFixtureAnalysis, __virtual_file_id: ::ide_db::ra_fixture::FileId, __real_file_id: ::ide_db::ra_fixture::FileId) -> Result<Self, ()>
```

Info associated with a text range.

---

## Cancellable

`type_alias` · `ra_ap_ide::Cancellable`

```rust
type Cancellable<T> = Result<T, ide_db::base_db::salsa::Cancelled>
```

---
