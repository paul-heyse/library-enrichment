//! One admitted equality selection for semantic reads, writer recovery and physical deletion.
use super::RowKey;
use crate::native_delta::StorageContract;
use arrow::datatypes::{Field, Schema};
use datafusion::{
    common::{Column, Result, metadata::ScalarAndMetadata},
    logical_expr::Expr,
};

pub(crate) struct Selection {
    column: Column,
    value: ScalarAndMetadata,
}

impl Selection {
    pub(crate) fn admit(row: &RowKey, schema: &Schema) -> Result<Self> {
        let expected = schema.field_with_name(&row.column)?;
        if !enrichment_core::native_schema::required(expected) {
            return datafusion::common::plan_err!("retention row key requires a required field");
        }
        let value = row.value.parameter()?;
        let actual = Field::new(&row.column, value.value.data_type(), false).with_metadata(
            value
                .metadata
                .as_ref()
                .map(|metadata| {
                    metadata
                        .inner()
                        .iter()
                        .map(|(key, value)| (key.clone(), value.clone()))
                        .collect()
                })
                .unwrap_or_default(),
        );
        enrichment_core::native_schema::function_arguments(
            &[actual.into()],
            &[expected.clone().into()],
        )?;
        Ok(Self {
            column: Column::from_name(&row.column),
            value,
        })
    }

    pub(crate) fn expression(self) -> Expr {
        Expr::Column(self.column).eq(Expr::Literal(self.value.value, self.value.metadata))
    }

