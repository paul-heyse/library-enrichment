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
pub mod atomic;
mod attempt_plan;
pub mod blob;
pub mod browse;
pub mod bundle;
pub mod comparison;
pub mod control;
pub mod control_jobs;
pub mod coverage;
mod coverage_plan;
pub mod dataset;
mod delta_cohort;
pub mod delta_evidence;
pub mod dependency_plan;
mod durable_store;
pub mod execution_policy;
pub mod http_cache;
pub mod ingest;
pub mod leases;
mod native_catalog;
pub mod native_delta;
pub mod native_effect;
mod native_policy;
pub mod native_rustdoc;
pub mod network_policy;
mod operation_index;
pub mod paths;
pub mod preparation;
pub mod projection;
mod provider;
mod publication_plan;
pub mod publication_probe;
pub mod query;
pub mod query_diagnostics;
pub mod query_failure;
pub mod repository;
pub mod resolution_policy;
pub mod result;
pub mod result_catalog;
pub mod runtime;
pub mod scoring;
pub mod search_plan;
pub mod search_projection;
mod semantic;
pub mod semantic_scope;
pub mod state;
pub mod views;

pub use blob::{BlobStore, StoredBlob};
pub use paths::{StatePathError, StatePaths};
pub use query::{Overview, QueryError, SnapshotReader};

mod execution_documents;

pub mod python_normalize;
pub mod python_registry;
pub mod record_writer;
pub mod registry;
pub mod rust_normalize;

#[doc(hidden)]
pub mod native_worker;

mod operation_policy;

pub mod operation_policies;

pub mod immutable_definitions;

pub mod process_grants;

mod telemetry_history;
