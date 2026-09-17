# `variable-lookbehind`

`(?<=.{0,20}x)`

Lookbehind of varying width, bounded.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.43 |
| established by | probe P004 |
| observed | confirmed |

## Evidence

Is bounded variable-length lookbehind accepted?

```
rg -coP (?<=.{0,20}said )\S+ docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -coP (?<=.*said )\S+ docs/quotes.txt
```

Exit 2.