    pub(crate) fn storage(row: &RowKey, contract: &StorageContract) -> Result<Expr> {
        let admitted = Self::admit(row, &contract.semantic_schema())?;
        let storage = contract.storage_schema();
        let target = storage.field_with_name(&row.column)?;
        // Only this checked representation boundary erases extension metadata. The
        // physical table cannot supply a semantic domain for unannotated bytes.
        let value = admitted.value.value.cast_to(target.data_type())?;
        Ok(Expr::Column(admitted.column).eq(Expr::Literal(value, None)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::retention::RowValue;
    use enrichment_core::{
        identity::*,
        native_union::{Cell, NativeStruct, Rule, field},
    };
    use std::sync::Arc;

    enrichment_core::native_struct! { struct Selected { key: RowKey => Rule::Text } }

    #[tokio::test]
    async fn plan19_typed_definition_row_keys_admit_domains_and_lower_native_binary() -> Result<()>
    {
        let directory = tempfile::tempdir()?;
        let runtime = crate::runtime::QueryRuntime::new(directory.path(), Default::default())?;
        let mut cases = Vec::new();
        macro_rules! case {
            ($id:ident) => {{
                let id = $id::try_from(format!("{}_{}", $id::PREFIX, "1".repeat(64))).unwrap();
                let other = $id::try_from(format!("{}_{}", $id::PREFIX, "2".repeat(64))).unwrap();
                assert!($id::try_from(format!("{}_{}", $id::PREFIX, "G".repeat(64))).is_err());
                assert!($id::try_from(format!("{}_{}", $id::PREFIX, "1".repeat(62))).is_err());
                let field = field::<$id>("key.with.dot", Rule::Text);
                cases.push((
                    field,
                    id.row_value(),
                    $id::encode(&[Some(&id), Some(&other)])?,
                ));
            }};
        }
        case!(OperationPolicyId);
        case!(ProcessOperationId);
        case!(ProcessEffectId);
        case!(StaticWorkerEffectId);
        case!(RustdocDecoderEffectId);
        case!(SemanticConversationId);
        case!(RegistryCaptureId);
        case!(RevisionCaptureId);
        let cohort = CohortId::from_component("00112233445566778899aabbccddeeff").unwrap();
        let other_cohort = CohortId::from_component("ffeeddccbbaa99887766554433221100").unwrap();
        cases.push((
            field::<CohortId>("key.with.dot", Rule::Text),
            RowValue::Cohort { value: cohort },
            CohortId::encode(&[Some(&cohort), Some(&other_cohort)])?,
        ));
        let contract_id =
            SchemaContractId::try_from(format!("schema_contract_{}", "01".repeat(32))).unwrap();
        let other_contract =
            SchemaContractId::try_from(format!("schema_contract_{}", "02".repeat(32))).unwrap();
        cases.push((
            field::<SchemaContractId>("key.with.dot", Rule::Text),
            RowValue::SchemaContract {
                value: contract_id.clone(),
            },
            SchemaContractId::encode(&[Some(&contract_id), Some(&other_contract)])?,
        ));
        cases.push((
            Field::new("key.with.dot", arrow::datatypes::DataType::Utf8, false),
            RowValue::Text {
                value: "first".into(),
            },
            Arc::new(arrow::array::StringArray::from(vec!["first", "second"])),
        ));
        for (index, (field, value, array)) in cases.iter().enumerate() {
            let row = RowKey {
                column: field.name().clone(),
                value: value.clone(),
            };
            let encoded = Selected::batch(&[Selected { key: row.clone() }])?;
            let rows = enrichment_core::evidence::arrow_model::cells::RowSet::batch(&encoded)?;
            assert_eq!(<Selected as NativeStruct>::decode(rows.row(0))?.key, row);
            assert_eq!(
                serde_json::from_value::<RowKey>(serde_json::to_value(&row).unwrap()).unwrap(),
                row
            );
            let schema = Arc::new(Schema::new(vec![field.clone()]));
            let contract = StorageContract::new(schema.clone())?;
            let session = runtime.session();
            let batch =
                arrow::record_batch::RecordBatch::try_new(schema.clone(), vec![array.clone()])?;
            let input = crate::native_catalog::batch(&session, "typed_row_selection", batch)?;
            assert_eq!(
                runtime
                    .execute(input.filter(Selection::admit(&row, &schema)?.expression())?)
                    .await?
                    .rows,
                1
            );
            let storage = contract.storage_schema();
            let array = arrow::compute::cast(array, storage.field(0).data_type())?;
            let batch = arrow::record_batch::RecordBatch::try_new(storage, vec![array])?;
            let input = crate::native_catalog::batch(&session, "physical_row_selection", batch)?;
            assert_eq!(
                runtime
                    .execute(input.filter(Selection::storage(&row, &contract)?)?)
                    .await?
                    .rows,
                1
            );
            for (other, (_, value, _)) in cases.iter().enumerate() {
                let key = RowKey {
                    value: value.clone(),
                    ..row.clone()
                };
                assert_eq!(Selection::admit(&key, &schema).is_ok(), index == other);
                assert_eq!(Selection::storage(&key, &contract).is_ok(), index == other);
            }
            assert!(
                Selection::admit(
                    &RowKey {
                        column: "missing".into(),
                        ..row.clone()
                    },
                    &schema
                )
                .is_err()
            );
            let optional = field.clone().with_nullable(true).with_metadata(
                field
                    .metadata()
                    .iter()
                    .filter(|(key, _)| key.as_str() != "enrichment.null")
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect(),
            );
            assert!(Selection::admit(&row, &Schema::new(vec![optional])).is_err());
            if index < 8 {
                let unannotated = Field::new(field.name(), field.data_type().clone(), false);
                assert!(Selection::admit(&row, &Schema::new(vec![unannotated])).is_err());
                let wrong_width = field
                    .clone()
                    .with_data_type(arrow::datatypes::DataType::FixedSizeBinary(16));
                assert!(Selection::admit(&row, &Schema::new(vec![wrong_width])).is_err());
            }
        }
        // Collection admission compares domains before native coercion. Concat must
        // also refuse a scalar item annotation its upstream return type would erase.
        use datafusion::functions_nested::expr_fn::{array_concat, array_except, array_has_any};
        use enrichment_core::evidence::arrow_model::expressions::literal;
        let policy =
            vec![OperationPolicyId::try_from(format!("policy_{}", "1".repeat(64))).unwrap()];
        let effect =
            vec![ProcessEffectId::try_from(format!("process_effect_{}", "1".repeat(64))).unwrap()];
        for expression in [
            array_except(literal(&policy)?, literal(&effect)?),
            array_has_any(literal(&policy)?, literal(&effect)?),
            array_concat(vec![literal(&policy)?, literal(&policy)?]),
        ] {
            let frame = runtime
                .session()
                .read_empty()?
                .select(vec![expression.alias("invalid")]);
            if let Ok(frame) = frame {
                assert!(runtime.execute(frame).await.is_err());
            }
        }
        assert!(serde_json::from_str::<RowKey>(r#"{"column":"key","value":"legacy"}"#).is_err());
        assert!(serde_json::from_str::<RowKey>(r#"{"column":"key","value":null}"#).is_err());
        runtime.close_diagnostics().await?;
        Ok(())
    }
}
