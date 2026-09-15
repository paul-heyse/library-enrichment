//! Canonical consumer documents and byte-accurate normalization of LSP ranges.
use enrichment_core::{
    evidence::{
        Symbol,
        execution::{Utf8Position, Utf8Range},
    },
    identity::Ecosystem,
    request::InspectionOptions,
};

pub struct Consumer {
    pub text: String,
    pub position: Option<Utf8Position>,
    pub uri: &'static str,
}

impl Consumer {
    pub fn new(
        symbol: &Symbol,
        ecosystem: Ecosystem,
        options: &InspectionOptions,
    ) -> Result<Self, String> {
        let needs_position = options.methods.is_empty()
            || options
                .methods
                .iter()
                .any(|m| *m != enrichment_core::evidence::execution::SemanticMethod::Diagnostics);
        let (text, generated) = match &options.snippet {
            Some(text) => (text.clone(), false),
            None => {
                let path = &symbol.path;
                let valid = path
                    .split(if ecosystem == Ecosystem::Rust {
                        "::"
                    } else {
                        "."
                    })
                    .all(|part| {
                        let part = part.strip_prefix("r#").unwrap_or(part);
                        let mut chars = part.chars();
                        chars.next().is_some_and(|c| c == '_' || c.is_alphabetic())
                            && chars.all(|c| c == '_' || c.is_alphanumeric())
                    });
                if !valid {
                    return Err("this qualified path needs an explicit consumer snippet; no source expression was guessed".into());
                }
                (
                    match ecosystem {
                        Ecosystem::Rust => {
                            format!("use {path} as _libenr_target;\nfn main() {{}}\n")
                        }
                        Ecosystem::Python => format!(
                            "import {}\n{path}\n",
                            path.split('.').next().ok_or("missing import root")?
                        ),
                    },
                    true,
                )
            }
        };
        let position = if !needs_position {
            None
        } else if let Some(position) = options.position {
            position.validate(&text)?;
            Some(position)
        } else {
            let matches: Vec<_> = text
                .match_indices(&symbol.name)
                .filter(|(offset, name)| {
                    let identifier = |c: char| c == '_' || c.is_alphanumeric();
                    text[..*offset]
                        .chars()
                        .next_back()
                        .is_none_or(|c| !identifier(c))
                        && text[*offset + name.len()..]
                            .chars()
                            .next()
                            .is_none_or(|c| !identifier(c))
                })
                .collect();
            let offset = if generated {
                matches.last().map(|(offset, _)| *offset)
            } else if matches.len() == 1 {
                Some(matches[0].0)
            } else { None }.ok_or("consumer anchor is ambiguous or absent; provide one exact UTF-8 position or request diagnostics only")?;
            let (line, byte) = enrichment_core::evidence::text::position(&text, offset)?;
            Some(Utf8Position { line, byte })
        };
        Ok(Self {
            text,
            position,
            uri: match ecosystem {
                Ecosystem::Rust => "file:///capsule/src/main.rs",
                Ecosystem::Python => "file:///capsule/consumer.py",
            },
        })
    }
}

pub fn protocol_position(
    document: &str,
    position: Utf8Position,
    encoding: &str,
) -> Result<(u32, u32), String> {
    position.validate(document)?;
    let line = enrichment_core::evidence::text::line(document, position.line)?;
    let prefix = &line[..position.byte as usize];
    let units = match encoding {
        "utf-8" => prefix.len(),
        "utf-16" => prefix.encode_utf16().count(),
        _ => return Err("unnegotiated position encoding".into()),
    };
    Ok((
        position.line,
        u32::try_from(units).map_err(|e| e.to_string())?,
    ))
}

pub fn byte_position(
    document: &str,
    line: u32,
    character: u32,
    encoding: &str,
) -> Result<Utf8Position, String> {
    if !matches!(encoding, "utf-8" | "utf-16") {
        return Err("unnegotiated position encoding".into());
    }
    let text = enrichment_core::evidence::text::line(document, line)?;
    let mut units = 0usize;
    for (byte, value) in text
        .char_indices()
        .map(|(b, c)| (b, Some(c)))
        .chain(std::iter::once((text.len(), None)))
    {
        if units == character as usize {
            return Ok(Utf8Position {
                line,
                byte: u32::try_from(byte).map_err(|e| e.to_string())?,
            });
        }
        units += match (encoding, value) {
            ("utf-8", Some(c)) => c.len_utf8(),
            ("utf-16", Some(c)) => c.len_utf16(),
            ("utf-8" | "utf-16", None) => 0,
            _ => return Err("unnegotiated position encoding".into()),
        };
    }
    Err("LSP position is outside the line or splits a character".into())
}

pub fn range(
    document: &str,
    range: &serde_json::Value,
    encoding: &str,
) -> Result<Utf8Range, String> {
    let coordinate = |end: &str, axis: &str| -> Result<u32, String> {
        u32::try_from(
            range[end][axis]
                .as_u64()
                .ok_or("invalid LSP range coordinate")?,
        )
        .map_err(|e| e.to_string())
    };
    let result = Utf8Range {
        start: byte_position(
            document,
            coordinate("start", "line")?,
            coordinate("start", "character")?,
            encoding,
        )?,
        end: byte_position(
            document,
            coordinate("end", "line")?,
            coordinate("end", "character")?,
            encoding,
        )?,
    };
    result.validate()?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_line_ending_preserves_retained_encoding_coordinates() {
        let text = "a\r\r\n🌎é\r\n";
        for (byte, units) in [(0, 0), (4, 2), (6, 3)] {
            let point = Utf8Position { line: 2, byte };
            point.validate(text).unwrap();
            assert_eq!(
                protocol_position(text, point, "utf-16").unwrap(),
                (2, units)
            );
            assert_eq!(byte_position(text, 2, units, "utf-16").unwrap(), point);
            assert_eq!(byte_position(text, 2, byte, "utf-8").unwrap(), point);
        }
        assert!(byte_position(text, 2, 1, "utf-16").is_err());
        assert!(byte_position(text, 0, 0, "unknown").is_err());
        assert_eq!(
            byte_position(text, 3, 0, "utf-16").unwrap(),
            Utf8Position { line: 3, byte: 0 }
        );
        assert!(Utf8Position { line: 0, byte: 2 }.validate(text).is_err());
    }
    #[test]
    fn unicode_ranges_are_exact_and_invalid_offsets_never_clamp() {
        let text = "# 🌎 é\r\nvalue\n";
        let point = Utf8Position { line: 0, byte: 7 };
        assert_eq!(protocol_position(text, point, "utf-16").unwrap(), (0, 5));
        assert_eq!(byte_position(text, 0, 5, "utf-16").unwrap(), point);
        assert!(byte_position(text, 0, 3, "utf-16").is_err());
        assert!(byte_position(text, 0, 3, "utf-8").is_err());
        assert!(byte_position(text, 0, 50, "utf-8").is_err());
        assert!(protocol_position(text, point, "unknown").is_err());
        assert_eq!(
            byte_position(text, 2, 0, "utf-16").unwrap(),
            Utf8Position { line: 2, byte: 0 }
        );
    }
}
