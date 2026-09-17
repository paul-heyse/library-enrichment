# `lookahead`

`(?=...) (?!...)`

Assert what follows, without consuming.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P002 |
| observed | confirmed |

## Evidence

Does positive lookahead match through -P?

```
rg -oP he(?= said) docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP he(?= shouted) docs/quotes.txt
```

Exit 1.
