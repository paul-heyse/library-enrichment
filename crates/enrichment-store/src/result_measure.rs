//! Native transport kernel conformance on isolated Arrow values.
mod tests {
    use datafusion::prelude::col;
    use enrichment_core::{
        native_union::{Cell, NativeStruct, Rule},
        operation::results::ResultRecord,
    };
    enrichment_core::native_struct! { struct Measured { bytes: u64 => Rule::Text } }

    #[tokio::test]
    async fn native_size_equals_actual_wire_bytes_with_unicode_escaping_and_exact_numbers() {
        let root = tempfile::tempdir().unwrap();
        let runtime = crate::runtime::QueryRuntime::new(root.path(), Default::default()).unwrap();
        for fixture in [
            include_str!("../../../tests/fixtures/wire/ok.fixture.json"),
            include_str!("../../../tests/fixtures/wire/partial.fixture.json"),
            include_str!("../../../tests/fixtures/wire/pending.fixture.json"),
            include_str!("../../../tests/fixtures/wire/error.fixture.json"),
        ] {
            let mut value: enrichment_core::wire::Envelope = serde_json::from_str(fixture).unwrap();
            value.summary = "é α 🦀 \"quoted\" \\ slash\nline\tcontrol".into();
            let record = ResultRecord::from_envelope(&value).unwrap();
            let session = runtime.session();
            let frame = crate::native_catalog::batch(
                &session,
                "result_measure",
                ResultRecord::batch(&[record]).unwrap(),
            )
            .unwrap();
            let expr = enrichment_core::evidence::arrow_model::expressions::record(
                &ResultRecord::data_type(),
                &ResultRecord::fields()
                    .iter()
                    .map(|field| (field.name().as_str(), col(field.name())))
                    .collect::<Vec<_>>(),
            )
            .unwrap();
            use enrichment_core::mcp_delivery::{DeliveryProfile, ProtocolEra};
            for profile in [
                DeliveryProfile::McpResourceStdio {
                    era: ProtocolEra::Modern,
                    framing_bytes: 37,
                    uri: "library-evidence://artifacts/art_fixture".into(),
                },
                DeliveryProfile::Envelope,
                DeliveryProfile::McpStdio {
                    era: ProtocolEra::Classic,
                    framing_bytes: 35,
                },
                DeliveryProfile::McpStdio {
                    era: ProtocolEra::Modern,
                    framing_bytes: 42,
                },
            ] {
                let measured = frame
                    .clone()
                    .select(vec![
                        enrichment_core::native_transport::delivery(
                            runtime.session().runtime_env().memory_pool.clone(),
                            expr.clone(),
                            &value.request_id,
                            profile.clone(),
                        )
                        .alias("bytes"),
                    ])
                    .unwrap();
                let rows = runtime.records::<Measured>(measured, 1).await.unwrap();
                let framing = match &profile {
                    DeliveryProfile::Envelope => 0,
                    DeliveryProfile::McpStdio { framing_bytes, .. }
                    | DeliveryProfile::McpResourceStdio { framing_bytes, .. } => {
                        *framing_bytes as usize
                    }
                };
                let expected = serde_json::to_vec(
                    &profile
                        .project(&value, &runtime.session().runtime_env().memory_pool)
                        .unwrap(),
                )
                .unwrap()
                .len()
                    + framing;
                assert_eq!(rows[0].bytes as usize, expected);
                // Each codec/outcome is measured above; exercise the exact selection boundary
                // once per transport profile rather than repeating the same predicate.
                if value.status() == enrichment_core::wire::Status::Ok {
                    assert!(
                        crate::result_delivery::inline(&runtime, &value, expected, profile.clone())
                            .await
                            .unwrap()
                    );
                    assert!(
                        !crate::result_delivery::inline(&runtime, &value, expected - 1, profile)
                            .await
                            .unwrap()
                    );
                }
            }
        }
        runtime.close_diagnostics().await.unwrap();
    }
}
