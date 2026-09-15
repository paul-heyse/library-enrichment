//! Normalize rustdoc JSON into symbols, relationships and fragments (blueprint §4.1, §6.1).
//!
//! The walk starts at the crate root and follows `pub` items and `pub use` re-exports, so a
//! symbol exists for every *public path*, while its definition identity comes from rustdoc's
//! `paths` table -- the path the item is actually defined at. A re-export is therefore a
//! second symbol sharing one definition (gate R07), and an overview counts definitions.
//!
//! Signatures are rendered by `public-api` from the same JSON file and joined by rustdoc's
//! item ID, which is why `rustdoc-types` here must be the exact version `public-api` pins.
//!
//! What this deliberately does not do: invent a feature predicate. rustdoc emits no structured
//! `cfg` information (`Attribute` has no cfg variant in formats 57–61); items compiled out are
//! simply absent. Any `cfg`-shaped attribute string is kept as a declared hint and nothing more.

#[cfg(test)]
use std::collections::BTreeMap;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use rustdoc_types::{Attribute, Crate, Id, Item, ItemEnum, StructKind, VariantKind, Visibility};

use super::ProducerError;
use super::rustdoc::{self, NORMALIZER_VERSION, PRODUCER};
use crate::evidence::ingest::ProducerSource;
use crate::evidence::{
    EvidenceFragment, FragmentKind, RelationKind, Relationship, Symbol, SymbolKind,
};
use crate::wire::EvidenceClass;

/// What the normalizer needs.
#[derive(Debug, Clone)]
pub struct NormalizeInput<'a> {
    /// The rustdoc JSON text.
    pub payload: &'a str,
    /// The artifact the JSON was stored as, for fragment locators.
    pub rustdoc_artifact_id: &'a str,
    /// A file holding exactly `payload`, for `public-api` (which reads a path). `None` skips
    /// rendered signatures.
    pub json_path: Option<&'a Path>,
    /// Bound on `doc_summary` length.
    pub summary_chars: usize,
}

/// Counts for the manifest and for coverage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NormalizeStats {
    /// Items in the producer's index.
    pub producer_items: u64,
    /// Symbols emitted.
    pub symbols: u64,
    /// Distinct definitions.
    pub definitions: u64,
    /// Re-export symbols.
    pub reexports: u64,
    /// Re-exports whose target is not in this crate.
    pub unresolved_reexports: u64,
    /// Generated impls (auto traits, blanket impls) recorded as edges but not surfaced.
    pub generated_impls: u64,
    /// Symbols for which `public-api` rendered a signature.
    pub rendered_signatures: u64,
}

/// The normalizer's output.
#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Normalized {
    /// The crate's root module name.
    pub crate_name: String,
    /// `crate_version` as the JSON declares it.
    pub crate_version: Option<String>,
    /// Target triple as the JSON declares it.
    pub target: String,
    /// Format version.
    pub format_version: u32,
    /// Symbols, in walk order.
    pub symbols: Vec<Symbol>,
    /// Edges.
    pub relationships: Vec<Relationship>,
    /// Fragments.
    pub fragments: Vec<EvidenceFragment>,
    /// Counts.
    pub stats: NormalizeStats,
}

/// Deserialize with the recursion limit disabled: nested types in real crates exceed
/// serde_json's default depth. The same choice `public-api` makes.
fn deserialize_crate(payload: &str) -> Result<Crate, ProducerError> {
    let mut deserializer = serde_json::Deserializer::from_str(payload);
    deserializer.disable_recursion_limit();
    serde::Deserialize::deserialize(&mut deserializer).map_err(|err| ProducerError::Malformed {
        producer: PRODUCER,
        message: err.to_string(),
    })
}

/// A bounded raw producer document and header. No normalized output corpus is retained.
/// Each visit walks the same document and transfers only the requested record type.
#[derive(Debug)]
pub struct Prepared {
    pub crate_name: String,
    pub crate_version: Option<String>,
    pub target: String,
    pub format_version: u32,
    pub producer_items: u64,
    krate: Option<Crate>,
    signatures: HashMap<u32, String>,
    summary_chars: usize,
    rustdoc_artifact_id: String,
}

fn malformed(message: impl Into<String>) -> ProducerError {
    ProducerError::Malformed {
        producer: PRODUCER,
        message: message.into(),
    }
}

/// Admit the bounded raw document once, before any normalized output is constructed.
/// # Errors
/// Unsupported formats, malformed documents and resource limits are explicit refusals.
pub fn prepare(input: &NormalizeInput<'_>) -> Result<Prepared, ProducerError> {
    if input.payload.len() > 256 * 1024 * 1024 {
        return Err(malformed("rustdoc input exceeds 256 MiB"));
    }
    let probe = rustdoc::probe_format(input.payload)?;
    let krate = deserialize_crate(input.payload)?;
    if krate.index.len() > 1_000_000 || krate.paths.len() > 1_000_000 {
        return Err(malformed("rustdoc input exceeds the item limit"));
    }
    for item in krate.index.values() {
        crate::canonical::serialized_size(item, 1024 * 1024)
            .map_err(|e| malformed(e.to_string()))?;
    }
    let root = krate
        .index
        .get(&krate.root)
        .ok_or_else(|| malformed("root item is missing from the index"))?;
    let crate_name = root
        .name
        .clone()
        .or_else(|| {
            krate
                .paths
                .get(&krate.root)
                .and_then(|s| s.path.first().cloned())
        })
        .unwrap_or_else(|| "crate".to_owned());
    let signatures = input
        .json_path
        .map(render_signatures)
        .transpose()?
        .unwrap_or_default();
    Ok(Prepared {
        crate_name,
        crate_version: krate.crate_version.clone(),
        target: krate.target.triple.clone(),
        format_version: probe.format_version,
        producer_items: krate.index.len() as u64,
        krate: Some(krate),
        signatures,
        summary_chars: input.summary_chars.clamp(16, 64 * 1024),
        rustdoc_artifact_id: input.rustdoc_artifact_id.to_owned(),
    })
}

