# `extended-class-perl`

`(?[A & B])`

Set algebra over classes, and since 10.48 it composes with lookarounds.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.45 |
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
