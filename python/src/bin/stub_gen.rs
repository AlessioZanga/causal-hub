use std::io::Read;

use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // Generate `causal_hub.pyi` stub file.
    let stub = causal_hub::stub_info()?;
    stub.generate()?;
    // Set path to the stub file.
    const STUB_FILE: &str = "causal_hub.pyi";
    // Open the `causal_hub.pyi` file for reading.
    let mut file = std::fs::File::open(STUB_FILE)
        .unwrap_or_else(|_| panic!("Failed to open `{STUB_FILE}` file"));
    // Read `causal_hub.pyi` file to string.
    let mut content = String::new();
    file.read_to_string(&mut content)
        .unwrap_or_else(|_| panic!("Failed to read `{STUB_FILE}` file"));
    // Workaround to fix `_cls` annotation.
    content = content.replace("cls, _cls:type", "cls");
    // Write `causal_hub.pyi` file.
    std::fs::write("causal_hub.pyi", content)
        .unwrap_or_else(|_| panic!("Failed to write `{STUB_FILE}` file"));

    Ok(())
}
