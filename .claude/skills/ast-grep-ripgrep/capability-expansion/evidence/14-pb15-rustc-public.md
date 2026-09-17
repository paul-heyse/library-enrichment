# Evidence 14 — PB15: `rustc_public` is reachable, and both MIR surfaces read one phase

Plan v2 §12.3 question 1 asked whether the typed `rustc_public` API is a usable extraction
surface for `staging.mir`, or whether `-Zdump-mir` text plus a parser is the pragmatic path.
§3.6 left `mapping_basis` open between `mir.rustc_public@<phase>` and `mir.unpretty@<phase>`.

The probe answers the question that was asked, and then answers a larger one that was not.

| | Verdict |
|---|---|
| Is `rustc_public` reachable on the pinned toolchain? | **Yes.** `Body`, `BasicBlock`, `TerminatorKind` all present; a body walks into §3.6's column shape including `cfg_edge` |
| Does it agree with the textual surface? | **Yes**, on all three fixture functions, with the counts differing between functions |
| Which phase does either surface read? | **`runtime-optimized`, and only that one.** §3.6's `mir_phase` vocabulary has three values; neither candidate `mapping_basis` reaches the other two |

- Source: [`probes/rustc-public-probe/src/main.rs`](probes/rustc-public-probe/src/main.rs)
- Fixture: [`probes/rustc-public-probe/fixtures/cfg_shapes.rs`](probes/rustc-public-probe/fixtures/cfg_shapes.rs)
- Capture: [`probes/PB15-rustc-public.out.txt`](probes/PB15-rustc-public.out.txt)
- Toolchain: `nightly-2026-09-13`, rustc `1.100.0-nightly (809936eac 2026-09-12)`. Verified
  2026-09-16: rolling `nightly` is the **same commit**, so the dated pin costs nothing here.

---

## What had to be true first

`rustc_public` is not on crates.io. It ships as `librustc_public-*.rlib` in the `rustc-dev`
rustup component and is reached through `#![feature(rustc_private)]`.

```
$ rustup component list --toolchain nightly-2026-09-13 --installed
cargo, llvm-tools, rust-std, rustc, rustc-dev
$ ls $(rustc +nightly-2026-09-13 --print sysroot)/lib/rustlib/*/lib | grep rustc_public
librustc_public-80391cf5293935b3.rlib
librustc_public_bridge-750b9b3d8f61a549.rlib
```

An earlier session recorded that this component was installed on no toolchain here. That was
wrong: it was checked against the crates.io registry and a component list, which is the wrong
place to look for a compiler-internal crate. `rustc-dev` also ships `rustc-src`, so the crate's
**source** is on disk and its API was read rather than guessed.

Two things the probe crate does that are not obvious:

- **No `rust-toolchain.toml`.** AGENTS.md says "There is no second `rust-toolchain.toml`" and
  "Never invoke a bare `cargo +nightly`; always use the dated pin", so the toolchain is named on
  the command line: `cargo +nightly-2026-09-13 build`. A toolchain file here would be a second
  producer identity in a file nothing joins to `config/toolchains.toml`.
- **`build.rs` asks for the sysroot and puts it on the rpath.** A `rustc_private` binary links
  `librustc_driver-*.so` dynamically and nothing in the default search path finds it. The path
  is asked for, never written down, so it cannot point at the wrong toolchain after a repin.

---

## A — the typed surface, projected into §3.6's columns

The probe does not count something convenient. It walks each body into
`(body_owner_handle, block_index, stmt_index, stmt_kind, terminator_kind, edge_target, edge_kind)`
— §3.6's shape — because a surface that exposes `Body` but cannot yield `cfg_edge` would pass a
block count and still be unusable.

```
cfg_shapes::straight_line 2 blocks, 3 locals,  4 staging.mir rows
cfg_shapes::branching     5 blocks, 4 locals, 10 staging.mir rows
cfg_shapes::looping       6 blocks, 9 locals, 17 staging.mir rows
```

The rows for `looping`, which is in the fixture for one reason — the **back edge**:

```
block  stmt  stmt_kind   terminator_kind  edge_target  edge_kind
0      0     Assign
0      1     Assign
0                        Goto             1            goto
1      0     Assign
1      1     Assign
1                        SwitchInt        5            switch:0
1                        SwitchInt        2            switch:otherwise
...
4      0     Assign
4                        Goto             1            goto          <- back edge
5                        Return
back edges (edge_target < block_index): 1
```

`program.cfg_edge` is fillable, and `edge_kind` distinguishes a matched `switchInt` value from
its fallthrough. A reader that flattened the CFG into a statement list would still have reported
6 blocks while losing this, which is why the probe prints the rows rather than the count.

**`mir_local` has no name, confirmed at the type level.** `rustc_public::mir::LocalDecl` is
`{ ty, span, mutability }` — there is no `name` field to be tempted by. §3.6 already says the
column is `debug_annotation`; the API makes the alternative unrepresentable.

---

## B — the control, and why it could fail

The same three functions through `rustc -Zunpretty=mir`, parsed for block counts.

