// Adapter and TestScalarUdf derived from pinned delta-rs test_utils/datafusion.rs.
// Upstream license: ../fixtures/LICENSE.txt; exact source retained in evidence/sources.
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use datafusion::catalog::CatalogProviderList;
use datafusion::catalog::{EmptyCatalogProviderList, Session as DataFusionSession};
use datafusion::common::{DFSchema, DataFusionError, ScalarValue};
use datafusion::config::TableOptions;
use datafusion::error::Result as DataFusionResult;
use datafusion::execution::TaskContext;
use datafusion::execution::runtime_env::RuntimeEnv;
use datafusion::execution::session_state::SessionState;
use datafusion::logical_expr::execution_props::ExecutionProps;
use datafusion::logical_expr::registry::{ExtensionTypeRegistryRef, FunctionRegistry};
use datafusion::logical_expr::{
    AggregateUDF, ColumnarValue, Expr, HigherOrderUDF, LogicalPlan, ScalarFunctionArgs, ScalarUDF,
    ScalarUDFImpl, Signature, TypeSignature, Volatility, WindowUDF,
};
use datafusion::physical_expr::PhysicalExpr;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::prelude::SessionConfig;

pub(crate) fn make_test_scalar_udf(name: &'static str) -> Arc<ScalarUDF> {
    Arc::new(ScalarUDF::from(TestScalarUdf { name }))
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct TestScalarUdf {
    name: &'static str,
}

impl ScalarUDFImpl for TestScalarUdf {
    fn name(&self) -> &str {
        self.name
    }

    fn signature(&self) -> &Signature {
        static SIGNATURE: std::sync::LazyLock<Signature> = std::sync::LazyLock::new(|| Signature {
            type_signature: TypeSignature::VariadicAny,
            volatility: Volatility::Immutable,
            parameter_names: Some(vec![]),
        });
        &SIGNATURE
    }

    fn return_type(
        &self,
        _arg_types: &[arrow_schema::DataType],
    ) -> DataFusionResult<arrow_schema::DataType> {
        Ok(arrow_schema::DataType::Int32)
    }

    fn invoke_with_args(&self, _args: ScalarFunctionArgs) -> DataFusionResult<ColumnarValue> {
        Ok(ColumnarValue::Scalar(ScalarValue::Int32(Some(1))))
    }
}

pub(crate) struct WrapperSession {
    inner: SessionState,
    planning_error: Option<&'static str>,
}

impl WrapperSession {
    pub(crate) fn new(inner: SessionState) -> Self {
        Self {
            inner,
            planning_error: None,
        }
    }
}

#[async_trait]
impl DataFusionSession for WrapperSession {
    fn catalog_list(&self) -> Arc<dyn CatalogProviderList> {
        Arc::new(EmptyCatalogProviderList)
    }

    fn session_id(&self) -> &str {
        DataFusionSession::session_id(&self.inner)
    }

    fn config(&self) -> &SessionConfig {
        DataFusionSession::config(&self.inner)
    }

    async fn create_physical_plan(
        &self,
        logical_plan: &LogicalPlan,
    ) -> DataFusionResult<Arc<dyn ExecutionPlan>> {
        if let Some(message) = self.planning_error {
            return Err(DataFusionError::Plan(message.to_string()));
        }
        DataFusionSession::create_physical_plan(&self.inner, logical_plan).await
    }

    fn create_physical_expr(
        &self,
        expr: Expr,
        df_schema: &DFSchema,
    ) -> DataFusionResult<Arc<dyn PhysicalExpr>> {
        if let Some(message) = self.planning_error {
            return Err(DataFusionError::Plan(message.to_string()));
        }
        DataFusionSession::create_physical_expr(&self.inner, expr, df_schema)
    }

    fn scalar_functions(&self) -> &HashMap<String, Arc<ScalarUDF>> {
        DataFusionSession::scalar_functions(&self.inner)
    }

    fn higher_order_functions(&self) -> &HashMap<String, Arc<HigherOrderUDF>> {
        DataFusionSession::higher_order_functions(&self.inner)
    }

    fn aggregate_functions(&self) -> &HashMap<String, Arc<AggregateUDF>> {
        DataFusionSession::aggregate_functions(&self.inner)
    }

    fn window_functions(&self) -> &HashMap<String, Arc<WindowUDF>> {
        DataFusionSession::window_functions(&self.inner)
    }

    fn extension_type_registry(&self) -> &ExtensionTypeRegistryRef {
        DataFusionSession::extension_type_registry(&self.inner)
    }

    fn runtime_env(&self) -> &Arc<RuntimeEnv> {
        DataFusionSession::runtime_env(&self.inner)
    }

    fn execution_props(&self) -> &ExecutionProps {
        DataFusionSession::execution_props(&self.inner)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn table_options(&self) -> &TableOptions {
        DataFusionSession::table_options(&self.inner)
    }

    fn table_options_mut(&mut self) -> &mut TableOptions {
        DataFusionSession::table_options_mut(&mut self.inner)
    }

    fn task_ctx(&self) -> Arc<TaskContext> {
        DataFusionSession::task_ctx(&self.inner)
    }
}
#[tokio::test]
async fn session_policies_preserve_discard_or_reject_wrapper_udf() {
    use arrow_array::{Int32Array, RecordBatch};
    use arrow_schema::{DataType, Field, Schema};
    use deltalake::delta_datafusion::{SessionFallbackPolicy, create_session};
    use futures::TryStreamExt;
    let session = create_session();
    let mut state = session.state();
    state
        .register_udf(make_test_scalar_udf("policy_udf"))
        .unwrap();
    let table = deltalake::DeltaTable::new_in_memory()
        .write([RecordBatch::try_new(
            Arc::new(Schema::new(vec![Field::new("id", DataType::Int32, true)])),
            vec![Arc::new(Int32Array::from(vec![9]))],
        )
        .unwrap()])
        .await
        .unwrap();
    let (concrete, _) = table
        .clone()
        .update()
        .with_update("id", "policy_udf(id)")
        .with_session_state(Arc::new(state.clone()))
        .await
        .unwrap();
    let wrapper: Arc<dyn DataFusionSession> = Arc::new(WrapperSession::new(state));
    let lost = concrete
        .clone()
        .update()
        .with_update("id", "policy_udf(id)")
        .with_session_state(wrapper.clone())
        .await;
    assert!(lost.unwrap_err().to_string().contains("policy_udf"));
    let rejected = concrete
        .clone()
        .update()
        .with_update("id", "policy_udf(id)")
        .with_session_state(wrapper.clone())
        .with_session_fallback_policy(SessionFallbackPolicy::RequireSessionState)
        .await;
    assert!(rejected.unwrap_err().to_string().contains("SessionState"));
    let (derived, _) = concrete
        .update()
        .with_update("id", "policy_udf(id)")
        .with_session_state(wrapper)
        .with_session_fallback_policy(SessionFallbackPolicy::DeriveFromTrait)
        .await
        .unwrap();
    let (_, stream) = derived.scan_table().await.unwrap();
    let batches: Vec<RecordBatch> = stream.try_collect().await.unwrap();
    assert_eq!(
        batches[0]
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .value(0),
        1
    );
}
