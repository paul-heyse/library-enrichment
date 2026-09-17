//! PB16 — when the proc-macro server is unavailable, is that a RECORDABLE fact or a silent one?
//!
//! PLAN-V2 §12.3's last open question, and §3.2 is what depends on it: `staging.project_load`
//! reserves `proc_macro_policy` with the values `enabled | disabled | server_failed`, and the
//! design leans on that distinction hard. Evidence 06 singles it out:
//!
//! > with proc macros disabled, family 3.4's resolutions are systematically incomplete in a way
//! > nothing else records
//!
//! So `proc_macro_policy` is not diagnostics. It is a precondition on the interpretation of every
//! `Resolution` row in the snapshot, and §9.2 counts coverage against it. If `server_failed` is
//! not reachable — if a failed server is indistinguishable from a disabled one — then the
//! systematically-incomplete case is one the model **cannot express**, and that is a finding about
//! the design rather than about rust-analyzer.
//!
//!     cargo run
//!
//! ARMS — the three values §3.2 reserves, asked for by name
//!   A  ProcMacroServerChoice::None            -> should mean `disabled`
//!   B  ProcMacroServerChoice::Explicit(<none>) -> should mean `server_failed`
//!   C  ProcMacroServerChoice::Sysroot          -> CONTROL: should mean `enabled`
//!
//! Arm C is load-bearing for PB07's reason. Without a server that starts, "the client was None"
//! could mean this machine has no proc-macro server at all rather than that the arm failed — the
//! same confound that made PB07's first run measure nothing.
//!
//! # What the vendored source says, read before writing this
//!
//! §3.4 says to write against `rust-code-model`'s vendored
//! `content/corpus/rust-analyzer/load-cargo-lib.rs` rather than against docs, because `ra_ap_hir`
//! is 12.84% documented. Doing so shows the answer has two halves:
//!
//! * The distinction IS made. `load-cargo-lib.rs:111-134` maps each choice to a distinct value:
//!   `None => Err(ProcMacroLoadingError::Disabled)`, `Explicit(path) => Err(ProcMacroSrvError(..))`
//!   on a spawn failure, `Ok(client)` on success.
//! * And it is then THROWN AWAY at the boundary. Line 204 returns
//!   `proc_macro_server.and_then(Result::ok)`, so `load_workspace_at` hands back
//!   `Option<ProcMacroClient>` — and `None` means both "disabled" and "failed". The reason
//!   survives only in `tracing::info!` and in the per-crate `proc_macros` map
//!   (`load-cargo-lib.rs:158-180`), where every crate carries its own `ProcMacroLoadResult`.
//!
//! This probe measures whether that reading holds at runtime, because a reading of what the code
//! does is not a measurement of what it does — which is the lesson PB17 recorded when the plan's
//! summary of a dossier turned out to name a method that does not exist.

use std::path::{Path, PathBuf};

use load_cargo::{LoadCargoConfig, ProcMacroServerChoice};
use project_model::CargoConfig;

/// A minimal but REAL Cargo project. `load_workspace_at` discovers a workspace; it does not accept
/// a synthetic one, so there has to be a manifest on disk.
///
/// Written under the system temp directory, never inside a repository: the boundary rule says a
/// repository under study is never an output destination, and the same applies to this tree.
fn fixture() -> std::io::Result<PathBuf> {
    let root = std::env::temp_dir().join("pb16-proc-macro-fixture");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src"))?;
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\n\n[package]\nname = \"pb16-fixture\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[dependencies]\n",
    )?;
    std::fs::write(
        root.join("src/lib.rs"),
        "pub fn answer() -> u32 { 42 }\npub fn caller() -> u32 { answer() }\n",
    )?;
    Ok(root)
}

/// What a caller can actually see, per arm.
struct Observed {
    loaded: bool,
    client: &'static str,
    error: String,
}

fn load(root: &Path, choice: ProcMacroServerChoice) -> Observed {
    let config = LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro_server: choice,
        prefill_caches: false,
        num_worker_threads: 1,
        proc_macro_processes: 1,
    };
    match load_cargo::load_workspace_at(root, &CargoConfig::default(), &config, &|_| {}) {
        Ok((_db, _vfs, client)) => Observed {
            loaded: true,
            client: if client.is_some() { "Some" } else { "None" },
            error: String::new(),
        },
        Err(e) => Observed {
            loaded: false,
            client: "-",
            error: e.to_string().lines().next().unwrap_or("").to_string(),
        },
    }
}

