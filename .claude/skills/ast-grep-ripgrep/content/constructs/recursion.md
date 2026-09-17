# `recursion`

`(?R)`

Match a balanced, nested construct.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P012 |
| observed | confirmed |

## Evidence

Does whole-pattern recursion match balanced delimiters, and what does the default engine do with the same pattern?

```
rg -oP \((?:[^()]|(?R))*\) docs/nested.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg --engine=default -o \((?:[^()]|(?R))*\) docs/nested.txt
```

Exit 0.
