# Sessions and runtime

A `SessionContext` is a handle over a `SessionState`, which pairs a `SessionConfig` (planning and execution settings) with a `RuntimeEnv` (memory pool, disk manager, object store registry, caches). `SessionContext::new()` takes the defaults for both, and the default `RuntimeEnv` has **no memory limit and no spill path** — a sort or aggregation larger than RAM aborts rather than spilling. Anything you want bounded is configured before the context exists, not after.

## Entry points

| Type | Kind | Methods | Prose | Records |
|---|---|---:|---|---|
| `datafusion::execution::context::SessionContext` | struct | 92 | [prose](../api/datafusion.execution.context.md#sessioncontext) | [records](../model/datafusion.execution.context.json) |
| `datafusion::execution::session_state::SessionState` | struct | 102 | [prose](../api/datafusion.execution.session_state.md#sessionstate) | [records](../model/datafusion.execution.session_state.json) |
| `datafusion::execution::session_state::SessionStateBuilder` | struct | 67 | [prose](../api/datafusion.execution.session_state.md#sessionstatebuilder) | [records](../model/datafusion.execution.session_state.json) |
| `datafusion_execution::config::SessionConfig` | struct | 63 | [prose](../api/datafusion_execution.config.md#sessionconfig) | [records](../model/datafusion_execution.config.json) |
| `datafusion_execution::runtime_env::RuntimeEnv` | struct | 10 | [prose](../api/datafusion_execution.runtime_env.md#runtimeenv) | [records](../model/datafusion_execution.runtime_env.json) |
| `datafusion_execution::runtime_env::RuntimeEnvBuilder` | struct | 20 | [prose](../api/datafusion_execution.runtime_env.md#runtimeenvbuilder) | [records](../model/datafusion_execution.runtime_env.json) |
| `datafusion_execution::task::TaskContext` | struct | 28 | [prose](../api/datafusion_execution.task.md#taskcontext) | [records](../model/datafusion_execution.task.json) |
| `datafusion_execution::disk_manager::DiskManager` | struct | 13 | [prose](../api/datafusion_execution.disk_manager.md#diskmanager) | [records](../model/datafusion_execution.disk_manager.json) |

## Configuration methods

Chainable `with_*` builders. These are invisible to anyone reading only the
constructor, which is why they are the most consistently missed part of the API.

**`SessionConfig`** — 29 builder methods

`with_allow_symmetric_joins_without_pruning`, `with_batch_size`, `with_coalesce_batches`, `with_collect_statistics`, `with_create_default_catalog_and_schema`, `with_default_catalog_and_schema`, `with_enable_ansi_mode`, `with_enable_sort_pushdown`, `with_enable_subquery_sort_elimination`, `with_enforce_batch_size_in_joins`, `with_extension`, `with_information_schema`, `with_option_extension`, `with_parquet_bloom_filter_pruning`, `with_parquet_page_index_pruning`, `with_parquet_pruning`, `with_prefer_existing_sort`, `with_prefer_existing_union`, `with_repartition_aggregations`, `with_repartition_file_min_size`, `with_repartition_file_scans`, `with_repartition_joins`, `with_repartition_sorts`, `with_repartition_windows`, `with_round_robin_repartition`, `with_sort_in_place_threshold_bytes`, `with_sort_spill_reservation_bytes`, `with_spill_compression`, `with_target_partitions`

**`RuntimeEnvBuilder`** — 12 builder methods

`with_cache_manager`, `with_disk_manager_builder`, `with_file_statistics_cache_limit`, `with_max_spill_merge_fan_in`, `with_max_temp_directory_size`, `with_memory_limit`, `with_memory_pool`, `with_metadata_cache_limit`, `with_object_list_cache_limit`, `with_object_list_cache_ttl`, `with_object_store_registry`, `with_temp_file_path`

**`SessionStateBuilder`** — 32 builder methods

`with_aggregate_functions`, `with_analyzer_rule`, `with_analyzer_rules`, `with_cache_factory`, `with_catalog_list`, `with_config`, `with_default_features`, `with_execution_props`, `with_expr_planners`, `with_extension_type_registry`, `with_file_formats`, `with_function_factory`, `with_higher_order_functions`, `with_object_store`, `with_optimizer_rule`, `with_optimizer_rules`, `with_physical_optimizer_rule`, `with_physical_optimizer_rules`, `with_query_planner`, `with_relation_planners`, `with_runtime_env`, `with_scalar_functions`, `with_serializer_registry`, `with_session_id`, `with_statistics_registry`, `with_table_factories`, `with_table_factory`, `with_table_function_list`, `with_table_functions`, `with_table_options`, `with_type_planner`, `with_window_functions`

## Settings (10)

Full table with Rust setters in [`../catalogs/config-options.md`](../catalogs/config-options.md).

| Setting | Default |
|---|---|
| `datafusion.execution.batch_size` | 8192 |
| `datafusion.execution.target_partitions` | 0 |
| `datafusion.runtime.file_statistics_cache_limit` | 20M |
| `datafusion.runtime.list_files_cache_limit` | 1M |
| `datafusion.runtime.list_files_cache_ttl` | NULL |
| `datafusion.runtime.max_spill_merge_fan_in` | 0 |
| `datafusion.runtime.max_temp_directory_size` | 100G |
| `datafusion.runtime.memory_limit` | NULL |
| `datafusion.runtime.metadata_cache_limit` | 50M |
| `datafusion.runtime.temp_directory` | NULL |

## Runnable examples (4)

- [`corpus/examples/execution_monitoring/main.rs`](../corpus/examples/execution_monitoring/main.rs)
- [`corpus/examples/execution_monitoring/memory_pool_execution_plan.rs`](../corpus/examples/execution_monitoring/memory_pool_execution_plan.rs)
- [`corpus/examples/execution_monitoring/memory_pool_tracking.rs`](../corpus/examples/execution_monitoring/memory_pool_tracking.rs)
- [`corpus/examples/execution_monitoring/tracing.rs`](../corpus/examples/execution_monitoring/tracing.rs)

## Upstream guides

- [`corpus/guides/user-guide/crate-configuration.md`](../corpus/guides/user-guide/crate-configuration.md)
- [`corpus/guides/user-guide/configs.md`](../corpus/guides/user-guide/configs.md)

## Decision rules

- Bounded work needs `RuntimeEnvBuilder` with a memory pool (`FairSpillPool` or `GreedyMemoryPool`), `with_temp_file_path`, and `with_max_temp_directory_size`; pass the result to `SessionContext::new_with_config_rt`.
- `SessionStateBuilder::new_from_existing` is the way to install a custom query planner, optimizer rule or analyzer rule while keeping everything else.
- A `SessionContext` is cheap to clone and shares its state; build one per service, not per query.

## Anti-patterns

- `SessionContext::new()` in anything that touches user-sized data — the default runtime is unbounded.
- Reading only the constructor. `SessionConfig` carries 64 methods and none of them appear in `new()`.

## Agent checklist

- Is there a memory pool, and does it match whether the operators can spill?
- Is a temp file path set, and is its size capped?
- Are target partitions matched to the deployment rather than left at the core count?
