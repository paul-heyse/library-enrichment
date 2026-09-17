//! Native artifact format, section and byte-window policy. Byte scanning/copying is external.
use crate::runtime::QueryRuntime;
use datafusion::{
    common::Result,
    functions::core::expr_ext::FieldAccessor,
    prelude::{col, lit},
};
use enrichment_core::{
    evidence::Artifact,
    native_union::{NativeStruct, Rule},
    operation::selections::{ArtifactReadPlan, ArtifactSlicePlan, ArtifactWindow},
    request::ReadArtifactRequest,
};

enrichment_core::native_struct! { struct Request {
    artifact: Artifact => Rule::Text,
    request: ReadArtifactRequest => Rule::Text,
    result: Option<ArtifactWindow> => Rule::Text,
} }

pub async fn plan(
    runtime: &QueryRuntime,
    artifact: &Artifact,
    request: &ReadArtifactRequest,
    result: Option<ArtifactWindow>,
) -> Result<ArtifactReadPlan> {
    let session = runtime.session();
    session.register_batch(
        "artifact_request",
        Request::batch(&[Request {
            artifact: artifact.clone(),
            request: request.clone(),
            result,
        }])?,
    )?;
    let frame = session
        .sql(
            r#"
        WITH formats AS (
          SELECT *, request.section.kind AS kind,
            result IS NOT NULL OR starts_with(artifact.media_type,'text/')
              OR contains(artifact.media_type,'json') OR contains(artifact.media_type,'toml')
              OR contains(artifact.media_type,'markdown') AS is_text
          FROM artifact_request
        )
        SELECT CASE
          WHEN trim(request.artifact_id)<>artifact.artifact_id THEN 'corrupt'
          WHEN result IS NOT NULL AND kind IS DISTINCT FROM 'result' THEN 'corrupt'
          WHEN kind='result' AND result IS NULL THEN 'missing'
          WHEN kind='result' AND (result.start>result.end OR result.end>artifact.size_bytes
            OR result.section IS DISTINCT FROM request.section.result.name) THEN 'corrupt'
          WHEN kind='markdown' AND (NOT is_text OR length(trim(request.section.markdown.heading))=0
            OR octet_length(request.section.markdown.heading)>512) THEN 'unsupported'
          WHEN kind='markdown' THEN 'markdown'
          ELSE 'bytes' END AS action,
          named_struct('start',coalesce(result.start,CAST(0 AS BIGINT UNSIGNED)),
            'end',coalesce(result.end,artifact.size_bytes),'section',result.section) AS window,
          trim(request.section.markdown.heading) AS heading, is_text
        FROM formats
    "#,
        )
        .await?;
    one(runtime, frame).await
}

/// The scanner reports every bounded heading window. This plan owns matching and first-match
/// ordering, including preamble and repeated headings; source bytes are never reconstructed.
pub async fn markdown(
    runtime: &QueryRuntime,
    windows: Vec<ArtifactWindow>,
    heading: &str,
) -> Result<Option<ArtifactWindow>> {
    let session = runtime.session();
    session.register_batch("markdown_windows", ArtifactWindow::batch(&windows)?)?;
    let frame = session.sql("SELECT * FROM markdown_windows WHERE lower(section)=lower($1) ORDER BY start,end,section LIMIT 1").await?
        .with_param_values(vec![datafusion::common::ScalarValue::from(heading)])?;
    Ok(runtime.records(frame, 1).await?.pop())
}

enrichment_core::native_struct! { struct SliceInput {
    window: ArtifactWindow => Rule::Text,
    offset: usize => Rule::Text,
    budget: usize => Rule::Text,
    is_text: bool => Rule::Text,
} }
pub async fn slice(
    runtime: &QueryRuntime,
    window: &ArtifactWindow,
    offset: usize,
    budget: usize,
    is_text: bool,
) -> Result<ArtifactSlicePlan> {
    let session = runtime.session();
    session.register_batch(
        "slice_input",
        SliceInput::batch(&[SliceInput {
            window: window.clone(),
            offset,
            budget,
            is_text,
        }])?,
    )?;
    let input = session.table("slice_input").await?;
    // Refuse corrupt caller-provided ranges before unsigned arithmetic can wrap.
    runtime
        .require_empty(
            input
                .filter(col("window").field("start").gt(col("window").field("end")))?
                .select(vec![lit("artifact_window").alias("witness")])?,
            "artifact_window",
            "artifact_slice",
        )
        .await?;
    let frame = session.sql(r#"
      WITH available AS (
        SELECT *,window.end-window.start AS length,
          CASE WHEN is_text THEN budget ELSE (budget / CAST(4 AS BIGINT UNSIGNED))*CAST(3 AS BIGINT UNSIGNED)
            + ((budget % CAST(4 AS BIGINT UNSIGNED))*CAST(3 AS BIGINT UNSIGNED))/CAST(4 AS BIGINT UNSIGNED) END AS raw_budget
        FROM slice_input
      ), bounded AS (
        SELECT *,window.start+least("offset",length) AS start FROM available
      ), selected AS (
        SELECT *, start+least(raw_budget,window.end-start) AS finish FROM bounded
      )
      SELECT start,finish AS end,
        finish-start+CASE WHEN finish<window.end THEN CAST(1 AS BIGINT UNSIGNED) ELSE CAST(0 AS BIGINT UNSIGNED) END AS read_bytes,
        "offset">length AS invalid_offset FROM selected
    "#).await?;
    one(runtime, frame).await
}

async fn one<T: NativeStruct>(
    runtime: &QueryRuntime,
    frame: datafusion::dataframe::DataFrame,
) -> Result<T> {
    runtime.records(frame, 1).await?.pop().ok_or_else(|| {
        datafusion::common::DataFusionError::Execution(
            "artifact selection requires one captured input".into(),
        )
    })
}
