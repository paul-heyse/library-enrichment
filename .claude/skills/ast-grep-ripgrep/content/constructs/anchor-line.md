# `anchor-line`

`^ $`

Start or end of a line, always, even under -u.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | yes |
| PCRE2 | yes |
| reachable through the rg CLI | yes |
| introduced in PCRE2 | not version-gated |
| established by | probe P020 |
| observed | confirmed |

## Evidence

Under -U, does ^ still anchor to a line rather than to the whole haystack?

```
rg -U -c ^end block docs/multiline.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -U -c \Aend block docs/multiline.txt
```

Exit 1.
