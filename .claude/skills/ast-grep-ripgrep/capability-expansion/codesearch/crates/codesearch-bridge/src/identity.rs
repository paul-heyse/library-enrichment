//! Three kinds of name, and the rules for minting them.
//!
//! | name           | scope           | stable across              | used for                        |
//! |----------------|-----------------|----------------------------|---------------------------------|
//! | `entity_key`   | global          | snapshots, tool versions   | `compare`; the legible identity |
//! | `entity_id`    | one snapshot    | nothing -- it is derived   | joins; the declared primary key |
//! | `native_handle`| one extraction  | nothing                    | crosswalk only                  |
//!
//! `entity_id = blake3(entity_key ‖ snapshot_id)`, so it is a **function of the other two**: never
//! authored, never migrated, never ambiguous.
//!
//! # Why two canonical names rather than one
//!
//! `compare` asks what changed between two snapshots, and by construction `entity_id` differs
//! between them for the *same* entity -- so `compare` joins on `entity_key`. Everything else joins
//! on `entity_id`, because it is short, uniform, and cheap to Z-order, and probe PB02 measured
//! that the optimiser's payoff comes from a *declared* key rather than a descriptive one.
//!
//! # Why native handles are neither
//!
//! Probe RD004 in the sibling `rust-code-model` skill measured it: inserting an unrelated item
//! *before* a type shifts that type's rustdoc `Id` from `0` to `41`, while an identical rebuild
//! keeps it. A native id is a position in one document, not an identity. The same holds for HIR
//! handles, `DefId`s and MIR local numbers, which do not even share a namespace with each other.
//!
//! # Two rules that are easy to get wrong
//!
//! **A mechanism key carries no version.** Flags move between a tool's help categories across
//! releases, so a key containing the category would churn and `compare` would report every
//! recategorised flag as removed-and-added. The version lives in the snapshot instead.
//!
//! **An operationally distinct configuration is its own mechanism.** `has` and `has` with
//! `stopBy: end` are different choices with different semantics, so they get different keys. The
//! `@k=v` suffix is reserved for exactly that and is used only where a behaviour assertion shows
//! the semantics differ -- never for ordinary parameters.

use std::sync::Arc;

use arrow::array::StringArray;
use arrow_schema::DataType;
use datafusion::common::Result as DFResult;
use datafusion::common::ScalarValue;
use datafusion::common::cast::as_string_array;
use datafusion::logical_expr::{
    ColumnarValue, Documentation, Expr, ScalarFunctionArgs, ScalarUDFImpl, Signature, Volatility,
};

/// Short, stable codes for the tools a mechanism can belong to.
///
/// Short because they appear in every key; stable because renaming one would invalidate every key
/// that contains it.
pub fn tool_code(tool: &str) -> &str {
    match tool {
        "ast-grep" | "sg" => "sg",
        "rg" | "ripgrep" => "rg",
        "pcre2" => "pcre2",
        other => other,
    }
}

/// `mech:<tool>/<surface>/<natural-key>`
pub fn mechanism_key(tool: &str, surface: &str, natural_key: &str) -> String {
    format!("mech:{}/{surface}/{natural_key}", tool_code(tool))
}

/// `cap:<kebab-ability>`
pub fn capability_key(ability: &str) -> String {
    format!("cap:{ability}")
}

/// The natural key of a CLI flag: the subcommand it belongs to, plus the long form.
///
/// `flags.tsv` records `command` as the full invocation path (`"ast-grep completions"`, or just
/// `"rg"`), so the tool name is stripped to leave the subcommand. A flag on the root command has
/// an empty subcommand and is keyed on the flag alone.
pub fn flag_natural_key(tool: &str, command: &str, long: &str) -> String {
    let subcommand = subcommand_path(tool, command);
    if subcommand.is_empty() {
        format!("--{long}")
    } else {
        format!("{subcommand}/--{long}")
    }
}

/// The subcommand part of an index `command` cell, with the tool name stripped.
///
/// `flags.tsv` and `exit-codes.tsv` both record the full invocation path -- `"ast-grep outline"`,
/// or just `"rg"` -- so both need the same reduction, and sharing it is what keeps a flag's key and
/// its command's result-contract key agreeing about what the subcommand is called. Empty for a
/// root command.
pub fn subcommand_path(tool: &str, command: &str) -> String {
    command
        .strip_prefix(tool)
        .unwrap_or(command)
        .trim()
        .replace(' ', "/")
}