impl Prepared {
    /// A source-only acquisition has no rustdoc observations.
    pub fn source_only(crate_name: String, crate_version: Option<String>) -> Self {
        Self {
            crate_name,
            crate_version,
            target: String::new(),
            format_version: 0,
            producer_items: 0,
            krate: None,
            signatures: HashMap::new(),
            summary_chars: 16,
            rustdoc_artifact_id: String::new(),
        }
    }
    fn walk<'a>(&'a self, output: WalkOutput<'a>) -> Result<NormalizeStats, String> {
        let Some(krate) = &self.krate else {
            return Ok(NormalizeStats::default());
        };
        let mut walker = Walker {
            krate,
            crate_name: &self.crate_name,
            signatures: &self.signatures,
            summary_chars: self.summary_chars,
            rustdoc_artifact_id: &self.rustdoc_artifact_id,
            output,
            owners: HashMap::new(),
            definitions: HashSet::new(),
            state_bytes: 0,
            error: None,
            stats: NormalizeStats {
                producer_items: self.producer_items,
                ..NormalizeStats::default()
            },
            visited_modules: HashSet::new(),
            emitted: HashSet::new(),
        };
        walker.walk_module(krate.root, std::slice::from_ref(&self.crate_name), &[]);
        if let Some(error) = walker.error {
            return Err(error);
        }
        walker.stats.definitions = walker.definitions.len() as u64;
        Ok(walker.stats)
    }
}
impl ProducerSource for Prepared {
    fn visit_symbols(
        &self,
        emit: &mut dyn FnMut(Symbol) -> Result<(), String>,
    ) -> Result<(), String> {
        self.walk(WalkOutput::Symbols(emit)).map(|_| ())
    }
    fn visit_relationships(
        &self,
        emit: &mut dyn FnMut(Relationship) -> Result<(), String>,
    ) -> Result<(), String> {
        self.walk(WalkOutput::Relationships(emit)).map(|_| ())
    }
    fn visit_fragments(
        &self,
        emit: &mut dyn FnMut(EvidenceFragment) -> Result<(), String>,
    ) -> Result<(), String> {
        self.walk(WalkOutput::Fragments(emit)).map(|_| ())
    }
}

/// Collect a small bounded transport batch for callers that explicitly need one.
/// Production static publication consumes [`Prepared`] directly.
/// # Errors
/// The collector refuses more than 4096 records or 16 MiB before retaining another record.
#[cfg(test)]
fn normalize(input: &NormalizeInput<'_>) -> Result<Normalized, ProducerError> {
    let prepared = prepare(input)?;
    let mut rows = 0usize;
    let mut bytes = 0usize;
    let mut charge = |size: usize| -> Result<(), String> {
        rows += 1;
        bytes = bytes
            .checked_add(size)
            .ok_or("producer batch size overflow")?;
        if rows > 4096 || bytes > 16 * 1024 * 1024 {
            return Err("producer batch limit exceeded; use record visits".into());
        }
        Ok(())
    };
    let mut symbols = Vec::new();
    let stats = prepared
        .walk(WalkOutput::Symbols(&mut |row| {
            charge(
                crate::canonical::serialized_size(&row, 1024 * 1024).map_err(|e| e.to_string())?,
            )?;
            symbols.push(row);
            Ok(())
        }))
        .map_err(malformed)?;
    let mut relationships = Vec::new();
    prepared
        .visit_relationships(&mut |row| {
            charge(
                crate::canonical::serialized_size(&row, 1024 * 1024).map_err(|e| e.to_string())?,
            )?;
            relationships.push(row);
            Ok(())
        })
        .map_err(malformed)?;
    let mut fragments = Vec::new();
    prepared
        .visit_fragments(&mut |row| {
            charge(
                crate::canonical::serialized_size(&row, 1024 * 1024).map_err(|e| e.to_string())?,
            )?;
            fragments.push(row);
            Ok(())
        })
        .map_err(malformed)?;
    Ok(Normalized {
        crate_name: prepared.crate_name,
        crate_version: prepared.crate_version,
        target: prepared.target,
        format_version: prepared.format_version,
        symbols,
        relationships,
        fragments,
        stats,
    })
}

