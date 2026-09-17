//! The index registry: which files each family reads, and what columns they carry.
//!
//! This lives in `codesearch-model` rather than beside the reader because it is a **declaration**,
//! not a parse. `schema::all_tables` derives one `staging.*` table per family from it, so the DDL,
//! the fixed-point check and the reader all resolve "what files exist and what shape are they"
//! through one registry -- the same reason [`schema::all_tables`] is a single list.
//!
//! Column names are not in the data. They live in each skill's `reference.md`; this module carries
//! the copy every reader cross-checks against.

/// Which coverage denominator an index file belongs to.
///
/// This is declared per file rather than inferred, because the alternative was a `match` in
/// `catalog::build_surface_entries` whose `_ => continue` arm quietly dropped anything it did not
/// name. `behaviors.tsv` was excluded deliberately and said so; five more files excluded by
/// falling through the same arm would be five facts nobody stated.
///
/// Coverage reports each denominator separately and never sums them, for the same reason §9.2
/// refuses a composite: "how much of the tool's surface is catalogued" and "how much of the
/// shipped API index became program rows" are different questions with different answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Denominator {
    /// What the tool exposes to a user: flags, rule fields, regex constructs, exit codes, file
    /// types, languages, and the capabilities known to be absent.
    ToolSurface,
    /// What the tool's own source declares: the rustdoc-derived index the `library-api` family
    /// reads. Not user-facing surface, and counting it as such would take surface coverage from
    /// 621/621 to 621/8,583 by mixing the two questions.
    ProgramApi,
    /// Read, but in no denominator. `behaviors.tsv` is the case: a probe is EVIDENCE about a
    /// mechanism, not an inventory item, so counting it would add rows that can never be
    /// "characterised".
    None,
}

/// Which input root a spec's file is read from.
///
/// Until the first adapter there was one root and this did not exist. It exists because
/// [`read_all`] iterates `SPECS` and **hard-errors on a missing file** -- so pointing the reader at
/// a producer's output, which ships a different file set, failed on the first index that was not
/// there. Tagging the spec is what lets one reader serve both, with no second code path and no
/// `_ => continue` arm quietly skipping a file nobody declared.
///
/// It also keeps stems from colliding. The `rust-code-model` skill ships `symbols.tsv`,
/// `methods.tsv`, `impls.tsv`, `aliases.tsv` and `unresolved.tsv` -- every one a name the
/// ast-grep-ripgrep skill also uses -- so a single map keyed by stem would silently merge two
/// subjects into one, and the first symptom would be a key collision naming no root.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    /// A capability-repository skill: `--skill-root`.
    Skill,
    /// `cargo metadata` output, written by `producers/cargo-metadata-adapter`.
    CargoMetadata,
}

impl Source {
    /// The flag that supplies this root, so an error can name what to pass.
    pub fn flag(self) -> &'static str {
        match self {
            Source::Skill => "--skill-root",
            Source::CargoMetadata => "--producer-root",
        }
    }
}

/// One index file: its name, the columns it carries in order, and what it is a denominator for.
#[derive(Debug, Clone, Copy)]
pub struct IndexSpec {
    pub file: &'static str,
    pub columns: &'static [&'static str],
    pub denominator: Denominator,
    /// Which root this file is read from. See [`Source`].
    pub source: Source,
}

impl IndexSpec {
    pub fn stem(&self) -> &'static str {
        self.file.trim_end_matches(".tsv")
    }
}

/// The specs in one denominator.
pub fn specs_for(denominator: Denominator) -> impl Iterator<Item = &'static IndexSpec> {
    SPECS.iter().filter(move |s| s.denominator == denominator)
}

/// The specs belonging to one input root.
pub fn specs_of(source: Source) -> impl Iterator<Item = &'static IndexSpec> {
    SPECS.iter().filter(move |s| s.source == source)
}

