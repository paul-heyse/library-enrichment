# Choose object storage and log publication adapters

Storage configuration, object access and log publication are distinct seams. Route by URL/service and compiled features, then inspect the adapter's options and registration requirements.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| Built-in backend crate | Its supported scheme/service matches deployment | Credentials, handler registration and native environment remain prerequisites |
| Explicit custom ObjectStore/LogStore integration | You have a concrete nonstandard storage need | Match root URL/path scoping and transactional publication semantics |
| OpenDAL adapter | An enabled OpenDAL service fits the required backend | The opendal+ scheme and compiled service set prevent assuming every service is available |

## Contract

**registration.** Session store-registration helpers preserve an existing mapping rather than replacing it. A stale mapping requires an explicit RuntimeEnv::register_object_store override; check-and-register is not an atomic concurrency guarantee.
Claim `delta.storage.1`; source_observation; evidence: upstream, source.

**backend.** LogStore delegates commit publication to backend-specific behavior. Local-memory/filesystem probes do not prove S3/Azure/GCS atomicity, auth or listing semantics.
Claim `delta.storage.2`; source_observation; evidence: upstream, source.

**features.** Use the complete crate/feature routes for AWS, Azure, GCP, HDFS, LakeFS, mount and OpenDAL; documentation inclusion is separate from a successfully deployed backend.
Claim `delta.storage.3`; source_observation; evidence: upstream, source.

**registration.** An existing wrong runtime store mapping survived provider execution and caused a read error. Explicitly replacing it with the table root store restored the expected row.
Claim `delta.storage.4`; runtime_observation; evidence: runtime.

## Implementation

- Resolve scheme, backend features, authentication and root-prefix behavior before constructing the table.
- Keep secrets out of retained evidence; retain option names and sanitized outcomes.
- Check object_store and DataFusion crate identities so trait objects belong to the same dependency graph.

## Effects

- remote or local object I/O
- configure backend factories
- publish log through selected backend

## Errors

- Unsupported scheme, absent feature/handler, credentials, network/native dependency and conflict errors are separate prerequisites.

## Limits and unknowns

- Cloud/native-service integration is not_run without its environment. No backend-wide durability or concurrency claim is made.

## Exact contracts

- [`deltalake_core::table::builder::DeltaTableBuilder::with_storage_options`](../operations/deltalake_core.table.builder.DeltaTableBuilder.md#op-b1ea50678ecca859dcd514a2) — `fn with_storage_options(self, storage_options: HashMap<String, String>) -> Self`
- [`deltalake_aws::register_handlers`](../operations/deltalake_aws.register_handlers.md#op-1bc1cda4486cb01abc5673f3) — `fn register_handlers(_additional_prefixes: Option<url::Url>)`
- [`deltalake_azure::register_handlers`](../operations/deltalake_azure.register_handlers.md#op-85561ec0f6e032ca2cd5b0d2) — `fn register_handlers(_additional_prefixes: Option<url::Url>)`
- [`deltalake_gcp::register_handlers`](../operations/deltalake_gcp.register_handlers.md#op-26254cbad8b6bfbf79bded73) — `fn register_handlers(_additional_prefixes: Option<url::Url>)`
- [`deltalake_opendal::register_handlers`](../operations/deltalake_opendal.register_handlers.md#op-a3b6ac60139298c49c68eb4f) — `fn register_handlers(_additional_prefixes: Option<url::Url>)`
- [`deltalake_core::logstore::LogStore`](../operations/deltalake_core.logstore.LogStore.md#op-05065cf369d14ac5f8039639) — `trait LogStore: Send + Sync + AsAny`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/delta_datafusion/session.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only
  Tests: existing_object_store_mapping_needs_explicit_replacement
