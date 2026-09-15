//! Sphinx v2 navigation inventory. A document index never establishes API completeness.
use flate2::read::ZlibDecoder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::io::Read;
use url::Url;

/// One inventory entry, including non-Python domains and exact inventory provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Entry {
    /// Object or label, including spaces.
    pub name: String,
    /// Domain and role, e.g. py:class or std:label.
    pub role: String,
    /// Upstream priority, not a confidence value.
    pub priority: i32,
    /// Resolved document URI; core fetch policy must still approve it.
    pub uri: String,
    /// Display label.
    pub display: String,
}

/// Bounded decoded inventory.
pub struct Inventory {
    /// Declared project name.
    pub project: String,
    /// Declared inventory version; may be absent or unrelated to selected release.
    pub version: String,
    /// Navigation entries.
    pub entries: Vec<Entry>,
    /// Malformed rows retained as explicit gaps rather than guessed.
    pub rejected: usize,
}

/// Read a bounded Sphinx v2 inventory and resolve links without fetching them.
///
/// # Errors
/// Refuses unsupported headers, compression errors, oversized output and invalid UTF8.
pub fn parse(
    bytes: &[u8],
    base: &Url,
    max_bytes: u64,
    max_entries: usize,
) -> Result<Inventory, String> {
    let mut offset = 0usize;
    let mut headers = Vec::new();
    for _ in 0..4 {
        let remaining = bytes.get(offset..).ok_or("short inventory")?;
        let length = remaining
            .iter()
            .position(|b| *b == b'\n')
            .ok_or("short inventory header")?;
        headers.push(
            std::str::from_utf8(&remaining[..length])
                .map_err(|e| e.to_string())?
                .trim_end_matches('\r')
                .to_owned(),
        );
        offset += length + 1;
    }
    if headers[0] != "# Sphinx inventory version 2" || !headers[3].contains("zlib") {
        return Err("unsupported documentation inventory format".into());
    }
    let project = headers[1]
        .strip_prefix("# Project: ")
        .ok_or("missing inventory project")?
        .into();
    let version = headers[2]
        .strip_prefix("# Version: ")
        .ok_or("missing inventory version")?
        .into();
    let mut body = Vec::new();
    ZlibDecoder::new(&bytes[offset..])
        .take(max_bytes.saturating_add(1))
        .read_to_end(&mut body)
        .map_err(|e| e.to_string())?;
    if body.len() as u64 > max_bytes {
        return Err("inventory decompressed byte bound exceeded".into());
    }
    let text = std::str::from_utf8(&body).map_err(|e| e.to_string())?;
    let mut entries = Vec::new();
    let mut rejected = 0;
    for (i, line) in text.lines().enumerate() {
        if i >= max_entries {
            return Err("inventory entry bound exceeded".into());
        }
        let parts: Vec<_> = line.split_whitespace().collect();
        let markers: Vec<_> = (1..parts.len().saturating_sub(3))
            .filter(|&j| parts[j].contains(':') && parts[j + 1].parse::<i32>().is_ok())
            .collect();
        if markers.len() != 1 {
            rejected += 1;
            continue;
        }
        let j = markers[0];
        let name = parts[..j].join(" ");
        let role = parts[j].into();
        let priority = parts[j + 1]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let location = parts[j + 2].strip_suffix('$').map_or_else(
            || parts[j + 2].to_owned(),
            |prefix| format!("{prefix}{name}"),
        );
        let Ok(uri) = base.join(&location) else {
            rejected += 1;
            continue;
        };
        let display = parts[j + 3..].join(" ");
        let display = if display == "-" {
            name.clone()
        } else {
            display
        };
        entries.push(Entry {
            name,
            role,
            priority,
            uri: uri.into(),
            display,
        });
    }
    Ok(Inventory {
        project,
        version,
        entries,
        rejected,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    #[test]
    fn inventory_preserves_names_labels_and_trailing_substitution() {
        let mut compressed =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        compressed.write_all(b"obj py:class 1 api.html#$ -\na label std:label -1 narrative.html Friendly label\nmalformed\n").expect("compress");
        let mut bytes=b"# Sphinx inventory version 2\n# Project: fixture\n# Version: unknown\n# The remainder is compressed using zlib.\n".to_vec();
        bytes.extend(compressed.finish().expect("zlib"));
        let got = parse(
            &bytes,
            &Url::parse("https://example.org/docs/").expect("url"),
            4096,
            10,
        )
        .expect("parse");
        assert_eq!(got.entries.len(), 2);
        assert_eq!(got.entries[0].uri, "https://example.org/docs/api.html#obj");
        assert_eq!(got.entries[1].name, "a label");
        assert_eq!(got.entries[1].display, "Friendly label");
        assert_eq!(got.rejected, 1);
        assert!(
            parse(
                &bytes,
                &Url::parse("https://example.org/").expect("url"),
                8,
                10
            )
            .is_err()
        );
    }
}
