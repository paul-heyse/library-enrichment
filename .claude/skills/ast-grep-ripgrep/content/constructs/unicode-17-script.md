# `unicode-17-script`

`\p{Sidetic}`

Match one of the four scripts unicode 17.0 added.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.48 |
| established by | probe P036 |
| observed | confirmed |

## Evidence

Do the Unicode 17.0 scripts resolve, and does the engine still reject junk?

```
rg -oP \p{Sidetic} docs/unicode17.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -P \p{Not_A_Real_Script_Xyz} docs/unicode17.txt
```

Exit 2.
