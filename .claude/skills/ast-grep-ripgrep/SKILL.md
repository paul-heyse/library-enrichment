---
name: ast-grep-ripgrep
description: Find what ast-grep and ripgrep can actually do, at pinned releases, from a prebuilt index of every flag, node kind, rule field and regex construct, joined to executed evidence and to the library crates underneath. Use when searching or rewriting code, when choosing between a literal, a regex, PCRE2 and a structural pattern, when a search returned less than expected, when a PCRE2 construct might not be available, and when embedding the crates. Do not use for general shell or regex questions unrelated to these two tools.
---

# ast-grep and ripgrep capability repository

`rg --pcre2-version` on the machine this targets prints **PCRE2 10.45**. The library it actually
links is **10.48** — three releases newer. The string is a constant compiled into ripgrep at
*its* build time, so it describes the build and never the runtime; PCRE2's own version
conditional, evaluated inside the library, is what settles it, and this repository refuses to
build unless that assertion comes back 10.48.

That is the shape of the failure this repository exists to prevent. It is not "I could not find
the answer". It is **a confident answer from the wrong oracle** — concluding a construct is
unavailable because a version string said so, deciding a symbol is absent when its file was never
in the candidate set, or trusting that an unsupported construct would have produced an error.
Sometimes it does: lookaround under the default engine fails loudly and names `-P`. Sometimes it
does not: `(?R)` is silently parsed as an inline group, degrading a balanced-delimiter pattern to
`\([^()]*\)` and returning `(b)` where PCRE2 returns `(a(b)c)`. Exit 0, no warning, wrong answer
(P012).

`content/` is prebuilt and pinned. Nothing here queries the network or a service.

## What is pinned

ast-grep **0.45.3** and ripgrep **15.2.0**, both from the installed binaries — the build refuses
to run if they are not the pinned versions. **PCRE2 10.48 is the baseline**, asserted the same
way and documented from tag `pcre2-10.48`; it carries Unicode **17.0.0**. Because the baseline is
asserted rather than discovered, the `since pcre2` column in the indexes is history — when
upstream introduced a construct — and never a gate on whether you have it. The 23 library crates
are resolved from ripgrep's own `Cargo.lock` at its tag, so `ignore` is 0.4.29 rather than the
newer release on crates.io — that is the code the probes actually observed.

193 flags · 3,024 node kinds across 24 languages · 631 tree-sitter field names · 70 rule fields ·
44 regex constructs · 49 executed behaviour probes · 716 library symbols with 4,727 methods ·
224 file types. `content/PROVENANCE.json` is authoritative.

## Escalation ladder

Stop at the first rung that answers the question.

0. **Which question is this?** `content/topics/00-map.md` routes by what you are trying to do,
   not by tool. Skip when you already know the flag or construct you want.
1. **Does it exist, and how is it spelled?** `rg` over `content/index/*.tsv`.
   ```
   rg -i 'lookbehind|backreference' content/index/regex.tsv
   rg -P '^rust\t' content/index/kinds.tsv
   rg -P '\tFILTER OPTIONS\t' content/index/flags.tsv
   ```
2. **A direct lookup.** `content/catalogs/` — `regex-matrix.md`, `flag-families.md`,
   `exit-codes.md`, `languages.md`, `file-types.md`, and `not-reachable.md`.
3. **What does this construct constrain, and what does it leave open?**
   `content/constructs/<name>.md`, each carrying the probe and control that established it.
4. **Can I use this as a library?** `content/api/<crate>.<module>.md`. The path is a rule, not a
   lookup: take the canonical path's module part and replace `::` with `.`.
   `ignore::walk::WalkBuilder` → `content/api/ignore.walk.md`.
5. **How does it really behave?** The vendored upstream evidence — ripgrep's own integration
   suite in `content/corpus/ripgrep/tests/`, PCRE2's manual in `content/corpus/pcre2/doc/`, and
   ast-grep's guides and rule catalogue in `content/corpus/ast-grep/`.
6. **What is my own code missing?** Run the capability-gap rules against the working repository.
   ```
   ast-grep scan -c queries/sgconfig.yml --filter '^project-' PATH
   ```

Rung 6 is the one to reach for unprompted after writing a search or a rule. It is cheap, and it
catches the mechanical mistakes that survive review because they do not fail loudly: `--json`
with a space, `-uuu` in ordinary code search, `sg` instead of `ast-grep`, and the UTS#18 class
form that ripgrep cannot compile as set algebra.

## Rules that keep answers correct

**Never read `rg --pcre2-version` to decide what PCRE2 can do.** It is a build-time constant
that understates the runtime by three releases here. The baseline is asserted with PCRE2's own
`(?(VERSION=10.48)…)` conditional, which is evaluated inside libpcre2 and therefore answers for
the library that runs. Every behavioural claim is additionally backed by a probe with a control
that must fail — a probe without a control cannot tell "the feature works" from "the pattern
matched for another reason" — and `content/index/regex.tsv` says `unknown` where no probe
exists.

**An unsupported construct does not reliably announce itself.** Lookaround errors; recursion
degrades silently (P012). Before concluding the default engine handled something, run the
control.

**Check the file set before the pattern.** Most empty results are a traversal decision, not a
matching one, and the ignore stack is layers rather than a switch: `--no-ignore-vcs` does not
reveal what `.ignore` excluded (P023). A path named explicitly bypasses the stack entirely
(P025), which is why binary handling looks opposite depending on how the file was reached (P029).

**A node kind is evidence about the grammar this ast-grep bundles.** The two shipped catalogues
disagree, so disputed kinds were adjudicated by running them; three published kinds are rejected
by this binary. Read the fourth column of `kinds.tsv` before trusting a kind from documentation.

**The exit vocabularies differ, and one of them carries no information.** ripgrep's 1 and 2 are
different answers. ast-grep exits 8 for an unparseable pattern, `ast-grep test` exits 3 when
`--filter` matches nothing and 0 even while reporting failures, and `outline` exits 0 for a file
that does not exist. `content/catalogs/exit-codes.md` has the table.

**ast-grep establishes syntax; ripgrep establishes lexical occurrence.** Neither resolves
imports, types or dispatch. A call matched structurally is syntax-confirmed; that it refers to
the function you have in mind is not established by either tool. Report the difference rather
than collapsing it.

## Reporting

Cite the file you read the answer in, and the probe id when the claim is a behavioural one. When
a capability exists but you are not recommending it, say so — the point of this repository is
that the caller learns the option existed. When an index is silent, report silence rather than
absence, and say which applies: not reachable from the CLI (`unreachable.tsv`), not present at
these pins, or not probed and therefore unknown.

## Additional references

`reference.md` has the layout, the column schemas for every index, the rule inventory, runnable
query recipes and the known limits. `queries/README.md` before writing or changing a rule.
`build/README.md` before rebuilding or re-pinning.
