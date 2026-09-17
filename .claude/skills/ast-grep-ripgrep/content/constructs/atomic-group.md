# `atomic-group`

`(?>...)`

Group that never backtracks into itself.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P009 |
| observed | confirmed |

## Evidence

Are atomic groups accepted?

```
rg -coP (?>\w+)  docs/notes.md
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -co (?>\w+)  docs/notes.md
```

Exit 2.