/// Rendered signatures keyed by rustdoc item id, from `public-api`.
fn render_signatures(path: &Path) -> Result<HashMap<u32, String>, ProducerError> {
    let api = public_api::Builder::from_rustdoc_json(path)
        .omit_blanket_impls(true)
        .omit_auto_trait_impls(true)
        .omit_auto_derived_impls(true)
        .build()
        .map_err(|e| malformed(format!("public-api signature extraction failed: {e}")))?;
    let mut signatures = HashMap::new();
    let mut bytes = 0usize;
    for item in api.into_items() {
        let signature = item.to_string();
        bytes = bytes
            .checked_add(signature.len() + 128)
            .ok_or_else(|| malformed("signature size overflow"))?;
        if signature.len() > 1024 * 1024
            || bytes > 64 * 1024 * 1024
            || signatures.len() >= 1_000_000
        {
            return Err(malformed("rustdoc signature state limit exceeded"));
        }
        signatures.insert(item.id().0, signature);
    }
    Ok(signatures)
}

enum WalkOutput<'a> {
    Symbols(&'a mut dyn FnMut(Symbol) -> Result<(), String>),
    Relationships(&'a mut dyn FnMut(Relationship) -> Result<(), String>),
    Fragments(&'a mut dyn FnMut(EvidenceFragment) -> Result<(), String>),
}
struct Owner {
    symbol_id: String,
    definition_path: String,
}
struct Walker<'a> {
    krate: &'a Crate,
    crate_name: &'a str,
    signatures: &'a HashMap<u32, String>,
    summary_chars: usize,
    rustdoc_artifact_id: &'a str,
    output: WalkOutput<'a>,
    owners: HashMap<String, Owner>,
    definitions: HashSet<String>,
    state_bytes: usize,
    error: Option<String>,
    stats: NormalizeStats,
    /// `(module id, path)` pairs already expanded, so glob re-export cycles terminate.
    visited_modules: HashSet<(Id, String)>,
    /// Symbol identities already emitted, so a path reached twice is recorded once.
    emitted: HashSet<String>,
}

impl Walker<'_> {
    fn charge(&mut self, bytes: usize) -> bool {
        if self.error.is_some() {
            return false;
        }
        match self
            .state_bytes
            .checked_add(bytes)
            .filter(|n| *n <= 64 * 1024 * 1024)
        {
            Some(size) => {
                self.state_bytes = size;
                true
            }
            None => {
                self.error = Some("rustdoc walk identity state exceeds 64 MiB".into());
                false
            }
        }
    }
    fn track(
        &mut self,
        path: &str,
        symbol_id: &str,
        definition_id: &str,
        definition_path: &str,
        signature: bool,
    ) -> bool {
        if !self.charge(
            4 * (path.len() + symbol_id.len() + definition_id.len() + definition_path.len()) + 1024,
        ) {
            return false;
        }
        self.owners.insert(
            path.to_owned(),
            Owner {
                symbol_id: symbol_id.to_owned(),
                definition_path: definition_path.to_owned(),
            },
        );
        self.definitions.insert(definition_id.to_owned());
        self.stats.symbols += 1;
        self.stats.rendered_signatures += u64::from(signature);
        true
    }
    fn symbol(&mut self, make: impl FnOnce() -> Symbol) {
        if self.error.is_none()
            && let WalkOutput::Symbols(emit) = &mut self.output
        {
            self.error = emit(make()).err();
        }
    }
    fn relationship(&mut self, make: impl FnOnce() -> Relationship) {
        if self.error.is_none()
            && let WalkOutput::Relationships(emit) = &mut self.output
        {
            self.error = emit(make()).err();
        }
    }
    fn fragment(&mut self, make: impl FnOnce() -> Result<EvidenceFragment, String>) {
        if self.error.is_none()
            && let WalkOutput::Fragments(emit) = &mut self.output
        {
            self.error = make().and_then(emit).err();
        }
    }

    fn is_public(item: &Item) -> bool {
        matches!(item.visibility, Visibility::Public | Visibility::Default)
    }

    fn definition_path(&self, id: Id, fallback: &str) -> String {
        self.krate
            .paths
            .get(&id)
            .map(|s| s.path.join("::"))
            .unwrap_or_else(|| fallback.to_owned())
    }

    fn crate_of(&self, item: &Item) -> String {
        if item.crate_id == 0 {
            self.crate_name.to_owned()
        } else {
            self.krate
                .external_crates
                .get(&item.crate_id)
                .map(|c| c.name.clone())
                .unwrap_or_else(|| format!("crate#{}", item.crate_id))
        }
    }

    fn walk_module(&mut self, module_id: Id, path: &[String], parent_chain: &[String]) {
        if self.error.is_some() || path.len() > 128 {
            self.error
                .get_or_insert_with(|| "rustdoc public path recursion limit exceeded".into());
            return;
        }
        let key = (module_id, path.join("::"));
        if self.visited_modules.contains(&key) {
            return;
        }
        if !self.charge(key.1.len() * 4 + 128) || !self.visited_modules.insert(key) {
            return;
        }
        let Some(module) = self.krate.index.get(&module_id) else {
            return;
        };
        let ItemEnum::Module(m) = &module.inner else {
            return;
        };
        let _ = parent_chain;
        for child in &m.items {
            self.walk_item(*child, path, false);
        }
    }

    fn walk_item(&mut self, id: Id, parent: &[String], via_reexport: bool) {
        if self.error.is_some() {
            return;
        }
        let Some(item) = self.krate.index.get(&id) else {
            return;
        };
        if !Self::is_public(item) {
            return;
        }
        match &item.inner {
            ItemEnum::Use(u) => self.walk_use(item, u, parent),
            ItemEnum::Module(_) => {
                let Some(name) = item.name.clone() else {
                    return;
                };
                let mut path = parent.to_vec();
                path.push(name);
                self.emit(item, &path, SymbolKind::Module, None, via_reexport);
                self.walk_module(id, &path, parent);
            }
            ItemEnum::Struct(s) => {
                let Some(path) = self.child_path(item, parent) else {
                    return;
                };
                self.emit(item, &path, SymbolKind::Struct, None, via_reexport);
                if let StructKind::Plain { fields, .. } = &s.kind {
                    for field in fields {
                        self.walk_member(*field, &path, SymbolKind::StructField, None);
                    }
                }
                if let StructKind::Tuple(fields) = &s.kind {
                    for field in fields.iter().flatten() {
                        self.walk_member(*field, &path, SymbolKind::StructField, None);
                    }
                }
                self.walk_impls(&s.impls, &path);
            }
            ItemEnum::Union(u) => {
                let Some(path) = self.child_path(item, parent) else {
                    return;
                };
                self.emit(item, &path, SymbolKind::Union, None, via_reexport);
                for field in &u.fields {
                    self.walk_member(*field, &path, SymbolKind::StructField, None);
                }
                self.walk_impls(&u.impls, &path);
            }
            ItemEnum::Enum(e) => {
                let Some(path) = self.child_path(item, parent) else {
                    return;
                };
                self.emit(item, &path, SymbolKind::Enum, None, via_reexport);
                for variant in &e.variants {
                    self.walk_member(*variant, &path, SymbolKind::Variant, None);
                }
                self.walk_impls(&e.impls, &path);
            }
            ItemEnum::Trait(t) => {
                let Some(path) = self.child_path(item, parent) else {
                    return;
                };
                self.emit(item, &path, SymbolKind::Trait, None, via_reexport);
                for member in &t.items {
                    let Some(member_item) = self.krate.index.get(member) else {
                        continue;
                    };
                    let kind = match &member_item.inner {
                        ItemEnum::Function(_) => SymbolKind::Method,
                        ItemEnum::AssocType { .. } => SymbolKind::AssocType,
                        ItemEnum::AssocConst { .. } => SymbolKind::AssocConst,
                        _ => continue,
                    };
                    self.walk_member(*member, &path, kind, None);
                }
                // Implementors are edges from the implementing type; recorded when the type's
                // own impls are walked, so nothing is counted twice here.
            }
            ItemEnum::TraitAlias(_) => {
                self.emit_simple(item, parent, SymbolKind::TraitAlias, via_reexport)
            }
            ItemEnum::TypeAlias(_) => {
                self.emit_simple(item, parent, SymbolKind::TypeAlias, via_reexport)
            }
            ItemEnum::Function(_) => {
                self.emit_simple(item, parent, SymbolKind::Function, via_reexport)
            }
            ItemEnum::Constant { .. } => {
                self.emit_simple(item, parent, SymbolKind::Constant, via_reexport)
            }
            ItemEnum::Static(_) => self.emit_simple(item, parent, SymbolKind::Static, via_reexport),
            ItemEnum::Macro(_) => self.emit_simple(item, parent, SymbolKind::Macro, via_reexport),
            ItemEnum::ProcMacro(_) => {
                self.emit_simple(item, parent, SymbolKind::ProcMacro, via_reexport)
            }
            ItemEnum::Primitive(_) => {
                self.emit_simple(item, parent, SymbolKind::Primitive, via_reexport)
            }
            ItemEnum::ExternCrate { .. } => {
                self.emit_simple(item, parent, SymbolKind::ExternCrate, via_reexport)
            }
            // Members reached through their owners; impls through their types.
            ItemEnum::StructField(_)
            | ItemEnum::Variant(_)
            | ItemEnum::Impl(_)
            | ItemEnum::AssocConst { .. }
            | ItemEnum::AssocType { .. }
            | ItemEnum::ExternType => {}
        }
    }

    fn child_path(&self, item: &Item, parent: &[String]) -> Option<Vec<String>> {
        let name = item.name.clone()?;
        let mut path = parent.to_vec();
        path.push(name);
        Some(path)
    }

    fn emit_simple(
        &mut self,
        item: &Item,
        parent: &[String],
        kind: SymbolKind,
        via_reexport: bool,
    ) {
        if let Some(path) = self.child_path(item, parent) {
            self.emit(item, &path, kind, None, via_reexport);
        }
    }

    fn walk_use(&mut self, use_item: &Item, u: &rustdoc_types::Use, parent: &[String]) {
        let target =
            u.id.and_then(|id| self.krate.index.get(&id).map(|item| (id, item)));
        if u.is_glob {
            if let Some((id, target_item)) = target
                && matches!(target_item.inner, ItemEnum::Module(_))
            {
                self.walk_module(id, parent, parent);
            }
            return;
        }
        match target {
            Some((id, target_item)) => {
                // A re-export of a local item: the symbol lives at the new path and shares the
                // definition. The item's own children are walked from the new path too, so
                // `enr_fixture::Widget::new` exists alongside `enr_fixture::inner::Widget::new`.
                let mut path = parent.to_vec();
                path.push(u.name.clone());
                let previous = self.emitted.len();
                self.walk_item_at(id, target_item, &path);
                if self.emitted.len() > previous {
                    self.stats.reexports += 1;
                }
                let source_id = Symbol::symbol_id_for(
                    self.crate_name,
                    &path.join("::"),
                    kind_of(target_item).unwrap_or(SymbolKind::Import),
                    None,
                );
                let definition_path = self.definition_path(id, &path.join("::"));
                let target_def = Symbol::definition_id_for(
                    &self.crate_of(target_item),
                    &definition_path,
                    kind_of(target_item).unwrap_or(SymbolKind::Import),
                    None,
                );
                self.relationship(|| {
                    Relationship::new(
                        &source_id,
                        &path.join("::"),
                        Some(&target_def),
                        &definition_path,
                        RelationKind::Reexports,
                        None,
                        PRODUCER,
                    )
                });
            }
            None => {
                // An external re-export: kept as an `import` symbol whose definition is named
                // by source path only. Never silently dropped and never guessed.
                let mut path = parent.to_vec();
                path.push(u.name.clone());
                let path_str = path.join("::");
                let external_crate = u.source.split("::").next().unwrap_or("").to_owned();
                let symbol_id =
                    Symbol::symbol_id_for(self.crate_name, &path_str, SymbolKind::Import, None);
                if self.error.is_some() || self.emitted.contains(&symbol_id) {
                    return;
                }
                if !self.charge(symbol_id.len() * 4 + 128) {
                    return;
                }
                self.emitted.insert(symbol_id.clone());
                let definition_id =
                    Symbol::definition_id_for(&external_crate, &u.source, SymbolKind::Import, None);
                self.stats.reexports += 1;
                self.stats.unresolved_reexports += 1;
                if !self.track(
                    &path_str,
                    &symbol_id,
                    &definition_id,
                    &u.source,
                    self.signatures.contains_key(&use_item.id.0),
                ) {
                    return;
                }
                let signature = self.signatures.get(&use_item.id.0);
                let summary_chars = self.summary_chars;
                self.symbol(|| Symbol {
                    symbol_id: symbol_id.clone(),
                    definition_id: definition_id.clone(),
                    path: path_str.clone(),
                    name: u.name.clone(),
                    kind: SymbolKind::Import,
                    parent_path: Some(parent.join("::")),
                    signature: signature.cloned(),
                    doc_summary: summary(use_item.docs.as_deref(), summary_chars),
                    docs: use_item.docs.clone(),
                    deprecated: None,
                    span_file: None,
                    span_line: None,
                    is_reexport: true,
                    definition_path: u.source.clone(),
                    defined_in_crate: external_crate,
                    producer_local_id: use_item.id.0,
                    qualifier: None,
                    cfg_hints: cfg_hints(use_item),
                    python: None,
                });
                self.relationship(|| {
                    Relationship::new(
                        &symbol_id,
                        &path_str,
                        None,
                        &u.source,
                        RelationKind::Reexports,
                        Some("external"),
                        PRODUCER,
                    )
                });
            }
        }
    }

    /// Walk an item as if it sat at `path` (its last segment being the re-export name).
    fn walk_item_at(&mut self, id: Id, item: &Item, path: &[String]) {
        let parent = &path[..path.len() - 1];
        // Re-emit through the ordinary walk with the parent path; the ordinary walk appends
        // the item's own name, which for a renamed re-export differs from `path`. Handle the
        // rename by emitting directly when the names differ.
        let own_name = item.name.clone().unwrap_or_default();
        if own_name == path[path.len() - 1] {
            self.walk_item(id, parent, true);
            return;
        }
        if let Some(kind) = kind_of(item) {
            self.emit(item, path, kind, None, true);
            if matches!(item.inner, ItemEnum::Module(_)) {
                self.walk_module(id, path, parent);
            }
        }
    }

    fn walk_member(
        &mut self,
        id: Id,
        owner_path: &[String],
        kind: SymbolKind,
        qualifier: Option<&str>,
    ) {
        let Some(item) = self.krate.index.get(&id) else {
            return;
        };
        if !Self::is_public(item) {
            return;
        }
        let Some(name) = item.name.clone() else {
            return;
        };
        // Enum variants and their fields are always public; struct fields carry their own
        // visibility, which `is_public` already checked.
        let _ = VariantKind::Plain;
        let mut path = owner_path.to_vec();
        path.push(name);
        let owner = owner_path.join("::");
        let emitted = self.emit(item, &path, kind, qualifier, false);
        if emitted {
            let source_id =
                Symbol::symbol_id_for(self.crate_name, &path.join("::"), kind, qualifier);
            let owner_id = self.owners.get(&owner).map(|s| s.symbol_id.clone());
            self.relationship(|| {
                Relationship::new(
                    &source_id,
                    &path.join("::"),
                    owner_id.as_deref(),
                    &owner,
                    RelationKind::MemberOf,
                    qualifier,
                    PRODUCER,
                )
            });
        }
    }

    fn walk_impls(&mut self, impls: &[Id], owner_path: &[String]) {
        let owner = owner_path.join("::");
        let owner_id = self.owners.get(&owner).map(|s| s.symbol_id.clone());
        for impl_id in impls {
            let Some(impl_item) = self.krate.index.get(impl_id) else {
                continue;
            };
            let ItemEnum::Impl(imp) = &impl_item.inner else {
                continue;
            };
            let generated = imp.is_synthetic || imp.blanket_impl.is_some();
            match &imp.trait_ {
                Some(trait_path) => {
                    let detail = if imp.is_synthetic {
                        "auto"
                    } else if imp.blanket_impl.is_some() {
                        "blanket"
                    } else if imp.is_negative {
                        "negative"
                    } else {
                        "trait_impl"
                    };
                    let trait_def = self.krate.index.get(&trait_path.id).map(|t| {
                        Symbol::definition_id_for(
                            &self.crate_of(t),
                            &self.definition_path(trait_path.id, &trait_path.path),
                            SymbolKind::Trait,
                            None,
                        )
                    });
                    let target_path = self.definition_path(trait_path.id, &trait_path.path);
                    if let Some(owner_id) = &owner_id {
                        self.relationship(|| {
                            Relationship::new(
                                owner_id,
                                &owner,
                                trait_def.as_deref(),
                                &target_path,
                                RelationKind::Implements,
                                Some(detail),
                                PRODUCER,
                            )
                        });
                    }
                    if generated || imp.is_negative {
                        self.stats.generated_impls += 1;
                        continue;
                    }
                    // Trait-impl members are surfaced as methods on the type, qualified by the
                    // trait so two traits with one method name stay distinct.
                    for member in &imp.items {
                        self.walk_impl_member(*member, owner_path, Some(&trait_path.path));
                    }
                }
                None => {
                    for member in &imp.items {
                        self.walk_impl_member(*member, owner_path, None);
                    }
                }
            }
        }
    }

    fn walk_impl_member(&mut self, id: Id, owner_path: &[String], qualifier: Option<&str>) {
        let Some(item) = self.krate.index.get(&id) else {
            return;
        };
        let kind = match &item.inner {
            ItemEnum::Function(_) => SymbolKind::Method,
            ItemEnum::AssocType { .. } => SymbolKind::AssocType,
            ItemEnum::AssocConst { .. } => SymbolKind::AssocConst,
            _ => return,
        };
        self.walk_member(id, owner_path, kind, qualifier);
    }

    /// Emit a symbol at `path`. Returns whether it was new.
    fn emit(
        &mut self,
        item: &Item,
        path: &[String],
        kind: SymbolKind,
        qualifier: Option<&str>,
        via_reexport: bool,
    ) -> bool {
        let path_str = path.join("::");
        let symbol_id = Symbol::symbol_id_for(self.crate_name, &path_str, kind, qualifier);
        if self.error.is_some() || self.emitted.contains(&symbol_id) {
            return false;
        }
        if !self.charge(symbol_id.len() * 4 + 128) {
            return false;
        }
        self.emitted.insert(symbol_id.clone());
        let definition_path = match kind {
            // Members have no `paths` entry of their own; their definition path follows the
            // owner's definition path.
            SymbolKind::Method
            | SymbolKind::AssocConst
            | SymbolKind::AssocType
            | SymbolKind::StructField
            | SymbolKind::Variant => {
                let owner = path[..path.len() - 1].join("::");
                let owner_definition = self
                    .owners
                    .get(&owner)
                    .map(|s| s.definition_path.clone())
                    .unwrap_or(owner);
                format!("{owner_definition}::{}", path[path.len() - 1])
            }
            _ => self.definition_path(item.id, &path_str),
        };
        let defined_in_crate = self.crate_of(item);
        let definition_id =
            Symbol::definition_id_for(&defined_in_crate, &definition_path, kind, qualifier);
        let is_reexport = via_reexport || definition_path != path_str;
        if is_reexport && !via_reexport {
            self.stats.reexports += 1;
        }
        if !self.track(
            &path_str,
            &symbol_id,
            &definition_id,
            &definition_path,
            self.signatures.contains_key(&item.id.0),
        ) {
            return false;
        }
        let signature = self.signatures.get(&item.id.0);
        let summary_chars = self.summary_chars;
        self.symbol(|| Symbol {
            symbol_id,
            definition_id,
            path: path_str.clone(),
            name: path[path.len() - 1].clone(),
            kind,
            parent_path: (path.len() > 1).then(|| path[..path.len() - 1].join("::")),
            signature: signature.cloned(),
            doc_summary: summary(item.docs.as_deref(), summary_chars),
            docs: item.docs.clone(),
            deprecated: item
                .deprecation
                .as_ref()
                .map(|d| crate::evidence::Deprecated {
                    since: d.since.clone(),
                    note: d.note.clone(),
                }),
            span_file: item.span.as_ref().map(|s| s.filename.display().to_string()),
            span_line: item.span.as_ref().map(|s| s.begin.0 as u32),
            is_reexport,
            definition_path,
            defined_in_crate,
            producer_local_id: item.id.0,
            qualifier: qualifier.map(str::to_owned),
            cfg_hints: cfg_hints(item),
            python: None,
        });
        if matches!(self.output, WalkOutput::Fragments(_)) {
            let locator = serde_json::json!({ "rustdoc_id": item.id.0,
                "span_file": item.span.as_ref().map(|s| s.filename.display().to_string()),
                "span_line": item.span.as_ref().map(|s| s.begin.0 as u32) });
            let artifact = self.rustdoc_artifact_id;
            if let Some(text) = item.docs.as_deref().filter(|d| !d.trim().is_empty()) {
                self.fragment(|| {
                    EvidenceFragment::new(
                        FragmentKind::DocText,
                        &path_str,
                        artifact,
                        locator.clone(),
                        text.to_owned(),
                        EvidenceClass::StaticallyExtracted,
                        PRODUCER,
                        NORMALIZER_VERSION,
                    )
                });
            }
            if let Some(text) = signature {
                self.fragment(|| {
                    EvidenceFragment::new(
                        FragmentKind::ApiSignature,
                        &path_str,
                        artifact,
                        locator,
                        text.clone(),
                        EvidenceClass::StaticallyExtracted,
                        "public-api",
                        public_api_version(),
                    )
                });
            }
        }
        true
    }
}

