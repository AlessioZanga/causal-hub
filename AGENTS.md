# AGENTS.md

## 1. Project Overview

**causal-hub** is a high-performance library for causal modeling, inference, and discovery.

When contributing to this project, prioritize the correctness and performance of this hybrid Rust/Python system.

### Core Objectives

1. **Preserve Causal Correctness:** Algorithms must mathematically adhere to SCM, BN, and CTBN definitions.
2. **Maintain FFI Integrity:** The Python frontend and Rust backend must remain strictly synchronized via `pyo3` bindings.
3. **Ensure Performance:** Critical paths must remain in Rust, utilizing BLAS/LAPACK where appropriate.

**Optimization Goal:** Computational Efficiency and Type Safety over rapid prototyping.

---

## 2. Architectural Invariants

### Layer Boundaries

- **Backend (Rust):** Heavy lifting, algorithms, data structures, and memory safety.
  - Core logic belongs in Rust. Do not implement heavy loops or numerical algorithms in Python.
- **Frontend (Python):** API accessibility, PyData integration (`numpy`, `pandas`), and developer experience.
  - Python classes should mirror their underlying Rust structs without leaking unsafe pointers.
- **Bridge (PyO3/Maturin):** The binding layer.
  - Minimize data conversion (e.g., `PyArray` ↔ `ndarray`); prefer zero-copy where possible.

### Data Ownership & API Contracts

- **Schema Parity:** Rust structs in `causal-hub/src/models` and their Python wrappers in `python/src/models` are a synchronization contract.
  - Changing a Rust struct requires updating the corresponding Python wrapper and `.pyi` type stub.
- **Safe Error Propagation:**
  - Rust code should not panic (`unwrap` / `expect` are forbidden in library code). Propagate errors as `Result` and convert to Python exceptions at the FFI boundary.

### State Management

- **Immutability:** Rust components should prefer immutability.
- **Interior Mutability:** Use `RefCell` or `Mutex` strictly only when necessary for caching or shared state, and never expose lock guards across the FFI boundary.

---

## 3. Allowed Changes

### You may

- **Refactor:** Consolidate common math logic in `src/utils` or abstract traits in `src/estimators`.
- **Optimize:** Replace safe iterator chains with optimized BLAS calls or parallel iterators (`rayon`) if performance benchmarks justify it.
- **Update:** Upgrade Rust crates or Python dependencies within semantic versioning safety limits.
- **Add Features:** Implement new Causal Discovery algorithms or Inference engines, following the trait-based architecture.

### Restricted Changes

The following changes require explicit user approval. Do not perform them autonomously:

- Rewriting the build system (e.g., switching from `maturin` to `setuptools-rust`).
- Introducing pure-Python implementations of core algorithms (trivial helpers are fine).
- Removing `openblas` dependency mechanisms.
- Relaxing linter strictness (`clippy`, `ruff`, `pre-commit`).
- Adding `unsafe` blocks (if unavoidable for FFI, document the safety invariants).
- Modifying a Python method signature without updating the corresponding `.pyi` stub.
- Adding heavy new dependencies to `causal-hub/Cargo.toml` without justification.
- Using `let _ = ...` to swallow `Result` values in Rust—handle or propagate errors instead.

---

## 4. Dependency Governance

- **Rust (Backend):**
  - Manage via `causal-hub/Cargo.toml`.
  - Critical: `openblas` (computational backend), `ndarray`, `pyo3`.
- **Python (Frontend):**
  - Manage via `pyproject.toml` and `python/Cargo.toml`.
  - Critical: `numpy`, `pandas`, `scipy`, `networkx`.

---

## 5. Code Modification Protocol

Follow this workflow when making changes. Steps are guidelines—use judgment on scope:

