# AGENTS.md

## 1. Mission of Autonomous Agents

You are an automated contributor to **causal-hub**, a high-performance library for causal modeling, inference, and discovery.

**Your primary mission is to maintain the rigorous correctness and high performance of this hybrid Rust/Python system.**

### Core Objectives

1. **Preserve Causal Correctness:** Algorithms must mathematically adhere to SCM, BN, and CTBN definitions.
2. **Maintain FFI Integrity:** The Python frontend and Rust backend must remain strictly synchronized via `pyo3` bindings.
3. **Ensure Performance:** Critical paths must remain in Rust, utilizing BLAS/LAPACK where appropriate.

**Optimization Goal:** Computational Efficiency and Type Safety over rapid prototyping.

---

## 2. Architectural Invariants (DO NOT VIOLATE)

### Layer Boundaries

- **Backend (Rust):** Responsible for heavy lifting, algorithms, data structures, and memory safety.
  - **Invariant:** Core logic **MUST** be implemented in Rust. Do not implement heavy loops or strictly numerical algorithms in Python.
- **Frontend (Python):** Responsible for API accessibility, integration with the PyData ecosystem (`numpy`, `pandas`), and developer experience.
  - **Invariant:** Python classes **MUST** mirror their underlying Rust structs without leaking unsafe pointers.
- **Bridge (PyO3/Maturin):** The binding layer.
  - **Invariant:** Data conversion (e.g., `PyArray` <-> `ndarray`) must be minimized or zero-copy where possible.

### Data Ownership & API Contracts

- **Schema Parity:** The Rust structs in `causal-hub/src/models` and their Python wrappers in `python/src/models` represent a **hard synchronization contract**.
  - *If you change a Rust struct, you MUST update the corresponding Python wrapper and type stub (`.pyi`).*
- **Safe Error Propagation:**
  - **Invariant:** Rust code **MUST NOT PANIC** (`unwrap` / `expect` are forbidden). Errors must be propagated as `Result` and converted to Python Exceptions at the boundary.

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

### You may NOT (without explicit instructions)

- Rewrite the build system (e.g., switch from `maturin` to `setuptools-rust`).
- Introduce pure-Python implementations of core algorithms (except for trivial helpers).
- Remove `openblas` dependency mechanisms.
- Relax linter strictness (`clippy`, `ruff`, `pre-commit`).

---

## 4. Forbidden Changes (Without Explicit Justification)

1. **Unsafe Rust Usage:**
    - *Rule:* Do not use `unsafe` blocks unless absolutely necessary for FFI or performance-critical unchecked indexing, and *always* document safety invariants.
2. **Breaking Type Stubs:**
    - *Rule:* Never modify a Python method signature without updating the corresponding `.pyi` file.
    - *Reasoning:* Breaks IDE autocomplete and static analysis for users.
3. **Dependency Bloat:**
    - *Rule:* Do not add heavy dependencies to `causal-hub/Cargo.toml` without justifying why the logic cannot be implemented with existing crates.
4. **Ignoring Errors:**
    - *Rule:* Do not use `let _ = ...` to swallow Results in Rust. Handle them or propagate them.

---

## 5. Dependency Governance

- **Rust (Backend):**
  - Manage via `causal-hub/Cargo.toml`.
  - Critical: `openblas` (computational backend), `ndarray`, `pyo3`.
- **Python (Frontend):**
  - Manage via `pyproject.toml` and `python/Cargo.toml`.
  - Critical: `numpy`, `pandas`, `scipy`, `networkx`.

---

## 6. Code Modification Protocol

1. **Analysis Phase:**
    - Identify the full call stack: Python call -> PyO3 wrapper -> Rust implementation.
    - If modifying a Model: Check `causal-hub/src/models`, `python/src/models`, and `python/causal_hub/*.pyi`.
2. **Execution Phase:**
    - Apply changes locally.
    - **CRITICAL:** If a struct field is added, update the `#[pyclass]` definition and the `__init__` constructor validation.
