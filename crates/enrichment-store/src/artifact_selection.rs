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
    crate::native_catalog::input(
        &session,
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
    crate::native_catalog::input(
        &session,
        "markdown_windows",
        ArtifactWindow::batch(&windows)?,
    )?;
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
    crate::native_catalog::input(
        &session,
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

// The full frame varies only with these decimal widths, encoding and terminal-cursor
// presence. Content prefix lengths come from the linear codec kernel, never copied strings.
enrichment_core::native_struct! { pub struct FrameClass {
    length: u64 => Rule::Text,
    utf8: bool => Rule::Text,
    end_digits: u64 => Rule::Text,
    remaining_digits: u64 => Rule::Text,
    offset_digits: u64 => Rule::Text,
    finished: bool => Rule::Text,
} }
enrichment_core::native_struct! { pub struct FrameMeasure {
    class: FrameClass => Rule::Text,
    bytes: u64 => Rule::Text,
} }
enrichment_core::native_struct! { pub struct PrefixSelection {
    length: Option<u64> => Rule::Text,
    utf8: Option<bool> => Rule::Text,
    minimum: u64 => Rule::Text,
} }
pub struct PrefixPlan {
    session: datafusion::prelude::SessionContext,
    // Covers format-fact construction, Arrow conversion and simultaneous native query
    // branches. Kept through final selection, including optimizer constant folding.
    _memory: datafusion::execution::memory_pool::MemoryReservation,
}
impl PrefixPlan {
    pub async fn prepare(
        runtime: &QueryRuntime,
        bytes: bytes::Bytes,
        text: bool,
        slice: &ArtifactSlicePlan,
        window: &ArtifactWindow,
    ) -> Result<Self> {
        use datafusion::{common::ScalarValue, execution::memory_pool::MemoryConsumer};
        let memory = MemoryConsumer::new("artifact-prefix-facts")
            .register(&runtime.session().runtime_env().memory_pool);
        let bound = bytes
            .len()
            .checked_add(1)
            .and_then(|value| value.checked_mul(256))
            .ok_or_else(|| {
                datafusion::common::exec_datafusion_err!("artifact prefix allocation overflow")
            })?;
        memory.try_grow(bound)?;
        let session = runtime.session();
        let frame = session
            .read_empty()?
            .select(vec![
                enrichment_core::native_artifact::boundaries()
                    .call(vec![
                        lit(ScalarValue::Binary(Some(bytes.to_vec()))),
                        lit(text),
                        lit((slice.end - slice.start) as u64),
                        lit(slice.end == window.end),
                    ])
                    .alias("boundary"),
            ])?
            .unnest_columns(&["boundary"])?;
        crate::native_catalog::work(&session, "artifact_prefix_facts", frame.into_view())?;
        let frame = session.sql(&format!(r#"
            SELECT boundary.length AS length, boundary.utf8 AS utf8, boundary.json_bytes AS json_bytes, boundary.resource_bytes AS resource_bytes,
                CAST(length(CAST({start}+boundary.length AS VARCHAR)) AS BIGINT UNSIGNED) AS end_digits,
                CAST(length(CAST({end}-{start}-boundary.length AS VARCHAR)) AS BIGINT UNSIGNED) AS remaining_digits,
                CAST(length(CAST({start}-{origin}+boundary.length AS VARCHAR)) AS BIGINT UNSIGNED) AS offset_digits,
                {start}+boundary.length={end} AS finished
            FROM artifact_prefix_facts
            WHERE boundary.length>0 OR {start}={end}
        "#, start=slice.start, end=window.end, origin=window.start)).await?;
        crate::native_catalog::work(&session, "artifact_prefixes", frame.into_view())?;
        Ok(Self {
            session,
            _memory: memory,
        })
    }
    pub async fn classes(&self, runtime: &QueryRuntime) -> Result<Vec<FrameClass>> {
        runtime.records(self.session.sql("SELECT min(length) AS length,utf8,end_digits,remaining_digits,offset_digits,finished FROM artifact_prefixes GROUP BY utf8,end_digits,remaining_digits,offset_digits,finished").await?, 256).await
    }
    pub async fn select(
        self,
        runtime: &QueryRuntime,
        measurements: Vec<FrameMeasure>,
        budget: usize,
        resource: bool,
    ) -> Result<PrefixSelection> {
        crate::native_catalog::input(
            &self.session,
            "artifact_frame_measurements",
            FrameMeasure::batch(&measurements)?,
        )?;
        let query = format!(
            r#"
            WITH measured AS (
                SELECT p.length,p.utf8,m.bytes+p.{content} AS bytes
                FROM artifact_prefixes p JOIN artifact_frame_measurements m
                ON p.utf8=m.class.utf8 AND p.end_digits=m.class.end_digits
                AND p.remaining_digits=m.class.remaining_digits AND p.offset_digits=m.class.offset_digits
                AND p.finished=m.class.finished
            ), selected AS (
                SELECT length,utf8 FROM measured WHERE bytes<={budget} ORDER BY length DESC LIMIT 1
            ) SELECT selected.length,selected.utf8,coalesce(minimum.bytes,CAST({fallback} AS BIGINT UNSIGNED)) AS minimum
              FROM (SELECT min(bytes) AS bytes FROM measured) minimum LEFT JOIN selected ON true
        "#,
            content = if resource {
                "resource_bytes"
            } else {
                "json_bytes"
            },
            fallback = budget.saturating_add(1)
        );
        one(runtime, self.session.sql(&query).await?).await
    }
}

#[cfg(test)]
mod prefix_tests {
    use super::*;
    #[tokio::test]
    async fn plan19_native_prefix_selection_handles_escaping_and_terminal_frames() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(&root.path().join("spill"), Default::default())?;
        let text = "a\"🦀\n0123456789";
        let window = ArtifactWindow {
            start: 0,
            end: text.len(),
            section: None,
        };
        let slice = ArtifactSlicePlan {
            start: 0,
            end: text.len(),
            read_bytes: text.len(),
            invalid_offset: false,
        };
        let plan = PrefixPlan::prepare(
            &runtime,
            bytes::Bytes::copy_from_slice(text.as_bytes()),
            true,
            &slice,
            &window,
        )
        .await?;
        let classes = plan.classes(&runtime).await?;
        assert!(!classes.is_empty());
        let measured = classes
            .into_iter()
            .map(|class| {
                // Independent format envelope: terminal frames omit a long cursor.
                let bytes = if class.finished { 8 } else { 30 };
                FrameMeasure { class, bytes }
            })
            .collect();
        let selected = plan
            .select(
                &runtime,
                measured,
                8 + serde_json::to_string(text).unwrap().len() - 2,
                false,
            )
            .await?;
        assert_eq!(selected.length, Some(text.len() as u64));
        assert_eq!(selected.utf8, Some(true));
        let plan = PrefixPlan::prepare(
            &runtime,
            bytes::Bytes::copy_from_slice(text.as_bytes()),
            true,
            &slice,
            &window,
        )
        .await?;
        let measured = plan
            .classes(&runtime)
            .await?
            .into_iter()
            .map(|class| FrameMeasure { class, bytes: 100 })
            .collect();
        let selected = plan.select(&runtime, measured, 104, false).await?;
        assert_eq!(selected.length, Some(2)); // a and quote cost 1+2, the next character costs 4.
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
