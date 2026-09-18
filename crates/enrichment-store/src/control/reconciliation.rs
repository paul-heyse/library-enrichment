//! An unknown acknowledgement is resolved by complete durable row content, never a TTL,
//! cache hit, current-row selection, or an expirable Delta application transaction marker.
use super::*;
use enrichment_core::native_union::Rule;

enrichment_core::native_struct! { struct Verdict { committed: bool => Rule::Text } }

async fn complete(runtime: &QueryRuntime, expected: DataFrame, durable: DataFrame) -> Result<bool> {
    if expected.schema().as_arrow() != durable.schema().as_arrow() {
        return Err(invalid("control reconciliation semantic schema changed"));
    }
    let session = runtime.session();
    let expected = crate::native_rows::fingerprints(expected, "control-command-reconciliation/1")?;
    let durable = crate::native_rows::fingerprints(durable, "control-command-reconciliation/1")?;
    crate::native_catalog::work(&session, "reconciliation_expected", expected.into_view())?;
    crate::native_catalog::work(&session, "reconciliation_durable", durable.into_view())?;
    // Compare native occurrence counts explicitly. A membership anti-join, including
    // DataFusion 55.1's EXCEPT ALL lowering, cannot prove duplicate multiplicity.
    let verdict = session.sql(r#"
        WITH required AS (SELECT fingerprint,count(*) AS copies FROM reconciliation_expected GROUP BY fingerprint),
          durable AS (SELECT fingerprint,count(*) AS copies FROM reconciliation_durable GROUP BY fingerprint),
          missing AS (SELECT r.fingerprint FROM required r LEFT JOIN durable d ON r.fingerprint=d.fingerprint WHERE coalesce(d.copies,0)<r.copies)
        SELECT e.required BETWEEN 1 AND 1024 AND m.missing=0 AS committed
        FROM (SELECT count(*) AS required FROM reconciliation_expected) e
        CROSS JOIN (SELECT count(*) AS missing FROM missing) m
    "#).await?;
    runtime
        .records::<Verdict>(verdict, 1)
        .await?
        .pop()
        .map(|row| row.committed)
        .ok_or_else(|| invalid("control reconciliation verdict missing"))
}

impl ControlStore {
    /// The bounded input has already passed native candidate admission. If acknowledgement
    /// fails, refresh without reissuing the write and establish the entire command in history.
    pub(super) async fn append_control(
        &self,
        table: LoadedTable,
        input: DataFrame,
        transactions: Vec<Transaction>,
    ) -> Result<u64> {
        let namespace = table.namespace().clone();
        let snapshot = table.snapshot().map_err(external)?;
        let table_id = snapshot.metadata().id().to_owned();
        let predecessor = snapshot.version();
        let written = self
            .delta
            .append(table, &self.contract, input.clone(), transactions)
            .await
            .and_then(|table| {
                crate::publication_probe::hit(
                    &self.root,
                    crate::publication_probe::Point::ControlAppendDurable,
                )?;
                Ok(table)
            });
        match written {
            Ok(table) => table
                .version()
                .ok_or_else(|| invalid("unloaded control result")),
            Err(error) => {
                let proof = async {
                    let lease = crate::leases::shared(&self.root)?;
                    let current = self.delta.load("control", None).await?;
                    let snapshot = current.snapshot().map_err(external)?;
                    if current.namespace() != &namespace || snapshot.metadata().id() != table_id {
                        return Err(invalid(
                            "control incarnation changed during acknowledgement reconciliation",
                        ));
                    }
                    if snapshot.version() <= predecessor {
                        return Ok(None);
                    }
                    let captured = self.pin_table(&current, Some(lease)).await?;
                    let durable = self
                        .runtime
                        .session()
                        .read_table(Arc::clone(&captured.control))?;
                    Ok(complete(&self.runtime, input, durable)
                        .await?
                        .then_some(captured.version))
                }
                .await;
                match proof {
                    Ok(Some(generation)) => Ok(generation),
                    Ok(None) => Err(error),
                    // A failed refresh is not a known conflict. Do not preserve a nested
                    // conflict discriminator that would cause a caller to reissue the write.
                    Err(proof) => Err(DataFusionError::Execution(format!(
                        "control outcome remains unknown: {error}; reconciliation failed: {proof}"
                    ))),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn native_control_acknowledgement_requires_complete_exact_history() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let session = runtime.session();
        let expected = session.sql("SELECT id, named_struct('sequence',sequence,'values',items,'optional',CAST(NULL AS VARCHAR)) AS value FROM (VALUES ('a',1,[1,2]),('b',1,[])) t(id,sequence,items)").await?;
        assert!(complete(&runtime, expected.clone(), expected.clone()).await?);
        assert!(
            !complete(
                &runtime,
                expected.clone(),
                expected.clone().limit(0, Some(1))?
            )
            .await?
        );
        assert!(
            !complete(
                &runtime,
                expected.clone().union(expected.clone())?,
                expected.clone()
            )
            .await?
        );
        let newer = session.sql("SELECT id, named_struct('sequence',sequence,'values',items,'optional',CAST(NULL AS VARCHAR)) AS value FROM (VALUES ('a',2,[2,1]),('b',2,[])) t(id,sequence,items)").await?;
        assert!(!complete(&runtime, expected.clone(), newer.clone()).await?);
        // Later transitions do not hide the exact acknowledged history rows.
        assert!(complete(&runtime, expected.clone(), expected.clone().union(newer)?).await?);
        let null = session.sql("SELECT id, named_struct('sequence',sequence,'values',items,'optional',CAST(NULL AS VARCHAR)) AS value FROM (VALUES ('a',1,[1,2]),('b',1,CAST(NULL AS INTEGER[]))) t(id,sequence,items)").await?;
        assert!(!complete(&runtime, expected.clone(), null).await?);
        let empty = expected.clone().limit(0, Some(0))?;
        assert!(!complete(&runtime, empty.clone(), expected).await?);
        assert!(!complete(&runtime, empty.clone(), empty).await?);
        // Use the actual wide control union as well as the small independent value oracle.
        // Most variants are null; the retained dependency and sequence must still participate.
        use enrichment_core::native_union::NativeStruct;
        let root = crate::retention::RetentionRoot {
            root_id: "published".into(),
            dependencies: vec![crate::retention::Dependency::Artifact {
                artifact_id: format!("art_{}", "a".repeat(64)),
            }],
            removed: false,
            sequence: 4,
        };
        let packed = |root| -> Result<DataFrame> {
            crate::native_catalog::batch(
                &session,
                "reconciliation",
                pack_rows(
                    Table::RetentionRoots,
                    crate::retention::RetentionRoot::batch(&[root])?,
                )?,
            )
        };
        let requested = packed(root.clone())?;
        assert!(complete(&runtime, requested.clone(), packed(root.clone())?).await?);
        let mut revoked = root;
        revoked.removed = true;
        assert!(!complete(&runtime, requested, packed(revoked)?).await?);
        runtime.close_diagnostics().await
    }
}