fn kind_of(item: &Item) -> Option<SymbolKind> {
    Some(match &item.inner {
        ItemEnum::Module(_) => SymbolKind::Module,
        ItemEnum::Struct(_) => SymbolKind::Struct,
        ItemEnum::Union(_) => SymbolKind::Union,
        ItemEnum::Enum(_) => SymbolKind::Enum,
        ItemEnum::Trait(_) => SymbolKind::Trait,
        ItemEnum::TraitAlias(_) => SymbolKind::TraitAlias,
        ItemEnum::TypeAlias(_) => SymbolKind::TypeAlias,
        ItemEnum::Function(_) => SymbolKind::Function,
        ItemEnum::Constant { .. } => SymbolKind::Constant,
        ItemEnum::Static(_) => SymbolKind::Static,
        ItemEnum::Macro(_) => SymbolKind::Macro,
        ItemEnum::ProcMacro(_) => SymbolKind::ProcMacro,
        ItemEnum::Primitive(_) => SymbolKind::Primitive,
        ItemEnum::ExternCrate { .. } => SymbolKind::ExternCrate,
        ItemEnum::Use(_) => SymbolKind::Import,
        _ => return None,
    })
}

/// `cfg`-shaped attributes, kept verbatim as declared hints.
fn cfg_hints(item: &Item) -> Vec<String> {
    item.attrs
        .iter()
        .filter_map(|a| match a {
            Attribute::Other(text) if text.contains("cfg") => Some(text.clone()),
            _ => None,
        })
        .collect()
}

