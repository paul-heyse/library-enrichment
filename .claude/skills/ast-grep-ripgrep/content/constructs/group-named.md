# `group-named`

`(?<n>...)`

Group under a name.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | yes |
| PCRE2 | yes |
| reachable through the rg CLI | yes |
| introduced in PCRE2 | not version-gated |
| established by | probe P006 |
| observed | confirmed |

## Evidence

Do named backreferences work, so paired delimiters can be required?

```
rg -oP (?<q>["'])\w+\k<q> docs/quotes.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?<q>["'])zzzz\k<q> docs/quotes.txt
```

Exit 1.
