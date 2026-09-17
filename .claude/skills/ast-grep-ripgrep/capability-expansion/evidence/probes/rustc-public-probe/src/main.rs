//! PB15 — is `rustc_public` a usable extraction surface for `staging.mir`?
//!
//! PLAN-V2 §3.6 leaves `mapping_basis` open between `mir.rustc_public@<phase>` and
//! `mir.unpretty@<phase>`, and §12.3 question 1 asks whether the typed API is reachable at all.
//! This probe answers both with a measurement instead of a preference.
//!
//! # The two arms
//!
//! **Treatment** — drive the compiler through `rustc_public`, walk each body, and project it
//! into §3.6's column shape: `(body_owner_handle, block_index, stmt_index, stmt_kind,
//! terminator_kind, edge_target, edge_kind)`. Projecting into the real shape rather than
//! counting something convenient is the point: a surface that exposes `Body` but cannot yield
//! `cfg_edge` would pass a block count and still be unusable.
//!
//! **Control** — the same three functions through `rustc -Zunpretty=mir`, the textual surface
//! §3.6 calls "a probe surface, not an extraction format". Its block counts must match.
//!
//! # Why the control can fail
//!
//! The fixture holds three functions with three deliberately different CFG shapes. A control
//! that compared one function's count against one number could agree by accident — two broken
//! readers both reporting 1 agree perfectly. Three shapes compared pairwise, in order, cannot:
//! agreement then requires the two arms to vary together.
//!
//! The probe also reports the MIR **phase** each arm saw, because §3.6 makes `mir_phase`
//! REQUIRED and a disagreement in phase would explain a disagreement in counts without either
//! arm being wrong. That distinction is the whole reason the column exists.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_public;

use std::ops::ControlFlow;
use std::path::{Path, PathBuf};
use std::process::Command;

use rustc_public::CrateDef;
use rustc_public::mir::{Body, StatementKind, TerminatorKind};

/// One row in §3.6's `staging.mir` shape. Only the columns this surface can actually fill are
/// present; `mir_phase` is carried on the body, not repeated per row.
#[derive(Debug)]
struct MirRow {
    block_index: i64,
    /// `None` for the block's terminator row, `Some(i)` for its i-th statement.
    stmt_index: Option<i64>,
    stmt_kind: Option<String>,
    terminator_kind: Option<String>,
    edge_target: Option<i64>,
    edge_kind: Option<String>,
}

#[derive(Debug)]
struct BodyRows {
    body_owner_handle: String,
    local_count: usize,
    block_count: usize,
    rows: Vec<MirRow>,
}

fn statement_kind(k: &StatementKind) -> &'static str {
    match k {
        StatementKind::Assign(..) => "Assign",
        StatementKind::FakeRead(..) => "FakeRead",
        StatementKind::SetDiscriminant { .. } => "SetDiscriminant",
        StatementKind::StorageLive(..) => "StorageLive",
        StatementKind::StorageDead(..) => "StorageDead",
        StatementKind::PlaceMention(..) => "PlaceMention",
        StatementKind::AscribeUserType { .. } => "AscribeUserType",
        StatementKind::Coverage(..) => "Coverage",
        StatementKind::Intrinsic(..) => "Intrinsic",
        StatementKind::ConstEvalCounter => "ConstEvalCounter",
        StatementKind::Nop => "Nop",
    }
}

// Eleven variants, and the match is exhaustive WITHOUT a wildcard on purpose. §3.6 says the
// operation vocabulary is "generated from the pinned rustc source, not hand-listed, so the
// vocabulary CHECK constraints stay true when the toolchain moves" -- a `_ =>` arm here would
// turn the next toolchain's new variant into a silently mislabelled row instead of a build
// error, which is the failure that discipline exists to prevent.
//
// It also records a finding: `rustc_public::mir::StatementKind` has ELEVEN variants where
// `rustc_middle::mir::StatementKind` has twelve. The generator must read the public enum.

