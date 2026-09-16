# `pyrefly_util::telemetry`

Crate `pyrefly_util` · 20 public items · structured records in [`model/pyrefly_util.telemetry.json`](../model/pyrefly_util.telemetry.json)

## DefinitionContext

`enum` · `pyrefly_util::telemetry::DefinitionContext`

```rust
enum DefinitionContext
```

**Variants**: `NameUse`, `NameDef`, `Attribute`, `KeywordArgument`, `ImportedModule`, `ImportedName`, `Definition`, `LocalBinding`, `MutableCapture`, `Operator`, `NoneLiteral`

**Implements**: `core::fmt::Display`

**Derives**: Clone, Debug

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

What kind of symbol the cursor was on when a definition lookup failed.

---

## EmptyResponseReason

`enum` · `pyrefly_util::telemetry::EmptyResponseReason`

```rust
enum EmptyResponseReason
```

**Variants**: `NoFilePath`, `LanguageServicesDisabled`, `MethodDisabled`, `NotebookNotSupported`, `ModuleInfoNotFound`, `AstNotFound`, `AnswersNotFound`, `BindingsNotFound`, `TypeTraceNotFound`, `ModuleNotFound`, `NotAnIdentifier`, `DefinitionNotFound`, `BundledModuleNotMaterialized`

**Derives**: Clone, Debug

**Methods** (2)

```rust
fn as_str(&self) -> &'static str
fn detail(&self) -> String
```

Why an LSP handler returned an empty/null response. Used for telemetry
to distinguish expected cases (whitespace, comments) from unexpected
failures (module not found, internal bugs).

---

## QueueName

`enum` · `pyrefly_util::telemetry::QueueName`

```rust
enum QueueName
```

**Variants**: `LspQueue`, `RecheckQueue`, `FindReferenceQueue`, `SourceDbQueue`

**Implements**: `core::fmt::Display`, `dupe::Dupe`

**Derives**: Clone, Copy, Debug

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

**via `core::fmt::Display`**

```rust
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
```

---

## TelemetryEventKind

`enum` · `pyrefly_util::telemetry::TelemetryEventKind`

```rust
enum TelemetryEventKind
```

**Variants**: `LspEvent`, `CodeAction`, `AdHocSolve`, `SetMemory`, `InvalidateDisk`, `InvalidateFind`, `InvalidateEvents`, `InvalidateConfig`, `InvalidateOnClose`, `PopulateProjectFiles`, `PopulateWorkspaceFiles`, `WorkspaceDiagnosticsRepopulation`, `SourceDbRebuild`, `SourceDbRebuildInstance`, `FindFromDefinition`, `ExternalReferences`, `ExternalWorkspaceSymbols`, `LspStartup`

---

## TelemetryInvalidateFindReason

`enum` · `pyrefly_util::telemetry::TelemetryInvalidateFindReason`

```rust
enum TelemetryInvalidateFindReason
```

**Variants**: `WatcherEvents`, `SourceDbConfigChanged`

**Derives**: Clone, Copy

**Methods** (1)

```rust
fn as_str(&self) -> &'static str
```

---

## ActivityKey

`struct` · `pyrefly_util::telemetry::ActivityKey`

```rust
struct ActivityKey
```

**Fields**: `id`, `name`

**Implements**: `serde_core::de::Deserialize`, `serde_core::ser::Serialize`

**Derives**: Clone, Debug, Eq, PartialEq, StructuralPartialEq

**via `serde_core::de::Deserialize`**

```rust
fn deserialize<__D>(__deserializer: __D) -> _serde::__private229::Result<Self, __D::Error> where __D: _serde::Deserializer<'de>
```

**via `serde_core::ser::Serialize`**

```rust
fn serialize<__S>(&self, __serializer: __S) -> _serde::__private229::Result<__S::Ok, __S::Error> where __S: _serde::Serializer
```

---

## NoTelemetry

`struct` · `pyrefly_util::telemetry::NoTelemetry`

```rust
struct NoTelemetry
```

