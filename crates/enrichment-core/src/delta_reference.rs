//! Shared semantic references to native Delta state. External table identifiers are opaque.
use crate::{identity::SchemaContractId, native_union::Rule};

crate::native_struct! {
    #[derive(Hash)]
    pub struct DeltaTableRef {
        table_uri: String => Rule::NonEmpty,
        table_id: String => Rule::NonEmpty,
        contract_id: SchemaContractId => Rule::Text,
    }
}
crate::native_struct! {
    #[derive(Hash)]
    pub struct DeltaVersionRef {
        table: DeltaTableRef => Rule::Text,
        version: u64 => Rule::Text,
    }
}
crate::native_struct! {
    pub struct TableSelection {
        source: DeltaVersionRef => Rule::Text,
        row: Option<crate::operation::retention::RowKey> => Rule::Text,
    }
}
crate::native_struct! {
    @checked RawCdfWindow;
    #[serde(try_from = "RawCdfWindow")]
    pub struct CdfWindow {
        table: DeltaTableRef => Rule::Text,
        start: u64 => Rule::Text,
        end: u64 => Rule::Text,
    }
}
impl TryFrom<RawCdfWindow> for CdfWindow {
    type Error = String;
    fn try_from(value: RawCdfWindow) -> Result<Self, Self::Error> {
        if value.start > value.end || value.end > i64::MAX as u64 {
            return Err("invalid inclusive CDF version range".into());
        }
        Ok(Self {
            table: value.table,
            start: value.start,
            end: value.end,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_union::{Cell, NativeStruct};

    #[test]
    fn typed_delta_references_preserve_external_ids_and_refuse_old_shapes() {
        let contract =
            SchemaContractId::try_from(format!("schema_contract_{}", "01".repeat(32))).unwrap();
        assert_eq!(contract.as_bytes(), &[1; 32]);
        assert_eq!(
            contract.parameter().value,
            datafusion::common::ScalarValue::FixedSizeBinary(32, Some(vec![1; 32]))
        );
        let table = DeltaTableRef {
            table_uri: "evidence_symbols".into(),
            table_id: "external-provider-id/opaque".into(),
            contract_id: contract.clone(),
        };
        let window = CdfWindow {
            table: table.clone(),
            start: 3,
            end: 7,
        };
        let wire = serde_json::to_value(&window).unwrap();
        assert_eq!(wire["table"]["contract_id"], contract.to_string());
        assert_eq!(
            serde_json::from_value::<CdfWindow>(wire.clone()).unwrap(),
            window
        );
        let batch = CdfWindow::batch(std::slice::from_ref(&window)).unwrap();
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert_eq!(
            <CdfWindow as NativeStruct>::decode(rows.row(0)).unwrap(),
            window
        );
        for (start, end) in [(8u64, 7u64), (0, i64::MAX as u64 + 1)] {
            let mut invalid = wire.clone();
            invalid["start"] = start.to_string().into();
            invalid["end"] = end.to_string().into();
            assert!(serde_json::from_value::<CdfWindow>(invalid).is_err());
        }
        let old = serde_json::json!({"table_uri":table.table_uri,"table_id":table.table_id,"contract_id":contract.to_string(),"version":"7"});
        assert!(serde_json::from_value::<DeltaVersionRef>(old).is_err());
        let wrong_domain = crate::identity::RetentionPolicyId::try_from(format!(
            "retention_policy_{}",
            "01".repeat(32)
        ))
        .unwrap();
        assert_eq!(contract.parameter().value, wrong_domain.parameter().value);
        assert_ne!(
            SchemaContractId::metadata(),
            crate::identity::RetentionPolicyId::metadata()
        );
        assert!(SchemaContractId::try_from(wrong_domain.to_string()).is_err());
    }
}
