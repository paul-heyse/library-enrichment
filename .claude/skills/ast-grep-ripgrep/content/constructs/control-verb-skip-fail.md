# `control-verb-skip-fail`

`(*SKIP)(*F)`

Discard a region so later alternatives cannot match inside it.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P018 |
| observed | confirmed |

## Evidence

Does the (*SKIP)(*F) exclusion idiom work?

```
rg -coP "[^"]*"(*SKIP)(*F)|said docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -coP said docs/quotes.txt
```

Exit 0.
