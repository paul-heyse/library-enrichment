//! One native decision owns publication replay, selection predecessors and current claim scope.
use super::*;
use enrichment_core::{
    evidence::arrow_model::expressions::{coalesce, record},
    native_union::{Cell, NativeStruct, Rule},
};

enrichment_core::native_struct! { struct Request {
    publication: Option<JobPublication> => Rule::Text,
    comparison: Option<ComparisonPublication> => Rule::Text,
    selection: Option<SelectionChange> => Rule::Text,
    fence: Option<PublicationFence> => Rule::Text,
} }
enrichment_core::native_vocabulary! { pub(super) enum Disposition {
    Append = "append", Committed = "committed", Conflict = "conflict",
} }
enrichment_core::native_struct! { pub(super) struct Admission {
    disposition: Disposition => Rule::Text,
    current: Option<SnapshotId> => Rule::Text,
    transactions: Vec<String> => Rule::Set,
} }

fn publication<T: NativeStruct + Cell>(frame: DataFrame) -> Result<DataFrame> {
    let fields = T::fields();
    let values = fields
        .iter()
        .map(|field| (field.name().as_str(), col(field.name())))
        .collect::<Vec<_>>();
    frame.select(vec![record(&T::data_type(), &values)?.alias("value")])
}

pub(super) async fn check(
    runtime: &QueryRuntime,
    session: &SessionContext,
    command: &ControlBatch,
) -> Result<Admission> {
    crate::native_catalog::input(
        session,
        "control_publication_request",
        Request::batch(&[Request {
            publication: command.publication.clone(),
            comparison: command.comparison.clone(),
            selection: command.selection.clone(),
            fence: command.publication_fence.clone(),
        }])?,
    )?;
    crate::native_catalog::work(
        session,
        "control_existing_publications",
        publication::<JobPublication>(session.table("state.records.job_publications").await?)?
            .into_view(),
    )?;
    crate::native_catalog::work(
        session,
        "control_existing_comparisons",
        publication::<ComparisonPublication>(
            session
                .table("state.records.comparison_publications")
                .await?,
        )?
        .into_view(),
    )?;
    let facts = session
        .sql(
            r#"
      SELECT r.*,p.value AS prior_publication,c.value AS prior_comparison,s.snapshot_id AS current,
        p.value IS NOT NULL OR c.value IS NOT NULL AS replay
      FROM control_publication_request r
      LEFT JOIN control_existing_publications p ON r.publication.job_id=p.value.job_id
      LEFT JOIN control_existing_comparisons c ON r.comparison.job_id=c.value.job_id
      LEFT JOIN state.records.selections s ON r.selection.context_id=s.context_id
    "#,
        )
        .await?;
    let facts = facts.with_column(
        "job_id",
        coalesce(vec![
            col("publication").field("job_id"),
            col("comparison").field("job_id"),
        ])?,
    )?;
    let context_field = SelectionChange::fields()[0].clone();
    let facts = facts.with_column(
        "selection_context_key",
        enrichment_core::native_id::diagnostic(
            &context_field,
            col("selection").field("context_id"),
        )?,
    )?;
    let job_key = enrichment_core::native_id::diagnostic(
        facts.schema().field_with_name(None, "job_id")?,
        col("job_id"),
    )?;
    let facts = facts.with_column("job_transaction_key", job_key)?;
    crate::native_catalog::work(session, "control_publication_facts", facts.into_view())?;
    let mut rules = crate::invariants::Invariants::default();
    rules.push(session.sql("SELECT 'publication_kind' AS witness FROM control_publication_facts WHERE publication IS NOT NULL AND comparison IS NOT NULL").await?, "publication_kind_exclusive", "control_command")?;
    rules.push(session.sql("SELECT job_id AS witness FROM control_publication_facts WHERE (prior_publication IS NOT NULL AND prior_publication IS DISTINCT FROM publication) OR (prior_comparison IS NOT NULL AND prior_comparison IS DISTINCT FROM comparison)").await?, "publication_replay_identity", "control_command")?;
    rules.push(session.sql(r#"
      SELECT r.job_id AS witness FROM control_publication_facts r
      LEFT ANTI JOIN (
        SELECT c.job_id,c.owner,c.fence FROM state.records.claims c
        JOIN state.records.job_transitions t ON c.job_id=t.job_id
        WHERE c.cleanup_state='owned' AND clock_instant(c.lease_expires_at)>now() AND t.state='running'
      ) c ON r.job_id=c.job_id AND r.fence.owner=c.owner AND r.fence.fence=c.fence
      WHERE r.job_id IS NOT NULL AND NOT r.replay
    "#).await?, "publication_current_claim", "control_command")?;
    runtime.admit(rules).await?;
    let verdict = session.sql(r#"
      SELECT CASE WHEN replay THEN 'committed'
        WHEN selection IS NOT NULL AND (current IS DISTINCT FROM selection.expected_base) AND (current IS DISTINCT FROM selection.snapshot_id) THEN 'conflict'
        ELSE 'append' END AS disposition,current,
        array_compact([
          CASE WHEN job_id IS NOT NULL THEN concat('job/',job_transaction_key) END,
          CASE WHEN job_id IS NOT NULL THEN concat('claim/',job_transaction_key) END,
          CASE WHEN selection IS NOT NULL THEN concat('context/',selection_context_key) END
        ]) AS transactions
      FROM control_publication_facts
    "#).await?;
    runtime
        .records(verdict, 1)
        .await?
        .pop()
        .ok_or_else(|| invalid("control command admission missing"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_catalog::{BindingKind, BoundCatalog, Tables};
    use enrichment_core::evidence::catalog::SnapshotSelection;

    fn session(
        runtime: &QueryRuntime,
        rows: Vec<(Table, RecordBatch)>,
        claim: Option<DataFrame>,
    ) -> Result<SessionContext> {
        let mut tables = Tables::new();
        for table in [
            Table::Selections,
            Table::JobPublications,
            Table::ComparisonPublications,
            Table::Claims,
            Table::JobTransitions,
        ] {
            tables.insert(
                table.name().into(),
                crate::native_catalog::batch(
                    &runtime.session(),
                    "admission",
                    RecordBatch::new_empty(table.schema()?),
                )?
                .into_view(),
            );
        }
        for (table, batch) in rows {
            tables.insert(
                table.name().into(),
                crate::native_catalog::batch(&runtime.session(), "admission", batch)?.into_view(),
            );
        }
        if let Some(claim) = claim {
            tables.insert("claims".into(), claim.into_view());
        }
        runtime.bound_session(BTreeMap::from([(
            "state".into(),
            Arc::new(BoundCatalog::default().with_schema(BindingKind::FoldedRecords, tables))
                as Arc<dyn datafusion::catalog::CatalogProvider>,
        )]))
    }

    #[tokio::test]
    async fn native_publication_predecessor_replay_and_claim_policy() -> Result<()> {
        let root = tempfile::tempdir()?;
        let runtime = QueryRuntime::new(root.path(), Default::default())?;
        let context = ContextId::try_from(format!("ctx_{}", "a".repeat(64))).unwrap();
        let before = SnapshotId::try_from(format!("snap_{}", "b".repeat(64))).unwrap();
        let after = SnapshotId::try_from(format!("snap_{}", "c".repeat(64))).unwrap();
        let other = SnapshotId::try_from(format!("snap_{}", "d".repeat(64))).unwrap();
        let mut command = ControlBatch {
            selection: Some(SelectionChange {
                context_id: context.clone(),
                snapshot_id: after.clone(),
                expected_base: Some(before.clone()),
            }),
            ..Default::default()
        };
        for (current, expected) in [
            (None, Disposition::Conflict),
            (Some(before.clone()), Disposition::Append),
            (Some(after.clone()), Disposition::Append),
            (Some(other), Disposition::Conflict),
        ] {
            let selection = current.clone().map(|snapshot_id| SnapshotSelection {
                context_id: context.clone(),
                snapshot_id,
                generation: 1,
            });
            let session = session(
                &runtime,
                vec![(
                    Table::Selections,
                    SnapshotSelection::batch(&selection.into_iter().collect::<Vec<_>>())?,
                )],
                None,
            )?;
            let admitted = check(&runtime, &session, &command).await?;
            assert_eq!(admitted.disposition, expected);
            assert_eq!(admitted.current, current);
            assert_eq!(admitted.transactions, vec![format!("context/{context}")]);
        }
        command.selection.as_mut().unwrap().expected_base = None;
        assert_eq!(
            check(&runtime, &session(&runtime, vec![], None)?, &command)
                .await?
                .disposition,
            Disposition::Append
        );
        let delivery = enrichment_core::evidence::Artifact {
            artifact_id: format!("art_{}", "e".repeat(64)),
            sha256: "e".repeat(64),
            size_bytes: 128,
            kind: enrichment_core::evidence::ArtifactKind::Other,
            media_type: "application/json".into(),
            source_uri: "service:delivery".into(),
            retrieved_at: enrichment_core::native_time::AcquisitionTime::now()?,
            final_url: None,
            etag: None,
            last_modified: None,
            compression: None,
        };
        let publication = JobPublication {
            job_id: format!("job_{}", "f".repeat(32)).try_into().unwrap(),
            context_id: context,
            snapshot_id: after,
            kind: enrichment_core::evidence::catalog::PublishedJobKind::Resolve,
            state: enrichment_core::wire::JobState::Succeeded,
            attempt_id: enrichment_core::identity::AttemptId::new(),
            result_artifact_ids: vec![delivery.artifact_id.clone()],
            delivery,
        };
        command.selection = None;
        command.publication = Some(publication.clone());
        assert!(
            check(&runtime, &session(&runtime, vec![], None)?, &command)
                .await
                .is_err(),
            "no claim cannot publish"
        );
        let existing = session(
            &runtime,
            vec![(
                Table::JobPublications,
                JobPublication::batch(std::slice::from_ref(&publication))?,
            )],
            None,
        )?;
        assert_eq!(
            check(&runtime, &existing, &command).await?.disposition,
            Disposition::Committed,
            "durable replay does not need a live claim"
        );
        let mut changed = publication.clone();
        changed.delivery.size_bytes += 1;
        let mismatched = session(
            &runtime,
            vec![(Table::JobPublications, JobPublication::batch(&[changed])?)],
            None,
        )?;
        assert!(
            check(&runtime, &mismatched, &command).await.is_err(),
            "one job cannot acknowledge a different delivery"
        );
        command.publication_fence = Some(PublicationFence {
            owner: "owner".into(),
            fence: 8,
        });
        let transition = enrichment_core::operation::jobs::Transition {
            job_id: publication.job_id,
            sequence: 8,
            predecessor: Some(7),
            state: enrichment_core::wire::JobState::Running,
            stage: "publishing".into(),
            updated_at: enrichment_core::native_time::UpdateTime::now()?,
            result: None,
            resolution: None,
        };
        for (owner, fence, expires, expected) in [
            ("owner", 8, "2099-01-01T00:00:00.000000Z", true),
            ("other", 8, "2099-01-01T00:00:00.000000Z", false),
            ("owner", 9, "2099-01-01T00:00:00.000000Z", false),
            ("owner", 8, "2000-01-01T00:00:00.000000Z", false),
        ] {
            let claim = runtime.session().read_empty()?.select(vec![
                lit(publication.job_id).alias("job_id"),
                lit(owner).alias("owner"),
                lit(fence as u64).alias("fence"),
                lit("owned").alias("cleanup_state"),
                enrichment_core::evidence::arrow_model::expressions::literal(
                    &enrichment_core::native_time::ExpiryTime::try_from(expires.to_owned())?,
                )?
                .alias("lease_expires_at"),
            ])?;
            let state = session(
                &runtime,
                vec![(
                    Table::JobTransitions,
                    enrichment_core::operation::jobs::Transition::batch(std::slice::from_ref(
                        &transition,
                    ))?,
                )],
                Some(claim),
            )?;
            let result = check(&runtime, &state, &command).await;
            assert_eq!(
                result.is_ok(),
                expected,
                "owner={owner}, fence={fence}, expires={expires}: {result:?}"
            );
            if let Ok(result) = result {
                assert_eq!(result.disposition, Disposition::Append);
                assert_eq!(
                    result.transactions,
                    vec![
                        format!("job/{}", publication.job_id),
                        format!("claim/{}", publication.job_id)
                    ]
                );
            }
        }
        runtime.close_diagnostics().await
    }
}
