# Reference: layout, schemas, recipes and limits

Everything below is generated from pinned inputs by `build/build.py` and checked by
`build/verify.py`. Counts are from the current build; `content/PROVENANCE.json` is authoritative.

## What is pinned

| Subject | Pin | How |
|---|---|---|
| ast-grep | 0.45.3 | the installed binary; the build refuses to run against any other |
| ripgrep | 15.2.0 | the installed binary, same check |
| PCRE2 documentation | tag `pcre2-10.48` | vendored from `PCRE2Project/pcre2` |
| PCRE2 baseline | **10.48**, asserted | `(?(VERSION=10.48)…)`, evaluated inside libpcre2 |
| PCRE2 linked library | read from the `.so` itself | `ldd` + its embedded version string |
| Unicode | **17.0.0** | `pcre2test -C` beside the linked library |
| ripgrep library crates | `Cargo.lock` at tag 15.2.0 | `ignore` 0.4.29, not crates.io latest |
| ast-grep library crates | 0.45.3 | docs.rs rustdoc JSON |
| ast-grep documentation | `ast-grep/ast-grep.github.io` | guides, reference, cheatsheet, catalogue |

## Layout

```
content/
  PROVENANCE.json        pins, counts, observations, per-file sha256
  index/*.tsv            line-oriented projections              -> ripgrep
  model/<module>.json    library structure and signatures       -> ast-grep
  api/<module>.md        the full rustdoc prose, stored once    -> Read
  topics/<slug>.md       20 use-case pages + 00-map.md          -> Read
  constructs/<name>.md   42 regex constructs, one page each     -> Read
  catalogs/*.md          6 direct-lookup tables                 -> Read / ripgrep
  corpus/                verbatim upstream: rg tests and guides, PCRE2 manual,
                         ast-grep guides and rule catalogue, the generated rg man page
queries/
  sgconfig.yml           never auto-discovered; always passed with -c
  rules/{model,corpus,project}/   rule-tests/ + __snapshots__/   utils/
build/                   the generator; standard library plus the two pinned binaries
```

## Index schemas

Tab-separated, sorted, one record per line, **no header row**. Tabs and newlines are scrubbed
from free text.

| File | Rows | Columns |
|---|---:|---|
| `flags.tsv` | 193 | `tool · command · long · short · category · arg · values · summary` |
| `kinds.tsv` | 3,024 | `language · kind · source · verdict` |
| `fields.tsv` | 631 | `language · field` |
| `rule-fields.tsv` | 70 | `scope · field · type · required · summary` |
| `regex.tsv` | 44 | `construct · syntax · rust_regex · pcre2 · reachable · since_pcre2 · probe · observed · purpose` |
| `behaviors.tsv` | 49 | `probe · tool · topic · verdict · exit · question · command · stdout · control · control_exit` |
| `languages.tsv` | 55 | `language · status · kinds · fields · rule_schema` |
| `exit-codes.tsv` | 9 | `tool · command · code · meaning · trap` |
| `unreachable.tsv` | 10 | `capability · exists_in · cli_reachable · why · instead` |
| `file-types.tsv` | 224 | `type · globs` |
| `symbols.tsv` | 716 | `canonical_path · kind · crate · family · api_page · aliases · methods · summary` |
| `methods.tsv` | 4,727 | `owner_path · method · via_trait · signature · summary` |
| `aliases.tsv` | 273 | `access_path · canonical_path · kind` |
| `impls.tsv` | 2,241 | `trait_path · implementor_path · crate` |
| `bindings.tsv` | 67 | `binding · language · symbol_type · name · role · file` |
| `unresolved.tsv` | 5 | `access_path` — re-exports leaving the indexed set |

`kinds.tsv`'s last two columns are the adjudication. `source` is `both`, `rule-schema` or
`languages-schema`; `verdict` is `not-disputed` where the two shipped catalogues agree, and
otherwise `accepted` or `rejected` **as decided by running the kind against the binary**.

`regex.tsv`'s `since_pcre2` is the release that introduced the construct upstream. It is
history, not a gate: PCRE2 10.48 is asserted at build time, so a row marked reachable is
reachable. The column is kept only for the different question of whether a pattern will also
work somewhere you do not control.