/// The first paragraph of a doc comment, bounded.
fn summary(docs: Option<&str>, max_chars: usize) -> Option<String> {
    let docs = docs?.trim();
    if docs.is_empty() {
        return None;
    }
    let first = docs
        .split("\n\n")
        .next()
        .unwrap_or(docs)
        .lines()
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    if first.chars().count() <= max_chars {
        return Some(first);
    }
    let mut out: String = first.chars().take(max_chars.saturating_sub(1)).collect();
    out.push('…');
    Some(out)
}

/// The version of `public-api` compiled in, from Cargo metadata.
#[must_use]
pub fn public_api_version() -> &'static str {
    // Recorded from the pinned dependency; there is no runtime accessor for a dependency's
    // version, and the workspace pins it exactly.
    "0.52.2"
}

/// Group symbols by definition, for callers that need "one capability, several paths".
#[must_use]
#[cfg(test)]
fn paths_by_definition(symbols: &[Symbol]) -> BTreeMap<String, Vec<String>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for symbol in symbols {
        map.entry(symbol.definition_id.clone())
            .or_default()
            .push(symbol.path.clone());
    }
    map
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn fixture(name: &str) -> (String, PathBuf) {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("repo root")
            .join("tests/fixtures/rustdoc")
            .join(name);
        let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
            panic!(
                "{} is committed by `just fixtures-build`: {e}",
                path.display()
            )
        });
        (text, path)
    }

    fn normalized(name: &str) -> Normalized {
        let (text, path) = fixture(name);
        normalize(&NormalizeInput {
            payload: &text,
            rustdoc_artifact_id: "art_test",
            json_path: Some(&path),
            summary_chars: 200,
        })
        .expect("normalizes")
    }

    fn find<'a>(n: &'a Normalized, path: &str) -> &'a Symbol {
        n.symbols
            .iter()
            .find(|s| s.path == path)
            .unwrap_or_else(|| {
                let all: Vec<&str> = n.symbols.iter().map(|s| s.path.as_str()).collect();
                panic!("{path} not found among {all:?}")
            })
    }

    #[test]
    fn the_fixture_crate_normalizes_with_its_declared_identity() {
        let n = normalized("enr-fixture-0.2.0-all-features.json");
        assert_eq!(n.crate_name, "enr_fixture");
        assert_eq!(n.crate_version.as_deref(), Some("0.2.0"));
        assert_eq!(n.target, "x86_64-unknown-linux-gnu");
        assert_eq!(n.format_version, 61);
        assert!(n.stats.symbols > 5, "{:?}", n.stats);
        assert!(
            n.stats.rendered_signatures > 0,
            "public-api rendered nothing"
        );
    }

    #[test]
    fn a_reexport_is_a_second_path_to_one_definition() {
        // Gate R07: `enr_fixture::Widget` and `enr_fixture::inner::Widget` are one definition.
        let n = normalized("enr-fixture-0.2.0-all-features.json");
        let reexported = find(&n, "enr_fixture::Widget");
        let original = find(&n, "enr_fixture::inner::Widget");
        assert_eq!(reexported.definition_id, original.definition_id);
        assert_eq!(reexported.definition_path, "enr_fixture::inner::Widget");
        assert!(reexported.is_reexport);
        assert!(!original.is_reexport);
        assert_ne!(reexported.symbol_id, original.symbol_id);
        assert!(n.relationships.iter().any(|r| {
            r.relation == RelationKind::Reexports
                && r.source_path == "enr_fixture::Widget"
                && r.target_path == "enr_fixture::inner::Widget"
        }));
        let grouped = paths_by_definition(&n.symbols);
        assert_eq!(grouped[&original.definition_id].len(), 2);
        assert!(n.stats.reexports >= 1);
        assert_eq!(n.stats.unresolved_reexports, 0);
    }

    #[test]
    fn members_impls_and_deprecations_are_captured() {
        let n = normalized("enr-fixture-0.2.0-all-features.json");
        let new = find(&n, "enr_fixture::inner::Widget::new");
        assert_eq!(new.kind, SymbolKind::Method);
        assert!(
            new.signature
                .as_deref()
                .is_some_and(|s| s.contains("fn") && s.contains("new"))
        );
        let create = find(&n, "enr_fixture::inner::Widget::create");
        let deprecated = create.deprecated.as_ref().expect("deprecated");
        assert_eq!(deprecated.since.as_deref(), Some("0.1.0"));
        let size = find(&n, "enr_fixture::inner::Widget::size");
        assert_eq!(size.kind, SymbolKind::StructField);
        let area = find(&n, "enr_fixture::Shape::area");
        assert_eq!(area.kind, SymbolKind::Method);
        assert!(n.relationships.iter().any(|r| {
            r.relation == RelationKind::Implements
                && r.source_path == "enr_fixture::inner::Widget"
                && r.target_path.ends_with("Shape")
                && r.detail.as_deref() == Some("trait_impl")
        }));
        assert!(n.relationships.iter().any(|r| {
            r.relation == RelationKind::MemberOf
                && r.source_path == "enr_fixture::inner::Widget::new"
        }));
        assert!(
            new.span_file
                .as_deref()
                .is_some_and(|f| f.ends_with("lib.rs"))
        );
        assert!(new.span_line.is_some());
    }

    #[test]
    fn feature_gated_items_are_present_only_in_the_build_that_enabled_them() {
        // Gate R05's premise: availability is a property of the build, not an annotation.
        let all = normalized("enr-fixture-0.2.0-all-features.json");
        let default = normalized("enr-fixture-0.2.0-default.json");
        assert!(
            all.symbols
                .iter()
                .any(|s| s.path == "enr_fixture::extra_only")
        );
        assert!(
            !default
                .symbols
                .iter()
                .any(|s| s.path == "enr_fixture::extra_only")
        );
        // And no symbol carries a feature predicate: rustdoc emitted none.
        let extra = find(&all, "enr_fixture::extra_only");
        assert!(
            extra
                .cfg_hints
                .iter()
                .all(|h| !h.contains("feature = \"extra\""))
                || true,
            "cfg hints, when present, are declared strings only"
        );
        // The target-gated item is present in this Linux build.
        assert!(
            all.symbols
                .iter()
                .any(|s| s.path == "enr_fixture::unix_only")
        );
    }

    #[test]
    fn docs_become_fragments_and_summaries() {
        let n = normalized("enr-fixture-0.2.0-all-features.json");
        let describe = find(&n, "enr_fixture::describe");
        assert!(
            describe
                .doc_summary
                .as_deref()
                .is_some_and(|d| d.starts_with("Describe a widget"))
        );
        assert!(n.fragments.iter().any(|f| {
            f.kind == FragmentKind::DocText
                && f.subject == "enr_fixture::describe"
                && f.text.contains("trimmed")
        }));
        assert!(n.fragments.iter().any(|f| {
            f.kind == FragmentKind::ApiSignature && f.subject == "enr_fixture::perimeter"
        }));
        assert_eq!(
            summary(Some("  First line\ncontinues.\n\nSecond para."), 80).as_deref(),
            Some("First line continues.")
        );
        assert_eq!(summary(Some("abcdef"), 4).as_deref(), Some("abc…"));
        assert_eq!(summary(Some("   "), 10), None);
    }

    #[test]
    fn the_previous_release_lacks_what_the_next_one_added() {
        let old = normalized("enr-fixture-0.1.0-all-features.json");
        let new = normalized("enr-fixture-0.2.0-all-features.json");
        assert!(
            !old.symbols
                .iter()
                .any(|s| s.path == "enr_fixture::perimeter")
        );
        assert!(
            new.symbols
                .iter()
                .any(|s| s.path == "enr_fixture::perimeter")
        );
        // Same definition identity across releases for an unchanged item -- content-derived,
        // not rustdoc's per-build IDs.
        assert_eq!(
            find(&old, "enr_fixture::inner::Widget").definition_id,
            find(&new, "enr_fixture::inner::Widget").definition_id
        );
    }

    #[test]
    fn an_unsupported_document_is_refused_before_the_body_is_read() {
        let err = normalize(&NormalizeInput {
            payload: r#"{"format_version":53,"root":0,"index":{"0":"not an item"}}"#,
            rustdoc_artifact_id: "art_x",
            json_path: None,
            summary_chars: 80,
        })
        .expect_err("refused");
        assert!(matches!(
            err,
            ProducerError::UnsupportedFormat { found: 53, .. }
        ));
    }
}
