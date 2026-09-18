//! UUID-backed job, attempt and caller-interest identities have distinct semantic domains.
//! Only the wire boundary renders the prefix; Arrow and native parameters carry 16 bytes.
use crate::native_union::{Cell, Domain};
use arrow::{array::ArrayRef, datatypes::DataType, error::ArrowError};
use datafusion::common::metadata::ScalarAndMetadata;
use std::{collections::HashMap, fmt};

macro_rules! uuid_identity {
    ($name:ident, $domain:ident) => {
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(into = "String", try_from = "String")]
#[schemars(inline, extend("pattern" = format!("^{}_[0-9a-f]{{32}}$", Domain::$domain.prefix())))]
pub struct $name([u8; 16]);

impl $name {
    pub fn new() -> Self {
        Self(*uuid::Uuid::new_v4().as_bytes())
    }
    /// Mechanical filename/driver representation, never a source of authority.
    pub fn component(self) -> String { uuid::Uuid::from_bytes(self.0).simple().to_string() }
    /// Parse one captured filesystem component, under an already observed owned root.
    pub fn from_component(value: &str) -> Result<Self, &'static str> {
        Self::try_from(format!("{}_{}", Domain::$domain.prefix(), value))
    }
    pub fn parameter(self) -> ScalarAndMetadata {
        crate::evidence::arrow_model::expressions::parameter(&self)
            .expect("declared UUID identity parameter")
    }
}
impl Default for $name {
    fn default() -> Self {
        Self::new()
    }
}
impl fmt::Display for $name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}_{}",
            Domain::$domain.prefix(),
            uuid::Uuid::from_bytes(self.0).simple()
        )
    }
}
impl From<$name> for String {
    fn from(value: $name) -> Self {
        value.to_string()
    }
}
impl TryFrom<String> for $name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let digits = value
            .strip_prefix(Domain::$domain.prefix()).and_then(|value| value.strip_prefix('_'))
            .ok_or("UUID identity domain")?;
        if digits.len() != 32
            || !digits
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err("UUID identity requires 32 lowercase hexadecimal digits");
        }
        let uuid = uuid::Uuid::parse_str(digits).map_err(|_| "invalid UUID identity")?;
        Ok(Self(*uuid.as_bytes()))
    }
}
impl Cell for $name {
    fn data_type() -> DataType {
        DataType::FixedSizeBinary(Domain::$domain.byte_width())
    }
    fn metadata() -> HashMap<String, String> {
        use arrow_schema::extension::ExtensionType;
        arrow::datatypes::Field::new("identity", Self::data_type(), false)
            .with_extension_type(
                crate::native_types::IdentityType::try_new(
                    &Self::data_type(),
                    crate::native_types::TypeMetadata::new(Domain::$domain),
                )
                .expect("declared UUID identity"),
            )
            .metadata()
            .clone()
    }
    fn encode(values: &[Option<&Self>]) -> Result<ArrayRef, ArrowError> {
        <[u8; 16] as Cell>::encode(&values.iter().map(|v| v.map(|v| &v.0)).collect::<Vec<_>>())
    }
    fn decode(
        row: crate::evidence::arrow_model::cells::Row<'_>,
        name: &str,
    ) -> Result<Self, ArrowError> {
        row.fixed_binary(name).map(Self)
    }
}
impl datafusion::logical_expr::Literal for $name {
    fn lit(&self) -> datafusion::logical_expr::Expr {
        let value = self.parameter();
        datafusion::logical_expr::Expr::Literal(value.value, value.metadata)
    }
}
impl datafusion::logical_expr::Literal for &$name {
    fn lit(&self) -> datafusion::logical_expr::Expr {
        datafusion::logical_expr::Literal::lit(*self)
    }
}

    };
}
uuid_identity!(InterestId, Interest);
uuid_identity!(JobId, Job);
uuid_identity!(AttemptId, Attempt);
uuid_identity!(RetentionLeaseId, RetentionLease);
uuid_identity!(CleanupObligationId, CleanupObligation);
uuid_identity!(MaintenanceRunId, MaintenanceRun);
uuid_identity!(PrivateDirectoryId, PrivateDirectory);
uuid_identity!(PhysicalOwnerId, PhysicalOwner);
uuid_identity!(StorageReservationId, StorageReservation);
uuid_identity!(CohortId, Cohort);

