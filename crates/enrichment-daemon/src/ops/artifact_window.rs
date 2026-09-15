//! Bounded byte-preserving Markdown section locations, independent of evidence extraction.
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};

pub(super) fn section(
    file: &mut std::fs::File,
    wanted: &str,
) -> io::Result<Option<(usize, usize, String)>> {
    file.rewind()?;
    let mut reader = BufReader::new(file);
    let mut offset = 0;
    let mut active = wanted
        .eq_ignore_ascii_case("preamble")
        .then(|| (0, "preamble".to_owned()));
    let mut fence = None;
    loop {
        let start = offset;
        let mut line = Vec::new();
        while let Some(&byte) = reader.fill_buf()?.first() {
            reader.consume(1);
            offset += 1;
            if byte == b'\r' || byte == b'\n' {
                if byte == b'\r' && reader.fill_buf()?.first() == Some(&b'\n') {
                    reader.consume(1);
                    offset += 1;
                }
                break;
            }
            if line.len() >= 1024 * 1024 {
                return Err(io::Error::other(
                    "section line exceeds one MiB scanning bound",
                ));
            }
            line.push(byte);
        }
        if start == offset {
            return Ok(active.map(|(body, heading)| (body, offset, heading)));
        }
        // Malformed body bytes remain original bytes; only valid heading syntax is decoded.
        let Ok(line) = std::str::from_utf8(&line) else {
            continue;
        };
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            let marker = trimmed.as_bytes()[0];
            match fence {
                None => fence = Some(marker),
                Some(open) if open == marker => fence = None,
                _ => {}
            }
            continue;
        }
        if fence.is_some() {
            continue;
        }
        let hashes = trimmed.bytes().take_while(|b| *b == b'#').count();
        if !(1..=6).contains(&hashes) {
            continue;
        }
        let heading = trimmed[hashes..].trim();
        if heading.is_empty() {
            continue;
        }
        if let Some((body, heading)) = active {
            return Ok(Some((body, start, heading)));
        }
        if heading.eq_ignore_ascii_case(wanted) {
            active = Some((offset, heading.to_owned()));
        }
    }
}

pub(super) fn range(file: &mut std::fs::File, start: usize, count: usize) -> io::Result<Vec<u8>> {
    file.seek(SeekFrom::Start(start as u64))?;
    let mut bytes = vec![0; count];
    file.read_exact(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    #[test]
    fn section_locations_preserve_original_crlf_and_malformed_body_bytes() {
        let mut file = tempfile::tempfile().unwrap();
        let bytes = b"preamble\r# Title\r\n\r\nbody\xff\r\n```\r# ignored\r```\r## Next\nend";
        file.write_all(bytes).unwrap();
        let (start, end, label) = section(&mut file, "title").unwrap().unwrap();
        assert_eq!(label, "Title");
        assert_eq!(&bytes[start..end], b"\r\nbody\xff\r\n```\r# ignored\r```\r");
        assert_eq!(
            range(&mut file, start, end - start).unwrap(),
            &bytes[start..end]
        );
        assert!(section(&mut file, "absent").unwrap().is_none());
        assert_eq!(
            section(&mut file, "preamble").unwrap().unwrap(),
            (0, 9, "preamble".into())
        );
    }
}