`regex.tsv`'s `observed` is `confirmed` when a probe and its control both came out right,
`recorded` for a shape probe with nothing to falsify, `unknown` where no probe exists, and
`not-probed` for constructs both engines obviously share.

## Query recipes

All paths are relative to the skill directory.

```
rg -i 'lookbehind|recursion' content/index/regex.tsv
rg -P '^rust\t' content/index/kinds.tsv
rg -P '\trejected$' content/index/kinds.tsv
rg -P '^python\t' content/index/fields.tsv
rg -P '\tFILTER OPTIONS\t' content/index/flags.tsv
rg -P '^ignore::walk::WalkBuilder\t' content/index/methods.tsv
rg 'pcre2' content/index/unreachable.tsv
ast-grep scan -c queries/sgconfig.yml --filter '^corpus-' content/corpus/ripgrep/tests
ast-grep scan -c queries/sgconfig.yml --filter '^model-' content/model
```

To find the upstream tests that pin a flag's behaviour, search the vendored suite by flag name
and read the assertions around it — they are more specific than the manual.

## Rule inventory

| Rule | Runs against | A hit means |
|---|---|---|
| `project-ast-grep-json-space` | your repository | `--json` written with a space; the style became a path |
| `project-sg-deprecated-executable` | your repository | `sg` invoked; deprecated and collides with setgroups |
| `project-rg-unrestricted-in-code-search` | your repository | `-uuu` where one named layer would do |
| `project-uts18-class-under-rg` | your repository | a class form ripgrep cannot compile as set algebra |
| `model-deprecated` | `content/model` | a library item deprecated at these pins |
| `corpus-rg-test-assertion` | `content/corpus` | an upstream `rgtest!` assertion — executable evidence |

All carry `severity: hint` and no `fix`. The four `project-` rules have valid/invalid fixtures
and recorded snapshots; the `model-` and `corpus-` rules query generated content instead.

## Known limits

1. **The PCRE2 build string understates the runtime by three releases here.**
   `rg --pcre2-version` reads 10.45 while the linked library is 10.48. Four readings sit side by
   side in `PROVENANCE.json` — the asserted baseline, the version read out of the linked `.so`,
   the Unicode version, and ripgrep's own stale string — and none is reconciled away, because
   the disagreement is the thing worth knowing. The `ldd`-based resolution is Linux-only and
   degrades to `resolution: "unresolved"` rather than guessing; the assertion still holds on any
   platform.

2. **`regex.tsv` covers the constructs an agent asks about, not every construct PCRE2 has.**
   PCRE2's full syntax is in `content/corpus/pcre2/doc/pcre2syntax.3`. A construct absent from
   the index is unindexed, not unsupported.

3. **`unknown` is a real value.** Rows without a probe say so. Nothing should be inferred from
   them in either direction.

4. **Node kinds describe the grammars this ast-grep bundles.** They are not a statement about
   tree-sitter generally, and they change when ast-grep updates a grammar. Three kinds published
   in `languages.json` are rejected by this binary.

5. **The language roster is what this binary accepted when probed.** A rejected candidate is
   evidence of rejection by this build, not proof that no such parser exists.

6. **Flags come from the installed binaries' own help.** A ripgrep built without PCRE2 has a
   different surface, and `rg --version` would say so in its `features:` line.

7. **The binding index is what the declarations declare.** `.d.ts` and `.pyi` files state an API;
   they do not establish that a name resolves at runtime. Members are indexed only where the
   outline extractor produces them, which for these declaration files is rarely.

8. **`unresolved.tsv` is a boundary, not a gap.** Five access paths re-export out of the indexed
   crate set. Use the owning crate's own documentation for those.

## Rebuilding

```
python3 build/acquire.py        # network; writes build/acquired and build/.cache
python3 build/build.py          # offline; regenerates content/ from acquired/
python3 build/verify.py         # all seven checks; exit 1 on any failure
```

`acquire.py` is never imported by `build.py`. That separation is what makes the determinism check
meaningful: if acquisition could run inside the build, a rebuild could pick up different bytes
from the network and still look reproducible.

`verify.py --skip-rebuild` checks the recorded digests without regenerating, which is the fast
path while editing. The full run rebuilds and compares byte-for-byte, re-runs all 49 behaviour
probes with their controls, and executes every command printed in a fenced block in `SKILL.md`,
this file, and the topic pages.
