# Evidence 17 — PB16: a failed proc-macro server is not distinguishable from a disabled one

Plan v2 §12.3's **last** open question, and the one the probe backlog has carried since round 2.

| | |
|---|---|
| Question | When the proc-macro server is unavailable, does `ra_ap_hir` report a recordable error, or silently return incomplete resolutions? Is §3.2's `proc_macro_policy='server_failed'` reachable? |
| Verdict | **RECORDED — reachable, but not from the API a family would naturally use.** `enabled` is distinguishable; `disabled` and `server_failed` both present as `client = None` |
| Consequence | A `project-load` family that recorded only what `load_workspace_at` returns would **collapse two states evidence 06 says must not be collapsed** |

- Source: [`probes/ra-ap-hir-probe/src/main.rs`](probes/ra-ap-hir-probe/src/main.rs)
- Capture: [`probes/PB16-proc-macro.out.txt`](probes/PB16-proc-macro.out.txt)
- Closes: plan v2 §12.3's remaining open item. **The probe backlog is now empty.**

---

## Why this mattered

§3.2 reserves `proc_macro_policy` with three values — `enabled | disabled | server_failed` — and
evidence 06 explains why it is not diagnostics:

> with proc macros disabled, family 3.4's resolutions are systematically incomplete in a way
> nothing else records

So the column is a **precondition on the interpretation of every `Resolution` row in the
snapshot**, and §9.2 counts coverage against it. If the three values are not distinguishable at
extraction time, the systematically-incomplete case is one the model cannot express.

## What ran

```
  control  …/1.94.0-x86_64-unknown-linux-gnu/libexec/rust-analyzer-proc-macro-srv

  A  None               intends `disabled      `  -> loaded=true client=None
  B  Explicit(<none>)   intends `server_failed `  -> loaded=true client=None
  C  Explicit(real)     intends `enabled       `  -> loaded=true client=Some
```

**Arm C is the control and it discriminates**, which is what makes A and B's agreement a finding
rather than an artefact. All three arms load the workspace successfully — `loaded=true` throughout
— so the difference is in the proc-macro channel alone.

### The control failed first, in exactly PB07's way

The first run used `ProcMacroServerChoice::Sysroot` for arm C and came out **inconclusive**: all
three arms returned `client=None`, so the control failed the same way as the treatments and
nothing had been measured. The cause was not a missing server — every toolchain on this machine
ships `libexec/rust-analyzer-proc-macro-srv` — but `CargoConfig::default()`, which discovers no
sysroot, leaving `find_sysroot_proc_macro_srv` nothing to find.

Pointing both B and C at explicit paths holds the mechanism fixed and varies only the path. That
is recorded here rather than quietly fixed: *a control that fails like its treatment measures
nothing*, and this is the second probe in this repository to be rescued by noticing it.

---

## Where the distinction goes

Read from `rust-code-model`'s vendored `content/corpus/rust-analyzer/load-cargo-lib.rs`, which is
what §3.4 says to write against because `ra_ap_hir` is 12.84% documented upstream.

**The distinction is made.** `load-cargo-lib.rs:111-134` maps each choice to its own value:

```rust
ProcMacroServerChoice::Sysroot  => …map_err(|e| ProcMacroLoadingError::ProcMacroSrvError(…))
ProcMacroServerChoice::Explicit(path) => ProcMacroClient::spawn(path, …)
                                   .map_err(|e| ProcMacroLoadingError::ProcMacroSrvError(…))
ProcMacroServerChoice::None     => Some(Err(ProcMacroLoadingError::Disabled))
```

**And then discarded**, at line 204:

```rust
Ok((vfs, proc_macro_server.and_then(Result::ok)))
```

`Result::ok` throws the error away, so `Option<ProcMacroClient>` carries "it worked" and nothing
else. The reason survives in two places the return type does not reach:

1. `tracing::info!` — three distinct messages at lines 135-144, which is logging, not data.
2. **The per-crate `proc_macros` map** (lines 158-180): every crate's entry is a
   `ProcMacroLoadResult`, and a failure is cloned into each one as
   `ProcMacroLoadingError::ProcMacroSrvError` or `::Disabled`.

So §3.2's value **is** reachable — from the crate graph, per crate, not from the workspace-level
return.

---

## What this means for the design

1. **The `project-load` family must read the per-crate `ProcMacroLoadResult`**, not the returned
   `Option<ProcMacroClient>`. Recording the Option would write `proc_macro_policy='disabled'` for a
   workspace whose server *failed*, which is the silent flattening §3.2 exists to prevent.
2. **`proc_macro_policy` is per crate, not per run.** The source stores a result per crate id, and
   a workspace can legitimately have some crates with proc macros loaded and others not.
   §3.2's staging table has one row per load with a single `proc_macro_policy`; that is one value
   too few, and `proc_macro_failures List<Utf8>` beside it is where the per-crate detail goes.
3. **`NoProcMacros` is a third failure kind** (line 492), distinct from both — a crate that has no
   proc macros at all, which is not a failure and must not be recorded as one.

None of this blocks the `hir` family. It constrains how `project-load` records what it saw, which
is exactly what the question was for.
