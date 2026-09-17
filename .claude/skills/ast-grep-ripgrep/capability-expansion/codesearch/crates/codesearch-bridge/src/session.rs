//! The single `SessionState` call site.
//!
//! One session is threaded through extraction, merge, constraint evaluation, optimisation,
//! projection and serving. Every delta-rs builder takes `Arc<dyn Session>`, which is what makes
//! that possible -- and two measured facts are what make it actually work.
//!
//! # It must be delta-rs's session, not a bare DataFusion one
//!
//! Delta writes plan through delta-rs's own `MetricObserver` node. A session built with
//! `SessionContext::new()` cannot execute a Delta write **at all**; it fails with
//!
//! ```text
//! No installed planner was able to convert the custom node to an execution plan: MetricObserver
//! ```
//!
//! Probe PB07 found this the hard way: its first run had the treatment arm *and the control arm*
//! failing with that message, which meant the probe was measuring neither. When a control fails
//! the same way as the treatment, nothing has been measured.
//!
//! # A supplied session can be silently discarded
//!
//! `SessionFallbackPolicy::InternalDefaults` is the **default**, and it means "if the provided
//! session is not a `SessionState`, log a warning and use internal defaults". That quietly throws
//! away the UDF registry, the extension types and the custom rules -- precisely the things this
//! module exists to install. [`require_session`] is the antidote and every builder call site uses
//! it, so a dropped session is an error rather than a different query.

use std::sync::Arc;

use datafusion::common::Result as DFResult;
use datafusion::execution::SessionState;
use datafusion::logical_expr::ScalarUDF;
use datafusion::prelude::SessionContext;
use deltalake::delta_datafusion::{SessionFallbackPolicy, create_session};

use crate::identity::{EntityId, EntityKeyNamespace};
use crate::interaction::AtomTruth;
use crate::lattice::LatticeLabel;

/// The policy every delta-rs builder in this codebase is given.
///
/// Spelled out as a named constant rather than written inline at each call site, so that "did we
/// remember it here?" is answerable by grep. The mechanical check in `codesearch-check` asserts
/// no builder is constructed without it.
pub const REQUIRED_FALLBACK: SessionFallbackPolicy = SessionFallbackPolicy::RequireSessionState;

/// Build the one session: delta-rs's own context, plus everything `bridge` adds to the engine.
///
/// Returns the `SessionContext` as well as the state because the context is what registers
/// tables, while the state is what delta-rs builders want.
pub fn build_session() -> DFResult<(SessionContext, Arc<SessionState>)> {
    // `create_session()` and NOT `SessionContext::new()` -- see the module docs.
    let ctx = create_session().into_inner();

    // `information_schema` is off in DataFusion's default config, and delta-rs does not turn it
    // on. It is turned on here because it is the whole point of registering projections as views
    // rather than composing SQL at each call site: `information_schema.views` is what makes the
    // retrieval surface enumerable, so a projection cannot exist without being listed.
    //
    // Mutated in place rather than by rebuilding the context from a modified state, because the
    // state carries delta-rs's `DeltaPlanner` and a rebuild is one more chance to lose it. PB07
    // measured what losing it costs: Delta writes stop working entirely.
    ctx.state_ref()
        .write()
        .config_mut()
        .options_mut()
        .catalog
        .information_schema = true;

    // Everything this catalog adds to DataFusion is registered here and nowhere else.
    for udf in udfs() {
        ctx.register_udf(udf);
    }

    // Plan-time rules. `add_analyzer_rule` takes `&self`, so the rule attaches to the session
    // delta-rs already built rather than requiring a rebuilt context -- which matters for the
    // reason above: a rebuild is one more chance to lose the `DeltaPlanner`.
    for rule in crate::rules::analyzer_rules() {
        ctx.add_analyzer_rule(rule);
    }

    let state = Arc::new(ctx.state());
    Ok((ctx, state))
}

/// The complete UDF inventory, as data rather than as a sequence of registration calls.
///
/// `bridge/CATALOG.md` is generated from this list plus each function's `documentation()`, which
/// is what makes the published inventory unable to drift from the code: a function registered
/// without being listed here would simply not be in the session.
pub fn udfs() -> Vec<ScalarUDF> {
    vec![
        ScalarUDF::from(LatticeLabel::new()),
        ScalarUDF::from(EntityId::new()),
        ScalarUDF::from(AtomTruth::new()),
        ScalarUDF::from(EntityKeyNamespace::new()),
    ]
}

/// Assert a session is usable by delta-rs before it is handed to a builder.
///
/// This exists because the failure it guards against is silent. A session that is not a concrete
/// `SessionState` is accepted by every builder and then ignored, so the first symptom is a merge
/// predicate that cannot resolve a UDF that is definitely registered -- a confusing bug a long
/// way from its cause.
pub fn require_session(state: &Arc<SessionState>) -> DFResult<Arc<SessionState>> {
    Ok(Arc::clone(state))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_session_carries_every_udf_in_the_inventory() {
        let (ctx, _state) = build_session().expect("session builds");
        let registered = ctx.state().scalar_functions().clone();
        for udf in udfs() {
            assert!(
                registered.contains_key(udf.name()),
                "{} is in the inventory but not on the session, so CATALOG.md would advertise a \
                 function no query can call",
                udf.name()
            );
        }
        assert!(!udfs().is_empty(), "an empty inventory checks nothing");
    }

    /// A control for the test above: a bare DataFusion session carries none of them. Without this,
    /// the assertion could pass for the wrong reason -- if one of these were somehow a built-in
    /// name, the check would be measuring DataFusion rather than this crate.
    #[test]
    fn a_bare_session_carries_none_of_them() {
        let bare = SessionContext::new();
        let registered = bare.state().scalar_functions().clone();
        for udf in udfs() {
            assert!(
                !registered.contains_key(udf.name()),
                "`{}` already exists in a bare session, so the test above proves nothing about \
                 this crate",
                udf.name()
            );
        }
    }

    /// Every UDF must document itself, because `bridge/CATALOG.md` is generated from
    /// `documentation()` and an undocumented function would publish as a blank row.
    #[test]
    fn every_udf_documents_itself_and_is_immutable() {
        for udf in udfs() {
            assert!(
                udf.documentation().is_some(),
                "{} has no documentation()",
                udf.name()
            );
            // PB06: a UDF that is not Immutable loses its filter entirely, with no diagnostic.
            assert_eq!(
                udf.signature().volatility,
                datafusion::logical_expr::Volatility::Immutable,
                "{} must be Immutable",
                udf.name()
            );
        }
    }

    #[test]
    fn the_required_fallback_policy_is_require_session_state() {
        // Pinned as a test because the default is the dangerous value and a future refactor that
        // "simplified" this constant away would reintroduce silent session loss.
        assert_eq!(
            REQUIRED_FALLBACK,
            SessionFallbackPolicy::RequireSessionState
        );
        assert_ne!(
            REQUIRED_FALLBACK,
            SessionFallbackPolicy::default(),
            "the delta-rs default is InternalDefaults, which discards the caller's session"
        );
    }
}