**Implements**: `pyrefly_util::telemetry::Telemetry`

**via `pyrefly_util::telemetry::Telemetry`**

```rust
fn agent_invocation_id(&self) -> Option<String>
fn agent_session_id(&self) -> Option<String>
fn record_event(&self, _event: TelemetryEvent, _process: Duration, _error: Option<&Error>)
fn surface(&self) -> Option<String>
```

---

## SubTaskTelemetry

`struct` · `pyrefly_util::telemetry::SubTaskTelemetry`

```rust
struct SubTaskTelemetry<'a>
```

**Methods** (3)

```rust
fn finish_task(&self, telemetry_event: TelemetryEvent, error: Option<&Error>)
fn new(telemetry: &'a dyn Telemetry, event: &TelemetryEvent) -> Self
fn new_task(&self, kind: TelemetryEventKind, start: Instant) -> TelemetryEvent
```

---

## TelemetryCommonSourceDbStats

`struct` · `pyrefly_util::telemetry::TelemetryCommonSourceDbStats`

```rust
struct TelemetryCommonSourceDbStats
```

**Fields**: `files`, `changed`, `forced`

**Derives**: Default

---

## TelemetryDidChangeWatchedFilesStats

`struct` · `pyrefly_util::telemetry::TelemetryDidChangeWatchedFilesStats`

```rust
struct TelemetryDidChangeWatchedFilesStats
```

**Fields**: `created_count`, `modified_count`, `removed_count`, `unknown_count`, `created`, `modified`, `removed`, `unknown`

**Derives**: Default

---

## TelemetryEvent

`struct` · `pyrefly_util::telemetry::TelemetryEvent`

```rust
struct TelemetryEvent
```

**Fields**: `kind`, `queue`, `start`, `invalidate`, `validate`, `transaction_stats`, `server_state`, `file_stats`, `queue_name`, `task_id`, `sourcedb_rebuild_stats`, `sourcedb_rebuild_instance_stats`, `file_watcher_stats`, `did_change_watched_files_stats`, `invalidate_find_reason`, `external_references_stats`, `external_workspace_symbols_stats`, `activity_key`, `canceled`, `empty_response_reason`, `request_id`, `error_detail`

**Methods** (16)

```rust
fn finish_and_record(self, telemetry: &dyn Telemetry, error: Option<&Error>) -> Duration
fn new_dequeued(kind: TelemetryEventKind, enqueued_at: Instant, server_state: TelemetryServerState, queue_name: QueueName, task_id: usize) -> (Self, Duration)
fn new_task(kind: TelemetryEventKind, server_state: TelemetryServerState, queue_name: QueueName, task_id: usize, start: Instant) -> Self
fn set_activity_key(&mut self, activity_key: Option<ActivityKey>)
fn set_did_change_watched_files_stats(&mut self, stats: TelemetryDidChangeWatchedFilesStats)
fn set_empty_response_reason(&mut self, reason: EmptyResponseReason)
fn set_external_references_stats(&mut self, stats: TelemetryExternalReferencesStats)
fn set_external_workspace_symbols_stats(&mut self, stats: TelemetryExternalWorkspaceSymbolsStats)
fn set_file_stats(&mut self, stats: TelemetryFileStats)
fn set_file_watcher_stats(&mut self, stats: TelemetryFileWatcherStats)
fn set_invalidate_duration(&mut self, duration: Duration)
fn set_invalidate_find_reason(&mut self, reason: TelemetryInvalidateFindReason)
fn set_sourcedb_rebuild_instance_stats(&mut self, stats: TelemetrySourceDbRebuildInstanceStats)
fn set_sourcedb_rebuild_stats(&mut self, stats: TelemetrySourceDbRebuildStats)
fn set_transaction_stats(&mut self, stats: TelemetryTransactionStats)
fn set_validate_duration(&mut self, duration: Duration)
```

---

## TelemetryExternalReferencesStats

`struct` · `pyrefly_util::telemetry::TelemetryExternalReferencesStats`

```rust
struct TelemetryExternalReferencesStats
```

