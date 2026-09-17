# `backreference`

`\1`

Require a repeat of an earlier capture.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | probe P005 |
| observed | confirmed |

## Evidence

Do numeric backreferences work through -P?

```
rg -oP (\w+) \1 docs/notes.md
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -o (\w+) \1 docs/notes.md
```

Exit 2.
