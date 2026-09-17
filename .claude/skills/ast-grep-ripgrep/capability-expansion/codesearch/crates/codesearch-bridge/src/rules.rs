//! Plan-time rules: what the engine refuses to run.
//!
//! One rule lives here. A second was designed and is deliberately **not** here, and the reason is
//! recorded below, because "we decided not to" is a fact a later reader needs as much as the rule
//! itself.
//!
//! # `IdentityDiscipline`
//!
//! §1.1 gives an entity three kinds of name, and two of them are join keys with different scopes:
//! `entity_key` is stable across snapshots, `entity_id` is `blake3(entity_key ‖ snapshot_id)` and
//! therefore is not. Joining one to the other is always wrong, and it fails in the worst possible
//! way -- **zero rows, no error**. A caller reads that as "nothing matched", which is a statement
//! about the data rather than about the query.
//!
//! The columns are already distinguishable at plan time. §1.4 stamps `ARROW:extension:name` on
//! both, and that the metadata survives all the way to the *logical plan schema* is measured three
//! times over rather than assumed:
//!
//! - a recursive CTE refused to plan because one term's `recall_rank` carried
//!   `codesearch.lattice` and the other's bare `CASE` did not -- found by something failing, which
//!   is the only way to learn this without trusting a doc;
//! - `the_run_filter_view_preserves_extension_metadata` proves a view over a provider keeps it;
//! - `a_lazily_resolved_view_preserves_extension_metadata` proves the on-demand `ViewTable` does
//!   too.
//!
//! Each of those carries a control column that must **not** have the metadata, so none of them can
//! pass by the metadata being universally present.
//!
//! # `LatticePropagation` is not here, and that is the decision
//!
//! §5.3 specifies a second rule: reject any projection that joins a parent to children without
//! folding each lattice. It is not implemented, and it should not be. Its trigger is
//! "joins a parent to children", and **parent-child is not decidable from a logical plan** -- the
//! plan has joins, not cardinalities or ownership. A version conservative enough not to block
//! legitimate queries catches almost nothing; one aggressive enough to catch something blocks
//! them.
//!
//! The population it was meant to police is enumerable: [`crate::projection::PROJECTIONS`]. So it
//! is a test over that list instead (`model/tests/projections.rs`), which gives the same guarantee
//! against a known set, as a red test rather than a rejected query.

use std::sync::Arc;

use datafusion::common::config::ConfigOptions;
use datafusion::common::tree_node::{TreeNode, TreeNodeRecursion};
use datafusion::common::{DFSchema, DataFusionError, Result as DFResult};
use datafusion::logical_expr::expr_schema::ExprSchemable;
use datafusion::logical_expr::{Expr, LogicalPlan, Operator};
use datafusion::optimizer::analyzer::AnalyzerRule;

use crate::identity::{EXT_ENTITY_ID, EXT_ENTITY_KEY};

/// The Arrow metadata key carrying an extension type's name.
const EXT_NAME: &str = "ARROW:extension:name";

/// Reject a join that equates an `entity_key` with an `entity_id`.
///
/// Narrow on purpose. It inspects only the equijoin pairs of a `Join` node, only when both sides
/// carry an identity extension type, and only when they disagree. Everything else -- a join on two
/// keys, a join on two ids, a join on columns carrying no extension type at all -- passes
/// untouched, so the rule cannot misfire on a legitimate query.
#[derive(Debug, Default)]
pub struct IdentityDiscipline;

impl IdentityDiscipline {
    pub fn new() -> Self {
        Self
    }
}

/// The identity extension name on the field an expression resolves to, if it has one.
///
/// Returns `None` rather than an error when the expression does not resolve: a rule that refused
/// plans it could not analyse would be a rule that rejects queries for being unusual, which is
/// exactly the misfire this one is designed not to have.
fn identity_of(expr: &Expr, schema: &DFSchema) -> Option<String> {
    let (_, field) = expr.to_field(schema).ok()?;
    let name = field.metadata().get(EXT_NAME)?;
    (name == EXT_ENTITY_KEY || name == EXT_ENTITY_ID).then(|| name.clone())
}

/// The refusal, worded so a reader learns the rule rather than only that they broke one.
fn refuse(left: &Expr, l: &str, right: &Expr, r: &str) -> DataFusionError {
    DataFusionError::Plan(format!(
        "join equates `{left}` ({l}) with `{right}` ({r}). An entity_id is \
         blake3(entity_key ‖ snapshot_id), so the two are equal for no entity in any snapshot and \
         this join returns zero rows without failing -- which reads as an answer about the data. \
         Join key to key, or id to id; `compare` is the one projection that crosses snapshots, and \
         it joins on entity_key."
    ))
}

/// Check one equality, whichever side of the join each operand came from.
fn check_pair(left: &Expr, right: &Expr, schema: &DFSchema) -> DFResult<()> {
    match (identity_of(left, schema), identity_of(right, schema)) {
        (Some(l), Some(r)) if l != r => Err(refuse(left, &l, right, &r)),
        _ => Ok(()),
    }
}

impl AnalyzerRule for IdentityDiscipline {
    fn analyze(&self, plan: LogicalPlan, _config: &ConfigOptions) -> DFResult<LogicalPlan> {
        // `apply` rather than `transform`: this rule reads the plan and refuses, it never rewrites
        // one. A rule that silently corrected the join would hide the mistake it exists to expose.
        plan.apply(|node| {
            let LogicalPlan::Join(join) = node else {
                return Ok(TreeNodeRecursion::Continue);
            };

            // Both operands are resolved against the two inputs merged, and both `on` and
            // `filter` are inspected, because **an analyzer rule runs before the optimizer**. A
            // SQL `JOIN ... ON a = b` reaches analysis with the equality in `filter`; it is
            // `ExtractEquijoinPredicate`, an optimizer rule, that later moves it into `on`.
            // Reading only `on` would have made this rule fire on nothing that came from SQL --
            // which is exactly how it first behaved.
            let schema = join.left.schema().join(join.right.schema())?;

            for (left, right) in &join.on {
                check_pair(left, right, &schema)?;
            }
            if let Some(filter) = &join.filter {
                filter.apply(|e| {
                    if let Expr::BinaryExpr(binary) = e
                        && binary.op == Operator::Eq
                    {
                        check_pair(&binary.left, &binary.right, &schema)?;
                    }
                    Ok(TreeNodeRecursion::Continue)
                })?;
            }
            Ok(TreeNodeRecursion::Continue)
        })?;
        Ok(plan)
    }

    fn name(&self) -> &str {
        "codesearch_identity_discipline"
    }
}

/// Every plan-time rule this crate installs, as data rather than as a sequence of calls.
///
/// The same argument as [`crate::session::udfs`]: a rule registered without being listed here
/// would not be on the session, so the published inventory cannot drift from what runs.
pub fn analyzer_rules() -> Vec<Arc<dyn AnalyzerRule + Send + Sync>> {
    vec![Arc::new(IdentityDiscipline::new())]
}
