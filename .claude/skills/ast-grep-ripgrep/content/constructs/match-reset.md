# `match-reset`

`\K`

Drop everything matched so far from the result.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P007 |
| observed | confirmed |

## Evidence

Does \K drop the prefix from the reported match?

```
rg -oP said \K"hello" docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP said "hello" docs/quotes.txt
```

Exit 0.
