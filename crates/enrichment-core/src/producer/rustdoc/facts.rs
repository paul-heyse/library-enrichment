//! Mechanical rustdoc and public-api decoding into Arrow. This boundary does not choose
//! visible declarations, traverse public paths, associate renderings or create evidence IDs.
use crate::evidence::declarations::{
    RustAbi, RustBody, RustCallable, RustDetails, RustParameter, RustStability, RustStabilityLevel,
};
use crate::native_union::{NativeStruct, Rule, Unit};
use arrow::{
    datatypes::{Schema, SchemaRef},
    error::ArrowError,
    record_batch::RecordBatch,
};
use rustdoc_types::{Attribute, Crate, Id, Item, ItemEnum, StructKind, VariantKind, Visibility};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

pub const VERSION: &str = "rustdoc-arrow-facts/2";
pub const PUBLIC_API_VERSION: &str = "0.52.2-format61-1";
pub const MAX_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_ROWS: u64 = 4_000_000;
pub const BATCH_ROWS: usize = 128;
pub const BATCH_BYTES: usize = 2 * 1024 * 1024;

crate::native_vocabulary! {
    #[derive(PartialOrd, Ord)]
    pub enum Fact {
        Header = "rust_header", Items = "rust_items", Links = "rust_links",
        Paths = "rust_paths", Externals = "rust_externals", Signatures = "rust_signatures",
        Missing = "rust_missing",
    }
}
impl Fact {
    pub const ALL: [Self; 7] = [
        Self::Header,
        Self::Items,
        Self::Links,
        Self::Paths,
        Self::Externals,
        Self::Signatures,
        Self::Missing,
    ];
    pub fn name(self) -> &'static str {
        self.as_str()
    }
    pub fn schema(self) -> SchemaRef {
        let fields = match self {
            Self::Header => Header::fields(),
            Self::Items => ItemFact::fields(),
            Self::Links => Link::fields(),
            Self::Paths => PathFact::fields(),
            Self::Externals => External::fields(),
            Self::Signatures => Signature::fields(),
            Self::Missing => Missing::fields(),
        };
        Arc::new(Schema::new_with_metadata(
            fields
                .iter()
                .map(|field| crate::native_schema::producer_field(field))
                .collect::<Vec<_>>(),
            [("enrichment.rustdoc.facts".into(), VERSION.into())].into(),
        ))
    }
}

