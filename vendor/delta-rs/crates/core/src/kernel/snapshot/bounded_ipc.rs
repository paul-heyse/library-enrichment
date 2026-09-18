//! Preflight for the uncompressed native materialized-file inventory IPC format.
//! Validate lengths before Arrow's reader allocates its footer or record blocks.
use std::io::{self, Write};
pub(super) const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_FIELDS: usize = 4096;
const MAX_NODES: usize = 8 * 1024 * 1024;

#[derive(Default)]
pub(super) struct Output {
    pub(super) bytes: Vec<u8>,
}
impl Write for Output {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > MAX_BYTES.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("snapshot IPC byte bound"));
        }
        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(super) fn validate(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 16 || bytes.len() > MAX_BYTES || &bytes[..6] != b"ARROW1" {
        return Err("snapshot IPC byte bound or magic".into());
    }
    let trailer: [u8; 10] = bytes[bytes.len() - 10..]
        .try_into()
        .map_err(|_| "IPC trailer")?;
    let footer_len = arrow_ipc::reader::read_footer_length(trailer).map_err(|e| e.to_string())?;
    let footer_start = bytes
        .len()
        .checked_sub(10 + footer_len)
        .filter(|start| *start >= 8)
        .ok_or("snapshot IPC footer length")?;
    let footer = arrow_ipc::root_as_footer(&bytes[footer_start..bytes.len() - 10])
        .map_err(|e| e.to_string())?;
    let schema = footer.schema().ok_or("snapshot IPC schema missing")?;
    if !schema.endianness().equals_to_target_endianness() {
        return Err("snapshot IPC endianness".into());
    }
    let fields = schema.fields().ok_or("snapshot IPC fields missing")?;
    let mut field_count = 0;
    for field in fields {
        validate_field(field, 0, &mut field_count)?;
    }
    if let Some(metadata) = footer.custom_metadata() {
        for pair in metadata {
            if pair.key().is_none() || pair.value().is_none() {
                return Err("snapshot IPC metadata presence".into());
            }
        }
    }
    let blocks = footer
        .recordBatches()
        .ok_or("snapshot IPC record blocks missing")?;
    if blocks.len() > MAX_FIELDS {
        return Err("snapshot IPC batch bound".into());
    }
    let mut node_values = 0usize;
    let mut block_bytes = 0usize;
    let mut previous = 8usize;
    if footer.dictionaries().map_or(0, |blocks| blocks.len()) > MAX_FIELDS {
        return Err("snapshot IPC dictionary bound".into());
    }
    let mut blocks = footer
        .dictionaries()
        .into_iter()
        .flat_map(|blocks| blocks.iter())
        .chain(blocks.iter())
        .copied()
        .collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.offset());
    for block in blocks {
        let start =
            usize::try_from(block.offset()).map_err(|_| "snapshot IPC negative block offset")?;
        let metadata =
            usize::try_from(block.metaDataLength()).map_err(|_| "snapshot IPC metadata length")?;
        let body = usize::try_from(block.bodyLength()).map_err(|_| "snapshot IPC body length")?;
        let total = metadata
            .checked_add(body)
            .ok_or("snapshot IPC block overflow")?;
        let end = start
            .checked_add(total)
            .filter(|end| *end <= footer_start)
            .ok_or("snapshot IPC block outside payload")?;
        // Repeated/overlapping blocks would multiply decoded allocations without
        // encoded bytes. Dictionaries can be interleaved with record blocks.
        if start < previous || metadata < 8 {
            return Err("snapshot IPC overlapping block or short metadata".into());
        }
        previous = end;
        block_bytes = block_bytes
            .checked_add(total)
            .filter(|size| *size <= MAX_BYTES)
            .ok_or("snapshot IPC cumulative byte bound")?;
        let metadata = &bytes[start..start + metadata];
        if metadata[..4] != [255; 4] {
            return Err("snapshot IPC current continuation marker required".into());
        }
        let declared = i32::from_le_bytes(
            metadata[4..8]
                .try_into()
                .map_err(|_| "snapshot IPC metadata header")?,
        );
        if usize::try_from(declared)
            .ok()
            .is_none_or(|length| length > metadata.len() - 8)
        {
            return Err("snapshot IPC metadata outside block".into());
        }
        let message = arrow_ipc::root_as_message(&metadata[8..]).map_err(|e| e.to_string())?;
        if message.bodyLength() != block.bodyLength() {
            return Err("snapshot IPC body disagreement".into());
        }
        let record = message
            .header_as_record_batch()
            .or_else(|| {
                message
                    .header_as_dictionary_batch()
                    .and_then(|dict| dict.data())
            })
            .ok_or("snapshot IPC block kind")?;
        if record.compression().is_some() {
            return Err("compressed snapshot IPC is not supported".into());
        }
        if record.length() < 0 || record.length() as u64 > MAX_NODES as u64 {
            return Err("snapshot IPC row bound".into());
        }
        for node in record.nodes().ok_or("snapshot IPC nodes missing")? {
            let length = usize::try_from(node.length()).map_err(|_| "snapshot IPC node length")?;
            if node.null_count() < 0 || node.null_count() > node.length() {
                return Err("snapshot IPC null count".into());
            }
            node_values = node_values
                .checked_add(length)
                .filter(|size| *size <= MAX_NODES)
                .ok_or("snapshot IPC decoded element bound")?;
        }
        for buffer in record.buffers().ok_or("snapshot IPC buffers missing")? {
            let offset =
                usize::try_from(buffer.offset()).map_err(|_| "snapshot IPC buffer offset")?;
            let length =
                usize::try_from(buffer.length()).map_err(|_| "snapshot IPC buffer length")?;
            if offset.checked_add(length).is_none_or(|end| end > body) {
                return Err("snapshot IPC buffer outside body".into());
            }
        }
    }
    Ok(())
}

