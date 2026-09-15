//! Bounded fixture collection over the actual producer visits; never a production export.
use enrichment_core::evidence::{EvidenceFragment, Relationship, Symbol, ingest::ProducerSource};
pub struct Fixture<T> {
    pub source: T,
    pub symbols: Vec<Symbol>,
    pub relationships: Vec<Relationship>,
    pub fragments: Vec<EvidenceFragment>,
}
pub fn collect<T: ProducerSource>(source: T) -> Result<Fixture<T>, String> {
    let mut rows = 0usize;
    let mut bytes = 0usize;
    let mut charge = |size: usize| -> Result<(), String> {
        rows += 1;
        bytes += size;
        if rows > 4096 || bytes > 16 * 1024 * 1024 {
            return Err("fixture exceeds its transport bound".into());
        }
        Ok(())
    };
    let mut symbols = Vec::new();
    source.visit_symbols(&mut |row| {
        charge(
            enrichment_core::canonical::serialized_size(&row, 1024 * 1024)
                .map_err(|e| e.to_string())?,
        )?;
        symbols.push(row);
        Ok(())
    })?;
    let mut relationships = Vec::new();
    source.visit_relationships(&mut |row| {
        charge(
            enrichment_core::canonical::serialized_size(&row, 1024 * 1024)
                .map_err(|e| e.to_string())?,
        )?;
        relationships.push(row);
        Ok(())
    })?;
    let mut fragments = Vec::new();
    source.visit_fragments(&mut |row| {
        charge(
            enrichment_core::canonical::serialized_size(&row, 1024 * 1024)
                .map_err(|e| e.to_string())?,
        )?;
        fragments.push(row);
        Ok(())
    })?;
    Ok(Fixture {
        source,
        symbols,
        relationships,
        fragments,
    })
}
