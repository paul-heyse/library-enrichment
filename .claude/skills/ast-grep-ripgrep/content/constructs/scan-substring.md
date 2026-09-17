# `scan-substring`

`(*scs:(1)...)`

Re-scan an already-captured substring.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.45 |
| established by | probe P014 |
| observed | confirmed |

## Evidence

Are scan-substring assertions accepted?

```
rg -coP (\w+)(*scs:(1)\w+) docs/notes.md
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -co (\w+)(*scs:(1)\w+) docs/notes.md
```

Exit 2.
