//! Delta Kernel module
//!
//! The Kernel module contains all the logic for reading and processing the Delta Lake transaction log.

#[cfg(feature = "datafusion")]
use datafusion::common::runtime::SpawnedTask as BlockingTask;
use delta_kernel::engine::arrow_expression::ArrowEvaluationHandler;
use std::sync::{Arc, LazyLock};
#[cfg(not(feature = "datafusion"))]
use tokio::task::JoinHandle as BlockingTask;
use tracing::Span;
use tracing::dispatcher;

pub mod arrow;
pub mod error;
/// Core Delta log action models (Add, Remove, Metadata, Protocol, ...) and related types.
pub mod models;
pub mod scalars;
/// Delta and Arrow schema types, conversions, casting and partition handling.
pub mod schema;
pub(crate) mod snapshot;
pub mod transaction;

pub use arrow::engine_ext::StructDataExt;
pub use delta_kernel::Version;
pub use delta_kernel::engine;
pub use error::*;
pub use models::*;
pub use schema::*;
pub use snapshot::*;

pub(crate) static ARROW_HANDLER: LazyLock<Arc<ArrowEvaluationHandler>> =
    LazyLock::new(|| Arc::new(ArrowEvaluationHandler {}));

pub(crate) fn spawn_blocking_with_span<F, R>(f: F) -> BlockingTask<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // Capture the current dispatcher and span
    let dispatch = dispatcher::get_default(|d| d.clone());
    let span = Span::current();

    let work = move || {
        dispatcher::with_default(&dispatch, || {
            let _enter = span.enter();
            f()
        })
    };
    // The configured DataFusion tracer carries application operation/effect ownership
    // across this kernel boundary. Span propagation alone does not carry task locals.
    #[cfg(feature = "datafusion")]
    {
        BlockingTask::spawn_blocking(work)
    }
    #[cfg(not(feature = "datafusion"))]
    {
        tokio::task::spawn_blocking(work)
    }
}