/// `contract:<tool>/<subcommand>#<code>`, or `contract:<tool>#<code>` for a root command.
pub fn result_contract_key(tool: &str, command: &str, code: &str) -> String {
    let subcommand = subcommand_path(tool, command);
    let tool = tool_code(tool);
    if subcommand.is_empty() {
        format!("contract:{tool}#{code}")
    } else {
        format!("contract:{tool}/{subcommand}#{code}")
    }
}

/// `domain:<tool>/<domain-kind>/<name>`
pub fn search_domain_key(tool: &str, domain_kind: &str, name: &str) -> String {
    format!("domain:{}/{domain_kind}/{name}", tool_code(tool))
}

/// `absent:<tool>/<capability>` -- a capability known not to be reachable.
///
/// The prefix is deliberately not `mech:` or `cap:`. A key that looked like a mechanism's would
/// let a join treat a known absence as a way of doing something, which is the one reading this
/// record must never license.
pub fn negative_space_key(tool: &str, capability: &str) -> String {
    format!("absent:{}/{capability}", tool_code(tool))
}

// ---- the `library-api` family's keys -----------------------------------------------------------
//
// These live here, beside the catalog family's, because `projection.violations_underived_entity_id`
// re-derives every id in SQL through the `derived_entity_id` UDF. A key minted anywhere else would
// be checked by nothing.

/// `pkg:<registry>/<name>@<version>` -- §1.2, in full.
///
/// # This used to degrade, and what it took to stop
///
/// The first version was `pkg:<name>@<version>`, minted from `PROVENANCE.json`'s `pins.crates` --
/// a bare `name -> version` map with no source. Writing `crates-io` into that gap would have
/// asserted a fact nothing measured.
///
/// Running `cargo metadata` did not fix it either, and that is worth recording because it is the
/// obvious thing to try: a registry crate read straight out of `~/.cargo/registry/src` is a **path
/// package** as far as cargo is concerned, with `source: null` and an id of `path+file:///…`. So
/// `--no-deps` over vendored checkouts yields `pkg:path/ignore@0.4.29`, which records how the
/// adapter read the crate rather than what the crate is.
///
/// What recovers it is a **resolve**: `producers/cargo-metadata-adapter/subject/Cargo.toml` pins
/// all 23 with `=` and is resolved against the registry, which reports
/// `registry+https://github.com/rust-lang/crates.io-index`. The registry is normalised to
/// `crates.io` rather than emitted raw -- §1.2's own example is `pkg:crates.io/serde@1.0.200`, and
/// a transport and a host in every key buy nothing for something people read and join on.
pub fn package_key(registry: &str, name: &str, version: &str) -> String {
    format!("pkg:{registry}/{name}@{version}")
}

/// `def:<crate>#<canonical-path>`.
pub fn definition_key(crate_name: &str, canonical_path: &str) -> String {
    format!("def:{crate_name}#{canonical_path}")
}

/// `def:<owner-path>#fn:<method>@<via-trait|inherent>/<signature-digest>`.
///
/// # Why `via_trait` is in the key
///
/// One owner can carry the same method name by several routes: an inherent `clone` and `clone` via
/// `core::clone::Clone` are two signatures with two different texts. A key on `(owner, method)`
/// alone would merge them and the catalog would hold one signature for a name that has two.
///
/// # Why the signature is in the key as well
///
/// **`methods.tsv` records the trait PATH, not the trait REFERENCE.** The generic arguments are
/// dropped, so `TryFrom<u16>`, `TryFrom<u32>`, `TryFrom<u64>` and `TryFrom<usize>` all arrive as
/// `core::convert::TryFrom`. In the shipped index that affects **34 owner/method/trait triples
/// across 126 rows** -- every one of them a generic `From`, `TryFrom` or `PartialEq`.
///
/// This is §4.2's rule one level up. It says of generic parameters that *"two parameters named `T`
/// are not one global type variable"*; two impls of one trait at different type arguments are not
/// one impl either. The signature text is the only thing in the index that tells them apart, so it
/// is part of the identity.
///
/// A **digest** rather than the text itself, for two reasons: the key grammar uses `:`, `#`, `~`
/// and `/` as separators and a signature contains all of them, and a key is a join value before it
/// is prose. Sixteen hex characters of blake3 over the exact signature text -- stable across
/// rebuilds, independent of row order, and therefore not the positional identity probe RD004
/// warned about.
///
/// Verified against the shipped index: zero fully-identical `(owner, method, via_trait, signature)`
/// tuples, so this separates every real case. The caller still runs collision detection, which now
/// catches only a genuine duplicate row.
pub fn signature_key(
    owner_path: &str,
    method: &str,
    via_trait: Option<&str>,
    signature_text: &str,
) -> String {
    let digest = &blake3::hash(signature_text.as_bytes()).to_hex()[..16];
    format!(
        "def:{owner_path}#fn:{method}@{}/{digest}",
        via_trait.unwrap_or("inherent")
    )
}

