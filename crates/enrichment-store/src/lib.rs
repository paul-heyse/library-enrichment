//! Native DataFusion execution, Delta control state and service-owned evidence/artifacts.
//!
//! ADR-0041–0045 define the target publication boundary. Control decisions commit through
//! complete Delta builders with captured native sessions and explicit semantic contracts.
//!
//! Every function here takes an explicit root. Resolving the roots from the environment happens
//! once, in the daemon binary, through [`paths::StatePaths::from_env`]; library code and tests
//! never read `LIBENR_*` themselves, so a parallel test run cannot race on a shared directory.

pub mod admission;
mod admitted_provider;
mod arrow_contract;
mod arrow_input;
pub mod artifact_catalog;
pub mod artifact_selection;
pub mod atomic;
mod attempt_plan;
pub mod availability;
pub mod blob;
pub mod browse;
pub mod bundle;
mod bundle_plan;
pub mod comparison;
pub mod comparison_context;
mod comparison_page;
pub mod comparison_policy;
mod contract_cache;
pub mod control;
pub mod control_jobs;
pub mod coverage;
mod coverage_plan;
pub mod dataset;
mod delta_cohort;
pub mod delta_evidence;
pub mod dependency_plan;
pub mod documentation_plan;
mod durable_store;
pub mod execution_policy;
mod field_admission;
pub mod http_cache;
pub mod immutable_root;
pub mod ingest;
mod private_directory;
pub use private_directory::PrivateDirectory;
mod invariants;
mod kernel_metrics;
mod kernel_runtime;
pub mod leases;
mod native_cache;
mod native_catalog;
pub mod native_delta;
pub mod native_discovery;
pub mod native_effect;
mod native_policy;
mod native_process;
mod native_rows;
pub mod native_rustdoc;
pub mod network_policy;
mod operation_index;
mod owned_batch;
pub mod owned_bytes;
mod owned_tree;
pub mod paths;
pub mod preparation;
pub mod projection;
mod provider;
mod provider_cache;
mod provider_replay;
mod publication_plan;
pub mod publication_probe;
pub mod query;
pub mod query_diagnostics;
pub mod query_failure;
mod read_admission;
pub mod repository;
pub mod resolution_policy;
pub mod result;
pub mod result_catalog;
pub mod result_delivery;
#[cfg(test)]
mod result_measure;
pub mod result_plan;
mod result_relations;
pub mod retention;
mod retention_tasks;
mod root_removal;
pub mod runtime;
pub mod scoring;
pub mod search_plan;
pub mod search_policy;
pub mod search_projection;
mod semantic;
#[cfg(test)]
mod semantic_contract_tests;
pub mod semantic_scope;
mod session_witness;
mod snapshot_registry;
mod snapshot_validation;
pub mod state;
pub mod status_plan;
mod task_context;
pub mod views;

pub use blob::{BlobStore, StoredBlob};
pub use paths::{StatePathError, StatePaths};
pub use query::{Overview, QueryError, SnapshotReader};

mod execution_documents;

pub mod python_normalize;
pub mod python_registry;
pub mod record_writer;
pub mod registry;
pub mod registry_capture;
pub mod revision_capture;
pub mod rust_normalize;
mod rustdoc_decoder_plan;

#[doc(hidden)]
pub mod native_worker;

pub mod inspection_execution_plan;
mod operation_policy;
pub mod request_admission;
pub mod research_citations;
pub mod research_fragments;
pub mod research_inspection;
pub mod research_outcomes;
pub mod research_overview;
pub mod research_resolution;
pub mod research_selection;
pub mod research_source;
pub mod source_capture;
pub mod static_worker;

pub mod operation_policies;

pub mod immutable_definitions;

pub mod capsule_input_plan;
pub mod environment_plan;
pub mod execution_fact_plan;
pub mod job_recovery;
pub mod preparation_plan;
pub mod probe_plan;
pub mod process_grants;
pub mod producer_plan;
pub mod producer_run_plan;
pub mod python_distribution;
pub mod runtime_object_plan;
pub mod semantic_grants;
pub mod semantic_response_plan;

mod telemetry_history;

pub mod physical_ownership;

mod prepared_capsule;

pub mod semantic_source_plan;

pub mod page_plan;
