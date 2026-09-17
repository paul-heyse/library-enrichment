# Regex construct matrix

What each engine supports, and how that was established. `observed` is the verdict of an executed probe with a control; `unknown` means no probe covers it and nothing should be inferred either way.

PCRE2 10.48 is the asserted baseline, so `since pcre2` is history rather than a gate: a construct marked reachable is reachable. **Do not read `rg --pcre2-version` to decide otherwise** -- it prints a constant from ripgrep's own build and reads 10.45 here while the linked library is 10.48.

| construct | syntax | default | pcre2 | reachable | min pcre2 | probe | observed |
|---|---|---|---|---|---|---|---|
| alternation | `a\|b` | yes | yes | yes | - | - | not-probed |
| anchor-haystack | `\A \z` | yes | yes | yes | - | P020 | confirmed |
| anchor-line | `^ $` | yes | yes | yes | - | P020 | confirmed |
| atomic-group | `(?>...)` | no | yes | requires -P | - | P009 | confirmed |
| backreference | `\1` | no | yes | requires -P | - | P005 | confirmed |
| branch-reset | `(?\|...)` | no | yes | requires -P | - | P010 | confirmed |
| bsr-control | `(*BSR_ANYCRLF)` | no | yes | requires -P | - | - | unknown |
| callout | `(?C1)` | no | yes | no | - | - | unknown |
| character-class | `[a-z]` | yes | yes | yes | - | - | not-probed |
| class-nested-rust | `[\w&&[^0-9]]` | yes | no | default-only | - | - | unknown |
| conditional | `(?(1)a\|b)` | no | yes | requires -P | - | P011 | confirmed |
| control-verb-accept | `(*ACCEPT) (*FAIL)` | no | yes | requires -P | - | - | unknown |
| control-verb-commit | `(*COMMIT) (*PRUNE) (*THEN)` | no | yes | requires -P | - | - | unknown |
| control-verb-skip-fail | `(*SKIP)(*F)` | no | yes | requires -P | - | P018 | confirmed |
| dot | `.` | yes | yes | yes | - | - | not-probed |
| extended-class-perl | `(?[A & B])` | no | yes | requires -P | 10.45 | P015 | confirmed |
| extended-class-uts18 | `[A&&[B]]` | no | yes | no | 10.45 | P016 | recorded |
| extended-class-with-lookaround | `(?[\d-[1]]).(?<!x)` | no | yes | requires -P | 10.48 | P035 | confirmed |
| group-capturing | `(...)` | yes | yes | yes | - | - | not-probed |
| group-named | `(?<n>...)` | yes | yes | yes | - | P006 | confirmed |
| group-non-capturing | `(?:...)` | yes | yes | yes | - | - | not-probed |
| inline-flags | `(?i)` | yes | yes | yes | - | - | not-probed |
| literal | `abc` | yes | yes | yes | - | - | not-probed |
| lookahead | `(?=...) (?!...)` | no | yes | requires -P | - | P002 | confirmed |
| lookbehind | `(?<=...) (?<!...)` | no | yes | requires -P | - | P003 | confirmed |
| match-reset | `\K` | no | yes | requires -P | - | P007 | confirmed |
| named-backreference | `\k<n>` | no | yes | requires -P | - | P006 | confirmed |
| negated-character-class | `[^a-z]` | yes | yes | yes | - | - | not-probed |
| newline-convention | `(*CRLF) (*ANYCRLF)` | no | yes | requires -P | - | - | unknown |
| no-jit | `(*NO_JIT)` | no | yes | requires -P | - | - | unknown |
| possessive-quantifier | `a++ a*+` | no | yes | requires -P | - | P008 | confirmed |
| quantifier-greedy | `a* a+ a?` | yes | yes | yes | - | - | not-probed |
| quantifier-lazy | `a*? a+?` | yes | yes | yes | - | - | not-probed |
| recursion | `(?R)` | no | yes | requires -P | - | P012 | confirmed |
| recursion-returned-captures | `(?&name(<cap>))` | no | yes | requires -P | 10.47 | P013 | confirmed |
| repetition-bounded | `a{2,5}` | yes | yes | yes | - | - | not-probed |
| resource-limits | `(*LIMIT_MATCH=n) (*LIMIT_DEPTH=n) (*LIMIT_HEAP=n)` | no | yes | requires -P | - | - | unknown |
| scan-substring | `(*scs:(1)...)` | no | yes | requires -P | 10.45 | P014 | confirmed |
| script-run | `(*sr:...)` | no | yes | requires -P | - | P017 | confirmed |
| subroutine | `(?&name)` | no | yes | requires -P | - | - | unknown |
| unicode-17-script | `\p{Sidetic}` | no | yes | requires -P | 10.48 | P036 | confirmed |
| unicode-property | `\p{L}` | yes | yes | yes | - | P015 | confirmed |
| variable-lookbehind | `(?<=.{0,20}x)` | no | yes | requires -P | 10.43 | P004 | confirmed |
| word-boundary | `\b` | yes | yes | yes | - | - | not-probed |
