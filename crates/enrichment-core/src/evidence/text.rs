//! Original-byte line coordinates shared by retained evidence and protocol adapters.

/// One logical line, excluding its LF, CRLF or CR delimiter from the content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line {
    pub start: usize,
    pub content_end: usize,
    pub end: usize,
}

fn delimiter_bytes(first: u8, next: Option<u8>) -> usize {
    if first == b'\r' && next == Some(b'\n') {
        2
    } else {
        1
    }
}

/// Bounded streaming counterpart of [`lines`]. Each item includes its original delimiter;
/// EOF preserves the same final empty line. Chunk boundaries do not affect CRLF recognition.
pub struct LineReader<R> {
    source: R,
    finished: bool,
    limit: usize,
}
impl<R: std::io::BufRead> LineReader<R> {
    pub fn new(source: R, line_bytes: usize) -> Self {
        Self {
            source,
            finished: false,
            limit: line_bytes,
        }
    }

    /// # Errors
    /// I/O and per-line byte excess are explicit, before the output buffer grows past its cap.
    pub fn next_line(&mut self) -> std::io::Result<Option<Vec<u8>>> {
        if self.finished {
            return Ok(None);
        }
        let mut bytes = Vec::new();
        loop {
            let chunk = self.source.fill_buf()?;
            if chunk.is_empty() {
                self.finished = true;
                return Ok(Some(bytes));
            }
            let delimiter = chunk.iter().position(|b| matches!(b, b'\r' | b'\n'));
            let take = delimiter.map_or(chunk.len(), |index| index + 1);
            if take > self.limit.saturating_sub(bytes.len()) {
                return Err(std::io::Error::other("source line exceeds byte bound"));
            }
            bytes.extend_from_slice(&chunk[..take]);
            self.source.consume(take);
            if delimiter.is_some() {
                let first = *bytes.last().expect("delimiter was copied");
                let next = self.source.fill_buf()?.first().copied();
                if delimiter_bytes(first, next) == 2 {
                    if bytes.len() == self.limit {
                        return Err(std::io::Error::other("source line exceeds byte bound"));
                    }
                    bytes.push(b'\n');
                    self.source.consume(1);
                }
                return Ok(Some(bytes));
            }
        }
    }
}

/// Scan without allocating a line index, preserving the final empty line.
pub fn lines(text: &str) -> impl Iterator<Item = Line> + '_ {
    let mut next = Some(0usize);
    std::iter::from_fn(move || {
        let start = next?;
        let bytes = text.as_bytes();
        let mut end = start;
        while end < bytes.len() && !matches!(bytes[end], b'\r' | b'\n') {
            end += 1;
        }
        let content_end = end;
        if end == bytes.len() {
            next = None;
        } else {
            end += delimiter_bytes(bytes[end], bytes.get(end + 1).copied());
            next = Some(end);
        }
        Some(Line {
            start,
            content_end,
            end,
        })
    })
}

/// The exact content of a zero-based logical line.
pub fn line(text: &str, number: u32) -> Result<&str, String> {
    let line = lines(text)
        .nth(number as usize)
        .ok_or("position outside document")?;
    Ok(&text[line.start..line.content_end])
}

/// Convert a scalar boundary in original bytes to a line/content-byte position.
pub fn position(text: &str, offset: usize) -> Result<(u32, u32), String> {
    if !text.is_char_boundary(offset) {
        return Err("position is not a UTF-8 character boundary".into());
    }
    for (number, line) in lines(text).enumerate() {
        if (line.start..=line.content_end).contains(&offset) {
            return Ok((
                u32::try_from(number).map_err(|e| e.to_string())?,
                u32::try_from(offset - line.start).map_err(|e| e.to_string())?,
            ));
        }
    }
    Err("position is inside a line delimiter or outside document".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_bytes_define_lines_and_only_content_positions() {
        let text = "a\r\r\n🌎é\r\n";
        let spans = lines(text).collect::<Vec<_>>();
        assert_eq!(
            spans.iter().map(|l| l.start).collect::<Vec<_>>(),
            [0, 2, 4, 12]
        );
        assert_eq!(
            spans
                .iter()
                .map(|l| &text[l.start..l.content_end])
                .collect::<Vec<_>>(),
            ["a", "", "🌎é", ""]
        );
        for (offset, expected) in [
            (0, (0, 0)),
            (1, (0, 1)),
            (2, (1, 0)),
            (4, (2, 0)),
            (8, (2, 4)),
            (10, (2, 6)),
            (12, (3, 0)),
        ] {
            assert_eq!(position(text, offset).unwrap(), expected);
        }
        for offset in [3, 5, 6, 7, 9, 11, 13] {
            assert!(position(text, offset).is_err(), "offset {offset}");
        }
        assert_eq!(line("", 0).unwrap(), "");
        assert!(line("", 1).is_err());
        assert_eq!(line("x\n", 1).unwrap(), "");
        assert_eq!(line("x\r", 1).unwrap(), "");
        assert_eq!(line("x\u{2028}y", 0).unwrap(), "x\u{2028}y");
    }

    #[test]
    fn streaming_lines_match_original_byte_spans_at_every_chunk_boundary() {
        for text in [
            "",
            "a",
            "\r",
            "\r\n",
            "\n\n",
            "a\r\r\n🌎é\r\n",
            "x\u{2028}y",
        ] {
            let expected = lines(text)
                .map(|line| text.as_bytes()[line.start..line.end].to_vec())
                .collect::<Vec<_>>();
            for chunk in 1..=7 {
                let source = std::io::BufReader::with_capacity(chunk, text.as_bytes());
                let mut reader = LineReader::new(source, 64);
                let mut actual = Vec::new();
                while let Some(line) = reader.next_line().expect("bounded line") {
                    actual.push(line);
                }
                assert_eq!(actual, expected, "{text:?}, chunk {chunk}");
            }
        }
        assert!(LineReader::new(&b"abcd\r\n"[..], 5).next_line().is_err());
    }
}
