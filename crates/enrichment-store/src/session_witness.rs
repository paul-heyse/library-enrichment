//! Captured effective session meaning. SessionConfig mutates options with Arc::make_mut,
//! so retaining its Arc makes unchanged clones a safe allocation-free reuse path.
use datafusion::{
    catalog::Session,
    common::config::ConfigOptions,
    execution::SessionState,
    logical_expr::{AggregateUDF, HigherOrderUDF, ScalarUDF, WindowUDF},
};
use std::{
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Function {
    Scalar(Arc<ScalarUDF>),
    Aggregate(Arc<AggregateUDF>),
    Window(Arc<WindowUDF>),
    HigherOrder(Arc<HigherOrderUDF>),
}
#[derive(Debug)]
pub(crate) struct Witness {
    options: Arc<ConfigOptions>,
    values: Vec<(String, Option<String>)>,
    functions: Vec<(String, Function)>,
}
impl PartialEq for Witness {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.functions == other.functions
    }
}
impl Eq for Witness {}
impl Hash for Witness {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.values.hash(state);
        self.functions.hash(state);
    }
}
impl Witness {
    /// Persistence is restricted to the compiled session template. A dynamically
    /// replaced function cannot be proven by its name or serialized debug output.
    pub(crate) fn persisted_options(
        session: &dyn Session,
    ) -> datafusion::common::Result<std::collections::BTreeMap<String, Option<String>>> {
        let witness = session
            .config()
            .get_extension::<Self>()
            .filter(|witness| witness.unchanged(session))
            .ok_or_else(|| {
                datafusion::common::plan_datafusion_err!(
                    "persisted provider requires the unchanged compiled session template"
                )
            })?;
        Ok(witness.values.iter().cloned().collect())
    }
    fn capture(session: &dyn Session) -> Self {
        let options = session.config().options().clone();
        let mut values = options
            .entries()
            .into_iter()
            .map(|entry| (entry.key, entry.value))
            .collect::<Vec<_>>();
        values.sort();
        let mut functions = session
            .scalar_functions()
            .iter()
            .map(|(name, function)| (name.clone(), Function::Scalar(function.clone())))
            .chain(
                session
                    .aggregate_functions()
                    .iter()
                    .map(|(name, function)| (name.clone(), Function::Aggregate(function.clone()))),
            )
            .chain(
                session
                    .window_functions()
                    .iter()
                    .map(|(name, function)| (name.clone(), Function::Window(function.clone()))),
            )
            .chain(
                session
                    .higher_order_functions()
                    .iter()
                    .map(|(name, function)| {
                        (name.clone(), Function::HigherOrder(function.clone()))
                    }),
            )
            .collect::<Vec<_>>();
        functions.sort_by(|(a, _), (b, _)| a.cmp(b));
        Self {
            options,
            values,
            functions,
        }
    }
    fn unchanged(&self, session: &dyn Session) -> bool {
        Arc::ptr_eq(&self.options, session.config().options())
            && self.functions.len()
                == session.scalar_functions().len()
                    + session.aggregate_functions().len()
                    + session.window_functions().len()
                    + session.higher_order_functions().len()
            && self
                .functions
                .iter()
                .all(|(name, function)| match function {
                    Function::Scalar(value) => session.scalar_functions().get(name) == Some(value),
                    Function::Aggregate(value) => {
                        session.aggregate_functions().get(name) == Some(value)
                    }
                    Function::Window(value) => session.window_functions().get(name) == Some(value),
                    Function::HigherOrder(value) => {
                        session.higher_order_functions().get(name) == Some(value)
                    }
                })
    }
    pub(crate) fn get(session: &dyn Session) -> Arc<Self> {
        if let Some(witness) = session.config().get_extension::<Self>()
            && witness.unchanged(session)
        {
            return witness;
        }
        Arc::new(Self::capture(session))
    }
    pub(crate) fn bind(state: SessionState) -> SessionState {
        let witness = Arc::new(Self::capture(&state));
        let config = state.config().clone().with_extension(witness);
        datafusion::execution::SessionStateBuilder::new_from_existing(state)
            .with_config(config)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn plan19_session_witness_reuses_clones_and_detects_cow_changes() {
        let context = datafusion::prelude::SessionContext::new();
        let state = Witness::bind(context.state());
        let first = Witness::get(&state);
        let clone = state.clone();
        assert!(Arc::ptr_eq(&first, &Witness::get(&clone)));
        let mut changed = clone;
        changed
            .config_mut()
            .options_mut()
            .execution
            .parquet
            .pushdown_filters = true;
        assert_ne!(first, Witness::get(&changed));
        assert!(Arc::ptr_eq(&first, &Witness::get(&state)));
    }
    #[tokio::test]
    async fn plan19_cursor_witness_captures_runtime_options_resources_and_policy()
    -> datafusion::common::Result<()> {
        use crate::runtime::{QueryLimits, QueryRuntime};
        use enrichment_core::operation::selections::{ComparisonSelection, SelectionWitness};
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&root.path().join("first"), QueryLimits::default())?;
        let cloned = runtime.clone();
        assert!(std::ptr::eq(
            runtime.selection_witness(),
            cloned.selection_witness()
        ));
        let changed = QueryRuntime::new(
            &root.path().join("changed"),
            QueryLimits {
                partitions: 2,
                ..Default::default()
            },
        )?;
        assert_ne!(runtime.selection_witness().1, changed.selection_witness().1);
        assert_ne!(
            runtime.selection_witness().0.options,
            changed.selection_witness().0.options
        );
        let mut selection = ComparisonSelection {
            witness: SelectionWitness {
                runtime: runtime.selection_witness().1.clone(),
                policy: enrichment_core::identity::OperationPolicyId::from_record(
                    &enrichment_core::config::Config::default(),
                ),
            },
            scopes: vec![enrichment_core::compare::Scope::Api],
            max_items: Some(1),
            max_bytes: 2048,
        };
        let first = selection.identity();
        selection.witness.policy = format!("policy_{}", "2".repeat(64)).try_into().unwrap();
        assert_ne!(first, selection.identity());
        selection.witness.policy = enrichment_core::identity::OperationPolicyId::from_record(
            &enrichment_core::config::Config::default(),
        );
        selection.witness.runtime = changed.selection_witness().1.clone();
        assert_ne!(first, selection.identity());
        let snapshots = enrichment_core::compare::page::SnapshotPair {
            before: format!("snap_{}", "a".repeat(64)).try_into().unwrap(),
            after: format!("snap_{}", "b".repeat(64)).try_into().unwrap(),
        };
        let cursor = enrichment_core::compare::page::AlternativeCursor::encode(
            enrichment_core::compare::page::ComparisonKey {
                plan: 0,
                subject: "subject".into(),
                key: "key".into(),
            },
            true,
            1,
            &snapshots,
            &first,
        )
        .unwrap();
        assert!(
            enrichment_core::compare::page::AlternativeCursor::decode(&cursor, &snapshots, &first)
                .is_ok()
        );
        assert!(
            enrichment_core::compare::page::AlternativeCursor::decode(
                &cursor,
                &snapshots,
                &selection.identity()
            )
            .is_err()
        );
        Ok(())
    }
}