fn terminator_kind(k: &TerminatorKind) -> &'static str {
    match k {
        TerminatorKind::Goto { .. } => "Goto",
        TerminatorKind::SwitchInt { .. } => "SwitchInt",
        TerminatorKind::Resume => "Resume",
        TerminatorKind::Abort => "Abort",
        TerminatorKind::Return => "Return",
        TerminatorKind::Unreachable => "Unreachable",
        TerminatorKind::Drop { .. } => "Drop",
        TerminatorKind::Call { .. } => "Call",
        TerminatorKind::Assert { .. } => "Assert",
        TerminatorKind::InlineAsm { .. } => "InlineAsm",
    }
}

/// What kind of control-flow edge a successor ordinal represents.
///
/// This is the column §3.6 calls `edge_kind`, and it is the reason the terminator is read as a
/// structure rather than printed: `switchInt -> [0: bb2, otherwise: bb1]` distinguishes a
/// matched value from the fallthrough, and the text does too, but only for a parser that
/// already knows the grammar of every terminator.
fn edge_kinds(k: &TerminatorKind) -> Vec<(i64, String)> {
    match k {
        TerminatorKind::Goto { target } => vec![(*target as i64, "goto".to_string())],
        TerminatorKind::SwitchInt { targets, .. } => {
            let mut out: Vec<(i64, String)> = targets
                .branches()
                .map(|(v, t)| (t as i64, format!("switch:{v}")))
                .collect();
            out.push((targets.otherwise() as i64, "switch:otherwise".to_string()));
            out
        }
        TerminatorKind::Call { target, .. } => target
            .iter()
            .map(|t| (*t as i64, "call:return".to_string()))
            .collect(),
        TerminatorKind::Assert { target, .. } => {
            vec![(*target as i64, "assert:success".to_string())]
        }
        TerminatorKind::Drop { target, .. } => vec![(*target as i64, "drop:continue".to_string())],
        TerminatorKind::InlineAsm { destination, .. } => destination
            .iter()
            .map(|t| (*t as i64, "asm:destination".to_string()))
            .collect(),
        TerminatorKind::Return
        | TerminatorKind::Resume
        | TerminatorKind::Abort
        | TerminatorKind::Unreachable => Vec::new(),
    }
}

fn project(name: &str, body: &Body) -> BodyRows {
    let mut rows = Vec::new();
    for (bi, block) in body.blocks.iter().enumerate() {
        for (si, stmt) in block.statements.iter().enumerate() {
            rows.push(MirRow {
                block_index: bi as i64,
                stmt_index: Some(si as i64),
                stmt_kind: Some(statement_kind(&stmt.kind).to_string()),
                terminator_kind: None,
                edge_target: None,
                edge_kind: None,
            });
        }
        let tk = terminator_kind(&block.terminator.kind);
        let edges = edge_kinds(&block.terminator.kind);
        if edges.is_empty() {
            rows.push(MirRow {
                block_index: bi as i64,
                stmt_index: None,
                stmt_kind: None,
                terminator_kind: Some(tk.to_string()),
                edge_target: None,
                edge_kind: None,
            });
        } else {
            for (target, kind) in edges {
                rows.push(MirRow {
                    block_index: bi as i64,
                    stmt_index: None,
                    stmt_kind: None,
                    terminator_kind: Some(tk.to_string()),
                    edge_target: Some(target),
                    edge_kind: Some(kind),
                });
            }
        }
    }
    BodyRows {
        body_owner_handle: name.to_string(),
        local_count: body.locals().len(),
        block_count: body.blocks.len(),
        rows,
    }
}

/// The treatment arm, run inside the compiler after analysis.
fn walk() -> ControlFlow<Vec<BodyRows>, ()> {
    let mut out = Vec::new();
    for item in rustc_public::all_local_items() {
        if let Some(body) = item.body() {
            out.push(project(&item.name(), &body));
        }
    }
    // Break, not Continue: this stops the hosted compilation after analysis. Codegen would add
    // nothing to the measurement and the probe must not write an artifact anywhere.
    ControlFlow::Break(out)
}