/// `impl:<crate>#<trait>~<implementor>`.
///
/// §1.2 embeds a source anchor in this key precisely so that two impls for similarly-printed types
/// cannot collapse into one. `impls.tsv` carries no byte offsets, so the anchor is **not
/// available** and the key degrades to the printed pair. The builder detects collisions and fails
/// rather than merging: a collision here is a real finding about the index, not an inconvenience to
/// route around.
pub fn implementation_key(crate_name: &str, trait_path: &str, implementor_path: &str) -> String {
    format!("impl:{crate_name}#{trait_path}~{implementor_path}")
}

/// `export:<access-path>` -- where an item can be named from.
pub fn export_path_key(access_path: &str) -> String {
    format!("export:{access_path}")
}

/// `native:<namespace>/<handle>` -- a handle that is meaningful only within one run (§1.3).
pub fn native_binding_key(namespace: &str, handle: &str) -> String {
    format!("native:{namespace}/{handle}")
}

/// The natural key of an ast-grep rule field: its scope, then the field.
///
/// Scope-qualified because 22 field names occur in more than one scope in the shipped rule
/// schema. `has` inside `rule` and `has` inside `constraints` apply in different places and take
/// different values, so they are two mechanisms; a bare field name would merge them and the
/// catalog would then authorise using one where only the other is valid.
pub fn rule_field_natural_key(scope: &str, field: &str) -> String {
    format!("{scope}.{field}")
}

/// The name of the column carrying each row's content digest.
///
/// A constant rather than a literal because four places must agree on it: the schema declares the
/// column, two builders fill it, and `projection.compare` decides `changed` by comparing it.
pub const PAYLOAD_DIGEST: &str = "payload_digest";

/// The Arrow extension name marking a column that holds an `entity_id`.
///
/// Duplicated from `codesearch_model::schema::EXT_ENTITY_ID` rather than imported, because that
/// crate depends on this one and not the other way round. A test in the model crate asserts the
/// two agree, so the duplication cannot drift silently.
pub const EXT_ENTITY_ID: &str = "codesearch.entity_id";

/// The Arrow extension name marking a column that holds an `entity_key`.
///
/// Its sibling above is what the digest rule reads; this one is what
/// [`crate::rules::IdentityDiscipline`] reads, because telling the two apart at plan time is the
/// whole mechanism by which `entity_key ⋈ entity_id` is refused rather than silently returning
/// nothing. Duplicated from the model crate for the same dependency-direction reason, and pinned
/// by the same test.
pub const EXT_ENTITY_KEY: &str = "codesearch.entity_key";

