# `unicode-property`

`\p{L}`

Match by unicode property.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | yes |
| PCRE2 | yes |
| reachable through the rg CLI | yes |
| introduced in PCRE2 | not version-gated |
| established by | probe P015 |
| observed | confirmed |

## Evidence

Is the Perl-style extended character class (?[...]) accepted?

```
rg -oP (?[\p{L} & \p{ASCII}])+ docs/unicode.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?[\p{Greek} & \p{ASCII}])+ docs/unicode.txt
```

Exit 1.
