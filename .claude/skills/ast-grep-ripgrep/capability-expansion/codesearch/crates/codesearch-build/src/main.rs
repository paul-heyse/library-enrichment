//! `codesearch-build` -- assemble the capability catalog from a skill's index files.
//!
//! Long-running by design, and it never serves queries. The query side is a separate one-shot
//! binary, so a build that is rewriting tables cannot be observed half-done by a reader.

mod capabilities;
mod cargo_metadata_family;
mod catalog;
#[cfg(test)]
mod catalog_tests;
mod implementation_bindings;
mod index;
mod interactions;
mod library_api;
mod plan_fragments;
mod runs;
mod writer;
#[cfg(test)]
mod writer_tests;

use std::path::PathBuf;

use clap::Parser;

/// Build the catalog from a capability-repository skill.
#[derive(Parser, Debug)]
#[command(name = "codesearch-build", about, long_about = None)]
struct Args {
    /// Root of the skill to read. Required, never inferred: the tool must work against any copy
    /// of the skill, not just the one it happens to live inside.
    #[arg(long)]
    skill_root: PathBuf,

    /// Where the catalog tables are written. Outside any repository under study.
    #[arg(long)]
    home: PathBuf,

    /// A producer's output directory, laid out like a skill: `content/index/*.tsv` plus
    /// `content/PROVENANCE.json`.
    ///
    /// Optional, and its absence is not a silent one -- without it the `cargo-metadata` family
    /// simply does not run, and `codesearch-query runs` shows two run rows instead of three.
    /// Written by `producers/cargo-metadata-adapter`, which is a separate crate on purpose: the
    /// build resolves twelve dependencies and linking the producers in would add 223 crates for
    /// `ra_ap_*` alone.
    #[arg(long)]
    producer_root: Option<PathBuf>,

    /// Remove one snapshot from the catalog and exit, building nothing.
    ///
    /// §12.2 named this as deferred and named its mechanism; PB17 measured it. It lives on the
    /// BUILD binary rather than the query one because it writes, and §8 is explicit that
    /// `codesearch-query` opens the catalog read-only.
    ///
    /// Without it the catalog only grows: every distinct set of inputs is a new snapshot and
    /// nothing ever removes one.
    #[arg(long, conflicts_with_all = ["producer_root", "snapshot_id"])]
    prune_snapshot: Option<String>,

    /// Identifies this catalog build. Defaults to a digest of every input root's bytes, so two
    /// builds of the same inputs produce the same snapshot and `compare` sees no spurious change.
    #[arg(long)]
    snapshot_id: Option<String>,
}

/// The `entity_key` column of a batch, as a set.
///
/// Used to validate an authored seed against rows that already exist rather than against rows the
/// build intends to mint.
fn keys_of(
    batch: &arrow::array::RecordBatch,
) -> Result<std::collections::BTreeSet<String>, Box<dyn std::error::Error>> {
    use arrow::array::Array;
    let column = batch
        .column_by_name("entity_key")
        .ok_or("batch has no entity_key column")?;
    let keys = column
        .as_any()
        .downcast_ref::<arrow::array::StringArray>()
        .ok_or("entity_key is not a string column")?;
    Ok((0..keys.len()).map(|i| keys.value(i).to_string()).collect())
}

/// Remove one snapshot from every table that carries one.
///
/// `catalog.lattice` is skipped because it has no `snapshot_id` -- rank-to-label is not an
/// observation about a snapshot, which is the same reason it is exempt from the run and snapshot
/// filters and from run-scoped writes.
///
/// Row counts are read before and after, per table, because PB17 measured that a write commits
/// even with nothing to do: reporting versions would say something happened when nothing did.
async fn prune(tables: &std::path::Path, snapshot: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (ctx, session) = codesearch_bridge::session::build_session()?;
    let mut removed = 0usize;
    let mut touched = 0usize;
    println!("prune     {snapshot}");
    for (name, _) in codesearch_model::schema::all_tables() {
        if name == "catalog.lattice" {
            continue;
        }
        let before = snapshot_rows(tables, name, snapshot, &ctx).await?;
        if before == 0 {
            continue;
        }
        writer::prune_snapshot(tables, name, snapshot, &session).await?;
        let after = snapshot_rows(tables, name, snapshot, &ctx).await?;
        println!(
            "removed   {:>5}  {name}{}",
            before - after,
            if after == 0 { "" } else { "  INCOMPLETE" }
        );
        removed += before - after;
        touched += 1;
    }
    if touched == 0 {
        println!(
            "nothing   no table holds rows for that snapshot -- `runs` lists the ones that exist"
        );
    } else {
        println!("removed   {removed} row(s) across {touched} table(s)");
    }
    Ok(())
}