/// The columns [`payload_digests`] does **not** cover by name, because each is a fact about the
/// *write* rather than about the row.
///
/// `compare` asks whether an entity's content changed between two snapshots. Every column here
/// differs between two snapshots of an *unchanged* entity, so including any one would make every
/// row report as changed and the comparison would answer nothing.
///
/// `entity_key` is deliberately **not** excluded. It is constant across a comparison by
/// definition, so including it costs nothing and buys something: equal digests then mean equal
/// rows, which makes the digest a content address rather than a bare change flag.
///
/// # This list is not sufficient on its own
///
/// A column holding **another row's** `entity_id` is snapshot-scoped for exactly the same reason
/// the row's own is, and there is no way to enumerate those by name -- `catalog.parameter` has
/// `mechanism_id`, `evidence.behavior_assertion` has `subject_entity_id`, `program.export_path`
/// has `definition_id`, and the next family will have its own. So the real rule is by **type**:
/// [`payload_digests`] skips every field carrying [`EXT_ENTITY_ID`], and this list covers only the
/// provenance columns that are not entity references.
///
/// This was found by the phase-7 fixture rather than by reasoning: the first `compare` run
/// reported all 165 `catalog.parameter` rows as `changed` when three rows had changed, because
/// `mechanism_id` differs between snapshots by construction. Nothing is lost by excluding them --
/// a parameter's key embeds its mechanism's key, so a re-pointed reference shows up as
/// added-and-removed, and a changed mechanism reports itself.
pub const DIGEST_EXCLUDES: &[&str] = &[
    // The dimension being compared.
    "snapshot_id",
    // Which run wrote the row, not what the row says.
    "run_id",
    // Delta table versions. Storage bookkeeping, not content.
    "first_seen_version",
    "last_seen_version",
    // A POSITION describes the sequence, not the element in it. Three tables set `ord`, and in
    // two of them -- `surface_binding` and `surface_entry` -- it is the row's index in an
    // alphabetically sorted sibling list, so removing ONE sibling shifts every later row and
    // `compare` would restate a single removal as a hundred changes. That is the opaque result
    // this catalog exists to refuse: a caller cannot find three real differences among a hundred
    // restatements of one of them. The sequence really did change, and `compare` says so exactly
    // once, on the row that was added or removed.
    //
    // Nothing is lost where order is genuinely semantic. `interaction_atom` is the one table whose
    // ordering carries meaning, and it encodes position twice over -- in its `entity_key`
    // (`...#<term>.<atom>`) and in the dedicated `term_ord` / `atom_ord` columns, both of which
    // stay in the digest. A reordered atom therefore still reports.
    "ord",
    // A digest cannot cover itself.
    PAYLOAD_DIGEST,
];

/// One content digest per row, over every column [`DIGEST_EXCLUDES`] does not exclude.
///
/// The other half of the identity story on this page. [`entity_id`] answers "which entity is this,
/// in this snapshot"; this answers "and is it still saying the same thing". `compare` joins on
/// `entity_key`, which settles `added` and `removed` and leaves `changed` undecidable -- because
/// `entity_id` differs between snapshots for an *unchanged* entity by construction, so it cannot
/// serve. A digest over the row's own content can, uniformly, for every table at once.
///
/// It lives here rather than in either builder so that the build and the fixtures compute it the
/// same way. Two implementations that agree today is how `compare` acquires a false negative
/// later.
///
/// # Why a formatter, and what that costs
///
/// Values render through `arrow::util::display`, which handles `List<Utf8>` and nested `Struct`
/// columns uniformly -- the alternative was a match arm per Arrow type, and the column it did not
/// handle would be the one silently missing from the digest.
///
/// **That display format is not a contractual one.** If an arrow upgrade changed how it renders
/// any type, every row would read as `changed` on the next `compare` across that boundary. Loud
/// rather than silent, and `snapshot.extraction_run.extractor_identity` records which binary wrote
/// each side, which is where a reader would look. Written down because a caveat nobody records is
/// a caveat nobody checks.
pub fn payload_digests(
    schema: &arrow_schema::SchemaRef,
    arrays: &[arrow::array::ArrayRef],
    rows: usize,
) -> Result<arrow::array::ArrayRef, arrow_schema::ArrowError> {
    use std::fmt::Write as _;

    // A NUL marker for null, so an absent value is distinguishable from an empty string. They are
    // different facts everywhere else in this model and they are different facts here.
    let options = arrow::util::display::FormatOptions::default().with_null("\u{0}");
    let mut formatters = Vec::with_capacity(schema.fields().len());
    for (index, field) in schema.fields().iter().enumerate() {
        let name = field.name().as_str();
        // By name for the provenance columns, and by TYPE for every entity reference -- including
        // the row's own `entity_id`, which needs no special case once the rule is the type.
        if DIGEST_EXCLUDES.contains(&name)
            || field
                .metadata()
                .get("ARROW:extension:name")
                .map(String::as_str)
                == Some(EXT_ENTITY_ID)
        {
            continue;
        }
        let formatter =
            arrow::util::display::ArrayFormatter::try_new(arrays[index].as_ref(), &options)?;
        formatters.push((name, formatter));
    }

    let mut digests = Vec::with_capacity(rows);
    let mut buffer = String::new();
    for row in 0..rows {
        buffer.clear();
        for (name, formatter) in &formatters {
            // The column NAME is in the digest, so a value that moves between two columns of the
            // same type changes the row rather than leaving it looking untouched.
            buffer.push_str(name);
            buffer.push('\u{1f}');
            write!(buffer, "{}", formatter.value(row))
                .map_err(|e| arrow_schema::ArrowError::ComputeError(e.to_string()))?;
            buffer.push('\u{1e}');
        }
        digests.push(blake3::hash(buffer.as_bytes()).to_hex()[..32].to_string());
    }
    Ok(Arc::new(StringArray::from(digests)))
}

