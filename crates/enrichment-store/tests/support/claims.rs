/// Obtain a real native claim for a fixture publication; no fabricated owner/fence witness.
pub async fn publication_fence(
    repo: &enrichment_store::repository::EvidenceRepository,
    job: &str,
    arguments: enrichment_store::control_jobs::Arguments,
) -> enrichment_store::control::PublicationFence {
    use enrichment_store::control_jobs::JobStore;
    let jobs = JobStore::new(
        repo.catalog.clone(),
        repo.runtime.clone(),
        "fixture_owner".into(),
        std::sync::Arc::new(enrichment_core::config::Config::default()),
    );
    jobs.submit(
        jobs.command_with_id(job.into(), arguments).await.unwrap(),
        format!("interest_{job}"),
    )
    .await
    .unwrap();
    let policy = enrichment_store::execution_policy::Policy::bind(
        &repo.runtime,
        &enrichment_core::config::Config::default(),
        enrichment_store::execution_policy::Capture {
            execution_root: "unused-static-acquisition".into(),
            containment_identity: None,
            containment_error: None,
            receipt: None,
            receipt_error: Some("no qualified image".into()),
            cleanup_error: None,
        },
    )
    .await
    .unwrap();
    assert!(
        jobs.start(job, &format!("attempt_{job}"), &policy)
            .await
            .unwrap()
    );
    let pin = jobs.pin().await.unwrap();
    let claim = jobs.claim(&pin, job).await.unwrap().unwrap();
    enrichment_store::control::PublicationFence {
        owner: claim.owner,
        fence: claim.fence,
    }
}
