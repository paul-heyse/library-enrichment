# `datafusion_spark::function::string::format_string::Formatter`

Full upstream contracts; raw type trees and source locators in [structured records](datafusion_spark.function.string.format_string.Formatter.json).

<a id="op-b47571e5a12b4add99d746d8"></a>
## Formatter

`struct` · `datafusion_spark::function::string::format_string::Formatter` · datafusion-spark 55.1.0

```rust
struct Formatter<'a>
```

Source: `src/function/string/format_string.rs:203`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Compatible with `java.util.Formatter`

<a id="op-ba3931186d387a88d392a607"></a>
## arg_num

`struct_field` · `datafusion_spark::function::string::format_string::Formatter::arg_num` · datafusion-spark 55.1.0

```rust
arg_num: usize
```

Source: `src/function/string/format_string.rs:205`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-47283b995d07164ee7169ba4"></a>
## elements

`struct_field` · `datafusion_spark::function::string::format_string::Formatter::elements` · datafusion-spark 55.1.0

```rust
elements: Vec<FormatElement<'a>>
```

Source: `src/function/string/format_string.rs:204`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-4fe1ea33c9509d0ce9573ee7"></a>
## fmt

`function` · `datafusion_spark::function::string::format_string::Formatter::fmt` · datafusion-spark 55.1.0

```rust
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_spark::function::string::format_string::Formatter", "path": "Formatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [202, 10], "end": [202, 15], "filename": "src/function/string/format_string.rs"}, "trait": {"args": null, "id": "core::fmt::Debug", "path": "Debug"}, "trait_path": "core::fmt::Debug"}`

Source: `src/function/string/format_string.rs:202`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-5dfb4245e6b1b5de3eb578c3"></a>
## format

`function` · `datafusion_spark::function::string::format_string::Formatter::format` · datafusion-spark 55.1.0

```rust
fn format(&self, args: &[ScalarValue]) -> Result<String>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_spark::function::string::format_string::Formatter", "path": "Formatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 1], "end": [382, 2], "filename": "src/function/string/format_string.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/format_string.rs:361`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-f9a5f6f3b7069954a21c5221"></a>
## new

`function` · `datafusion_spark::function::string::format_string::Formatter::new` · datafusion-spark 55.1.0

```rust
fn new(elements: Vec<FormatElement<'a>>) -> Self
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_spark::function::string::format_string::Formatter", "path": "Formatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 1], "end": [382, 2], "filename": "src/function/string/format_string.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/format_string.rs:209`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

No upstream documentation on this item; consult its owner/trait contract.

<a id="op-e2b94d1e1ad0ec96496bbb05"></a>
## parse

`function` · `datafusion_spark::function::string::format_string::Formatter::parse` · datafusion-spark 55.1.0

```rust
fn parse(fmt: &'a str, arg_types: &[DataType]) -> Result<Self>
```

Implementation context: `{"for": {"resolved_path": {"args": {"angle_bracketed": {"args": [{"lifetime": "'a"}], "constraints": []}}, "id": "datafusion_spark::function::string::format_string::Formatter", "path": "Formatter"}}, "generics": {"params": [{"kind": {"lifetime": {"outlives": []}}, "name": "'a"}], "where_predicates": []}, "is_negative": false, "span": {"begin": [208, 1], "end": [382, 2], "filename": "src/function/string/format_string.rs"}, "trait": null, "trait_path": null}`

Source: `src/function/string/format_string.rs:285`. [Exact documentation build](https://docs.rs/crate/datafusion-spark/55.1.0/json).

Parses a printf-style format string into a Formatter with validation.

This method implements a comprehensive parser for Java `java.util.Formatter` syntax,
processing the format string character by character to identify and validate format
specifiers against the provided argument types.

# Arguments

* `fmt` - The format string containing literal text and format specifiers
* `arg_types` - Array of DataFusion DataTypes corresponding to the arguments

# Parsing Process

The parser operates in several phases:

1. **String Scanning**: Iterates through the format string looking for '%' characters
   that mark the beginning of format specifiers or special sequences.

2. **Special Sequence Handling**: Processes escape sequences:
   - `%%` becomes a literal '%' character
   - `%n` becomes a newline character
   - `%<` indicates reuse of the previous argument with a new format specifier

3. **Argument Index Resolution**: Determines which argument each format specifier refers to:
   - Sequential indexing: arguments are consumed in order (1, 2, 3, ...)
   - Positional indexing: explicit argument position using `%n$` syntax
   - Previous argument reuse: `%<` references the last used argument

4. **Format Specifier Parsing**: For each format specifier, extracts:
   - Flags (-, +, space, #, 0, ',', '(')
   - Width specification (minimum field width)
   - Precision specification (decimal places or maximum characters)
   - Conversion type (d, s, f, x, etc.)

5. **Type Validation**: Verifies that each format specifier's conversion type
   is compatible with the corresponding argument's DataType. For example:
   - Integer conversions (%d, %x, %o) require integer DataTypes
   - String conversions (%s, %S) accept any DataType
   - Float conversions (%f, %e, %g) require numeric DataTypes

6. **Element Construction**: Creates FormatElement instances for:
   - Verbatim text sections (copied directly to output)
   - Validated format specifiers with their parsed parameters

# Internal State Management

The parser maintains several state variables:
- `argument_index`: Tracks the current sequential argument position
- `prev`: Remembers the last used argument index for `%<` references
- `res`: Accumulates the parsed FormatElement instances
- `rem`: Points to the remaining unparsed portion of the format string

# Validation and Error Handling

The parser performs extensive validation including:
- Argument index bounds checking against the provided arg_types array
- Format specifier syntax validation
- Type compatibility verification between conversion types and DataTypes
- Detection of malformed numeric parameters and invalid flag combinations

# Returns

Returns a Formatter containing the parsed elements and the maximum argument
index encountered, enabling efficient argument validation during formatting.