/// Fill the derived digest column of an already-assembled column set, then build the batch.
///
/// Every producer of a canonical batch goes through here, so none of them has to remember that the
/// column exists -- and a producer that could forget is a producer that eventually does.
pub fn batch_with_digest(
    schema: &arrow_schema::SchemaRef,
    mut columns: Vec<arrow::array::ArrayRef>,
    rows: usize,
) -> Result<arrow::array::RecordBatch, arrow_schema::ArrowError> {
    if let Ok(index) = schema.index_of(PAYLOAD_DIGEST) {
        columns[index] = payload_digests(schema, &columns, rows)?;
    }
    arrow::array::RecordBatch::try_new(Arc::clone(schema), columns)
}

/// A per-snapshot join key, derived from the stable key and the snapshot.
///
/// Truncated to 16 bytes: long enough that a collision across one catalog is not a practical
/// concern, short enough to keep the join column narrow.
pub fn entity_id(entity_key: &str, snapshot_id: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(entity_key.as_bytes());
    hasher.update(b"\x1f"); // unit separator: `ab`+`c` must not collide with `a`+`bc`
    hasher.update(snapshot_id.as_bytes());
    let digest = hasher.finalize();
    digest.to_hex()[..32].to_string()
}

/// `derived_entity_id(entity_key, snapshot_id)` -- the derivation of [`entity_id`], in SQL.
///
/// # Why this exists at all
///
/// `entity_id` is defined as a *function* of the other two names, which means the claim is
/// checkable: a stored id either equals the digest of its own key and snapshot, or the row was
/// built by something that minted an id some other way. Without this UDF that property is
/// asserted only by the Rust that writes the rows, so a second writer -- a future extraction
/// family, a hand-repaired row, a merge from another catalog -- could violate it and nothing would
/// notice. With it, `projection.violations` recomputes every id on every `validate`.
///
/// It is `Immutable` for the same reason every other function here is: PB06 measured that a
/// non-`Immutable` UDF loses its filter entirely, with no diagnostic.
///
/// **It must never appear in a Delta CHECK constraint.** PB07 measured that a constraint calling a
/// UDF is accepted, stored as SQL text, and re-parsed by every future writer -- so the table
/// becomes unwritable by any session that has not registered it. A view is a different matter: it
/// lives in this process and is gone when the process exits.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityId {
    signature: Signature,
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityId {
    pub fn new() -> Self {
        Self {
            signature: Signature::exact(
                vec![DataType::Utf8, DataType::Utf8],
                Volatility::Immutable,
            ),
        }
    }
}

impl ScalarUDFImpl for EntityId {
    fn name(&self) -> &str {
        "derived_entity_id"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Utf8)
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let keys = as_string_array(&arrays[0])?;
        let snapshots = as_string_array(&arrays[1])?;
        let out: StringArray = keys
            .iter()
            .zip(snapshots.iter())
            .map(|pair| match pair {
                (Some(key), Some(snapshot)) => Some(entity_id(key, snapshot)),
                _ => None,
            })
            .collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }

    fn documentation(&self) -> Option<&Documentation> {
        static DOC: std::sync::OnceLock<Documentation> = std::sync::OnceLock::new();
        Some(DOC.get_or_init(|| {
            Documentation::builder(
                datafusion::logical_expr::scalar_doc_sections::DOC_SECTION_OTHER,
                "Recompute the per-snapshot entity id from the stable key and the snapshot. Used \
                 by projection.violations to check that stored ids really are derived; never in a \
                 CHECK constraint, which would make the table unwritable without this UDF.",
                "derived_entity_id(entity_key, snapshot_id)",
            )
            .with_argument("entity_key", "The stable, snapshot-independent key.")
            .with_argument("snapshot_id", "The snapshot the row belongs to.")
            .build()
        }))
    }
}

