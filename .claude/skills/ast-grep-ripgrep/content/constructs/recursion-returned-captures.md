# `recursion-returned-captures`

`(?&name(<cap>))`

Return selected captures from a subroutine call to the caller.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | 10.47 |
| established by | probe P013 |
| observed | confirmed |

## Evidence

Do subroutine calls return selected capture groups to the caller?

```
rg -oP (?(DEFINE)(?<day>(?|(?<s>Sat)urday|(?<s>Sun)day)))(?&day(<s>)),\k<s> docs/weekend.txt
```

Exit 0, verdict `confirmed`.

Control (must come out the other way):

```
rg -oP (?(DEFINE)(?<day>(?|(?<s>Sat)urday|(?<s>Sun)day)))(?&day),\k<s> docs/weekend.txt
```

Exit 1.
