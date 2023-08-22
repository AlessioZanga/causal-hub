use criterion::criterion_main;

// Modules
mod pc_stable;

// Main
criterion_main!(pc_stable::discrete::discrete);
