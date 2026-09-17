# `resource-limits`

`(*LIMIT_MATCH=n) (*LIMIT_DEPTH=n) (*LIMIT_HEAP=n)`

Lower the engine's own limits from inside the pattern.

## Availability

| engine | answer |
|---|---|
| ripgrep default (Rust regex) | no |
| PCRE2 | yes |
| reachable through the rg CLI | requires -P |
| introduced in PCRE2 | not version-gated |
| established by | not probed |
| observed | unknown |

## Evidence

No probe covers this construct, so availability here is **unknown rather than asserted**. Write a probe with a control before depending on it.
