# `conditional`

`(?(1)a|b)`

Branch on whether a group matched.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P011 |
| observed | confirmed |

## Evidence

Are conditional patterns accepted?

```
rg -coP (")?\w+(?(1)"|\b) docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -co (")?\w+(?(1)"|\b) docs/quotes.txt
```

Exit 2.
