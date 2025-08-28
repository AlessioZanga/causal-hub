use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // Generate `causal_hub.pyi` stub file.
    let stub = causal_hub::stub_info()?;
    stub.generate()?;

    Ok(())
}
