# `branch-reset`

`(?|...)`

Reuse capture numbers across alternatives.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P010 |
| observed | confirmed |

## Evidence

Is branch reset accepted, reusing capture numbers across alternatives?

```
rg -oP (?|(?<d>Sat)urday|(?<d>Sun)day) docs/weekend.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?|(?<d>Tue)sday|(?<d>Wed)nesday) docs/weekend.txt
```

Exit 1.
