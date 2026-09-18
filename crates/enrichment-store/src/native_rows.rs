//! Native canonical record deduplication without hashing a wide nested Arrow row.
//! This uses the same complete value encoding as semantic identities. A business key
//! alone is insufficient: two different records with one key must survive for admission.
use datafusion::{
    dataframe::DataFrame,
    error::Result,
    logical_expr::ExprFunctionExt,
    prelude::{col, lit},
};

fn fingerprint(frame: &DataFrame, domain: &str) -> datafusion::logical_expr::Expr {
    let fields = frame.schema().fields().clone();
    datafusion::functions::crypto::expr_fn::sha256(
        enrichment_core::native_identity::canonical_bytes(domain, fields.clone())
            .call(fields.iter().map(|field| col(field.name())).collect()),
    )
}

/// Complete semantic row witnesses; neither a business key nor a selected field subset.
pub(crate) fn fingerprints(frame: DataFrame, domain: &str) -> Result<DataFrame> {
    let digest = fingerprint(&frame, domain);
    frame.select(vec![digest.alias("fingerprint")])
}

pub(crate) fn distinct(frame: DataFrame, domain: &str) -> Result<DataFrame> {
    const DIGEST: &str = "__native_record_digest";
    const POSITION: &str = "__native_record_position";
    let fields = frame.schema().fields().clone();
    if fields
        .iter()
        .any(|field| [DIGEST, POSITION].contains(&field.name().as_str()))
    {
        return datafusion::common::plan_err!("reserved native deduplication field");
    }
    let digest = fingerprint(&frame, domain);
    let position = datafusion::functions_window::expr_fn::row_number()
        .partition_by(vec![col(DIGEST)])
        .order_by(vec![col(DIGEST).sort(true, false)])
        .build()?
        .alias(POSITION);
    frame
        .with_column(DIGEST, digest)?
        .window(vec![position])?
        .filter(col(POSITION).eq(lit(1u64)))?
        .select(
            fields
                .iter()
                .map(|field| col(field.name()))
                .collect::<Vec<_>>(),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn native_dedup_preserves_conflicting_keys_nulls_and_sequence_order() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default())?;
        let session = runtime.session();
        let input = session.sql("SELECT 'same' AS id, named_struct('optional',v,'sequence',a) AS value FROM (VALUES (CAST(NULL AS VARCHAR),[1,2]),(CAST(NULL AS VARCHAR),[1,2]),('x',[1,2]),('x',[2,1])) t(v,a)").await?;
        let native = distinct(input.clone(), "dedup-oracle/1")?;
        assert_eq!(runtime.execute(native.clone()).await?.rows, 3);
        let expected = input.distinct()?;
        let mismatch = native
            .clone()
            .except(expected.clone())?
            .union(expected.except(native)?)?;
        assert_eq!(runtime.execute(mismatch).await?.rows, 0);
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
