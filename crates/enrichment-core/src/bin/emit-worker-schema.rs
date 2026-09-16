//! Generate the worker's Arrow schema from its Rust owner.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() != 1 {
        return Err("usage: emit-worker-schema <output.arrow>".into());
    }
    enrichment_core::producer::python::worker::write_schema(std::fs::File::create(&args[0])?)?;
    Ok(())
}