3. **Verification Phase:**
    - Run Rust tests: `cargo test --workspace --features openblas-system`.
    - Run Python tests: `pytest python/tests`.
    - Run Linters: `pre-commit run --all-files`.

---

## 7. Feature Addition Protocol

- **Rust Backend:**
  - New logic belongs in `causal-hub/src/`.
  - Create: Struct/Trait impl -> Unit Test -> `pub mod` export.
- **Python Bindings:**
  - Expose via `python/src/`.
  - Create: `#[pyclass]` or `#[pyfunction]` -> Register in `lib.rs` -> Add `.pyi` stub -> Add `pytest` case.

---

## 8. Refactoring Rules

- **Preserve Traits:** Do not break key traits like `DirectedGraph` or `Estimator` which facilitate polymorphism.
- **Generics:** Maintain generic implementations over floating point types (`F: Float`) to support both `f32` and `f64`.
- **Strict Typing:** Maintain strict Rust typing. Minimize `Box<dyn Any>` usage.

---

## 9. Testing Obligations

- **Rust:**
  - `#[test]` functions for all new logic are mandatory.
  - Test edge cases (empty graphs, singular matrices).
- **Python:**
  - Integration tests in `python/tests/` must verify that data round-trips correctly between Python and Rust.
  - Test for correct exception raising (e.g. passing invalid headers to a dataset loader).

---

## 10. Performance & Scalability Constraints

- **Memory:** Avoid cloning large vectors/matrices (`Vec`, `DMatrix`) unless necessary. Use views/slices.
- **Parallelism:** Use `rayon` for data parallelism in heavy estimators, but ensure it doesn't conflict with BLAS threading configurations.
- **Builds:** Ensure code compiles without warnings to keep CI times low and artifacts clean.

---

## 11. Security & Safety Constraints

- **Input Validation:**
  - Panic on invalid input is forbidden in `pub` functions. Return `Result`.
  - Validate array dimensions before performing matrix operations.
- **FFI Safety:**
  - Ensure objects passed from Python (like PyList, PyArray) are checked for type and consistency before casting to Rust types.

---

## 12. Observability Rules

- **Logging:**
  - Rust: Use `log` crate macros (`info!`, `warn!`, `debug!`).
  - Python: Use `pyo3-log` so Rust logs appear in standard Python logging streams.
- **Errors:**
  - Error messages must be descriptive (e.g., "Matrix is not positive definite" vs "Math error").

---

## 13. Documentation Synchronization

- **Rust Docs:** Update `///` documentation comments for any changed public API.
- **Python Docs:** Update `"""Docstrings"""` in the PyO3 wrappers.
- **Stubs:** `*.pyi` files are the source of truth for the user's IDE. They must exactly match the implementation.

---

## 14. Multi-Agent Coordination Rules

- **Cross-Language Atomicity:** A PR that adds a feature to Rust but fails to expose it in Python (or vice-versa) is incomplete.
- **Stub First:** Define the desired Python API signature (in `.pyi` or pseudo-code) *before* implementing the Rust binding logic.

---

## 15. Change Risk Classification

- **Low Risk:** Doc string updates, adding a helper function in `utils`, adding a test.
- **Medium Risk:** Modifying an estimator's algorithm, updating a dependency version.
- **High Risk:** Changing `Cargo.toml` features, modifying `unsafe` blocks, changing the memory layout of core structs (`BN`, `Digraph`). -> **REQUIRES EXTENSIVE TESTING.**

---

## 16. Stop Conditions

**HALT execution if:**

1. `cargo check` or `cargo clippy` fails.
2. You cannot identify where a Python class's underlying Rust struct is defined.
3. You are tempted to use `unwrap()` to solve a type error.
4. Pre-existing tests fail and you do not understand why.
5. You are about to modify a lockfile (`Cargo.lock`) manually.