/// How many rows one table holds for one snapshot.
async fn snapshot_rows(
    tables: &std::path::Path,
    name: &str,
    snapshot: &str,
    ctx: &datafusion::prelude::SessionContext,
) -> Result<usize, Box<dyn std::error::Error>> {
    let Some(batch) = writer::read_table(tables, name, ctx).await? else {
        return Ok(0);
    };
    use arrow::array::Array;
    let Some(column) = batch.column_by_name("snapshot_id") else {
        return Ok(0);
    };
    let cast = arrow::compute::cast(column, &arrow_schema::DataType::Utf8)?;
    let ids = cast
        .as_any()
        .downcast_ref::<arrow::array::StringArray>()
        .ok_or("snapshot_id is not a string column")?;
    Ok((0..ids.len()).filter(|i| ids.value(*i) == snapshot).count())
}

/// One extraction family's output: what it is called, the run that produced it, and the canonical
/// tables it writes.
struct Family {
    name: &'static str,
    run_id: String,
    targets: Vec<(&'static str, arrow::array::RecordBatch)>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Pruning is a different operation, not a phase of a build: it reads no index and writes no
    // run row. Doing it first and returning keeps the two from sharing state neither needs.
    if let Some(snapshot) = &args.prune_snapshot {
        return prune(&args.home.join("catalog"), snapshot).await;
    }

    let indexes = index::read_all(&args.skill_root, index::Source::Skill)?;
    // Read separately and kept separate. Two roots can ship the same filename -- `rust-code-model`
    // uses five of ast-grep-ripgrep's -- so one map keyed by stem would merge two subjects and the
    // first symptom would be a key collision naming no root.
    let producer_indexes = match &args.producer_root {
        Some(root) => index::read_all(root, index::Source::CargoMetadata)?,
        None => std::collections::BTreeMap::new(),
    };
    let loaded: usize = indexes.values().map(Vec::len).sum::<usize>()
        + producer_indexes.values().map(Vec::len).sum::<usize>();

    // Every root, in a fixed order, so the identity covers the bytes actually read.
    let mut roots: Vec<(&std::path::Path, index::Source)> =
        vec![(args.skill_root.as_path(), index::Source::Skill)];
    if let Some(root) = &args.producer_root {
        roots.push((root.as_path(), index::Source::CargoMetadata));
    }
    let snapshot_id = match args.snapshot_id {
        Some(id) => id,
        None => index::snapshot_id_from_inputs(&roots)?,
    };
    let context_id = index::context_id_from_pins(&args.skill_root)?;
    // ---- both families, assembled ------------------------------------------------------------
    //
    // One process, deliberately. `assert_single_writer` is a within-call duplicate check and
    // cannot see across processes, and `runs::merged` does a read-modify-write of
    // `snapshot.extraction_run` that two racing builds would corrupt. Two families in one process
    // share both, and the concatenated target list below is what proves they write disjoint tables
    // -- the precondition that keeps whole-table overwrite sound.
    let catalog_run_id = format!("run:{snapshot_id}/catalog/0");
    let catalog_run = catalog::RunContext {
        snapshot_id: &snapshot_id,
        run_id: &catalog_run_id,
    };

    // The seeds live beside the binary's own crate, not in the skill: they are this tool's
    // authored judgement, not something the skill ships.
    let seeds = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../seeds");
    let seed = capabilities::load(&seeds.join("capabilities.json"))?;
    let interaction_seed = interactions::load(&seeds.join("interactions.json"))?;
    let fragment_seed = plan_fragments::load(&seeds.join("plan_fragments.json"))?;
    let binding_seed = implementation_bindings::load(&seeds.join("implementation_bindings.json"))?;

    // ---- families, in §6.2's DAG order -------------------------------------------------------
    //
    // **A before B**: `cargo-metadata` is pass A and owns `program.package`, which the authored
    // bindings reference and which `library-api` used to mint. `catalog` is last because its
    // `surface_binding` seed is validated against keys from both -- and validating against keys
    // the build merely intends to mint would check nothing.
    //
    // `cargo-metadata` runs only when a producer root was given. Its absence is visible rather
    // than silent: two run rows instead of three, no `program.package` at all, and any authored
    // binding naming a `pkg:` key fails the build.
    let mut cargo_family = None;
    let mut package_keys = std::collections::BTreeSet::new();
    if !producer_indexes.is_empty() {
        let resolve_present = index::resolve_present(
            args.producer_root
                .as_deref()
                .expect("a producer root was read"),
        )?;
        let cargo_run_id = format!("run:{snapshot_id}/cargo-metadata/0");
        let cargo_run = catalog::RunContext {
            snapshot_id: &snapshot_id,
            run_id: &cargo_run_id,
        };
        let (built, keys) =
            cargo_metadata_family::build(&producer_indexes, resolve_present, cargo_run)?;
        package_keys = keys;
        cargo_family = Some((cargo_run_id, built));
    }

    let api_run_id = format!("run:{snapshot_id}/library-api/0");
    let api_run = catalog::RunContext {
        snapshot_id: &snapshot_id,
        run_id: &api_run_id,
    };
    let api = library_api::build(&indexes, api_run)?;

    // The catalog family is built twice-over in effect: once to learn which mechanisms exist, and
    // then for real with the bindings attached. Cheaper than it sounds -- the first pass is the
    // same in-memory assembly -- and it is what lets a binding naming a mechanism the catalog does
    // not hold fail the build rather than dangle.
    let probe = catalog::build(
        &indexes,
        &seed,
        &interaction_seed,
        &fragment_seed,
        &[],
        catalog_run,
    )?;
    let impl_bindings = implementation_bindings::validate(
        &binding_seed,
        &keys_of(&probe.mechanism)?,
        &keys_of(&api.definition)?,
        &package_keys,
    )?;
    let batches = catalog::build(
        &indexes,
        &seed,
        &interaction_seed,
        &fragment_seed,
        &impl_bindings,
        catalog_run,
    )?;

    let mut families = vec![
        Family {
            name: "catalog",
            run_id: catalog_run_id,
            targets: vec![
                ("catalog.lattice", batches.lattice),
                ("catalog.capability", batches.capability),
                ("catalog.mechanism", batches.mechanism),
                ("catalog.parameter", batches.parameter),
                ("catalog.result_contract", batches.result_contract),
                ("catalog.search_domain", batches.search_domain),
                ("catalog.negative_space", batches.negative_space),
                ("catalog.plan_fragment", batches.plan_fragment),
                ("catalog.interaction", batches.interaction),
                ("catalog.interaction_atom", batches.interaction_atom),
                ("catalog.surface_binding", batches.surface_binding),
                ("catalog.surface_entry", batches.surface_entry),
                ("evidence.behavior_assertion", batches.behavior_assertion),
            ],
        },
        Family {
            name: "library-api",
            run_id: api_run_id,
            targets: vec![
                ("program.definition", api.definition),
                ("program.signature", api.signature),
                ("program.implementation", api.implementation),
                ("program.export_path", api.export_path),
                ("snapshot.native_binding", api.native_binding),
            ],
        },
    ];

    if let Some((run_id, built)) = cargo_family {
        families.push(Family {
            name: "cargo-metadata",
            run_id,
            targets: vec![
                ("program.package", built.package),
                ("program.crate_unit", built.crate_unit),
                ("program.feature_declaration", built.feature_declaration),
                ("program.dependency_edge", built.dependency_edge),
            ],
        });
    }

    // Across ALL families, not within one. This is the check that would catch a second family
    // quietly taking over a table the first already writes -- which whole-table overwrite would
    // resolve by discarding the first family's rows, with no error anywhere.
    let all_names: Vec<&str> = families
        .iter()
        .flat_map(|f| f.targets.iter().map(|(n, _)| *n))
        .collect();
    writer::assert_single_writer(&all_names)?;

    let tables = args.home.join("catalog");
    std::fs::create_dir_all(&tables)?;
    let (ctx, session) = codesearch_bridge::session::build_session()?;
    let run_schema = codesearch_model::schema::snapshot_extraction_run();

    let mut written = Vec::new();
    for family in families {
        // Each family brackets its own tables: run row first at `running`, then the tables, then
        // the flip to `complete`. A crash therefore leaves THAT family unaccounted for while a
        // family that already finished stays visible -- which is the whole reason visibility is
        // per-run rather than per-catalog.
        let mut record = runs::RunRecord {
            key: family.run_id.clone(),
            snapshot_id: snapshot_id.clone(),
            family: family.name.to_string(),
            extractor_identity: concat!("codesearch-build@", env!("CARGO_PKG_VERSION")).to_string(),
            // The interpretive context: which TOOLS produced the indexes. Deliberately not a
            // restatement of the snapshot -- since the snapshot id covers the index bytes too, two
            // skills with different content but the same `ast-grep` and `ripgrep` share a context
            // and not a snapshot, which is the distinction the two columns exist to carry.
            context_id: context_id.clone(),
            skill_root: args.skill_root.display().to_string(),
            index_files: (indexes.len() + producer_indexes.len()) as i32,
            index_rows: loaded as i64,
            completion: runs::RUNNING,
            started_at: runs::now_micros(),
            finished_at: None,
            input_artifact_ids: indexes.keys().map(|k| (*k).to_string()).collect(),
            failed_pass: None,
        };

        // Re-read rather than threading the previous batch through: the first family has already
        // written its own row by the time the second starts, and `merged` drops only rows matching
        // the current key, so each family carries the other forward untouched.
        let previous = writer::read_table(&tables, "snapshot.extraction_run", &ctx).await?;
        let started = runs::merged(&run_schema, previous.as_ref(), &record)?;
        writer::write_table(
            &tables,
            "snapshot.extraction_run",
            started.clone(),
            &family.run_id,
            &session,
        )
        .await?;

        for (name, batch) in family.targets {
            let rows = batch.num_rows();
            if let Err(e) = writer::write_table(&tables, name, batch, &record.key, &session).await {
                // A pass that failed is recorded as `partial` with the pass NAMED, then the error
                // is returned. Reporting `partial` without saying partial in what is not a report,
                // and swallowing the error to keep the run row tidy would be worse still.
                record.completion = runs::PARTIAL;
                record.finished_at = Some(runs::now_micros());
                record.failed_pass = Some(name.to_string());
                let stopped = runs::merged(&run_schema, Some(&started), &record)?;
                writer::write_table(
                    &tables,
                    "snapshot.extraction_run",
                    stopped,
                    &record.key,
                    &session,
                )
                .await?;
                return Err(e.into());
            }
            written.push((name.to_string(), rows));
        }

        record.completion = runs::COMPLETE;
        record.finished_at = Some(runs::now_micros());
        let finished = runs::merged(&run_schema, Some(&started), &record)?;
        let run_rows = finished.num_rows();
        writer::write_table(
            &tables,
            "snapshot.extraction_run",
            finished,
            &record.key,
            &session,
        )
        .await?;
        written.push((
            format!("snapshot.extraction_run ({})", family.name),
            run_rows,
        ));
    }

    // A build reports what it did, in the four states the surrounding project uses. This is a
    // summary, not a claim about coverage -- `codesearch-query coverage` answers that, against a
    // denominator it computes rather than one this build asserts.
    println!("snapshot  {snapshot_id}");
    println!("skill     {}", args.skill_root.display());
    println!("tables    {}", tables.display());
    println!(
        "read      {loaded} index rows across {} files from {} root(s)",
        indexes.len() + producer_indexes.len(),
        roots.len()
    );
    for (name, rows) in &written {
        println!("wrote     {rows:>5}  {name}");
    }
    Ok(())
}