**Fields**: `qualified_name`, `db_name`, `result_file_count`, `result_span_count`, `find_repo_ms`, `warmup_wait_ms`, `angle_query_ms`, `cas_init_error`, `resolve_locations_ms`, `resolve_disk_read_ms`, `resolve_hash_ms`, `resolve_cas_download_ms`, `resolve_textdiff_ms`, `resolve_unchanged_file_count`, `resolve_changed_file_count`, `resolve_unreadable_file_count`

**Derives**: Default

---

## TelemetryExternalWorkspaceSymbolsStats

`struct` · `pyrefly_util::telemetry::TelemetryExternalWorkspaceSymbolsStats`

```rust
struct TelemetryExternalWorkspaceSymbolsStats
```

**Fields**: `query`, `db_name`, `result_count`, `find_repo_ms`, `warmup_wait_ms`, `angle_query_ms`

**Derives**: Default

---

## TelemetryFileStats

`struct` · `pyrefly_util::telemetry::TelemetryFileStats`

```rust
struct TelemetryFileStats
```

**Fields**: `uri`, `config_root`

**Derives**: Clone

---

## TelemetryFileWatcherStats

`struct` · `pyrefly_util::telemetry::TelemetryFileWatcherStats`

```rust
struct TelemetryFileWatcherStats
```

**Fields**: `duration`, `count`

**Derives**: Default

---

## TelemetryServerState

`struct` · `pyrefly_util::telemetry::TelemetryServerState`

```rust
struct TelemetryServerState
```

**Fields**: `has_sourcedb`, `id`, `surface`, `server_start_time`, `agent_session_id`, `agent_invocation_id`, `active_experiments`

**Derives**: Clone

---

## TelemetrySourceDbRebuildInstanceStats

`struct` · `pyrefly_util::telemetry::TelemetrySourceDbRebuildInstanceStats`

```rust
struct TelemetrySourceDbRebuildInstanceStats
```

**Fields**: `common`, `build_id`, `build_time`, `parse_time`, `process_time`, `raw_size`, `exit_reason`

**Derives**: Default

---

## TelemetrySourceDbRebuildStats

`struct` · `pyrefly_util::telemetry::TelemetrySourceDbRebuildStats`

```rust
struct TelemetrySourceDbRebuildStats
```

**Fields**: `count`, `had_error`, `common`

**Derives**: Default

---

## TelemetryTransactionStats

`struct` · `pyrefly_util::telemetry::TelemetryTransactionStats`

```rust
struct TelemetryTransactionStats
```

**Fields**: `modules`, `dirty_rdeps`, `cycle_rdeps`, `run_steps`, `run_time`, `committed`, `state_lock_blocked`, `fresh`, `set_memory_dirty`, `compute_stdlib_time`, `compute_stdlib_cached`, `compute_stdlib_prewarm_time`, `run_dirty_count`, `run_todo_count`, `run_work_time`, `search_exports_time`, `search_exports_dispatch_time`, `cancelled`, `step_load_time`, `step_load_count`, `step_ast_time`, `step_ast_count`, `step_exports_time`, `step_exports_count`, `step_answers_time`, `step_answers_count`, `step_solutions_time`, `step_solutions_count`, `clean_time`, `clean_count`, `find_import_time`, `find_import_count`, `demand_wait_time`, `demand_wait_count`, `cold_modules`, `warm_modules`, `total_stat_count`, `slow_stat_count`, `slow_stat_time`, `total_read_count`, `slow_read_count`, `slow_read_time`

**Derives**: Default

---

## Telemetry

`trait` · `pyrefly_util::telemetry::Telemetry`

```rust
trait Telemetry: Send + Sync
```

**Implementors** (1)

- `pyrefly_util::telemetry::NoTelemetry`

**Methods** (4)

```rust
fn agent_invocation_id(&self) -> Option<String>
fn agent_session_id(&self) -> Option<String>
fn record_event(&self, event: TelemetryEvent, process: Duration, error: Option<&Error>)
fn surface(&self) -> Option<String>
```

---
