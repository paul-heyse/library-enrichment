//! Named payload alternatives use the same finite field and discriminator contract as unions.
#[macro_export]
macro_rules! native_payload {
    ($(#[$attr:meta])* $vis:vis enum $name:ident { $($variant:ident($ty:ty) = $tag:literal),* $(,)? }) => {
        $crate::native_payload! { @tag "kind", "value"; $(#[$attr])* $vis enum $name { $($variant($ty) = $tag),* } }
    };
    (@tag $discriminator:literal, $content:literal; $(#[$attr:meta])* $vis:vis enum $name:ident { $($variant:ident($ty:ty) = $tag:literal),* $(,)? }) => {
        $crate::native_payload! { @impl $discriminator, $content; #[serde(tag = $discriminator, content = $content, deny_unknown_fields)] $(#[$attr])* $vis enum $name { $($variant($ty) = $tag),* } }
    };
    (@untagged $discriminator:literal; $(#[$attr:meta])* $vis:vis enum $name:ident { $($variant:ident($ty:ty) = $tag:literal),* $(,)? }) => {
        $crate::native_payload! { @impl $discriminator, ""; #[serde(untagged, deny_unknown_fields)] $(#[$attr])* $vis enum $name { $($variant($ty) = $tag),* } }
    };
    (@impl $discriminator:literal, $content:literal; $(#[$attr:meta])* $vis:vis enum $name:ident { $($variant:ident($ty:ty) = $tag:literal),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        $(#[$attr])*
        $vis enum $name { $(#[serde(rename = $tag)] $variant($ty)),* }
        impl $name {
            pub const VALUES: &[&str] = &[$($tag),*];
            pub fn kind(&self) -> &'static str { <Self as $crate::native_union::NativeUnion>::kind(self) }
            /// The selected record retains its complete native fields and transport rules.
            pub fn selected_payload(&self) -> Result<(arrow::datatypes::FieldRef, arrow::array::ArrayRef), arrow::error::ArrowError> {
                match self { $(Self::$variant(value) => Ok((
                    std::sync::Arc::new($crate::native_union::field::<$ty>($tag, $crate::native_union::Rule::Text)),
                    <$ty as $crate::native_union::Cell>::encode(&[Some(value)])?,
                ))),* }
            }
        }
        $crate::native_union_cell!($name);
        impl $crate::native_union::NativeUnion for $name {
            fn kind(&self) -> &'static str { match self { $(Self::$variant(_) => $tag),* } }
            fn fields() -> arrow::datatypes::Fields {
                let mut fields = vec![{
                    let mut field = $crate::native_union::discriminator_named($discriminator, &[$($tag),*]);
                    field.metadata_mut().insert("enrichment.union.content".into(), $content.into());
                    field
                }];
                $(let kind = <$ty as $crate::native_union::Cell>::data_type();
                  if !($content.is_empty() && matches!(&kind, arrow::datatypes::DataType::Struct(fields) if fields.is_empty())) {
                      fields.push(arrow::datatypes::Field::new($tag,kind,true));
                  })*
                fields.into()
            }
            fn encode(rows: &[&Self]) -> Result<arrow::array::ArrayRef, arrow::error::ArrowError> {
                let fields = Self::fields();
                let mut columns = vec![(fields[0].as_ref().clone(), $crate::evidence::arrow_model::cells::text(rows.iter().map(|row| row.kind())))];
                $(if let Some((_,field)) = fields.find($tag) {
                    columns.push((field.as_ref().clone(), <$ty as $crate::native_union::Cell>::encode(
                        &rows.iter().map(|row| if let Self::$variant(value) = row { Some(value) } else { None }).collect::<Vec<_>>()
                    )?));
                })*
                $crate::evidence::arrow_model::cells::structure(columns,None)
            }
            fn decode(row: $crate::evidence::arrow_model::cells::Row<'_>) -> Result<Self, arrow::error::ArrowError> {
                row.exact_fields(&Self::fields())?;
                let kind = row.text($discriminator)?;
                row.variant_named($discriminator, &[kind])?;
                match kind {
                    $($tag => Ok(Self::$variant(if Self::fields().find($tag).is_none() { <$ty as $crate::native_union::Cell>::empty_record()? } else { <$ty as $crate::native_union::Cell>::decode(row, $tag)? }))),*,
                    _ => Err($crate::evidence::arrow_model::cells::invalid(concat!("unknown ", stringify!($name)))),
                }
            }
        }
    };
}