1. **Analyze:** Trace the relevant call path (Python call → PyO3 wrapper → Rust implementation). For model changes, check `causal-hub/src/models`, `python/src/models`, and `python/causal_hub/*.pyi`.
2. **Apply:** Make the code changes. Remember: if a struct field is added, update the `#[pyclass]` definition and `__init__` constructor.
3. **Verify (when appropriate):**
    - Rust tests: `cargo test --workspace --features openblas-system`
    - Python tests: `pytest python/tests`
    - Linters: `pre-commit run --all-files`

---

## 6. Feature Addition Protocol

- **Rust Backend:**
  - New logic belongs in `causal-hub/src/`.
  - Create: Struct/Trait impl -> Unit Test -> `pub mod` export.
- **Python Bindings:**
  - Expose via `python/src/`.
  - Create: `#[pyclass]` or `#[pyfunction]` -> Register in `lib.rs` -> Add `.pyi` stub -> Add `pytest` case.

---

## 7. Refactoring Rules

- **Preserve Traits:** Do not break key traits like `DirectedGraph` or `Estimator` which facilitate polymorphism.
- **Generics:** Maintain generic implementations over floating point types (`F: Float`) to support both `f32` and `f64`.
- **Strict Typing:** Maintain strict Rust typing. Minimize `Box<dyn Any>` usage.

---

## 8. Testing Obligations

- **Rust:**
  - `#[test]` functions for all new logic are mandatory.
  - Test edge cases (empty graphs, singular matrices).
- **Python:**
  - Integration tests in `python/tests/` must verify that data round-trips correctly between Python and Rust.
  - Test for correct exception raising (e.g. passing invalid headers to a dataset loader).

---

## 9. Performance & Scalability Constraints

- **Memory:** Avoid cloning large vectors/matrices (`Vec`, `DMatrix`) unless necessary. Use views/slices.
- **Parallelism:** Use `rayon` for data parallelism in heavy estimators, but ensure it doesn't conflict with BLAS threading configurations.
- **Builds:** Ensure code compiles without warnings to keep CI times low and artifacts clean.

---

## 10. Security & Safety Constraints

- **Input Validation:**
  - Panic on invalid input is forbidden in `pub` functions. Return `Result`.
  - Validate array dimensions before performing matrix operations.
- **FFI Safety:**
  - Ensure objects passed from Python (like PyList, PyArray) are checked for type and consistency before casting to Rust types.

---

## 11. Observability Rules

- **Logging:**
  - Rust: Use `log` crate macros (`info!`, `warn!`, `debug!`).
  - Python: Use `pyo3-log` so Rust logs appear in standard Python logging streams.
- **Errors:**
  - Error messages must be descriptive (e.g., "Matrix is not positive definite" vs "Math error").

---

## 12. Documentation Synchronization

- **Rust Docs:** Update `///` documentation comments for any changed public API.
- **Python Docs:** Update `"""Docstrings"""` in the PyO3 wrappers.
- **Stubs:** `*.pyi` files are the source of truth for the user's IDE. They must exactly match the implementation.

---

## 13. Multi-Agent Coordination Rules

- **Cross-Language Atomicity:** A PR that adds a feature to Rust but fails to expose it in Python (or vice-versa) is incomplete.
- **Stub First:** Define the desired Python API signature (in `.pyi` or pseudo-code) *before* implementing the Rust binding logic.

---

## 14. Change Risk Classification

- **Low Risk:** Doc string updates, adding a helper function in `utils`, adding a test.
- **Medium Risk:** Modifying an estimator's algorithm, updating a dependency version.
- **High Risk:** Changing `Cargo.toml` features, modifying `unsafe` blocks, changing the memory layout of core structs (`BN`, `Digraph`). -> **REQUIRES EXTENSIVE TESTING.**

---

## 15. Stop Conditions

**If any of the following occur, stop the current task and report the issue to the user before continuing:**

1. `cargo check` or `cargo clippy` produces errors you cannot resolve.
2. After a reasonable search, you cannot locate the Rust struct backing a Python class.
3. You find yourself reaching for `unwrap()` to work around a type error.
4. Pre-existing tests fail and the cause is unclear.
5. A change would require manually editing `Cargo.lock`.