/// The namespace prefix of an `entity_key` — `mech:rg/cli/--pcre2` is `mech`.
///
/// §1.2 makes the key space one namespace per kind family, which is what lets the program half and
/// the capability half share a key space without colliding. This is the function that reads it
/// back, and it is the only one of §10.1's twelve that is genuinely a *pure function of its
/// arguments* rather than a relation wearing a function's clothes — see [`analyzer_note`].
///
/// # It carries a `preimage`, and PB18 measured that it works
///
/// `entity_key_namespace(k) = 'mech'` holds exactly for keys in the half-open range
/// `['mech:', 'mech;')`, because `;` is `:` plus one in ASCII. PB18 (evidence 16) confirmed that a
/// `Utf8` `Interval` is constructible at this version, that the rewrite fires, and that it reaches
/// the same predicate a person would have written by hand — with a control, identical but for
/// returning `PreimageResult::None`, that does not rewrite.
///
/// Two of PB06's rules are load-bearing here and both are invisible when broken:
///
/// - **half-open, never closed.** A closed upper bound is wrong by exactly one row, and only a
///   test sitting on the boundary can see it. `a_namespace_preimage_is_half_open` is that test.
/// - **over the bare stored column.** `args[0]`, never a derived expression: Delta's statistics
///   are keyed on stored columns, so a derived one rewrites and then prunes nothing.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct EntityKeyNamespace {
    signature: Signature,
}

impl Default for EntityKeyNamespace {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityKeyNamespace {
    pub fn new() -> Self {
        Self {
            signature: Signature::exact(vec![DataType::Utf8], Volatility::Immutable),
        }
    }
}

/// The namespace of one key, or `None` if it has none.
///
/// A key with no `:` is not a key, and answering `""` for it would invent a namespace. `None` is
/// the honest answer and it is distinguishable from every real one.
pub fn namespace_of(entity_key: &str) -> Option<&str> {
    entity_key.split_once(':').map(|(namespace, _)| namespace)
}

/// The half-open range of keys in a namespace: `['<ns>:', '<ns>;')`.
///
/// Exposed so the boundary test can assert on the bound itself rather than only on the rewrite,
/// which is what makes the closed/half-open error visible.
pub fn namespace_bounds(namespace: &str) -> (String, String) {
    (format!("{namespace}:"), format!("{namespace};"))
}

impl ScalarUDFImpl for EntityKeyNamespace {
    fn name(&self) -> &str {
        "entity_key_namespace"
    }

    fn signature(&self) -> &Signature {
        &self.signature
    }

    fn return_type(&self, _arg_types: &[DataType]) -> DFResult<DataType> {
        Ok(DataType::Utf8)
    }

    fn invoke_with_args(&self, args: ScalarFunctionArgs) -> DFResult<ColumnarValue> {
        let arrays = ColumnarValue::values_to_arrays(&args.args)?;
        let keys = as_string_array(&arrays[0])?;
        let out: StringArray = keys
            .iter()
            .map(|key| key.and_then(namespace_of).map(str::to_string))
            .collect();
        Ok(ColumnarValue::Array(Arc::new(out)))
    }

    fn preimage(
        &self,
        args: &[Expr],
        lit_expr: &Expr,
        _info: &datafusion::logical_expr::simplify::SimplifyContext,
    ) -> DFResult<datafusion::logical_expr::preimage::PreimageResult> {
        use datafusion::logical_expr::preimage::PreimageResult;
        let Expr::Literal(ScalarValue::Utf8(Some(namespace)), _) = lit_expr else {
            return Ok(PreimageResult::None);
        };
        let (lower, upper) = namespace_bounds(namespace);
        Ok(PreimageResult::Range {
            // `args[0]` -- the BARE column. PB06: a derived expression rewrites and prunes nothing.
            expr: args[0].clone(),
            interval: Box::new(
                datafusion::logical_expr::interval_arithmetic::Interval::try_new(
                    ScalarValue::Utf8(Some(lower)),
                    ScalarValue::Utf8(Some(upper)),
                )?,
            ),
        })
    }

