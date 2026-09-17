# Languages and node kinds

ast-grep does not enumerate its languages -- `--help` links to a web page. So this table was built by **probing the installed binary** with candidate names.

32 accepted, 23 rejected by this binary.

| language | status | kinds | fields | rule schema |
|---|---|---|---|---|
| bash | accepted | 59 | 18 | yes |
| c | accepted | 125 | 38 | yes |
| cpp | accepted | 206 | 49 | yes |
| cs | accepted | 0 | 0 | no |
| csharp | accepted | 215 | 25 | yes |
| css | accepted | 64 | 0 | yes |
| dart | accepted | 221 | 0 | yes |
| elixir | accepted | 45 | 8 | yes |
| go | accepted | 107 | 34 | yes |
| haskell | accepted | 174 | 83 | yes |
| html | accepted | 19 | 0 | yes |
| java | accepted | 142 | 39 | yes |
| javascript | accepted | 114 | 35 | yes |
| js | accepted | 0 | 0 | no |
| json | accepted | 12 | 1 | yes |
| kotlin | accepted | 142 | 2 | yes |
| kt | accepted | 0 | 0 | no |
| lua | accepted | 47 | 20 | yes |
| markdown | accepted | 0 | 0 | no |
| php | accepted | 155 | 30 | yes |
| py | accepted | 0 | 0 | no |
| python | accepted | 123 | 31 | yes |
| rb | accepted | 0 | 0 | no |
| ruby | accepted | 134 | 31 | yes |
| rust | accepted | 163 | 30 | yes |
| scala | accepted | 171 | 31 | yes |
| solidity | accepted | 0 | 0 | no |
| swift | accepted | 186 | 44 | yes |
| ts | accepted | 0 | 0 | no |
| tsx | accepted | 184 | 42 | yes |
| typescript | accepted | 177 | 39 | yes |
| yaml | accepted | 36 | 1 | yes |

## Rejected by this binary

These candidate names were probed and refused. That is evidence about this build, not proof that no such parser exists anywhere.

`angular`, `cjs`, `clojure`, `cmake`, `cts`, `dockerfile`, `erlang`, `julia`, `make`, `mjs`, `mts`, `nim`, `ocaml`, `perl`, `protobuf`, `r`, `sh`, `sql`, `svelte`, `toml`, `vue`, `xml`, `zig`

## The two kind catalogues disagree

ast-grep publishes node kinds twice, in `schemas/languages.json` and in each `schemas/<lang>_rule.json`, and the two are not consistent. Disputed kinds were adjudicated by running them: `ast-grep run -k <kind>` exits 8 for a kind it cannot parse. Three published kinds turned out to be rejected -- see the `rejected` rows of `../index/kinds.tsv`. A rule using one of them fails rather than matching nothing.