impl PhysicalOwnerId {
    pub fn broker_name(self) -> String {
        format!("libenr-{}", self.component())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native_union::Rule;
    use arrow::{
        array::Array,
        datatypes::{Field, Schema},
        record_batch::RecordBatch,
    };
    use arrow_schema::extension::ExtensionType;
    use std::sync::Arc;

    #[test]
    fn typed_lifecycle_ids_preserve_bytes_and_reject_cross_domain_values() {
        let component = "00112233445566778899aabbccddeeff";
        let expected = vec![
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
            0xee, 0xff,
        ];
        let mut fields = Vec::new();
        macro_rules! check {
            ($ty:ty) => {{
                let value = <$ty>::from_component(component).unwrap();
                assert_eq!(value.component(), component);
                assert_eq!(
                    value.parameter().value,
                    datafusion::common::ScalarValue::FixedSizeBinary(16, Some(expected.clone()))
                );
                let field = crate::native_union::field::<$ty>("id", Rule::Text);
                let array = <$ty>::encode(&[Some(&value)]).unwrap();
                let mut wire = Vec::new();
                crate::native_json::write_value(
                    &mut wire,
                    128,
                    &Arc::new(field.clone()),
                    array.as_ref(),
                    0,
                )
                .unwrap();
                assert_eq!(serde_json::from_slice::<$ty>(&wire).unwrap(), value);
                assert_eq!(
                    serde_json::from_slice::<String>(&wire).unwrap(),
                    value.to_string()
                );
                for invalid in [
                    "../escape",
                    "00112233445566778899AABBCCDDEEFF",
                    "00112233-4455-6677-8899-aabbccddeeff",
                    "00",
                ] {
                    assert!(<$ty>::from_component(invalid).is_err());
                }
                assert!(
                    <$ty>::try_from("job_00112233445566778899aabbccddeeff".to_owned()).is_err()
                );
                for other in &fields {
                    assert!(
                        crate::native_analysis::compatible(&field, other, "lifecycle domain")
                            .is_err()
                    );
                }
                fields.push(field);
            }};
        }
        check!(RetentionLeaseId);
        check!(CleanupObligationId);
        check!(MaintenanceRunId);
        check!(PrivateDirectoryId);
        check!(PhysicalOwnerId);
        check!(StorageReservationId);
        check!(CohortId);
        assert_eq!(fields.len(), 7);
        assert_eq!(
            PhysicalOwnerId::from_component(component)
                .unwrap()
                .broker_name(),
            "libenr-00112233445566778899aabbccddeeff"
        );
    }

    #[test]
    fn uuid_identities_have_distinct_binary_and_wire_domains() {
        let text = "interest_00112233445566778899aabbccddeeff";
        let interest = InterestId::try_from(text.to_owned()).unwrap();
        let field = Arc::new(crate::native_union::field::<InterestId>(
            "interest",
            Rule::Text,
        ));
        let array = InterestId::encode(&[Some(&interest)]).unwrap();
        assert_eq!(array.data_type(), &DataType::FixedSizeBinary(16));
        assert_eq!(
            array
                .as_any()
                .downcast_ref::<arrow::array::FixedSizeBinaryArray>()
                .unwrap()
                .value(0),
            &[
                0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd,
                0xee, 0xff
            ]
        );
        let batch = RecordBatch::try_new(
            Arc::new(Schema::new(vec![field.clone()])),
            vec![array.clone()],
        )
        .unwrap();
        let rows = crate::evidence::arrow_model::cells::RowSet::batch(&batch).unwrap();
        assert_eq!(
            InterestId::decode(rows.row(0), "interest").unwrap(),
            interest
        );
        let mut encoded = Vec::new();
        crate::native_json::write_value(&mut encoded, 128, &field, array.as_ref(), 0).unwrap();
        assert_eq!(encoded, serde_json::to_vec(&text).unwrap());
        assert_eq!(
            serde_json::from_slice::<InterestId>(&encoded).unwrap(),
            interest
        );
        for invalid in [
            "interest_1",
            "job_00112233445566778899aabbccddeeff",
            "interest_00112233445566778899AABBCCDDEEFF",
            "interest_00112233-4455-6677-8899-aabbccddeeff",
        ] {
            assert!(InterestId::try_from(invalid.to_owned()).is_err());
        }
        assert!(
            crate::native_types::IdentityType::try_new(
                &DataType::FixedSizeBinary(32),
                crate::native_types::TypeMetadata::new(Domain::Interest)
            )
            .is_err()
        );
        assert!(
            crate::native_types::IdentityType::try_new(
                &DataType::FixedSizeBinary(16),
                crate::native_types::TypeMetadata::new(Domain::Snapshot)
            )
            .is_err()
        );
        assert!(
            crate::native_analysis::compatible(
                &field,
                &Field::new("plain", DataType::FixedSizeBinary(16), false),
                "equality"
            )
            .is_err()
        );
        let job = JobId::try_from("job_00112233445566778899aabbccddeeff".to_owned()).unwrap();
        assert!(JobId::try_from(text.to_owned()).is_err());
        let job_field = crate::native_union::field::<JobId>("job", Rule::Text);
        assert!(crate::native_analysis::compatible(&field, &job_field, "UUID domains").is_err());
        // Identical physical bytes never establish identity equality across domains.
        assert_eq!(job.parameter().value, interest.parameter().value);
        assert_ne!(job.parameter().metadata, interest.parameter().metadata);
        let attempt =
            AttemptId::try_from("attempt_00112233445566778899aabbccddeeff".to_owned()).unwrap();
        let attempt_field = crate::native_union::field::<AttemptId>("attempt", Rule::Text);
        assert!(
            crate::native_analysis::compatible(&attempt_field, &job_field, "attempt versus job")
                .is_err()
        );
        assert!(AttemptId::try_from("job_00112233445566778899aabbccddeeff".to_owned()).is_err());
        assert_eq!(attempt.parameter().value, job.parameter().value);
        let encoded = AttemptId::encode(&[Some(&attempt)]).unwrap();
        let mut wire = Vec::new();
        crate::native_json::write_value(
            &mut wire,
            128,
            &Arc::new(attempt_field),
            encoded.as_ref(),
            0,
        )
        .unwrap();
        assert_eq!(wire, br#""attempt_00112233445566778899aabbccddeeff""#);
        assert_eq!(serde_json::from_slice::<AttemptId>(&wire).unwrap(), attempt);
        assert_eq!(
            serde_json::to_value(job).unwrap(),
            "job_00112233445566778899aabbccddeeff"
        );
        let hash = crate::native_id::from_hash(
            Domain::Job,
            datafusion::prelude::lit(datafusion::common::ScalarValue::Binary(Some(vec![0; 32]))),
        );
        assert!(
            datafusion::logical_expr::ExprSchemable::to_field(
                &hash,
                &datafusion::common::DFSchema::empty()
            )
            .is_err()
        );
    }
}
