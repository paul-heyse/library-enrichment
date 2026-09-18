# Preserve session capabilities and configure Delta planning

Use a Delta-configured session for Delta operations. A concrete SessionState and an arbitrary Session trait wrapper follow different resolution paths; choose fallback policy when caller configuration matters.

Reviewed 2026-09-18. reviewed decision brief; runtime scope is limited to named tests.

## Choice

| Candidate | Choose when | Consideration |
|---|---|---|
| create_session / DeltaSessionContext | You want the supplied Delta planner and defaults | Add caller functions/runtime settings to this state |
| Concrete configured SessionState | Your existing state includes required Delta planning support | Preserves the supplied state, including any incompatible planner |
| DeriveFromTrait | A Session wrapper must contribute runtime/configuration/function registries | Derived state is a documented subset; arbitrary custom planner/catalog behavior is not fully cloned |
| RequireSessionState | Silently replacing a trait wrapper would violate caller requirements | Rejects wrappers; make the desired concrete state explicit |

## Contract

**default.** InternalDefaults is the default fallback for a non-SessionState wrapper. A caller UDF can disappear; DeriveFromTrait preserves the tested UDF while RequireSessionState rejects the wrapper.
Claim `delta.session.1`; runtime_observation; evidence: runtime.

**planner.** A plain SessionContext state passed to the logical-plan write probe fails to plan MetricObserver. The same plan with create_session().state() writes successfully.
Claim `delta.session.2`; runtime_observation; evidence: runtime.

**resources.** create_session_state_with_spill_config(None,None) uses the default runtime. The named max_spill_size configures the FairSpillPool memory budget, while max_temp_directory_size bounds temporary disk; these are separate resources.
Claim `delta.session.3`; source_observation; evidence: upstream, source.

**resource boundary.** A configured 1024-byte FairSpillPool accepted a 512-byte reservation, rejected another 1024 bytes and released the reservation on drop. This tests pool configuration, not total operation/process RSS or automatic spilling.
Claim `delta.session.4`; runtime_observation; evidence: runtime.

## Implementation

- Choose policy based on required caller capabilities, then test with a distinguishing UDF/configuration rather than state type alone.
- Do not reuse an arbitrary session solely to avoid construction cost; verify planner, store mappings and runtime sharing semantics.
- Record runtime limits separately from batch/target-file-size options.

## Effects

- configure planner and runtime
- share or derive session state

## Errors

- Resolution rejection and missing functions/planner failures occur before a successful commit; verify table state when diagnosing a failed operation.

## Limits and unknowns

- The probes establish UDF preservation, not equivalence of every Session implementation or process memory bounds.

## Exact contracts

- [`deltalake_core::delta_datafusion::session::create_session`](../operations/deltalake_core.delta_datafusion.session.create_session.md#op-45c70950a9bc56476e6885e2) — `fn create_session() -> DeltaSessionContext`
- [`deltalake_core::delta_datafusion::session::SessionFallbackPolicy`](../operations/deltalake_core.delta_datafusion.session.SessionFallbackPolicy.md#op-6763b79544161b4d7375c648) — `enum SessionFallbackPolicy`
- [`deltalake_core::delta_datafusion::session::DeltaRuntimeEnvBuilder::with_max_spill_size`](../operations/deltalake_core.delta_datafusion.session.DeltaRuntimeEnvBuilder.md#op-5c3b599ea49399c9b847c098) — `fn with_max_spill_size(self, size: usize) -> Self`
- [`deltalake_core::delta_datafusion::session::create_session_state_with_spill_config`](../operations/deltalake_core.delta_datafusion.session.create_session_state_with_spill_config.md#op-6718e986726f6ba946bb0cd3) — `fn create_session_state_with_spill_config(max_spill_size: Option<usize>, max_temp_directory_size: Option<u64>) -> datafusion::execution::SessionState`
- [`deltalake_core::operations::write::WriteBuilder::with_session_state`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-c287ad75f18e28c8f06ebb86) — `fn with_session_state(self, session: Arc<dyn Session>) -> Self`
- [`deltalake_core::operations::write::WriteBuilder::with_session_fallback_policy`](../operations/deltalake_core.operations.write.WriteBuilder.md#op-b97917aa9ff6c7d66ed99cbd) — `fn with_session_fallback_policy(self, policy: SessionFallbackPolicy) -> Self`

## Evidence

`upstream` refers to the exact contracts above, preserving source spans and raw types.
- [source](../../skill_improvement/evidence/sources/delta-rs/crates/core/src/delta_datafusion/session.rs): Pinned delta-rs implementation; full file retained with acquisition provenance
  Tests: 
- [runtime](../../skill_improvement/evidence/implementation/probe-results.json): Named local assertions only; see profile, fixtures and source digests
  Tests: session_policies_preserve_discard_or_reject_wrapper_udf, logical_plan_write_and_staged_writer_visibility, configured_memory_pool_enforces_reservation_budget
