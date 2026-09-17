# `extended-class-uts18`

`[A&&[B]]`

Set algebra, uts#18 form.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | no |
| introduced in PCRE2 | 10.45 |
| established by | probe P016 |
| observed | recorded |

## Evidence

Is the UTS#18 [A&&[B]] form usable through stock rg?

```
rg -oP [\p{L}&&[\p{Greek}]]+ docs/unicode.txt
```

Exit 1, verdict `recorded`.

## Not reachable

This construct exists in the library but cannot be reached through the ripgrep CLI. See `../index/unreachable.tsv` for the reason and the alternative.
