//! Mechanical Arrow projections retain the full declared Field at the DataFusion seam.
//! Arrow owns decoding and union selection; admission and reference policy remain expressions.
use super::*;

/// A logical value field after one storage-encoding layer is removed.
pub fn decoded_field(field: &Field) -> Option<Field> {
    let kind = match field.data_type() {
        DataType::Dictionary(_, value) => value.as_ref().clone(),
        DataType::RunEndEncoded(_, value) => {
            return Some(
                value
                    .as_ref()
                    .clone()
                    .with_nullable(!crate::native_schema::required(field) && value.is_nullable()),
            );
        }
        DataType::ListView(item) => DataType::List(item.clone()),
        DataType::LargeListView(item) => DataType::LargeList(item.clone()),
        _ => return None,
    };
    Some(field.clone().with_data_type(kind))
}

/// Decode a physical encoding with Arrow's native cast, preserving semantic field metadata.
pub fn decoded_values() -> ScalarUDF {
    ScalarUDF::from(Projection {
        kind: ProjectionKind::Decoded,
        signature: Signature::user_defined(Volatility::Immutable),
    })
}

/// Select an Arrow union child. Inactive rows are NULL; the branch Field keeps its metadata.
pub fn union_member(type_id: i8) -> ScalarUDF {
    ScalarUDF::from(Projection {
        kind: ProjectionKind::Union(type_id),
        signature: Signature::user_defined(Volatility::Immutable),
    })
}

pub(super) fn is_projection(function: &ScalarUDF) -> bool {
    function.inner().downcast_ref::<Projection>().is_some()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum ProjectionKind {
    Decoded,
    Union(i8),
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Projection {
    kind: ProjectionKind,
    signature: Signature,
}
impl Projection {
    fn field(&self, field: &Field) -> Result<Field> {
        match self.kind {
            ProjectionKind::Decoded => decoded_field(field).ok_or_else(|| {
                datafusion::common::plan_datafusion_err!(
                    "value projection requires an encoded container"
                )
            }),
            ProjectionKind::Union(type_id) => {
                let DataType::Union(fields, _) = field.data_type() else {
                    return datafusion::common::plan_err!("union projection requires Union");
                };
                fields
                    .iter()
                    .find(|(id, _)| *id == type_id)
                    .map(|(_, child)| child.as_ref().clone().with_nullable(true))
                    .ok_or_else(|| {
                        datafusion::common::plan_datafusion_err!(
                            "union projection names an absent branch"
                        )
                    })
            }
        }
    }
}
impl ScalarUDFImpl for Projection {
    fn name(&self) -> &str {
        match self.kind {
            ProjectionKind::Decoded => "native_decoded_values",
            ProjectionKind::Union(_) => "native_union_member",
        }
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("value projection requires full fields")
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        let [kind] = take_function_args(self.name(), types)?;
        self.field(&Field::new("value", kind.clone(), true))?;
        Ok(types.to_vec())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let [field] = take_function_args(self.name(), args.arg_fields)?;
        crate::native_schema::validate_derived_fields(&vec![field.clone()].into())?;
        // A nullable dictionary key or encoded value can introduce logical NULLs.
        Ok(Arc::new(
            self.field(field)?
                .with_name(self.name())
                .with_nullable(true),
        ))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let [value] = take_function_args(self.name(), &args.args)?;
        let value = value.to_array(args.number_rows)?;
        let array = match self.kind {
            ProjectionKind::Decoded => {
                arrow::compute::cast(value.as_ref(), args.return_field.data_type())?
            }
            ProjectionKind::Union(type_id) => arrow::compute::union_extract_by_id(
                datafusion::common::cast::as_union_array(&value)?,
                type_id,
            )?,
        };
        Ok(ColumnarValue::Array(array))
    }
}
