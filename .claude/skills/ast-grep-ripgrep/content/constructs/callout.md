# `callout`

`(?C1)`

Invoke a host callback mid-match.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | no |
| introduced in PCRE2 | not version-gated |
| established by | not probed |
| observed | unknown |

## Evidence

No probe covers this construct, so availability here is **unknown rather than asserted**. Write a probe with a control before depending on it.

## Not reachable

This construct exists in the library but cannot be reached through the ripgrep CLI. See `../index/unreachable.tsv` for the reason and the alternative.
