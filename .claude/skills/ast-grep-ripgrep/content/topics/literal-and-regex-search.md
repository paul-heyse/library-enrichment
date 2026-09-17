# Plain text and ordinary regex

**Find this string or this ordinary pattern.**

Most searches are literal, and saying so makes them faster and more correct. `-F` treats the pattern as text, which matters most when searching for code punctuation: `println!(` is an unclosed group as a regex and an ordinary substring as a literal (P032).

Case handling has three modes and the middle one is usually right: `-s` exact, `-i` insensitive, `-S` smart-case, which stays sensitive until you type an uppercase letter. Smart-case is the good default for interactive use and a poor one for scripts, where the behaviour then depends on the pattern's spelling.

`-w` and `-x` wrap the pattern rather than requiring you to write boundaries by hand, which avoids the usual mistake of anchoring one end and forgetting the other.

## Flags

Every flag upstream files under INPUT OPTIONS, SEARCH OPTIONS. The grouping is ripgrep's own, taken from `rg --help`.

| tool | long | short | what it does |
|---|---|---|---|
| rg | --auto-hybrid-regex |  | DEPRECATED. |
| rg | --case-sensitive | -s | Execute the search case sensitively. |
| rg | --crlf |  | When enabled, ripgrep will treat CRLF (\r\n) as a line terminator |
| rg | --dfa-size-limit |  | The upper size limit of the regex DFA. |
| rg | --encoding | -E | Specify the text encoding that ripgrep will use on all files searched. |
| rg | --engine |  | Specify which regular expression engine to use. |
| rg | --file | -f | Search for patterns from the given file, with one pattern per line. |
| rg | --fixed-strings | -F | Treat all patterns as literals instead of as regular expressions. |
| rg | --ignore-case | -i | When this flag is provided, all patterns will be searched case |
| rg | --invert-match | -v | This flag inverts matching. |
| rg | --line-regexp | -x | When enabled, ripgrep will only show matches surrounded by line |
| rg | --max-count | -m | Limit the number of matching lines per file searched to NUM. |
| rg | --mmap |  | When enabled, ripgrep will search using memory maps when possible. |
| rg | --multiline | -U | This flag enables searching across multiple lines. |
| rg | --multiline-dotall |  | This flag enables "dot all" mode in all regex patterns. |
| rg | --no-pcre2-unicode |  | DEPRECATED. |
| rg | --no-unicode |  | This flag disables Unicode mode for all patterns given to ripgrep. |
| rg | --null-data |  | Enabling this flag causes ripgrep to use NUL as a line terminator |
| rg | --pcre2 | -P | When this flag is present, ripgrep will use the PCRE2 regex engine |
| rg | --pre |  | For each input PATH, this flag causes ripgrep to search the standard |
| rg | --pre-glob |  | This flag works in conjunction with the --pre flag. |
| rg | --regex-size-limit |  | The size limit of the compiled regex, where the compiled regex |
| rg | --regexp | -e | A pattern to search for. |
| rg | --search-zip | -z | This flag instructs ripgrep to search in compressed files. |
| rg | --smart-case | -S | This flag instructs ripgrep to searches case insensitively if the |
| rg | --stop-on-nonmatch |  | Enabling this option will cause ripgrep to stop reading a file once it |
| rg | --text | -a | This flag instructs ripgrep to search binary files as if they were |
| rg | --threads | -j | This flag sets the approximate number of threads to use. |
| rg | --word-regexp | -w | When enabled, ripgrep will only show matches surrounded by word |

## Constructs

`default` is ripgrep's Rust engine; `reachable` says whether the ripgrep CLI can actually get at it, which is narrower than whether PCRE2 supports it. `observed` is the verdict of an executed probe, not a reading of a version string.

| construct | syntax | default | reachable | since pcre2 | observed |
|---|---|---|---|---|---|
| literal | `abc` | yes | yes | - | not-probed |
| character-class | `[a-z]` | yes | yes | - | not-probed |
| alternation | `a\|b` | yes | yes | - | not-probed |
| word-boundary | `\b` | yes | yes | - | not-probed |
| quantifier-greedy | `a* a+ a?` | yes | yes | - | not-probed |
| repetition-bounded | `a{2,5}` | yes | yes | - | not-probed |

## Evidence

Each row was executed against the fixture tree at build time. `confirmed` means the probe came out as expected **and** its control came out the other way.

| probe | verdict | question | command |
|---|---|---|---|
| P032 | confirmed | Does -F disable metacharacters? | `rg -F -c println!( src/main.rs` |
| P033 | confirmed | Does -F still mean literal when combined with -P? | `rg -P -F -c (?<=said ) docs/quotes.txt` |

## Decision rules

- `-F` whenever the pattern is text, especially when it contains punctuation.
- `-S` for a human at a terminal; an explicit `-s` or `-i` in a script.
- `-w` rather than hand-written `\b` on both ends.
- Several patterns: repeat `-e`, or `-f` a file. They are OR-ed into one search.
- `-F` still means literal under `-P` (P033); the pattern is escaped before PCRE2 compiles it.

## Anti-patterns

- Escaping punctuation by hand instead of using `-F`.
- Smart-case in a script, where behaviour then depends on the pattern's spelling.
- One enormous alternation where a type filter was the real requirement.

## Checklist

- Is this literal? Then `-F`.
- Is the case mode explicit?
- Would `-w` express the boundary more simply?
