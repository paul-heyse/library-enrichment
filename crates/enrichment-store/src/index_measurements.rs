//! Bounded manual alternatives for retained research key indexes.
use super::*;
use datafusion::{datasource::MemTable, prelude::col};
use std::time::Instant;

#[tokio::test]
#[ignore = "manual paired P8 retained-index strategy measurement"]
async fn paired_retained_index_strategies() {
    let dir = tempfile::tempdir().unwrap();
    for rows in [0, 3, 8192] {
        for round in 0..3 {
            let mut expected = None;
            for strategy in if round % 2 == 0 {
                ["ipc", "sorted_ipc", "memory"]
            } else {
                ["memory", "sorted_ipc", "ipc"]
            } {
                let runtime = QueryRuntime::new(
                    &dir.path().join(format!("{rows}-{round}-{strategy}")),
                    Default::default(),
                )
                .unwrap();
                let session = runtime.session();
                let sql = format!(
                    "SELECT CAST(0 AS BIGINT UNSIGNED) AS plan, CAST(value AS VARCHAR) AS key, CASE WHEN value % 7 = 0 THEN NULL ELSE CAST(value % 41 AS VARCHAR) END AS label FROM generate_series(1, {rows}) ORDER BY value * 17 % 101"
                );
                let frame = session.sql(&sql).await.unwrap();
                let order = vec![col("label").sort(true, true), col("key").sort(true, false)];
                let start = Instant::now();
                let mut memory = None;
                let provider: Arc<dyn TableProvider> = match strategy {
                    "memory" => {
                        let reservation = MemoryConsumer::new("measured_index_memory")
                            .register(&session.runtime_env().memory_pool);
                        let output = runtime
                            .fold_blocking(
                                frame,
                                QueryFamily::ComparisonKeys,
                                8192,
                                (vec![], reservation),
                                |(mut batches, reservation), batch| {
                                    reservation.try_grow(batch.get_array_memory_size())?;
                                    batches.push(batch.clone());
                                    Ok((batches, reservation))
                                },
                            )
                            .await
                            .unwrap();
                        assert_eq!(output.rows, rows);
                        let (batches, reservation) = output.value;
                        memory = Some(reservation);
                        let schema = if let Some(batch) = batches.first() {
                            batch.schema()
                        } else {
                            Arc::new(arrow::datatypes::Schema::new(vec![
                                arrow::datatypes::Field::new(
                                    "plan",
                                    arrow::datatypes::DataType::UInt64,
                                    false,
                                ),
                                arrow::datatypes::Field::new(
                                    "key",
                                    arrow::datatypes::DataType::Utf8,
                                    true,
                                ),
                                arrow::datatypes::Field::new(
                                    "label",
                                    arrow::datatypes::DataType::Utf8,
                                    true,
                                ),
                            ]))
                        };
                        Arc::new(MemTable::try_new(schema, vec![batches]).unwrap())
                    }
                    _ => {
                        let frame = if strategy == "sorted_ipc" {
                            frame.sort(order.clone()).unwrap()
                        } else {
                            frame
                        };
                        let completed = materialize(&runtime, frame, QueryFamily::ComparisonKeys)
                            .await
                            .unwrap();
                        assert_eq!(completed.rows, rows as u64);
                        assert_eq!(runtime.diagnostic_summary().index_reads, 0);
                        completed.provider
                    }
                };
                let construction_us = start.elapsed().as_micros();
                let retained_memory = session.runtime_env().memory_pool.reserved();
                let spill_bytes = session.runtime_env().disk_manager.used_disk_space();
                let mut page_us = vec![];
                let mut outputs = vec![];
                for page in 0..4 {
                    let frame = session
                        .read_table(provider.clone())
                        .unwrap()
                        .sort(order.clone())
                        .unwrap()
                        .limit(page * 16, Some(16))
                        .unwrap();
                    let start = Instant::now();
                    let output = runtime.execute(frame).await.unwrap();
                    page_us.push(start.elapsed().as_micros());
                    outputs.push(
                        arrow::util::pretty::pretty_format_batches(&output.batches)
                            .unwrap()
                            .to_string(),
                    );
                }
                if let Some(expected) = &expected {
                    assert_eq!(expected, &outputs);
                } else {
                    expected = Some(outputs);
                }
                drop(provider);
                drop(memory);
                assert_eq!(session.runtime_env().memory_pool.reserved(), 0);
                assert_eq!(session.runtime_env().disk_manager.used_disk_space(), 0);
                eprintln!(
                    "P14_INDEX {}",
                    serde_json::json!({"rows":rows,"round":round,"strategy":strategy,"construction_us":construction_us,"page_us":page_us,"retained_memory":retained_memory,"spill_bytes":spill_bytes,"counters":runtime.operational_counters()})
                );
            }
        }
    }
}