type Emit<'a> = dyn FnMut(Fact, RecordBatch) -> Result<(), ArrowError> + 'a;
struct Encoder<'a, T> {
    fact: Fact,
    rows: Vec<T>,
    bytes: usize,
    total: u64,
    emit: &'a mut Emit<'a>,
}
impl<'a, T: Serialize + NativeStruct> Encoder<'a, T> {
    fn new(fact: Fact, emit: &'a mut Emit<'a>) -> Self {
        Self {
            fact,
            rows: Vec::with_capacity(BATCH_ROWS),
            bytes: 0,
            total: 0,
            emit,
        }
    }
    fn push(&mut self, row: T) -> Result<(), ArrowError> {
        let bytes = crate::canonical::serialized_size(&row, 1024 * 1024)
            .map_err(|e| invalid(e.to_string()))?;
        if self.bytes + bytes > BATCH_BYTES || self.rows.len() == BATCH_ROWS {
            self.flush()?;
        }
        self.total += 1;
        if self.total > MAX_ROWS {
            return Err(invalid("rustdoc fact row bound"));
        }
        self.rows.push(row);
        self.bytes += bytes;
        Ok(())
    }
    fn flush(&mut self) -> Result<(), ArrowError> {
        if !self.rows.is_empty() {
            let array = T::encode(&self.rows.iter().map(Some).collect::<Vec<_>>())?;
            let array = arrow::array::AsArray::as_struct(array.as_ref());
            let schema = self.fact.schema();
            let columns = array
                .columns()
                .iter()
                .zip(schema.fields())
                .map(|(array, field)| {
                    crate::native_schema::check_projection(
                        &arrow::datatypes::Field::new(
                            field.name(),
                            array.data_type().clone(),
                            true,
                        ),
                        field,
                    )
                    .map_err(|error| invalid(error.to_string()))?;
                    arrow::compute::cast_with_options(
                        array,
                        field.data_type(),
                        &arrow::compute::CastOptions {
                            safe: false,
                            ..Default::default()
                        },
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let batch = RecordBatch::try_new(schema, columns)?;
            (self.emit)(self.fact, batch)?;
        }
        self.rows.clear();
        self.bytes = 0;
        Ok(())
    }
}
fn invalid(message: impl Into<String>) -> ArrowError {
    ArrowError::ParseError(message.into())
}

crate::native_vocabulary! { pub enum RawKind {
    Module = "module",
    ExternCrate = "extern_crate",
    Use = "use",
    Struct = "struct",
    StructField = "struct_field",
    Union = "union",
    Enum = "enum",
    Variant = "variant",
    Function = "function",
    TypeAlias = "type_alias",
    Constant = "constant",
    Trait = "trait",
    TraitAlias = "trait_alias",
    Impl = "impl",
    Static = "static",
    ExternType = "extern_type",
    Macro = "macro",
    ProcAttribute = "proc_attribute",
    ProcDerive = "proc_derive",
    AssocConst = "assoc_const",
    AssocType = "assoc_type",
    Primitive = "primitive",
    Keyword = "keyword",
    Attribute = "attribute",
} }
impl From<rustdoc_types::ItemKind> for RawKind {
    fn from(value: rustdoc_types::ItemKind) -> Self {
        match value {
            rustdoc_types::ItemKind::Module => Self::Module,
            rustdoc_types::ItemKind::ExternCrate => Self::ExternCrate,
            rustdoc_types::ItemKind::Use => Self::Use,
            rustdoc_types::ItemKind::Struct => Self::Struct,
            rustdoc_types::ItemKind::StructField => Self::StructField,
            rustdoc_types::ItemKind::Union => Self::Union,
            rustdoc_types::ItemKind::Enum => Self::Enum,
            rustdoc_types::ItemKind::Variant => Self::Variant,
            rustdoc_types::ItemKind::Function => Self::Function,
            rustdoc_types::ItemKind::TypeAlias => Self::TypeAlias,
            rustdoc_types::ItemKind::Constant => Self::Constant,
            rustdoc_types::ItemKind::Trait => Self::Trait,
            rustdoc_types::ItemKind::TraitAlias => Self::TraitAlias,
            rustdoc_types::ItemKind::Impl => Self::Impl,
            rustdoc_types::ItemKind::Static => Self::Static,
            rustdoc_types::ItemKind::ExternType => Self::ExternType,
            rustdoc_types::ItemKind::Macro => Self::Macro,
            rustdoc_types::ItemKind::ProcAttribute => Self::ProcAttribute,
            rustdoc_types::ItemKind::ProcDerive => Self::ProcDerive,
            rustdoc_types::ItemKind::AssocConst => Self::AssocConst,
            rustdoc_types::ItemKind::AssocType => Self::AssocType,
            rustdoc_types::ItemKind::Primitive => Self::Primitive,
            rustdoc_types::ItemKind::Keyword => Self::Keyword,
            rustdoc_types::ItemKind::Attribute => Self::Attribute,
        }
    }
}
crate::native_struct! { struct Header {
    root: u32 => Rule::Coordinate(Unit::RustdocItem),
    crate_version: Option<String> => Rule::Text,
    target: String => Rule::NonEmpty,
    format_version: u32 => Rule::Coordinate(Unit::Ordinal),
    includes_private: bool => Rule::Text,
    producer_items: u64 => Rule::Coordinate(Unit::Ordinal),
} }
crate::native_struct! { struct ItemFact {
    index_id: u32 => Rule::Coordinate(Unit::RustdocItem),
    id: u32 => Rule::Coordinate(Unit::RustdocItem),
    crate_id: u32 => Rule::Coordinate(Unit::RustdocItem),
    name: Option<String> => Rule::Text,
    raw_kind: RawKind => Rule::Vocabulary(RawKind::VALUES.iter().map(|v| (*v).into()).collect()),
    proc_macro_kind: Option<String> => Rule::Text,
    visibility: String => Rule::Text,
    file: Option<String> => Rule::Text,
    line: Option<u32> => Rule::Coordinate(Unit::LineOneBased),
    docs: Option<String> => Rule::Text,
    deprecated: bool => Rule::Text,
    deprecated_since: Option<String> => Rule::Text,
    deprecated_note: Option<String> => Rule::Text,
    attrs: Vec<String> => Rule::Sequence,
    use_source: Option<String> => Rule::Text,
    use_name: Option<String> => Rule::Text,
    use_target: Option<u32> => Rule::Coordinate(Unit::RustdocItem),
    use_glob: Option<bool> => Rule::Text,
    trait_id: Option<u32> => Rule::Coordinate(Unit::RustdocItem),
    trait_path: Option<String> => Rule::Text,
    impl_negative: Option<bool> => Rule::Text,
    impl_synthetic: Option<bool> => Rule::Text,
    impl_blanket: Option<bool> => Rule::Text,
    provided_methods: Vec<String> => Rule::Sequence,
    stripped: Option<bool> => Rule::Text,
    rust: RustDetails => Rule::Text,
} }
impl ItemFact {
    fn new(krate: &Crate, index_id: Id, item: &Item) -> Result<Self, ArrowError> {
        let use_ = if let ItemEnum::Use(u) = &item.inner {
            Some(u)
        } else {
            None
        };
        let impl_ = if let ItemEnum::Impl(i) = &item.inner {
            Some(i)
        } else {
            None
        };
        let stripped = match &item.inner {
            ItemEnum::Module(m) => Some(m.is_stripped),
            ItemEnum::Struct(s) => match &s.kind {
                StructKind::Plain {
                    has_stripped_fields,
                    ..
                } => Some(*has_stripped_fields),
                _ => None,
            },
            ItemEnum::Union(u) => Some(u.has_stripped_fields),
            ItemEnum::Enum(e) => Some(e.has_stripped_variants),
            ItemEnum::Variant(v) => match &v.kind {
                VariantKind::Struct {
                    has_stripped_fields,
                    ..
                } => Some(*has_stripped_fields),
                _ => None,
            },
            _ => None,
        };
        Ok(Self {
            index_id: index_id.0,
            id: item.id.0,
            crate_id: item.crate_id,
            name: item.name.clone(),
            raw_kind: item.inner.item_kind().into(),
            proc_macro_kind: if let ItemEnum::ProcMacro(pm) = &item.inner {
                Some(
                    match pm.kind {
                        rustdoc_types::MacroKind::Bang => "bang",
                        rustdoc_types::MacroKind::Attr => "attr",
                        rustdoc_types::MacroKind::Derive => "derive",
                    }
                    .into(),
                )
            } else {
                None
            },
            visibility: match item.visibility {
                Visibility::Public => "public",
                Visibility::Default => "default",
                Visibility::Crate => "crate",
                Visibility::Restricted { .. } => "restricted",
            }
            .into(),
            file: item
                .span
                .as_ref()
                .map(|s| s.filename.to_string_lossy().into_owned()),
            line: item
                .span
                .as_ref()
                .map(|s| {
                    u32::try_from(s.begin.0)
                        .map_err(|_| invalid("rustdoc source coordinate overflow"))
                })
                .transpose()?,
            docs: item.docs.clone(),
            deprecated: item.deprecation.is_some(),
            deprecated_since: item.deprecation.as_ref().and_then(|d| d.since.clone()),
            deprecated_note: item.deprecation.as_ref().and_then(|d| d.note.clone()),
            attrs: item
                .attrs
                .iter()
                .filter_map(|a| {
                    if let Attribute::Other(s) = a {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect(),
            use_source: use_.map(|u| u.source.clone()),
            use_name: use_.map(|u| u.name.clone()),
            use_target: use_.and_then(|u| u.id.map(|i| i.0)),
            use_glob: use_.map(|u| u.is_glob),
            trait_id: impl_.and_then(|i| i.trait_.as_ref().map(|p| p.id.0)),
            trait_path: impl_.and_then(|i| i.trait_.as_ref().map(|p| p.path.clone())),
            impl_negative: impl_.map(|i| i.is_negative),
            impl_synthetic: impl_.map(|i| i.is_synthetic),
            impl_blanket: impl_.map(|i| i.blanket_impl.is_some()),
            provided_methods: impl_.map_or_else(Vec::new, |i| i.provided_trait_methods.clone()),
            stripped,
            rust: rust_details(krate, item)?,
        })
    }
}
crate::native_struct! { struct Link {
    owner: u32 => Rule::Coordinate(Unit::RustdocItem),
    role: String => Rule::NonEmpty,
    ordinal: u64 => Rule::Coordinate(Unit::Ordinal),
    child: Option<u32> => Rule::Coordinate(Unit::RustdocItem),
} }
fn links(item: &Item, encoder: &mut Encoder<'_, Link>) -> Result<(), ArrowError> {
    let mut emit = |role: &str, children: Vec<Option<Id>>| {
        for (ordinal, child) in children.into_iter().enumerate() {
            encoder.push(Link {
                owner: item.id.0,
                role: role.into(),
                ordinal: ordinal as u64,
                child: child.map(|i| i.0),
            })?;
        }
        Ok::<_, ArrowError>(())
    };
    let all = |ids: &[Id]| ids.iter().copied().map(Some).collect();
    match &item.inner {
        ItemEnum::Module(m) => emit("module.items", all(&m.items))?,
        ItemEnum::Struct(s) => {
            match &s.kind {
                StructKind::Unit => (),
                StructKind::Tuple(fields) => emit("struct.tuple", fields.clone())?,
                StructKind::Plain { fields, .. } => emit("struct.fields", all(fields))?,
            }
            emit("type.impls", all(&s.impls))?;
        }
        ItemEnum::Union(u) => {
            emit("union.fields", all(&u.fields))?;
            emit("type.impls", all(&u.impls))?;
        }
        ItemEnum::Enum(e) => {
            emit("enum.variants", all(&e.variants))?;
            emit("type.impls", all(&e.impls))?;
        }
        ItemEnum::Variant(v) => match &v.kind {
            VariantKind::Plain => (),
            VariantKind::Tuple(fields) => emit("variant.tuple", fields.clone())?,
            VariantKind::Struct { fields, .. } => emit("variant.fields", all(fields))?,
        },
        ItemEnum::Trait(t) => {
            emit("trait.items", all(&t.items))?;
            emit("trait.implementations", all(&t.implementations))?;
        }
        ItemEnum::Impl(i) => emit("impl.items", all(&i.items))?,
        ItemEnum::Primitive(p) => emit("type.impls", all(&p.impls))?,
        _ => (),
    }
    Ok(())
}
crate::native_struct! { struct PathFact {
    id: u32 => Rule::Coordinate(Unit::RustdocItem),
    crate_id: u32 => Rule::Coordinate(Unit::RustdocItem),
    raw_kind: RawKind => Rule::Vocabulary(RawKind::VALUES.iter().map(|v| (*v).into()).collect()),
    components: Vec<String> => Rule::Sequence,
} }
crate::native_struct! { struct External {
    crate_id: u32 => Rule::Coordinate(Unit::RustdocItem),
    name: String => Rule::NonEmpty,
    html_root_url: Option<String> => Rule::Text,
} }
crate::native_struct! { struct Signature {
    ordinal: u64 => Rule::Coordinate(Unit::Ordinal),
    id: u32 => Rule::Coordinate(Unit::RustdocItem),
    parent_id: Option<u32> => Rule::Coordinate(Unit::RustdocItem),
    text: String => Rule::Text,
} }
crate::native_struct! { struct Missing { id: u32 => Rule::Coordinate(Unit::RustdocItem) } }

fn stability(value: &rustdoc_types::Stability) -> RustStability {
    RustStability {
        feature: value.feature.clone(),
        level: match &value.level {
            rustdoc_types::StabilityLevel::Stable { since } => RustStabilityLevel::Stable {
                since: since.clone(),
            },
            rustdoc_types::StabilityLevel::Unstable => RustStabilityLevel::Unstable,
        },
    }
}
fn abi(value: &rustdoc_types::Abi) -> RustAbi {
    use rustdoc_types::Abi;
    match value {
        Abi::Rust => RustAbi::Rust,
        Abi::C { unwind } => RustAbi::C { unwind: *unwind },
        Abi::Cdecl { unwind } => RustAbi::Cdecl { unwind: *unwind },
        Abi::Stdcall { unwind } => RustAbi::Stdcall { unwind: *unwind },
        Abi::Fastcall { unwind } => RustAbi::Fastcall { unwind: *unwind },
        Abi::Aapcs { unwind } => RustAbi::Aapcs { unwind: *unwind },
        Abi::Win64 { unwind } => RustAbi::Win64 { unwind: *unwind },
        Abi::SysV64 { unwind } => RustAbi::SysV64 { unwind: *unwind },
        Abi::System { unwind } => RustAbi::System { unwind: *unwind },
        Abi::Other(name) => RustAbi::Other { name: name.clone() },
    }
}
fn rust_details(krate: &Crate, item: &Item) -> Result<RustDetails, ArrowError> {
    let defaults = match &item.inner {
        ItemEnum::Function(f) => Some((f.has_body, &f.default_unstable)),
        ItemEnum::AssocConst {
            value,
            default_unstable,
            ..
        } => Some((value.is_some(), default_unstable)),
        ItemEnum::AssocType {
            type_,
            default_unstable,
            ..
        } => Some((type_.is_some(), default_unstable)),
        _ => None,
    };
    let callable = if let ItemEnum::Function(f) = &item.inner {
        Some(RustCallable {
            parameters: f
                .sig
                .inputs
                .iter()
                .enumerate()
                .map(|(ordinal, (pattern, ty))| {
                    Ok(RustParameter {
                        ordinal: u32::try_from(ordinal)
                            .map_err(|_| invalid("parameter ordinal overflow"))?,
                        pattern: pattern.clone(),
                        type_rendering: public_api::type_rendering(krate, ty),
                    })
                })
                .collect::<Result<_, ArrowError>>()?,
            output: f
                .sig
                .output
                .as_ref()
                .map(|ty| public_api::type_rendering(krate, ty)),
            c_variadic: f.sig.is_c_variadic,
            is_const: f.header.is_const,
            is_unsafe: f.header.is_unsafe,
            is_async: f.header.is_async,
            abi: abi(&f.header.abi),
            generics: public_api::generics_rendering(krate, &f.generics),
        })
    } else {
        None
    };
    Ok(RustDetails {
        stability: item.stability.as_deref().map(stability),
        const_stability: item.const_stability.as_deref().map(stability),
        body: defaults.map(|(present, instability)| {
            if present {
                RustBody::Present {
                    unstable_default_feature: instability.as_ref().map(|s| s.feature.clone()),
                }
            } else {
                RustBody::Absent
            }
        }),
        callable,
    })
}

/// Decode syntax only inside the externally bounded producer. public-api materializes its
/// own corpus; the worker's OS memory limit, not this iterator, contains that allocation.
pub fn extract(path: &Path, payload: &str, emit: &mut Emit<'_>) -> Result<(), ArrowError> {
    if payload.len() as u64 > MAX_BYTES {
        return Err(invalid("rustdoc byte bound"));
    }
    super::probe_format(payload).map_err(|e| invalid(e.to_string()))?;
    let mut decoder = serde_json::Deserializer::from_str(payload);
    decoder.disable_recursion_limit();
    let krate = Crate::deserialize(&mut decoder).map_err(|e| invalid(e.to_string()))?;
    if krate.index.len() > 1_000_000 || krate.paths.len() > 1_000_000 {
        return Err(invalid("rustdoc input item bound"));
    }
    let mut out = Encoder::new(Fact::Header, emit);
    out.push(Header {
        root: krate.root.0,
        crate_version: krate.crate_version.clone(),
        target: krate.target.triple.clone(),
        format_version: krate.format_version,
        includes_private: krate.includes_private,
        producer_items: krate.index.len() as u64,
    })?;
    out.flush()?;
    let mut out = Encoder::new(Fact::Items, emit);
    for (id, item) in &krate.index {
        out.push(ItemFact::new(&krate, *id, item)?)?;
    }
    out.flush()?;
    let mut out = Encoder::new(Fact::Links, emit);
    for item in krate.index.values() {
        links(item, &mut out)?;
    }
    out.flush()?;
    let mut out = Encoder::new(Fact::Paths, emit);
    for (id, p) in &krate.paths {
        out.push(PathFact {
            id: id.0,
            crate_id: p.crate_id,
            raw_kind: p.kind.into(),
            components: p.path.clone(),
        })?;
    }
    out.flush()?;
    let mut out = Encoder::new(Fact::Externals, emit);
    for (id, e) in &krate.external_crates {
        out.push(External {
            crate_id: *id,
            name: e.name.clone(),
            html_root_url: e.html_root_url.clone(),
        })?;
    }
    out.flush()?;
    drop(krate);
    let api = public_api::Builder::from_rustdoc_json(path)
        .sorted(false)
        .omit_blanket_impls(false)
        .omit_auto_trait_impls(false)
        .omit_auto_derived_impls(false)
        .include_function_parameter_names(true)
        .build()
        .map_err(|e| invalid(e.to_string()))?;
    let mut out = Encoder::new(Fact::Missing, emit);
    for id in api.missing_item_ids() {
        out.push(Missing { id: *id })?;
    }
    out.flush()?;
    let mut out = Encoder::new(Fact::Signatures, emit);
    for (ordinal, item) in api.into_items().enumerate() {
        let bytes = item
            .tokens()
            .try_fold(0usize, |n, t| n.checked_add(t.len()))
            .filter(|n| *n <= 1024 * 1024)
            .ok_or_else(|| invalid("rustdoc rendered item byte bound"))?;
        let text = item.to_string();
        if text.len() != bytes {
            return Err(invalid("public-api rendering size changed"));
        }
        out.push(Signature {
            ordinal: ordinal as u64,
            id: item.id().0,
            parent_id: item.parent_id().map(|i| i.0),
            text,
        })?;
    }
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evidence::arrow_model::cells::RowSet;

    fn fixture() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/rustdoc/schema-contract-format61.json")
    }

    #[test]
    fn format61_facts_preserve_stability_defaults_and_callable_fields() {
        let path = fixture();
        let payload = std::fs::read_to_string(&path).unwrap();
        let mut items = std::collections::BTreeMap::new();
        let mut signatures = Vec::new();
        let mut relations = std::collections::BTreeSet::new();
        extract(&path, &payload, &mut |fact, batch| {
            relations.insert(fact);
            let rows = RowSet::batch(&batch)?;
            for index in 0..batch.num_rows() {
                if fact == Fact::Items {
                    let item = ItemFact::decode(rows.row(index))?;
                    if item.crate_id == 0
                        && let Some(name) = &item.name
                    {
                        // Independent fixture declarations have unique names among local items.
                        items.insert(name.clone(), item);
                    }
                } else if fact == Fact::Signatures {
                    signatures.push(Signature::decode(rows.row(index))?.text);
                }
            }
            Ok(())
        })
        .unwrap();
        assert!(relations.contains(&Fact::Items));
        assert!(relations.contains(&Fact::Signatures));
        let stable = items["Stable"].rust.stability.as_ref().unwrap();
        assert_eq!(stable.feature, "ordinary_stable");
        assert_eq!(
            stable.level,
            RustStabilityLevel::Stable {
                since: Some("1.2.0".into())
            }
        );
        assert_eq!(
            items["Unstable"].rust.stability.as_ref().unwrap().level,
            RustStabilityLevel::Unstable
        );
        assert!(items["Stable"].rust.const_stability.is_none());
        assert!(items["Stable"].rust.body.is_none());
        assert_eq!(
            items["stable_const"]
                .rust
                .const_stability
                .as_ref()
                .unwrap()
                .level,
            RustStabilityLevel::Stable {
                since: Some("1.3.0".into())
            }
        );
        assert_eq!(
            items["unstable_const"]
                .rust
                .const_stability
                .as_ref()
                .unwrap()
                .level,
            RustStabilityLevel::Unstable
        );
        for (name, feature) in [
            ("method", "function_default"),
            ("VALUE", "constant_default"),
            ("Value", "type_default"),
        ] {
            assert_eq!(
                items[name].rust.body,
                Some(RustBody::Present {
                    unstable_default_feature: Some(feature.into())
                })
            );
        }
        assert_eq!(items["required"].rust.body, Some(RustBody::Absent));
        let method = items["method"].rust.callable.as_ref().unwrap();
        assert_eq!(
            method.parameters,
            vec![
                RustParameter {
                    ordinal: 0,
                    pattern: "self".into(),
                    type_rendering: "&Self".into()
                },
                RustParameter {
                    ordinal: 1,
                    pattern: "(left, right)".into(),
                    type_rendering: "(u32, u32)".into()
                },
            ]
        );
        assert_eq!(method.output.as_deref(), Some("u32"));
        assert!(!method.is_const);
        assert!(!method.c_variadic);
        assert_eq!(
            items["stable_const"]
                .rust
                .callable
                .as_ref()
                .unwrap()
                .parameters[0]
                .pattern,
            "value"
        );
        assert!(
            signatures
                .iter()
                .any(|s| s
                    == "pub const fn schema_contract_fixture::stable_const(value: u32) -> u32")
        );
    }

    #[test]
    fn malformed_format61_stability_is_refused() {
        let path = fixture();
        let mut value: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
        let stable = value["index"]
            .as_object_mut()
            .unwrap()
            .values_mut()
            .find(|item| item["name"] == "Stable")
            .unwrap();
        // The old flattened shape must not be accepted as format 61 or stripped of stability.
        stable["stability"] =
            serde_json::json!({"feature":"ordinary_stable","level":"stable","since":"1.2.0"});
        let mut emitted = 0;
        let error = extract(&path, &value.to_string(), &mut |_, _| {
            emitted += 1;
            Ok(())
        })
        .unwrap_err();
        assert!(matches!(error, ArrowError::ParseError(_)));
        assert_eq!(emitted, 0);
    }
}