    fn documentation(&self) -> Option<&Documentation> {
        static DOC: std::sync::OnceLock<Documentation> = std::sync::OnceLock::new();
        Some(DOC.get_or_init(|| {
            Documentation::builder(
                datafusion::logical_expr::scalar_doc_sections::DOC_SECTION_OTHER,
                "The namespace prefix of an entity_key, per the §1.2 grammar. Carries a preimage, \
                 so `entity_key_namespace(k) = 'mech'` is rewritten to a half-open range over the \
                 stored key column and can prune.",
                "entity_key_namespace(entity_key) = 'mech'",
            )
            .with_argument(
                "entity_key",
                "A key in the §1.2 grammar, e.g. mech:rg/cli/--pcre2.",
            )
            .build()
        }))
    }
}

/// Why §10.1's inventory of twelve is really nine plus three relations.
///
/// Building the UDFs found that three of them are not functions of their arguments at all:
///
/// - `regex_engine_supports(construct, engine)` would need the 44-row two-engine matrix from
///   `regex.tsv` compiled into this crate, where a skill update could silently disagree with it.
///   `catalog.mechanism` already holds one row per construct **per engine** (§12.1b: a construct
///   both engines support is two surface entries), so the question is a filter on a key.
/// - `supports_relationship(mech, rel)` would need each mechanism's relationship vocabulary.
///   `catalog.capability.facet_relationship` joined through `surface_binding` is that vocabulary,
///   and `projection.discover` is the join.
/// - `coverage_denominator(surface)` is `projection.coverage_surface`, which already computes the
///   denominator from the per-index `Denominator` declaration rather than from a function.
///
/// This is the same error as §5.3's `LatticePropagation`: a rule or a function proposed where a
/// relation is the right shape. Recorded in §12.1b rather than fixed by writing three functions
/// that would each duplicate a table.
pub fn analyzer_note() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_namespace_is_the_prefix_and_absence_is_not_an_empty_one() {
        assert_eq!(namespace_of("mech:rg/cli/--pcre2"), Some("mech"));
        assert_eq!(namespace_of("def:crate#path::Item"), Some("def"));
        // A key with no separator has no namespace. Answering `""` would invent one, and an
        // invented namespace is indistinguishable from a real one at every later join.
        assert_eq!(namespace_of("not-a-key"), None);
        assert_eq!(namespace_of(""), None);
        // The first separator wins: a scheme-specific part may contain colons and often does.
        assert_eq!(namespace_of("run:snap:abcd/catalog/0"), Some("run"));
    }

    /// The preimage bound is HALF-OPEN, and this test sits on the boundary.
    ///
    /// §10.2 requires a bucket-boundary case for every filter-eligible UDF because PB06 measured
    /// that the closed/half-open error is invisible without one: a closed upper bound is wrong by
    /// exactly the rows sitting on it, and every other row agrees either way.
    ///
    /// `;` is `:` plus one in ASCII, so `['mech:', 'mech;')` holds every key in the namespace and
    /// nothing else. The cases below are the four that can distinguish the two bounds.
    #[test]
    fn a_namespace_preimage_is_half_open() {
        let (lower, upper) = namespace_bounds("mech");
        let (lower, upper) = (lower.as_str(), upper.as_str());
        assert_eq!((lower, upper), ("mech:", "mech;"));

        // Inside: the lowest possible key in the namespace is the lower bound itself.
        assert!("mech:" >= lower && "mech:" < upper);
        assert!("mech:rg/cli/--pcre2" >= lower);
        assert!("mech:rg/cli/--pcre2" < upper);
        // The upper bound is the first key NOT in the namespace, and must be excluded. A closed
        // bound would admit it -- and `mech;` is a legal string, so this is not hypothetical.
        assert!(!("mech;" < upper));
        // A neighbouring namespace that shares a prefix must fall outside on the correct side.
        assert!(
            "mechanism:x" >= upper,
            "a longer namespace sorts above the bound"
        );
        assert!("meah:x" < lower, "an earlier namespace sorts below it");
    }

