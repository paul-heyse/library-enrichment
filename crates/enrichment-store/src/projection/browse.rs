use super::cells::{RowSet, invalid};
use arrow::{error::ArrowError, record_batch::RecordBatch};
use enrichment_core::{
    evidence::SymbolKind,
    wire::data::{NamespaceFacet, OverviewChild},
};
use std::collections::BTreeMap;

pub(crate) fn counts(batches: &[RecordBatch]) -> Result<BTreeMap<String, u64>, ArrowError> {
    let mut out = BTreeMap::new();
    for batch in batches {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let row = rows.row(i);
            out.insert(row.text("kind")?.into(), row.number("count")?);
        }
    }
    Ok(out)
}

pub(crate) fn facets(
    nodes: &[RecordBatch],
    counts: &[RecordBatch],
    children: &[RecordBatch],
) -> Result<Vec<NamespaceFacet>, ArrowError> {
    let mut positions = BTreeMap::new();
    let mut out = Vec::new();
    for batch in nodes {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let row = rows.row(i);
            positions.insert(
                (row.text("ecosystem")?.to_owned(), row.list("components")?),
                out.len(),
            );
            out.push(NamespaceFacet {
                path: row.text("path")?.into(),
                doc_summary: row.owned("doc_summary")?,
                counts_by_kind: BTreeMap::new(),
                children: Vec::new(),
                truncated_children: row.number("total")?,
            });
        }
    }
    for batch in counts {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let row = rows.row(i);
            let key = (
                row.text("ecosystem")?.to_owned(),
                row.list("namespace_components")?,
            );
            let index = *positions
                .get(&key)
                .ok_or_else(|| invalid("overview count outside selected namespaces"))?;
            out[index]
                .counts_by_kind
                .insert(row.text("kind")?.into(), row.number("count")?);
        }
    }
    for batch in children {
        let rows = RowSet::batch(batch)?;
        for i in 0..batch.num_rows() {
            let row = rows.row(i);
            let key = (
                row.text("ecosystem")?.to_owned(),
                row.list("namespace_components")?,
            );
            let index = *positions
                .get(&key)
                .ok_or_else(|| invalid("overview child outside selected namespaces"))?;
            out[index].children.push(OverviewChild {
                path: row.text("path")?.into(),
                kind: SymbolKind::parse(row.text("kind")?)
                    .ok_or_else(|| invalid("unknown child kind"))?,
                doc_summary: row.owned("doc_summary")?,
                is_reexport: row.boolean("is_reexport")?,
                deprecated: row.boolean("is_deprecated")?,
            });
            out[index].truncated_children = out[index]
                .truncated_children
                .checked_sub(1)
                .ok_or_else(|| invalid("overview sample exceeds count"))?;
        }
    }
    for facet in &mut out {
        facet.children.sort_by(|a, b| {
            a.path
                .cmp(&b.path)
                .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
        });
    }
    Ok(out)
}
