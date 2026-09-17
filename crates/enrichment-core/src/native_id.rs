//! Checked identity construction and diagnostic representation at native expression boundaries.
//! Hashing, comparisons and hex formatting remain built-in DataFusion operations.
use crate::{
    native_types::{IdentityType, TypeMetadata},
    native_union::Domain,
};
use arrow::{
    array::ArrayRef,
    datatypes::{DataType, Field, FieldRef},
};
use arrow_schema::extension::ExtensionType;
use datafusion::{
    common::Result,
    logical_expr::{
        ColumnarValue, Expr, ReturnFieldArgs, ScalarFunctionArgs, ScalarUDF, ScalarUDFImpl,
        Signature, Volatility,
    },
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    FromHash,
    Bytes,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct IdentityProjection {
    domain: Domain,
    direction: Direction,
    name: String,
    signature: Signature,
}

fn function(domain: Domain, direction: Direction) -> ScalarUDF {
    let name = format!(
        "native_identity_{}_{}/10",
        domain.prefix(),
        match direction {
            Direction::FromHash => "from_hash",
            Direction::Bytes => "bytes",
        }
    );
    ScalarUDF::from(IdentityProjection {
        domain,
        direction,
        name,
        signature: Signature::user_defined(Volatility::Immutable),
    })
}
fn identity_field(domain: Domain, nullable: bool) -> Field {
    let kind = DataType::FixedSizeBinary(32);
    let extension =
        IdentityType::try_new(&kind, TypeMetadata::new(domain)).expect("declared identity domain");
    Field::new("identity", kind, nullable).with_extension_type(extension)
}

/// Only the typed native hash path grants an identity domain to its 32-byte result.
pub fn from_hash(domain: Domain, hash: Expr) -> Expr {
    function(domain, Direction::FromHash).call(vec![hash])
}

/// One-way diagnostic projection: the result is text and cannot compare as an identity.
pub fn diagnostic(field: &Field, value: Expr) -> Result<Expr> {
    use datafusion::{
        functions::{encoding::expr_fn::encode, string::expr_fn::concat},
        prelude::lit,
    };
    crate::native_types::validate_field(field)?;
    if field
        .metadata()
        .get("ARROW:extension:name")
        .map(String::as_str)
        == Some(IdentityType::NAME)
    {
        let metadata = IdentityType::deserialize_metadata(
            field
                .metadata()
                .get("ARROW:extension:metadata")
                .map(String::as_str),
        )?;
        let domain = *metadata.meaning();
        Ok(concat(vec![
            lit(format!("{}_", domain.prefix())),
            encode(
                function(domain, Direction::Bytes).call(vec![value]),
                lit("hex"),
            ),
        ]))
    } else if crate::native_analysis::has_semantics(field) {
        datafusion::common::plan_err!("diagnostic key has no declared identity representation")
    } else {
        Ok(concat(vec![lit(""), value]))
    }
}

pub(crate) fn is_projection(function: &ScalarUDF) -> bool {
    function
        .inner()
        .downcast_ref::<IdentityProjection>()
        .is_some()
}

impl ScalarUDFImpl for IdentityProjection {
    fn name(&self) -> &str {
        &self.name
    }
    fn signature(&self) -> &Signature {
        &self.signature
    }
    fn return_type(&self, _: &[DataType]) -> Result<DataType> {
        datafusion::common::plan_err!("identity projection requires full fields")
    }
    fn coerce_types(&self, types: &[DataType]) -> Result<Vec<DataType>> {
        let expected = match self.direction {
            Direction::FromHash => DataType::Binary,
            Direction::Bytes => DataType::FixedSizeBinary(32),
        };
        if types != [expected] {
            return datafusion::common::plan_err!(
                "identity projection input representation mismatch"
            );
        }
        Ok(types.to_vec())
    }
    fn return_field_from_args(&self, args: ReturnFieldArgs) -> Result<FieldRef> {
        let input = match self.direction {
            Direction::FromHash => Field::new("hash", DataType::Binary, true),
            Direction::Bytes => identity_field(self.domain, true),
        };
        crate::native_schema::function_arguments(args.arg_fields, &[Arc::new(input)])?;
        if args.scalar_arguments.len() != 1 {
            return datafusion::common::plan_err!("identity projection scalar arity");
        }
        let nullable = args.arg_fields[0].is_nullable();
        Ok(Arc::new(match self.direction {
            Direction::FromHash => identity_field(self.domain, nullable),
            Direction::Bytes => Field::new(self.name(), DataType::Binary, nullable),
        }))
    }
    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> Result<ColumnarValue> {
        crate::native_schema::function_call(self, &args)?;
        let input = args.args[0].to_array(args.number_rows)?;
        let output: ArrayRef = arrow::compute::cast(&input, args.return_field.data_type())?;
        Ok(ColumnarValue::Array(output))
    }
}