/// The indexes this catalog loads, with their documented columns.
///
/// **Thirteen of the skill's sixteen.** The three that are absent are absent on purpose, and each
/// says so here, because an undeclared exclusion is exactly the silent gap [`Denominator`] was
/// introduced to end -- and one of these three went undocumented for two phases, which is how the
/// omission was noticed at all.
///
/// - `kinds.tsv` (3,024 rows) and `fields.tsv` (631) are target-language **grammar metadata**,
///   which changes on every ast-grep grammar update. §3.8 keeps them referenced from the skill
///   rather than copied into the catalog; `catalog.grammar_fragment` is where a pointer to them
///   will live.
/// - `bindings.tsv` (67 rows) is ast-grep's **napi/pyo3 binding surface** -- which TypeScript and
///   Python names its Rust items are exported under. It looks like a crosswalk and is not one: its
///   `role` column is uniformly `item`, so it maps a name to a name with no statement about what
///   the relationship is. `snapshot.native_binding` needs a `mapping_basis` and a binding lattice
///   value, and this file supplies neither. Loading it would produce 67 rows asserting a
///   relationship nobody recorded.
pub const SPECS: &[IndexSpec] = &[
    IndexSpec {
        file: "flags.tsv",
        columns: &[
            "tool", "command", "long", "short", "category", "arg", "values", "summary",
        ],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "rule-fields.tsv",
        columns: &["scope", "field", "type", "required", "summary"],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "regex.tsv",
        columns: &[
            "construct",
            "syntax",
            "rust_regex",
            "pcre2",
            "reachable",
            "since_pcre2",
            "probe",
            "observed",
            "purpose",
        ],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "behaviors.tsv",
        columns: &[
            "probe",
            "tool",
            "topic",
            "verdict",
            "exit",
            "question",
            "command",
            "stdout",
            "control",
            "control_exit",
        ],
        denominator: Denominator::None,
        source: Source::Skill,
    },
    IndexSpec {
        file: "exit-codes.tsv",
        columns: &["tool", "command", "code", "meaning", "trap"],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "file-types.tsv",
        columns: &["type", "globs"],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "languages.tsv",
        columns: &["language", "status", "kinds", "fields", "rule_schema"],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    IndexSpec {
        file: "unreachable.tsv",
        columns: &["capability", "exists_in", "cli_reachable", "why", "instead"],
        denominator: Denominator::ToolSurface,
        source: Source::Skill,
    },
    // ---- the `library-api` family's sources ---------------------------------------------------
    //
    // These describe the search tools' own Rust API, not the surface they expose to a user, which
    // is why they are a separate denominator rather than more of the first one.
    //
    // Two column names below are deliberately NOT the ones the file's documentation uses, because
    // the files have no headers -- `build/surfaces.py` writes them sorted and bare, and
    // `reference.md` is the only place names live. Where the documented name would mislead, the
    // spec uses the honest one:
    //
    //   `symbols.tsv` column 4 is documented `family`, but `build/api.py` computes it as
    //   `FAMILY.get(item.crate, "engine")` -- an editorial bucket, not a rustdoc fact. It is read
    //   as `index_bucket` and goes nowhere near `program.definition`.
    //
    //   Columns 6 and 7 are documented `aliases` and `methods` but hold COUNTS
    //   (`str(len(item.aliases))`), so they are read as `alias_count` and `method_count`.
    IndexSpec {
        file: "symbols.tsv",
        columns: &[
            "canonical_path",
            "kind",
            "crate",
            "index_bucket",
            "api_page",
            "alias_count",
            "method_count",
            "summary",
        ],
        denominator: Denominator::ProgramApi,
        source: Source::Skill,
    },
    IndexSpec {
        file: "methods.tsv",
        columns: &["owner_path", "method", "via_trait", "signature", "summary"],
        denominator: Denominator::ProgramApi,
        source: Source::Skill,
    },
    // `crate` is the IMPLEMENTOR's crate, not the trait's: every `core::fmt::Debug` row carries
    // the crate of the type implementing it.
    IndexSpec {
        file: "impls.tsv",
        columns: &["trait_path", "implementor_path", "crate"],
        denominator: Denominator::ProgramApi,
        source: Source::Skill,
    },
    IndexSpec {
        file: "aliases.tsv",
        columns: &["access_path", "canonical_path", "kind"],
        denominator: Denominator::ProgramApi,
        source: Source::Skill,
    },
    // ONE column, holding `source -> target` as text. There is no `mapping_status` column: that
    // value is the `binding` lattice's `unbound`, which the builder assigns because the file's
    // existence is what records the fact.
    IndexSpec {
        file: "unresolved.tsv",
        columns: &["access_path"],
        denominator: Denominator::ProgramApi,
        source: Source::Skill,
    },
    // ---- producers/cargo-metadata-adapter ------------------------------------------------------
    //
    // §3.1's staging table uses one wide relation with a `row_kind` discriminator. A TSV cannot:
    // `read_index` checks an exact field count per file, which is the drift check earning its
    // keep, so one file per row kind is the honest analogue -- the same substitution `library-api`
    // made when it read §3.5 from an index instead of a live rustdoc run.
    IndexSpec {
        file: "packages.tsv",
        columns: &[
            "package_key",
            "name",
            "version",
            "registry",
            "repository",
            "edition",
            "rust_version",
            "license",
            "in_subject_pins",
        ],
        denominator: Denominator::ProgramApi,
        source: Source::CargoMetadata,
    },
    IndexSpec {
        file: "crate_units.tsv",
        columns: &[
            "package_key",
            "target_name",
            "target_kind",
            "src_path",
            "edition",
        ],
        denominator: Denominator::ProgramApi,
        source: Source::CargoMetadata,
    },
    IndexSpec {
        file: "features.tsv",
        columns: &["package_key", "feature_name", "implies"],
        denominator: Denominator::ProgramApi,
        source: Source::CargoMetadata,
    },
    IndexSpec {
        file: "dep_edges.tsv",
        columns: &[
            "package_key",
            "dep_package_key",
            "dep_name",
            "dep_req",
            "dep_kind",
            "dep_target_cfg",
        ],
        denominator: Denominator::ProgramApi,
        source: Source::CargoMetadata,
    },
];