/// The proc-macro server shipped with the toolchain this crate is pinned to.
///
/// Named explicitly rather than discovered, for the reason the control comment gives: sysroot
/// discovery is a second variable, and a probe that varies two things measures neither.
fn real_proc_macro_srv() -> Option<PathBuf> {
    let home = match std::env::var("RUSTUP_HOME") {
        Ok(h) => PathBuf::from(h),
        Err(_) => PathBuf::from(std::env::var("HOME").ok()?).join(".rustup"),
    }
    .join("toolchains");
    let mut found: Vec<PathBuf> = std::fs::read_dir(home)
        .ok()?
        .flatten()
        .map(|e| e.path().join("libexec/rust-analyzer-proc-macro-srv"))
        .filter(|p| p.is_file())
        .collect();
    found.sort();
    found.into_iter().next()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture()?;
    println!("PB16 — is a failed proc-macro server distinguishable from a disabled one?\n");
    println!("  fixture  {}", root.display());
    println!("  asking   the three values §3.2 reserves: disabled, server_failed, enabled\n");

    let nowhere = root.join("no-such-proc-macro-srv");
    // The CONTROL points at the real server explicitly rather than asking for `Sysroot`.
    //
    // The first run of this probe used `Sysroot` and came out INCONCLUSIVE: all three arms
    // returned `client=None`, so the control failed the same way as the treatments and nothing was
    // measured. The cause was not a missing server -- every toolchain on this machine ships
    // `libexec/rust-analyzer-proc-macro-srv` -- but `CargoConfig::default()`, which discovers no
    // sysroot, so `find_sysroot_proc_macro_srv` had nothing to find.
    //
    // Using `Explicit` for both B and C holds the mechanism fixed and varies only the path, which
    // is what makes the difference attributable. Recording this rather than quietly switching:
    // a control that fails like its treatment is PB07's lesson and it recurred here.
    let real = real_proc_macro_srv().ok_or(
        "no rust-analyzer-proc-macro-srv on this machine, so the control cannot be run and \
         the probe would be inconclusive by construction",
    )?;
    println!("  control  {}\n", real.display());
    let arms: Vec<(&str, &str, ProcMacroServerChoice)> = vec![
        ("A  None             ", "disabled", ProcMacroServerChoice::None),
        (
            "B  Explicit(<none>) ",
            "server_failed",
            ProcMacroServerChoice::Explicit(paths::AbsPathBuf::assert_utf8(nowhere).to_path_buf()),
        ),
        (
            "C  Explicit(real)   ",
            "enabled",
            ProcMacroServerChoice::Explicit(paths::AbsPathBuf::assert_utf8(real).to_path_buf()),
        ),
    ];

    let mut seen: Vec<(String, String)> = Vec::new();
    for (label, intended, choice) in arms {
        let o = load(&root, choice);
        let visible = format!("loaded={} client={} {}", o.loaded, o.client, o.error);
        println!("  {label}  intends `{intended:<14}`  -> {visible}");
        seen.push((intended.to_string(), format!("loaded={} client={}", o.loaded, o.client)));
    }

    println!("\n  ---");
    let disabled = &seen[0].1;
    let failed = &seen[1].1;
    let enabled = &seen[2].1;
    if enabled == disabled {
        println!("  INCONCLUSIVE  the CONTROL is indistinguishable from the disabled arm, so this");
        println!("                machine has no working proc-macro server and no arm measured");
        println!("                anything. Same confound as PB07's first run.");
        return Err("PB16 did not confirm".into());
    }
    if disabled == failed {
        println!("  RECORDED   `disabled` and `server_failed` are INDISTINGUISHABLE from what");
        println!("             `load_workspace_at` returns: both are `client=None`.");
        println!("             The distinction IS made internally -- load-cargo-lib.rs:111-134");
        println!("             maps them to ProcMacroLoadingError::Disabled and ::ProcMacroSrvError");
        println!("             -- and is discarded at line 204 by `and_then(Result::ok)`.");
        println!("             It survives per crate in the `proc_macros` map (lines 158-180),");
        println!("             so §3.2's value is reachable, but NOT from this return type.");
        println!("             An adapter must read the per-crate ProcMacroLoadResult, and a");
        println!("             `project-load` family that recorded only the Option would collapse");
        println!("             two states evidence 06 says must not be collapsed.");
    } else {
        println!("  CONFIRMED  all three states are distinguishable from the public return value.");
    }
    Ok(())
}
