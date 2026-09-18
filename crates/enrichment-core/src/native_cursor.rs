//! One bounded format boundary for every native continuation declaration.
//! Selection/checksum witnesses belong to the native cursor record, never this codec.
use crate::search::CursorError;
use serde::{Serialize, de::DeserializeOwned};

pub const MAX_BYTES: usize = 32_768;
#[derive(Clone, Copy)]
pub(crate) enum Kind {
    Artifact,
    Rows,
    Search,
    Comparison,
    Alternatives,
}
impl Kind {
    fn prefix(self) -> &'static str {
        match self {
            Self::Artifact => "artifact10_",
            Self::Rows => "rows10_",
            Self::Search => "search10_",
            Self::Comparison => "comparison10_",
            Self::Alternatives => "alternatives10_",
        }
    }
    pub(crate) fn encode<T: Serialize>(self, value: &T) -> Result<String, serde_json::Error> {
        let prefix = self.prefix();
        let mut bytes = Vec::new();
        serde_json::to_writer(
            crate::native_json::BoundedWriter::new(&mut bytes, (MAX_BYTES - prefix.len()) / 2),
            value,
        )?;
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut output = String::with_capacity(prefix.len() + 2 * bytes.len());
        output.push_str(prefix);
        for byte in bytes {
            output.push(HEX[usize::from(byte >> 4)] as char);
            output.push(HEX[usize::from(byte & 15)] as char);
        }
        Ok(output)
    }
    pub(crate) fn decode<T: DeserializeOwned>(self, text: &str) -> Result<T, CursorError> {
        if text.len() > MAX_BYTES {
            return Err(CursorError::Malformed);
        }
        let text = text
            .strip_prefix(self.prefix())
            .ok_or(CursorError::Malformed)?;
        if !text.len().is_multiple_of(2)
            || !text
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return Err(CursorError::Malformed);
        }
        let nibble = |b| if b <= b'9' { b - b'0' } else { b - b'a' + 10 };
        let bytes: Vec<_> = text
            .as_bytes()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|[a, b]| 16 * nibble(*a) + nibble(*b))
            .collect();
        serde_json::from_slice(&bytes).map_err(|_| CursorError::Malformed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cursor_families_bound_escaped_bytes_before_encoding_and_refuse_wrong_routes() {
        for kind in [
            Kind::Artifact,
            Kind::Rows,
            Kind::Search,
            Kind::Comparison,
            Kind::Alternatives,
        ] {
            let available = (MAX_BYTES - kind.prefix().len()) / 2;
            let exact = "x".repeat(available - 2);
            let encoded = kind.encode(&exact).unwrap();
            assert!(encoded.len() <= MAX_BYTES);
            assert_eq!(kind.decode::<String>(&encoded).unwrap(), exact);
            assert!(kind.encode(&(exact + "x")).is_err());
            for oversized in [
                "\n".repeat(available / 2),
                "λ".repeat(available / 2),
                "\0".repeat(available / 6 + 1),
            ] {
                assert!(kind.encode(&oversized).is_err());
            }
            let valid = kind.encode(&"quoted λ\n\" value").unwrap();
            assert_eq!(kind.decode::<String>(&valid).unwrap(), "quoted λ\n\" value");
            assert!(
                kind.decode::<String>(&format!("{}7A", kind.prefix()))
                    .is_err()
            );
            assert!(
                kind.decode::<String>(&format!("{}0", kind.prefix()))
                    .is_err()
            );
            assert!(
                kind.decode::<String>(&format!("{}{}", kind.prefix(), "0".repeat(MAX_BYTES)))
                    .is_err()
            );
        }
        assert!(
            Kind::Rows
                .decode::<String>(&Kind::Search.encode(&"valid").unwrap())
                .is_err()
        );
    }
}
