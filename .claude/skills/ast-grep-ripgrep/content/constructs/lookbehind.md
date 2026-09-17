# `lookbehind`

`(?<=...) (?<!...)`

Assert what precedes.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P003 |
| observed | confirmed |

## Evidence

Does positive lookbehind match through -P?

```
rg -oP (?<=said )"hello" docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?<=shouted )"hello" docs/quotes.txt
```

Exit 1.
