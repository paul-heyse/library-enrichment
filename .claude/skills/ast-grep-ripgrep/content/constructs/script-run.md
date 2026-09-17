# `script-run`

`(*sr:...)`

Require one unicode script throughout.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P017 |
| observed | confirmed |

## Evidence

Are script runs accepted?

```
rg -coP (*sr:\w+) docs/unicode.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -co (*sr:\w+) docs/unicode.txt
```

Exit 2.