    /// Every namespace this codebase mints round-trips through the function.
    ///
    /// A denominator rather than a sample: a key minter added without a namespace the reader
    /// handles would otherwise be found by a caller rather than by this test.
    #[test]
    fn every_minted_namespace_is_readable() {
        let minted = [
            "pkg:crates.io/serde@1.0.0",
            "def:crate#path",
            "impl:crate#Type~Trait",
            "mech:rg/cli/--pcre2",
            "cap:find-a-call",
            "param:mech:rg/cli/--type/TYPE",
            "bind:ability/mech:rg/cli/--type",
            "surf:rg/flags/--type",
            "assert:rg/A001",
            "contract:rg/rg/0",
            "domain:rg/file_type/rust",
            "absent:rg/multiline-json",
            "ix:regex/lookbehind",
            "export:std::io::Read",
            "native:rustdoc.id/0",
            "run:snap:abcd/catalog/0",
        ];
        for key in minted {
            let namespace = namespace_of(key)
                .unwrap_or_else(|| panic!("`{key}` is a minted key with no readable namespace"));
            let (lower, upper) = namespace_bounds(namespace);
            assert!(
                key >= lower.as_str() && key < upper.as_str(),
                "`{key}` falls outside its own namespace range [{lower}, {upper})"
            );
        }
    }

    #[test]
    fn a_mechanism_key_carries_no_version() {
        let key = mechanism_key("rg", "cli", "--pcre2");
        assert_eq!(key, "mech:rg/cli/--pcre2");
        assert!(
            !key.contains("15.2"),
            "a version in the key would make `compare` report every upgrade as churn"
        );
    }

    #[test]
    fn tool_codes_are_normalised() {
        assert_eq!(mechanism_key("ast-grep", "rule", "has"), "mech:sg/rule/has");
        assert_eq!(mechanism_key("sg", "rule", "has"), "mech:sg/rule/has");
        assert_eq!(
            mechanism_key("ripgrep", "cli", "--pcre2"),
            "mech:rg/cli/--pcre2"
        );
    }

    #[test]
    fn a_root_flag_and_a_subcommand_flag_key_differently() {
        assert_eq!(flag_natural_key("rg", "rg", "pcre2"), "--pcre2");
        assert_eq!(
            flag_natural_key("ast-grep", "ast-grep completions", "config"),
            "completions/--config"
        );
        // Two different subcommands offering the same flag are two mechanisms, because their
        // effective behaviour can differ.
        assert_ne!(
            flag_natural_key("ast-grep", "ast-grep run", "config"),
            flag_natural_key("ast-grep", "ast-grep scan", "config")
        );
    }

    /// An operationally distinct configuration is its own mechanism.
    #[test]
    fn a_distinguished_configuration_gets_its_own_key() {
        let plain = mechanism_key("ast-grep", "rule", "has");
        let stop_at_end = mechanism_key("ast-grep", "rule", "has@stopBy=end");
        assert_ne!(
            plain, stop_at_end,
            "these traverse differently, so collapsing them would make the catalog wrong"
        );
    }

    #[test]
    fn entity_id_is_a_function_of_the_key_and_the_snapshot() {
        let a = entity_id("mech:rg/cli/--pcre2", "snap-1");
        let b = entity_id("mech:rg/cli/--pcre2", "snap-1");
        assert_eq!(a, b, "derivation must be deterministic");
        assert_eq!(a.len(), 32);

        let other_snapshot = entity_id("mech:rg/cli/--pcre2", "snap-2");
        assert_ne!(
            a, other_snapshot,
            "the same entity in two snapshots has two ids -- which is why `compare` joins on the key"
        );
    }

    /// The separator matters: without it, concatenation is ambiguous and two different
    /// (key, snapshot) pairs could hash the same.
    #[test]
    fn the_key_and_snapshot_boundary_is_unambiguous() {
        assert_ne!(entity_id("ab", "c"), entity_id("a", "bc"));
    }
}
