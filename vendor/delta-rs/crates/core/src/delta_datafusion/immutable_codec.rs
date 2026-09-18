//! One bounded binary provider format. This is not a generic logical-plan decoder.
//! The native CBOR parser validates framing before Serde sees allocation size hints.
use super::DeltaScanNext;
use ciborium_ll::{Decoder, Header};
use datafusion::common::{DataFusionError, Result};
use std::io::{Cursor, Write};

const MAGIC: &[u8] = b"DFDELTA\x01";
const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_ITEMS: usize = 262_144;
const MAX_DEPTH: usize = 64;
const MAX_TEXT: usize = 1024 * 1024;

pub(super) fn encode(value: &DeltaScanNext, writer: impl Write) -> Result<()> {
    value.validate_immutable_codec()?;
    let mut writer = Bounded {
        writer,
        remaining: MAX_BYTES,
    };
    writer.write_all(MAGIC)?;
    ciborium::into_writer(value, writer).map_err(|error| invalid(error.to_string()))
}

pub(super) fn decode(bytes: &[u8]) -> Result<DeltaScanNext> {
    if bytes.len() > MAX_BYTES || !bytes.starts_with(MAGIC) {
        return Err(invalid("immutable provider format or byte bound"));
    }
    let body = &bytes[MAGIC.len()..];
    preflight(body)?;
    let mut input = Cursor::new(body);
    let value: DeltaScanNext =
        ciborium::de::from_reader_with_recursion_limit(&mut input, MAX_DEPTH)
            .map_err(|error| invalid(error.to_string()))?;
    if input.position() != body.len() as u64 {
        return Err(invalid("immutable provider trailing content"));
    }
    value.validate_immutable_codec()?;
    Ok(value)
}

struct Bounded<W> {
    writer: W,
    remaining: usize,
}
impl<W: Write> Write for Bounded<W> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(std::io::Error::other("immutable provider byte bound"));
        }
        let written = self.writer.write(bytes)?;
        self.remaining -= written;
        Ok(written)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}

fn invalid(message: impl Into<String>) -> DataFusionError {
    DataFusionError::Plan(message.into())
}

fn preflight(bytes: &[u8]) -> Result<()> {
    let mut decoder = Decoder::from(bytes);
    // Exactly one root item. Native headers own syntax; this stack limits structural
    // work and verifies every declared element even when a custom visitor stops early.
    let mut remaining = [0usize; MAX_DEPTH];
    remaining[0] = 1;
    let mut depth = 1;
    let mut items = 0usize;
    let mut scratch = [0u8; 4096];
    while depth != 0 {
        if remaining[depth - 1] == 0 {
            depth -= 1;
            continue;
        }
        remaining[depth - 1] -= 1;
        items += 1;
        if items > MAX_ITEMS {
            return Err(invalid("immutable provider item bound"));
        }
        let header = decoder
            .pull()
            .map_err(|error| invalid(format!("CBOR header: {error:?}")))?;
        let children = match header {
            Header::Positive(_) | Header::Negative(_) | Header::Simple(20..=22) => 0,
            Header::Float(value) if value.is_finite() => 0,
            Header::Array(Some(length)) => length,
            Header::Map(Some(length)) => length
                .checked_mul(2)
                .ok_or_else(|| invalid("CBOR map overflow"))?,
            Header::Bytes(Some(length)) => {
                if length > bytes.len().saturating_sub(decoder.offset()) {
                    return Err(invalid("CBOR bytes length"));
                }
                let mut segments = decoder.bytes(Some(length));
                while let Some(mut segment) = segments
                    .pull()
                    .map_err(|error| invalid(format!("CBOR bytes: {error:?}")))?
                {
                    while segment
                        .pull(&mut scratch)
                        .map_err(|error| invalid(format!("CBOR bytes: {error:?}")))?
                        .is_some()
                    {}
                }
                0
            }
            Header::Text(Some(length)) => {
                if length > MAX_TEXT || length > bytes.len().saturating_sub(decoder.offset()) {
                    return Err(invalid("CBOR text length"));
                }
                let mut segments = decoder.text(Some(length));
                while let Some(mut segment) = segments
                    .pull()
                    .map_err(|error| invalid(format!("CBOR text: {error:?}")))?
                {
                    while segment
                        .pull(&mut scratch)
                        .map_err(|error| invalid(format!("CBOR text: {error:?}")))?
                        .is_some()
                    {}
                }
                0
            }
            _ => return Err(invalid("unsupported CBOR framing in immutable provider")),
        };
        if children > MAX_ITEMS - items || children > bytes.len().saturating_sub(decoder.offset()) {
            return Err(invalid("immutable provider collection bound"));
        }
        if children != 0 {
            if depth == MAX_DEPTH {
                return Err(invalid("immutable provider depth bound"));
            }
            remaining[depth] = children;
            depth += 1;
        }
    }
    if decoder.offset() != bytes.len() {
        return Err(invalid("immutable provider trailing CBOR item"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_unbounded_incomplete_and_extra_containers() {
        for bytes in [
            &[0x83, 1, 2][..],
            &[0x9f, 1, 0xff],
            &[1, 2],
            &[0x9b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
        ] {
            assert!(preflight(bytes).is_err());
        }
        assert!(preflight(&[0x82, 1, 2]).is_ok());
        assert!(preflight(&[0x81; MAX_DEPTH + 1]).is_err());
    }
}