```
straight_line  2 blocks
branching      5 blocks
looping        6 blocks
```

The fixture holds **three** functions with three deliberately different shapes, because a control
comparing one count against one number can agree by accident: two broken readers both reporting
`1` agree perfectly. Three shapes compared pairwise cannot — agreement then requires the two arms
to vary together. The probe refuses to return `CONFIRMED` if fewer than two distinct counts were
observed, so the "cannot fail" case is detected rather than assumed away.

One thing the control taught, at the cost of a run: **`rustc` must be named as
`$sysroot/bin/rustc`, not bare.** A bare `rustc` goes through the rustup proxy, which resolves
against the repository's `rust-toolchain.toml`, lands on stable 1.98.1, and refuses `-Z` outright.
Naming the binary inside the sysroot the treatment arm already uses also makes the two arms the
same compiler by construction, so a difference between them cannot be a difference of commit.

---

## C — the arm that changes the design

Arms A and B agreeing on a block count does **not** establish that they saw the same body. §3.6
says so in words — "a `MirBody` without a phase is three different bodies wearing one name" — and
this arm turns that sentence into numbers by dumping every phase of `looping`:

```
built.after                14 blocks
analysis.after              8 blocks
runtime-optimized.after     6 blocks
96 phases dumped, 3 distinct block counts among the three named phases
```

One function. Three phases. **14, 8, 6.** Arms A and B both reported 6.

So the agreement is real and it is attributable: both surfaces read `runtime-optimized`.

### The consequence

`rustc_public::CrateItem::body()` goes through `rustc_public_bridge`, which calls
`tcx.instance_mir(InstanceKind::Item(item))` — `optimized_mir` for an ordinary item. There is no
phase parameter on the public API. `-Zunpretty=mir` reads the same query.

**Neither candidate `mapping_basis` can reach `built` or `analysis`.** They are reachable only
through `-Zdump-mir`, which is a third surface with a different shape: it writes one file per
pass into a directory and labels the phase in the *filename*, rather than returning anything on
stdout.

This does not invalidate §3.6's `mir_phase` column — it vindicates it, and narrows it:

- `mir_phase` stays REQUIRED. The 14/8/6 spread is the strongest argument for it yet measured.
- The vocabulary `'built' | 'analysis' | 'runtime'` stays, but only `'runtime'` is populated by
  either surface named in §3.6. A `built` or `analysis` row requires the `-Zdump-mir` surface and
  a different adapter, and should not be written until one exists.
- `mapping_basis` is decided **on capability, not preference**: `mir.rustc_public@runtime`. The
  typed API yields structured terminators and successor kinds directly; the textual one requires
  a parser for the grammar of every terminator to recover the same `edge_kind`, and both see
  exactly the same body. There is no accuracy argument for the parser, so the parser is not worth
  writing.

---

## A fourth finding, free

`rustc_public::mir::StatementKind` has **eleven** variants;
`rustc_middle::mir::StatementKind` has **twelve** (the extra is `BackwardIncompatibleDropHint`).

```
$ SRC=$(rustc +nightly-2026-09-13 --print sysroot)/lib/rustlib/rustc-src/rust/compiler
$ sed -n '/^pub enum StatementKind/,/^}/p' $SRC/rustc_public/src/mir/body.rs | grep -cE '^    [A-Z]'
11
$ sed -n '/^pub enum StatementKind/,/^}/p' $SRC/rustc_middle/src/mir/syntax.rs | grep -cE '^    [A-Z]'
12
```

§3.6 says the 105 operation-vocabulary entries across 16 enums are "generated from the pinned
rustc source, not hand-listed, so the vocabulary CHECK constraints stay true when the toolchain
moves". This says **which** source: the generator must read `rustc_public`'s enums, not
`rustc_middle`'s. Reading the internal ones would put a variant into a CHECK constraint that the
extraction surface can never produce — a constraint that cannot be violated, which is the same
family of mistake as a probe that cannot fail.

The probe's own `match` over `StatementKind` is exhaustive **without a wildcard arm** for the same
reason: a `_ =>` would turn the next toolchain's new variant into a silently mislabelled row
instead of a build error.

---

## Reproducing

```bash
cd evidence/probes/rustc-public-probe
export CARGO_TARGET_DIR="${LIBENR_HOME:-$HOME/.cache/codesearch}/target/rustc-public-probe"
cargo +nightly-2026-09-13 build
"$CARGO_TARGET_DIR/debug/rustc-public-probe"
```

Exit status is `0` only on `CONFIRMED`; an `INCONCLUSIVE` from either the control failing or the
fixture degenerating to one distinct count exits `1`.

---

## What this closes and what it does not

**Closes** §12.3 question 1, on a measurement rather than a preference.

**Does not close** anything about the `hir` family. §12.3 question 2 needs the `ra_ap_*` crates,
which are not on this machine and are not a rustup component — that question is untouched by this
probe and stays open.

**Nothing in the current build depends on this.** No `mir` family is being extracted; §3.6 is
design, and this probe is what makes its `mapping_basis` a decided design rather than an open one.
