use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // Generate stub files.
    let stub = causal_hub::stub_info()?;
    stub.generate()?;

    Ok(())
}
