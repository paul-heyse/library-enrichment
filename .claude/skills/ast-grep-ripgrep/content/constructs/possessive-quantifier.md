# `possessive-quantifier`

`a++ a*+`

Repeat without giving back.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P008 |
| observed | confirmed |

## Evidence

Are possessive quantifiers accepted?

```
rg -coP \w++  docs/notes.md
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -coP \w++  does-not-exist.txt
```

Exit 2.