/// The control arm: block counts per function, parsed from the textual surface.
fn unpretty_block_counts(sysroot: &str, fixture: &Path) -> Result<Vec<(String, usize)>, String> {
    // `$sysroot/bin/rustc`, NOT a bare `rustc`. A bare one goes through the rustup proxy, which
    // resolves against the repository's `rust-toolchain.toml` and lands on stable -- where `-Z`
    // is refused outright. Naming the binary inside the sysroot the treatment arm already uses
    // also makes the two arms the same compiler by construction, so a difference between them
    // cannot be a difference of commit.
    let out = Command::new(format!("{sysroot}/bin/rustc"))
        .args(["-Zunpretty=mir", "--crate-type", "lib"])
        .arg(fixture)
        .output()
        .map_err(|e| format!("spawning rustc: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "rustc -Zunpretty=mir failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut counts: Vec<(String, usize)> = Vec::new();
    for line in text.lines() {
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix("fn ")
            && let Some(paren) = rest.find('(')
        {
            counts.push((rest[..paren].to_string(), 0));
        } else if t.starts_with("bb")
            && t.contains(": {")
            && let Some(last) = counts.last_mut()
        {
            last.1 += 1;
        }
    }
    Ok(counts)
}

/// Arm C: the same function at every MIR phase the compiler will dump.
///
/// Arms A and B agreeing on a block count does NOT establish that they saw the same body.
/// §3.6 makes `mir_phase` REQUIRED because "built -> SimplifyCfg-initial -> PromoteTemps ->
/// analysis -> nll -> runtime all exist for one body", and this arm is what turns that sentence
/// into numbers: it dumps every phase and reports the count at each, so the agreement in the
/// verdict can be attributed to a phase instead of assumed.
///
/// It also answers a question the other two arms cannot: which phases are REACHABLE through a
/// candidate `mapping_basis` at all.
fn phase_block_counts(sysroot: &str, fixture: &Path, func: &str) -> Result<Vec<(String, usize)>, String> {
    let dir = std::env::temp_dir().join(format!("pb15-mir-{}", std::process::id()));
    std::fs::create_dir_all(&dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
    // `--out-dir` into the same scratch directory, and deliberately NOT `--emit=metadata`:
    // metadata-only never requests `optimized_mir`, so the runtime phases would be absent and
    // the arm would report their absence as a finding when it was only an artefact of the flags.
    let out = Command::new(format!("{sysroot}/bin/rustc"))
        .arg(format!("-Zdump-mir={func}"))
        .arg(format!("-Zdump-mir-dir={}", dir.display()))
        .args(["--crate-type", "lib", "--out-dir"])
        .arg(&dir)
        .arg(fixture)
        .output()
        .map_err(|e| format!("spawning rustc -Zdump-mir: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "rustc -Zdump-mir failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let mut found: Vec<(String, usize)> = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| format!("reading {}: {e}", dir.display()))? {
        let path = entry.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|e| e != "mir") {
            continue;
        }
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let blocks = text
            .lines()
            .filter(|l| {
                let t = l.trim_start();
                t.starts_with("bb") && t.contains(": {")
            })
            .count();
        found.push((name, blocks));
    }
    let _ = std::fs::remove_dir_all(&dir);
    found.sort();
    Ok(found)
}

fn main() {
    let sysroot = env!("PROBE_SYSROOT");
    let fixture: PathBuf = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/cfg_shapes.rs"));

    println!("PB15 — is `rustc_public` a usable extraction surface for `staging.mir`?\n");
    println!("  sysroot: {sysroot}");
    println!("  fixture: {}\n", fixture.display());

    // ---- CONTROL first, so a treatment that aborts the process still leaves it recorded -----
    println!("  B CONTROL  rustc -Zunpretty=mir, block count per fn");
    println!("      expectation: must produce the SAME counts, in the same order, as arm A");
    let control = unpretty_block_counts(sysroot, &fixture);
    match &control {
        Ok(counts) => {
            for (name, n) in counts {
                println!("      {name:<14} {n} blocks");
            }
        }
        Err(e) => println!("      RESULT:      FAILED — {e}"),
    }
    println!();

    // ---- TREATMENT --------------------------------------------------------------------------
    println!("  A          rustc_public, body walked into §3.6's staging.mir shape");
    println!("      expectation: Body / BasicBlock / TerminatorKind reachable, and cfg_edge fillable");
    // `--out-dir` into scratch space, NOT the working directory.
    //
    // The hosted compilation is stopped after analysis and never codegens, and it still writes
    // `lib<crate>.rmeta` next to wherever it was invoked from. The surrounding repository's
    // boundary rule is that a repository under study is never an extraction destination, and a
    // probe that drops a build artifact into the source tree breaks it just as surely as an
    // extractor would. This was caught by `git status`, which is not a control anybody designed.
    let scratch = std::env::temp_dir().join(format!("pb15-out-{}", std::process::id()));
    if let Err(e) = std::fs::create_dir_all(&scratch) {
        println!("      RESULT:      INCONCLUSIVE — cannot create {}: {e}", scratch.display());
        std::process::exit(1);
    }
    let args: Vec<String> = vec![
        "rustc".to_string(),
        "--sysroot".to_string(),
        sysroot.to_string(),
        "--crate-type".to_string(),
        "lib".to_string(),
        "--out-dir".to_string(),
        scratch.display().to_string(),
        fixture.display().to_string(),
    ];
    let treatment = rustc_public::run!(&args, walk);

    let bodies = match treatment {
        Err(rustc_public::CompilerError::Interrupted(bodies)) => bodies,
        Ok(()) => {
            println!("      RESULT:      INCONCLUSIVE — the callback never ran");
            std::process::exit(1);
        }
        Err(rustc_public::CompilerError::Failed) => {
            println!("      RESULT:      FAILED — the hosted compilation errored");
            std::process::exit(1);
        }
        Err(rustc_public::CompilerError::Skipped) => {
            println!("      RESULT:      INCONCLUSIVE — compilation was skipped");
            std::process::exit(1);
        }
    };

    let _ = std::fs::remove_dir_all(&scratch);

    for b in &bodies {
        println!(
            "      {:<14} {} blocks, {} locals, {} staging.mir rows",
            b.body_owner_handle,
            b.block_count,
            b.local_count,
            b.rows.len()
        );
    }
    println!();

    // ---- The §3.6 shape, shown rather than asserted -----------------------------------------
    if let Some(b) = bodies
        .iter()
        .find(|b| b.body_owner_handle.ends_with("::looping"))
    {
        println!("  A′         the rows for `looping`, in §3.6's columns");
        println!("      block  stmt  stmt_kind         terminator_kind  edge_target  edge_kind");
        for r in &b.rows {
            println!(
                "      {:<6} {:<5} {:<17} {:<16} {:<12} {}",
                r.block_index,
                r.stmt_index.map(|i| i.to_string()).unwrap_or_default(),
                r.stmt_kind.clone().unwrap_or_default(),
                r.terminator_kind.clone().unwrap_or_default(),
                r.edge_target.map(|i| i.to_string()).unwrap_or_default(),
                r.edge_kind.clone().unwrap_or_default(),
            );
        }
        let back_edges = b
            .rows
            .iter()
            .filter(|r| r.edge_target.is_some_and(|t| t < r.block_index))
            .count();
        println!("      back edges (edge_target < block_index): {back_edges}");
        println!();
    }

    // ---- ARM C: which phase did arms A and B actually see? ----------------------------------
    println!("  C          the same `looping` body at every phase the compiler will dump");
    println!("      expectation: the counts must DIFFER between phases -- if they did not, the");
    println!("      agreement in arms A and B would carry no information about phase at all");
    let mut phase_summary: Vec<(String, usize)> = Vec::new();
    match phase_block_counts(sysroot, &fixture, "looping") {
        Err(e) => println!("      RESULT:      INCONCLUSIVE — {e}"),
        Ok(found) => {
            // Report the named milestones §3.6's vocabulary uses, plus the terminal one.
            for marker in ["built.after", "analysis.after", "runtime-optimized.after"] {
                if let Some((name, n)) = found.iter().find(|(name, _)| name.contains(marker)) {
                    println!("      {marker:<26} {n} blocks   ({name})");
                    phase_summary.push((marker.to_string(), *n));
                }
            }
            let distinct: std::collections::BTreeSet<usize> =
                phase_summary.iter().map(|(_, n)| *n).collect();
            println!(
                "      {} phases dumped, {} distinct block counts among the three named phases",
                found.len(),
                distinct.len()
            );
        }
    }
    println!();

    // ---- The comparison ---------------------------------------------------------------------
    println!("  VERDICT");
    match control {
        Err(e) => {
            println!("      INCONCLUSIVE — the control did not run ({e}), so arm A measured");
            println!("      nothing it can be compared against.");
            std::process::exit(1);
        }
        Ok(counts) => {
            // The textual surface prints a bare `fn straight_line(`; the typed one yields the
            // qualified `cfg_shapes::straight_line`. The qualified form is what
            // `body_owner_handle` should hold, so the handle is kept and only the COMPARISON
            // is normalised.
            let treated: Vec<(String, usize)> = bodies
                .iter()
                .map(|b| {
                    let short = b
                        .body_owner_handle
                        .rsplit("::")
                        .next()
                        .unwrap_or(&b.body_owner_handle)
                        .to_string();
                    (short, b.block_count)
                })
                .collect();
            let mut sorted_control = counts.clone();
            sorted_control.sort();
            let mut sorted_treated = treated.clone();
            sorted_treated.sort();

            let distinct: std::collections::BTreeSet<usize> =
                sorted_treated.iter().map(|(_, n)| *n).collect();
            if distinct.len() < 2 {
                println!(
                    "      INCONCLUSIVE — every fixture function reported the same block count,"
                );
                println!("      so an agreeing control would not have been able to disagree.");
                std::process::exit(1);
            }

            if sorted_control == sorted_treated {
                println!("      CONFIRMED — the typed surface and the textual surface agree on");
                println!("      all {} functions, and the counts differ BETWEEN functions, so the", treated.len());
                println!("      agreement is not the agreement of two constants.");

                // Arm C turns "they agree" into "they agree AT A PHASE".
                let looping = sorted_treated
                    .iter()
                    .find(|(n, _)| n == "looping")
                    .map(|(_, c)| *c);
                if let (Some(observed), false) = (looping, phase_summary.is_empty()) {
                    let matched: Vec<&str> = phase_summary
                        .iter()
                        .filter(|(_, n)| *n == observed)
                        .map(|(p, _)| p.as_str())
                        .collect();
                    println!();
                    println!("      AND arm C attributes that agreement to a PHASE. `looping` is");
                    for (phase, n) in &phase_summary {
                        println!("        {phase:<26} {n} blocks");
                    }
                    println!("      and arms A and B both reported {observed}, which is {matched:?}.");
                    println!("      So BOTH candidate mapping_bases read the same phase, and it is");
                    println!("      the only one either of them can reach. §3.6's `mir_phase`");
                    println!("      vocabulary has three values; these two surfaces reach one.");
                }
            } else {
                println!("      DISAGREEMENT — the two surfaces do not see the same body.");
                println!("      A (rustc_public): {sorted_treated:?}");
                println!("      B (unpretty):     {sorted_control:?}");
                println!("      This is a finding about MIR PHASE, not about either reader:");
                println!("      §3.6 makes `mir_phase` REQUIRED for exactly this reason.");
            }
        }
    }
}
