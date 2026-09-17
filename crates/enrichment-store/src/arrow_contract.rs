//! Native schema projection at the Arrow/Delta format boundary.
//!
//! The pinned Delta writer strips schema metadata when deriving its file schema but preserves
//! it on incoming batches. This logical node survives optimization and lowers to DataFusion's
//! own metadata-aware ProjectionExec and native casts. Physical string-view rewrites satisfy
//! the declared storage types at this boundary; no custom value interpreter is involved.
use arrow::datatypes::{DataType, FieldRef};
use async_trait::async_trait;
use datafusion::{
    catalog::Session,
    common::{DFSchemaRef, Result},
    execution::context::QueryPlanner,
    logical_expr::{
        ColumnarValue, Expr, Extension, LogicalPlan, ReturnFieldArgs, ScalarFunctionArgs,
        ScalarUDF, ScalarUDFImpl, Signature, UserDefinedLogicalNode, UserDefinedLogicalNodeCore,
        Volatility, physical_planning_context::PhysicalPlanningContext,
    },
    physical_expr::expressions::Column,
    physical_plan::{
        ExecutionPlan,
        projection::{ProjectionExec, ProjectionExpr},
    },
    physical_planner::{DefaultPhysicalPlanner, ExtensionPlanner, PhysicalPlanner},
};
use std::{collections::HashSet, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ArrowContract {
    input: LogicalPlan,
    schema: DFSchemaRef,
}
impl PartialOrd for ArrowContract {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.schema != other.schema {
            None
        } else {
            self.input.partial_cmp(&other.input)
        }
    }
}
impl UserDefinedLogicalNodeCore for ArrowContract {
    fn name(&self) -> &str {
        "ArrowContract"
    }
    fn inputs(&self) -> Vec<&LogicalPlan> {
        vec![&self.input]
    }
    fn schema(&self) -> &DFSchemaRef {
        &self.schema
    }
    fn expressions(&self) -> Vec<Expr> {
        vec![]
    }
    fn prevent_predicate_push_down_columns(&self) -> HashSet<String> {
        self.schema
            .fields()
            .iter()
            .map(|field| field.name().clone())
            .collect()
    }
    fn supports_limit_pushdown(&self) -> bool {
        true
    }
    fn fmt_for_explain(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ArrowContract: native schema projection")
    }
    fn with_exprs_and_inputs(
        &self,
        exprs: Vec<Expr>,
        mut inputs: Vec<LogicalPlan>,
    ) -> Result<Self> {
        if !exprs.is_empty() || inputs.len() != 1 {
            return datafusion::common::internal_err!(
                "Arrow contract requires one input and no expressions"
            );
        }
        Ok(Self {
            input: inputs.remove(0),
            schema: Arc::clone(&self.schema),
        })
    }
}
pub(crate) fn bind(input: LogicalPlan, schema: DFSchemaRef) -> LogicalPlan {
    LogicalPlan::Extension(Extension {
        node: Arc::new(ArrowContract { input, schema }),
    })
}

struct ContractPlanner;

/// This physical-only kernel composes native name-based casts with Arrow's value-checked
/// layout cast. It is never registered as a SQL function or exposed as a semantic conversion.
#[derive(Debug, PartialEq, Eq, Hash)]
struct Representation {
    source: FieldRef,
    target: FieldRef,
    signature: Signature,
}

impl ScalarUDFImpl for Representation {
    fn name(&self) -> &str {
        "native_arrow_representation_v1"
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        Ok(self.target.data_type().clone())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        enrichment_core::native_schema::function_arguments(
            args.arg_fields,
            std::slice::from_ref(&self.source),
        )?;
        if args.scalar_arguments.len() != 1 {
            return datafusion::common::plan_err!("Arrow representation input contract changed");
        }
        enrichment_core::native_schema::check_projection(&self.source, &self.target)?;
        Ok(self.target.clone())
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        enrichment_core::native_schema::function_call(self, &args)?;
        let options = arrow::compute::CastOptions {
            safe: false,
            ..Default::default()
        };
        let reordered = args.args[0]
            .cast_to(
                enrichment_core::native_schema::nullable_layout(&self.target).data_type(),
                Some(&options),
            )?
            .into_array(args.number_rows)?;
        // The first native cast has established target order. Arrow now checks actual
        // child validity while applying the exact declared layout, without null filling.
        Ok(ColumnarValue::Array(arrow::compute::cast_with_options(
            &reordered,
            self.target.data_type(),
            &options,
        )?))
    }
}

#[async_trait]
impl ExtensionPlanner for ContractPlanner {
    async fn plan_extension(
        &self,
        _: &dyn PhysicalPlanner,
        node: &dyn UserDefinedLogicalNode,
        _: &[&LogicalPlan],
        inputs: &[Arc<dyn ExecutionPlan>],
        session: &dyn Session,
        _: &PhysicalPlanningContext,
    ) -> Result<Option<Arc<dyn ExecutionPlan>>> {
        let Some(contract) = node.as_any().downcast_ref::<ArrowContract>() else {
            return Ok(None);
        };
        if inputs.len() != 1 {
            return datafusion::common::internal_err!("Arrow contract physical arity");
        }
        let input = Arc::clone(&inputs[0]);
        if input.schema().fields().len() != contract.schema.fields().len() {
            return datafusion::common::internal_err!("Arrow contract lost input fields");
        }
        let exprs = input
            .schema()
            .fields()
            .iter()
            .zip(contract.schema.fields())
            .enumerate()
            .map(|(index, (field, expected))| {
                enrichment_core::native_schema::check_projection(field, expected)?;
                let converter = ScalarUDF::from(Representation {
                    source: field.clone(),
                    target: expected.clone(),
                    signature: Signature::exact(
                        vec![field.data_type().clone()],
                        Volatility::Immutable,
                    ),
                });
                let expr = Arc::new(datafusion::physical_expr::ScalarFunctionExpr::try_new(
                    Arc::new(converter),
                    vec![Arc::new(Column::new(field.name(), index))],
                    &input.schema(),
                    Arc::new(session.config_options().clone()),
                )?);
                Ok(ProjectionExpr {
                    expr,
                    alias: field.name().clone(),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Some(Arc::new(
            ProjectionExec::try_new_with_schema_metadata(exprs, input, contract.schema.as_arrow())?,
        )))
    }
}

#[derive(Debug)]
pub(crate) struct NativePlanner;
#[async_trait]
impl QueryPlanner for NativePlanner {
    async fn create_physical_plan(
        &self,
        logical: &LogicalPlan,
        session: &dyn Session,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        DefaultPhysicalPlanner::with_extension_planners(vec![
            Arc::new(ContractPlanner),
            Arc::new(crate::native_effect::CommandPlanner),
            deltalake::delta_datafusion::planner::DeltaExtensionPlanner::new(),
        ])
        .create_physical_plan(logical, session)
        .await
    }
}
