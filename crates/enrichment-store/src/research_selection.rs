//! Research selections are native relations, including omitted-request defaults.
use crate::runtime::QueryRuntime;
use datafusion::common::Result;
use enrichment_core::{
    native_union::{Cell, NativeStruct},
    wire::research::{
        AspectSelection, DiscoveryKind, DiscoverySelection, InspectionAspect, ResearchSelection,
    },
};

/// Expand a validated version-pair request into its two typed acquisition prerequisites.
pub async fn comparison_prerequisites(
    runtime: &QueryRuntime,
    request: &enrichment_core::request::CompareRequest,
    input_bound: usize,
) -> Result<Vec<enrichment_core::request::ResolveRequest>> {
    use datafusion::prelude::col;
    use enrichment_core::request::{CompareRequest, ResolveRequest};
    crate::request_admission::research(runtime, &request.clone().into(), input_bound).await?;
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "comparison_request",
        CompareRequest::batch(std::slice::from_ref(request))?,
    )?;
    crate::native_catalog::input(
        &session,
        "resolution_defaults",
        ResolveRequest::batch(&[ResolveRequest::default()])?,
    )?;
    let frame = session.sql("SELECT d.*,c.ecosystem AS selected_ecosystem,c.name AS selected_name,unnest([c.from_version,c.to_version]) AS selected_version FROM comparison_request c CROSS JOIN resolution_defaults d WHERE c.before_context_id IS NULL").await?;
    let columns: Vec<_> = ResolveRequest::fields()
        .iter()
        .map(|field| {
            let source = match field.name().as_str() {
                "ecosystem" => "selected_ecosystem",
                "name" => "selected_name",
                "version" => "selected_version",
                name => name,
            };
            col(source).alias(field.name())
        })
        .collect();
    runtime.records(frame.select(columns)?, 2).await
}

pub async fn inspection(
    runtime: &QueryRuntime,
    selection: &ResearchSelection,
) -> Result<Vec<AspectSelection>> {
    let session = runtime.session();
    let values = ResearchSelection::encode(&[Some(selection)])?;
    crate::native_catalog::input(
        &session,
        "selection",
        arrow::record_batch::RecordBatch::from(
            datafusion::common::cast::as_struct_array(&values)?.clone(),
        ),
    )?;
    let frame = session.sql(r#"
        SELECT selected.aspect AS aspect,selected.cursor AS cursor,selected.max_items AS max_items,selected.max_characters AS max_characters
        FROM (SELECT unnest(explicit.aspects) AS selected FROM selection WHERE mode='explicit')
        UNION ALL
        SELECT aspect,CAST(NULL AS VARCHAR) AS cursor,default_max_items AS max_items,
               default_max_characters AS max_characters FROM
          (SELECT d.* FROM operation.declarations.inspection_aspects d CROSS JOIN selection s
           WHERE s.mode='default' AND d.default_max_items IS NOT NULL ORDER BY ordinal)
    "#).await?;
    runtime.records(frame, InspectionAspect::VALUES.len()).await
}

pub async fn discovery(
    runtime: &QueryRuntime,
    selection: &Option<Vec<DiscoverySelection>>,
) -> Result<Vec<DiscoverySelection>> {
    enrichment_core::native_struct! {
        struct Input { facets: Option<Vec<DiscoverySelection>> => enrichment_core::native_union::Rule::Sequence }
    }
    let session = runtime.session();
    crate::native_catalog::input(
        &session,
        "selection",
        Input::batch(&[Input {
            facets: selection.clone(),
        }])?,
    )?;
    let frame = session.sql(r#"
        SELECT selected.kind AS kind,selected.cursor AS cursor,selected.max_items AS max_items,selected.max_characters AS max_characters
        FROM (SELECT unnest(facets) AS selected FROM selection)
        UNION ALL
        SELECT kind,CAST(NULL AS VARCHAR) AS cursor,default_max_items AS max_items,
               default_max_characters AS max_characters FROM
          (SELECT d.* FROM operation.declarations.discovery_facets d CROSS JOIN selection s WHERE s.facets IS NULL ORDER BY ordinal)
    "#).await?;
    runtime.records(frame, DiscoveryKind::VALUES.len()).await
}
