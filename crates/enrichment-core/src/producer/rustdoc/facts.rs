//! Mechanical rustdoc and public-api decoding into Arrow. This boundary does not choose
//! visible declarations, traverse public paths, associate renderings or create evidence IDs.
use arrow::{
    datatypes::{DataType, Field, Schema, SchemaRef},
    error::ArrowError,
    record_batch::RecordBatch,
};
use rustdoc_types::{Attribute, Crate, Id, Item, ItemEnum, StructKind, VariantKind, Visibility};
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};

pub const VERSION: &str = "rustdoc-arrow-facts/1";
pub const PUBLIC_API_VERSION: &str = "0.52.2";
pub const MAX_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_ROWS: u64 = 4_000_000;
pub const BATCH_ROWS: usize = 128;
pub const BATCH_BYTES: usize = 2 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Fact {
    Header,
    Items,
    Links,
    Paths,
    Externals,
    Signatures,
    Missing,
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
        match self {
            Self::Header => "rust_header",
            Self::Items => "rust_items",
            Self::Links => "rust_links",
            Self::Paths => "rust_paths",
            Self::Externals => "rust_externals",
            Self::Signatures => "rust_signatures",
            Self::Missing => "rust_missing",
        }
    }
    pub fn schema(self) -> SchemaRef {
        let text = |name, nullable| Field::new(name, DataType::Utf8, nullable);
        let id = |name, nullable| Field::new(name, DataType::UInt32, nullable);
        let flag = |name, nullable| Field::new(name, DataType::Boolean, nullable);
        let strings = |name| Field::new(name, DataType::List(Arc::new(text("item", false))), false);
        let fields = match self {
            Self::Header => vec![
                id("root", false),
                text("crate_version", true),
                text("target", false),
                id("format_version", false),
                flag("includes_private", false),
                Field::new("producer_items", DataType::UInt64, false),
            ],
            Self::Items => vec![
                id("index_id", false),
                id("id", false),
                id("crate_id", false),
                text("name", true),
                text("raw_kind", false),
                text("proc_macro_kind", true),
                text("visibility", false),
                text("file", true),
                id("line", true),
                text("docs", true),
                flag("deprecated", false),
                text("deprecated_since", true),
                text("deprecated_note", true),
                strings("attrs"),
                text("use_source", true),
                text("use_name", true),
                id("use_target", true),
                flag("use_glob", true),
                id("trait_id", true),
                text("trait_path", true),
                flag("impl_negative", true),
                flag("impl_synthetic", true),
                flag("impl_blanket", true),
                strings("provided_methods"),
                flag("stripped", true),
            ],
            Self::Links => vec![
                id("owner", false),
                text("role", false),
                Field::new("ordinal", DataType::UInt64, false),
                id("child", true),
            ],
            Self::Paths => vec![
                id("id", false),
                id("crate_id", false),
                text("raw_kind", false),
                strings("components"),
            ],
            Self::Externals => vec![
                id("crate_id", false),
                text("name", false),
                text("html_root_url", true),
            ],
            Self::Signatures => vec![
                Field::new("ordinal", DataType::UInt64, false),
                id("id", false),
                id("parent_id", true),
                text("text", false),
            ],
            Self::Missing => vec![id("id", false)],
        };
        Arc::new(Schema::new_with_metadata(
            fields,
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
impl<'a, T: Serialize> Encoder<'a, T> {
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
        let mut decoder = arrow::json::ReaderBuilder::new(self.fact.schema())
            .with_batch_size(BATCH_ROWS)
            .build_decoder()?;
        decoder.serialize(&self.rows)?;
        if let Some(batch) = decoder.flush()? {
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

#[derive(Serialize)]
struct Header<'a> {
    root: u32,
    crate_version: &'a Option<String>,
    target: &'a str,
    format_version: u32,
    includes_private: bool,
    producer_items: u64,
}
#[derive(Serialize)]
struct ItemFact<'a> {
    index_id: u32,
    id: u32,
    crate_id: u32,
    name: &'a Option<String>,
    raw_kind: rustdoc_types::ItemKind,
    proc_macro_kind: Option<rustdoc_types::MacroKind>,
    visibility: &'static str,
    file: Option<String>,
    line: Option<u32>,
    docs: &'a Option<String>,
    deprecated: bool,
    deprecated_since: Option<&'a str>,
    deprecated_note: Option<&'a str>,
    attrs: Vec<&'a str>,
    use_source: Option<&'a str>,
    use_name: Option<&'a str>,
    use_target: Option<u32>,
    use_glob: Option<bool>,
    trait_id: Option<u32>,
    trait_path: Option<&'a str>,
    impl_negative: Option<bool>,
    impl_synthetic: Option<bool>,
    impl_blanket: Option<bool>,
    provided_methods: &'a [String],
    stripped: Option<bool>,
}
impl<'a> ItemFact<'a> {
    fn new(index_id: Id, item: &'a Item) -> Result<Self, ArrowError> {
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
            name: &item.name,
            raw_kind: item.inner.item_kind(),
            proc_macro_kind: if let ItemEnum::ProcMacro(pm) = &item.inner {
                Some(pm.kind)
            } else {
                None
            },
            visibility: match item.visibility {
                Visibility::Public => "public",
                Visibility::Default => "default",
                Visibility::Crate => "crate",
                Visibility::Restricted { .. } => "restricted",
            },
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
            docs: &item.docs,
            deprecated: item.deprecation.is_some(),
            deprecated_since: item.deprecation.as_ref().and_then(|d| d.since.as_deref()),
            deprecated_note: item.deprecation.as_ref().and_then(|d| d.note.as_deref()),
            attrs: item
                .attrs
                .iter()
                .filter_map(|a| {
                    if let Attribute::Other(s) = a {
                        Some(s.as_str())
                    } else {
                        None
                    }
                })
                .collect(),
            use_source: use_.map(|u| u.source.as_str()),
            use_name: use_.map(|u| u.name.as_str()),
            use_target: use_.and_then(|u| u.id.map(|i| i.0)),
            use_glob: use_.map(|u| u.is_glob),
            trait_id: impl_.and_then(|i| i.trait_.as_ref().map(|p| p.id.0)),
            trait_path: impl_.and_then(|i| i.trait_.as_ref().map(|p| p.path.as_str())),
            impl_negative: impl_.map(|i| i.is_negative),
            impl_synthetic: impl_.map(|i| i.is_synthetic),
            impl_blanket: impl_.map(|i| i.blanket_impl.is_some()),
            provided_methods: impl_.map_or(&[], |i| i.provided_trait_methods.as_slice()),
            stripped,
        })
    }
}
#[derive(Serialize)]
struct Link {
    owner: u32,
    role: &'static str,
    ordinal: u64,
    child: Option<u32>,
}
fn links(item: &Item, encoder: &mut Encoder<'_, Link>) -> Result<(), ArrowError> {
    let mut emit = |role, children: Vec<Option<Id>>| {
        for (ordinal, child) in children.into_iter().enumerate() {
            encoder.push(Link {
                owner: item.id.0,
                role,
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
#[derive(Serialize)]
struct PathFact<'a> {
    id: u32,
    crate_id: u32,
    raw_kind: rustdoc_types::ItemKind,
    components: &'a [String],
}
#[derive(Serialize)]
struct External<'a> {
    crate_id: u32,
    name: &'a str,
    html_root_url: &'a Option<String>,
}
#[derive(Serialize)]
struct Signature {
    ordinal: u64,
    id: u32,
    parent_id: Option<u32>,
    text: String,
}
#[derive(Serialize)]
struct Missing {
    id: u32,
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
        crate_version: &krate.crate_version,
        target: &krate.target.triple,
        format_version: krate.format_version,
        includes_private: krate.includes_private,
        producer_items: krate.index.len() as u64,
    })?;
    out.flush()?;
    let mut out = Encoder::new(Fact::Items, emit);
    for (id, item) in &krate.index {
        out.push(ItemFact::new(*id, item)?)?;
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
            raw_kind: p.kind,
            components: &p.path,
        })?;
    }
    out.flush()?;
    let mut out = Encoder::new(Fact::Externals, emit);
    for (id, e) in &krate.external_crates {
        out.push(External {
            crate_id: *id,
            name: &e.name,
            html_root_url: &e.html_root_url,
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
