# Capability map

Routing is by the question you arrived with, not by tool. Find the row that matches what you are trying to do; the page names the flags, constructs and recorded behaviour for it.

## Either tool, or both

| Topic | Answers |
|---|---|
| [Choosing the tool](choosing-the-tool.md) | Which of these tools answers my question at all? |
| [Deciding which files get searched](search-space.md) | Why was this file skipped, and how do I control the file set? |
| [Using both tools together](composing-the-two.md) | How do I combine lexical and structural search? |
| [Consuming results from a program](machine-output.md) | Another program has to read these results. |
| [Why did this not match?](debugging-a-query.md) | I expected results and got none, or got the wrong ones. |
| [Patterns and repositories you do not control](untrusted-input.md) | The pattern or the corpus comes from someone else. |
| [Making a slow search fast](performance.md) | This is taking too long on a large tree. |

## Text search — ripgrep

| Topic | Answers |
|---|---|
| [Regex beyond the default engine](pcre2-advanced-regex.md) | I need lookaround, backreferences, or something the default engine rejects. |
| [Matching across line boundaries](multiline-matching.md) | My pattern needs to span more than one line. |
| [Getting a value out, not a line](output-shaping-and-extraction.md) | I want the matched value itself, or a count, or context around it. |
| [Plain text and ordinary regex](literal-and-regex-search.md) | Find this string or this ordinary pattern. |
| [Text that is not plain ASCII UTF-8](unicode-and-encodings.md) | The text has accents, another script, or another encoding. |
| [Archives, binaries, and generated input](reading-odd-inputs.md) | The content is compressed, binary, or has to be produced first. |

## Syntax search and rewriting — ast-grep

| Topic | Answers |
|---|---|
| [Matching code by its shape](structural-patterns.md) | Find this code construct, wherever and however it is formatted. |
| [Mapping code without reading it](code-navigation.md) | What is in this file or directory, without opening everything? |
| [Writing a durable rule](rule-authoring.md) | I need this check to run repeatedly, not once. |
| [Changing code at scale](codemods-and-rewrites.md) | I need to rewrite this construct across a repository. |
| [Running rules in CI](repo-linting-and-ci.md) | These rules should gate a pipeline. |
| [Languages, grammars and custom parsers](languages-and-parsers.md) | Is my language supported, and what if it is not? |
| [Using these as libraries](embedding.md) | I want this behaviour inside my own program. |
