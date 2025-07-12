use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = causal_hub::stub_info()?;
    stub.generate()?;
    Ok(())
}