fn validate_field(
    field: arrow_ipc::Field<'_>,
    depth: usize,
    count: &mut usize,
) -> Result<(), String> {
    *count += 1;
    if depth > 32 || *count > MAX_FIELDS || field.name().is_none() {
        return Err("snapshot IPC field bound or presence".into());
    }
    let children = field.children();
    let arity = children.map_or(0, |fields| fields.len());
    if let Some(dictionary) = field.dictionary() {
        let integer = dictionary
            .indexType()
            .ok_or("snapshot IPC dictionary index type")?;
        if ![8, 16, 32, 64].contains(&integer.bitWidth()) {
            return Err("snapshot IPC dictionary width".into());
        }
    }
    // Materialized files use the kernel action schema, not arbitrary evidence fields.
    // Refuse new representations until their allocation and conversion are qualified.
    let valid = match field.type_type() {
        arrow_ipc::Type::Null
        | arrow_ipc::Type::Bool
        | arrow_ipc::Type::Utf8
        | arrow_ipc::Type::Binary
        | arrow_ipc::Type::LargeUtf8
        | arrow_ipc::Type::LargeBinary => arity == 0,
        arrow_ipc::Type::Int => {
            arity == 0
                && field
                    .type_as_int()
                    .is_some_and(|value| [8, 16, 32, 64].contains(&value.bitWidth()))
        }
        arrow_ipc::Type::Struct_ => true,
        arrow_ipc::Type::List | arrow_ipc::Type::LargeList => arity == 1,
        arrow_ipc::Type::Map => {
            arity == 1
                && field.type_as_map().is_some()
                && children.is_some_and(|fields| {
                    let entries = fields.get(0);
                    entries.type_type() == arrow_ipc::Type::Struct_
                        && entries.children().is_some_and(|parts| parts.len() == 2)
                })
        }
        _ => false,
    };
    if !valid {
        return Err(format!(
            "unsupported snapshot IPC field: {:?}",
            field.type_type()
        ));
    }
    for child in children.into_iter().flat_map(|fields| fields.iter()) {
        validate_field(child, depth + 1, count)?;
    }
    Ok(())
}
