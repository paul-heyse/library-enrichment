# `extended-class-with-lookaround`

`(?[\d-[1]]).(?<!x)`

An extended class in the same pattern as a lookaround.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.48 |
| established by | probe P035 |
| observed | confirmed |

## Evidence

Can an extended class appear in the same pattern as a lookaround?

```
rg -oP (?[\d-[1]]).(?<!x) docs/eclass.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?[\d && ]).(?<!x) docs/eclass.txt
```

Exit 2.
