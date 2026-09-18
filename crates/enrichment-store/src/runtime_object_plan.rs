//! Native selection and physical-outcome rules for the finite runtime-object producer.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::{DataFusionError, Result, ScalarValue},
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::{
        arrow_model::expressions::{literal, record},
        execution::RuntimeObject,
    },
    native_runtime::{Capture, ReportInput},
    native_union::{Cell, NativeStruct},
};

pub async fn lower(runtime: &QueryRuntime, capture: &Capture) -> Result<RuntimeObject> {
    let frame = crate::native_catalog::batch(
        &runtime.session(),
        "runtime_object_plan",
        Capture::batch(std::slice::from_ref(capture))?,
    )?;
    let observation = col("observation");
    let success = observation
        .clone()
        .field("end")
        .eq(lit("exited"))
        .and(observation.clone().field("exit_code").eq(lit(0i32)));
    let report = datafusion::logical_expr::when(success, observation.clone().field("stdout"))
        .otherwise(lit(ScalarValue::Utf8(None)))?;
    let parsed = enrichment_core::native_runtime::report().call(vec![record(
        &ReportInput::data_type(),
        &[("report", report), ("subject", col("subject"))],
    )?]);
    let end = observation.clone().field("end");
    let outcome =
        datafusion::logical_expr::when(end.clone().eq(lit("cancelled")), lit("cancelled"))
            .when(
                end.clone()
                    .eq(lit("deadline"))
                    .or(end.eq(lit("output_limit"))),
                lit("incomplete"),
            )
            .otherwise(lit("failed"))?;
    let fallback = record(
        &RuntimeObject::data_type(),
        &[
            ("module", col("selection").field("module")),
            ("selection", col("selection").field("attributes")),
            ("outcome", outcome),
            ("attributes", literal(&Vec::<String>::new())?),
            (
                "limitations",
                datafusion::functions_nested::expr_fn::make_array(vec![
                    datafusion::functions::string::expr_fn::concat(vec![
                        lit("Runtime observation ended "),
                        observation.field("end"),
                        lit("; the exact exit status and process log are retained."),
                    ]),
                ]),
            ),
        ],
    )?;
    let value =
        datafusion::functions::core::expr_fn::coalesce(vec![parsed.field("result"), fallback]);
    let frame = frame.with_column("result", value)?;
    runtime
        .require_empty(
            frame
                .clone()
                .filter(
                    col("result")
                        .field("module")
                        .not_eq(col("selection").field("module"))
                        .or(col("result")
                            .field("selection")
                            .not_eq(col("selection").field("attributes"))),
                )?
                .select(vec![lit("runtime_selection_mismatch").alias("witness")])?,
            "runtime_selection_scope",
            "runtime_normalization",
        )
        .await?;
    let frame = frame.select(
        RuntimeObject::fields()
            .iter()
            .map(|field| col("result").field(field.name()).alias(field.name()))
            .collect::<Vec<_>>(),
    )?;
    runtime
        .records::<RuntimeObject>(frame, 1)
        .await?
        .pop()
        .ok_or_else(|| DataFusionError::Execution("native runtime lowering missing".into()))
}

/// One native external-format encoder supplies both the staged file and its admission digest.
pub async fn selection_transport(
    runtime: &crate::runtime::QueryRuntime,
    selection: &enrichment_core::request::RuntimeSelection,
    max_output_bytes: usize,
) -> datafusion::common::Result<String> {
    use enrichment_core::native_union::Rule;
    enrichment_core::native_struct! { struct Encoded { value: String => Rule::Text } }
    let input = enrichment_core::native_runtime::SelectionTransport {
        selection: selection.clone(),
        max_output_bytes: max_output_bytes as u64,
    };
    let frame = runtime.session().read_empty()?.select(vec![
        enrichment_core::native_runtime::selection_transport()
            .call(vec![
                enrichment_core::evidence::arrow_model::expressions::literal(&input)?,
            ])
            .alias("value"),
    ])?;
    runtime
        .records::<Encoded>(frame, 1)
        .await?
        .pop()
        .map(|value| value.value)
        .ok_or_else(|| {
            datafusion::common::DataFusionError::Execution(
                "runtime selection transport missing".into(),
            )
        })
}
