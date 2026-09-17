//! Purpose-specific native result payloads, captured by exact Delta version in control.
use crate::native_delta::{DeltaStore, StorageContract, missing_table, transaction_conflict};
use arrow::datatypes::{DataType, Field, Schema};
use datafusion::{
    common::{DataFusionError, Result},
    functions::core::expr_ext::FieldAccessor,
    prelude::*,
};
use enrichment_core::{
    evidence::arrow_model::expressions::{literal, record},
    native_union::{Cell, NativeStruct, NativeUnion},
    operation::results::{ResultRecord, ResultVersion},
    wire::data::ToolData,
};
use std::sync::Arc;

fn contract(tool: &str) -> Result<StorageContract> {
    if !ToolData::VALUES.contains(&tool) {
        return datafusion::common::plan_err!("unknown result relation");
    }
    let data_fields = ToolData::fields();
    let mut fields = vec![Field::new("result_artifact_id", DataType::Utf8, false)];
    for field in &ResultRecord::fields() {
        if field.name() == "data" {
            if let Some((_, payload)) = data_fields.find(tool) {
                fields.push(
                    payload
                        .as_ref()
                        .clone()
                        .with_name("data")
                        .with_nullable(false),
                );
            }
        } else {
            fields.push(field.as_ref().clone());
        }
    }
    StorageContract::new(Arc::new(Schema::new(fields)))
}

fn capture(
    tool: &str,
    table: &deltalake::DeltaTable,
    contract: &StorageContract,
) -> Result<ResultVersion> {
    Ok(ResultVersion {
        tool: tool.into(),
        table_id: table
            .snapshot()
            .map_err(|error| DataFusionError::External(Box::new(error)))?
            .metadata()
            .id()
            .to_string(),
        version: table
            .version()
            .ok_or_else(|| DataFusionError::Execution("unloaded result table".into()))?,
        contract_id: contract.identity().into(),
    })
}

/// Reconciliation keys are immutable result artifact identities. A race reloads and compares
/// the stored native record before returning its exact version; no last-writer result wins.
pub(crate) async fn retain(
    delta: &DeltaStore,
    id: &str,
    value: &ResultRecord,
) -> Result<ResultVersion> {
    let tool = value.data.kind();
    let contract = contract(tool)?;
    let name = format!("result_{tool}");
    delta.prepare_root(&name)?;
    let session = delta.runtime.session();
    let input = session.read_batch(ResultRecord::batch(std::slice::from_ref(value))?)?;
    let mut columns = vec![lit(id).alias("result_artifact_id")];
    for field in contract.semantic_schema().fields().iter().skip(1) {
        columns.push(if field.name() == "data" {
            col("data").field(tool).alias("data")
        } else {
            col(field.name())
        });
    }
    let input = input.select(columns)?;
    for _ in 0..16 {
        let table = match delta.load(&name, None).await {
            Ok(table) => table,
            Err(error) if missing_table(&error) => {
                match delta.create(&name, &contract, false).await {
                    Ok(table) => table,
                    Err(error) if transaction_conflict(&error) => continue,
                    Err(error) => {
                        // Concurrent first creation can report AlreadyExists instead of a log conflict.
                        if delta.load(&name, None).await.is_ok() {
                            continue;
                        }
                        return Err(error);
                    }
                }
            }
            Err(error) => return Err(error),
        };
        let binding = capture(tool, &table, &contract)?;
        let selected = session
            .read_table(delta.provider(&table, &contract).await?)?
            .filter(col("result_artifact_id").eq(lit(id)))?;
        let found = delta
            .runtime
            .execute(
                selected
                    .select(vec![col("result_artifact_id")])?
                    .limit(0, Some(2))?,
            )
            .await?
            .rows;
        if found != 0 {
            if found != 1 || read(delta, id, &binding).await? != *value {
                return datafusion::common::exec_err!(
                    "immutable native result identity has conflicting records"
                );
            }
            return Ok(binding);
        }
        let predecessor = i64::try_from(binding.version + 1)
            .map_err(|error| DataFusionError::External(Box::new(error)))?;
        match delta
            .append(
                table,
                &contract,
                input.clone(),
                vec![deltalake::kernel::Transaction::new(
                    format!("result/{id}"),
                    predecessor,
                )],
            )
            .await
        {
            Ok(table) => return capture(tool, &table, &contract),
            Err(error) if transaction_conflict(&error) => continue,
            Err(error) => return Err(error),
        }
    }
    datafusion::common::exec_err!("native result conflict bound exceeded")
}

pub(crate) async fn read(
    delta: &DeltaStore,
    id: &str,
    binding: &ResultVersion,
) -> Result<ResultRecord> {
    let contract = contract(&binding.tool)?;
    let table = delta
        .load(&format!("result_{}", binding.tool), Some(binding.version))
        .await?;
    if capture(&binding.tool, &table, &contract)? != *binding {
        return datafusion::common::exec_err!(
            "result table identity, version or contract mismatch"
        );
    }
    let session = delta.runtime.session();
    let input = session
        .read_table(delta.provider(&table, &contract).await?)?
        .filter(col("result_artifact_id").eq(lit(id)))?;
    let data = if ToolData::fields().find(&binding.tool).is_some() {
        record(
            &ToolData::data_type(),
            &[("tool", lit(&binding.tool)), (&binding.tool, col("data"))],
        )?
    } else {
        literal(&ToolData::default())?
    };
    let fields = ResultRecord::fields()
        .iter()
        .map(|field| {
            if field.name() == "data" {
                data.clone().alias("data")
            } else {
                col(field.name())
            }
        })
        .collect::<Vec<_>>();
    let mut rows = delta
        .runtime
        .records::<ResultRecord>(input.select(fields)?, 1)
        .await?;
    rows.pop().ok_or_else(|| {
        DataFusionError::Execution("native result missing from captured table version".into())
    })
}
