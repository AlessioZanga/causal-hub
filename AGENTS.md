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

## 2. Architecture Overview

```text
causal-hub/src/           (Rust backend crate)
├── lib.rs                # Crate root — 9 public modules
├── assets/               # Embedded benchmark models (BIF/JSON/JSON-Schema)
├── datasets/             # Data containers for tables & trajectories
│   ├── missing.rs        # MissingMechanism, MissingMethod, MissingType, IncDataset trait
│   ├── table/
│   │   ├── categorical/  # CatTable, CatEv, CatIncTable, CatWtdTable
│   │   └── gaussian/     # GaussTable, GaussEv, GaussIncTable, GaussWtdTable
│   └── trajectory/
│       └── categorical/  # CatTrj, CatTrjs, CatTrjEv, CatWtdTrj, CatWtdTrjs
├── estimators/           # Parameter & structure learning
│   ├── parameters/       # MLE, BE, SSE, EM, RAWE
│   └── structures/       # CTPC, CTHC, PK, CI tests, scoring criteria
├── inference/            # Approximate / causal inference & graph ops
│   ├── approximate_inference.rs, causal_inference.rs
│   ├── backdoor_criterion.rs, graphical_separation.rs
│   ├── topological_order.rs, v_structures.rs
├── io/                   # Serialization (BIF/PEG, CSV, JSON/JSON-Schema)
├── models/               # Causal model definitions
│   ├── graphs/           # Graph trait, DiGraph, UnGraph
│   ├── bayesian_network/ # BN trait + CatBN, GaussBN, MixedBN
│   └── continuous_time_bayesian_network/  # CTBN trait + CatCTBN
├── random/               # Controllable random generators
│   ├── datasets/         # RngCatIncTable, RngGaussIncTable, RngCatTrjEv
│   └── models/           # RngDiGraph, RngDag, RngUnGraph, RngMissingMechanism,
│                         # RngCatBN, RngCatCPD, RngGaussBN, RngGaussCPD
├── samplers/             # ForwardSampler, ImportanceSampler
├── types/                # Error, ErrorKind, Result, Cache, Map, Set, Labels, States
└── utils/                # MI (multi-index), PseudoInverse trait

python/src/               (PyO3 bridge crate)
├── lib.rs                # 4 submodules + Error exception
├── error.rs              # causal_hub.Error (Python exception)
├── bin/stub_gen.rs       # Auto-generates .pyi stubs via pyo3-stub-gen
├── assets/mod.rs         # 25 load_* functions
├── utils/                # Bridge utility macros
├── models/               # PyDiGraph, PyCatBN, PyGaussBN, PyCatCTBN, Py*CPD, Py*CIM
├── datasets/             # PyCatTable, PyGaussTable, Py*IncTable, Py*Ev, Py*Trj, PyMissing*
└── estimators/           # PyPK, em(), sem()

python/causal_hub/        (Python package — nearly pure FFI)
├── __init__.py           # from .causal_hub import *
├── py.typed              # PEP 561 marker
├── models.pyi            # Auto-generated type stubs (1929 lines)
├── datasets.pyi          # Auto-generated type stubs (1465 lines)
├── estimators.pyi        # Auto-generated type stubs (61 lines)
└── assets.pyi            # Auto-generated type stubs (129 lines)
```

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

## 3. Module Reference

### Core Traits (`models/`)

| Trait | Location | Key Methods |
|-------|----------|-------------|
| `Labelled` | `models/mod.rs` | `labels()`, `label_to_index()`, `index_to_label()`, `index_to()`, `indices_to()`, `index_from()`, `indices_from()` |
| `Graph` | `models/graphs/mod.rs` | `empty(labels)`, `complete(labels)`, `vertices()`, `edges()`, `has_vertex(x)`, `has_edge(x,y)`, `add_edge(x,y)`, `del_edge(x,y)`, `select(x)`, `from_adjacency_matrix(labels, mat)`, `to_adjacency_matrix()` |
| `CPD` | `models/mod.rs` | `conditioning_labels()`, `parameters()`, `parameters_size()`, `fitted_statistics()`, `fitted_log_likelihood()`, `pf(x, z)`, `sample(rng, z)` |
| `CIM` | `models/mod.rs` | `conditioning_labels()`, `parameters()`, `parameters_size()`, `fitted_statistics()`, `fitted_log_likelihood()` |
| `Phi` | `models/mod.rs` | `parameters()`, `parameters_size()`, `condition(e)`, `marginalize(x)`, `normalize()`, `from_cpd(cpd)`, `into_cpd(x, z)` |
| `BN` | `models/bayesian_network/mod.rs` | `new(graph, cpds)`, `name()`, `description()`, `graph()`, `cpds()`, `parameters_size()`, `select(x)`, `topological_order()`, `with_optionals(...)` |
| `CTBN` | `models/continuous_time_bayesian_network/mod.rs` | `new(graph, cims)`, `initial_distribution()`, `graph()`, `cims()`, `parameters_size()`, `with_optionals(...)` |

### Model Implementations

| Struct | Description | Source |
|--------|-------------|--------|
| `DiGraph` | Directed graph (adjacency-matrix-backed) | `models/graphs/directed.rs` |
| `UnGraph` | Undirected graph | `models/graphs/undirected.rs` |
| `CatBN` | Categorical Bayesian network | `models/bayesian_network/categorical/bn.rs` |
| `CatCPD` | Categorical CPD (CPT as 2D array) | `models/bayesian_network/categorical/cpd.rs` |
| `CatCPDS` | CatCPD sufficient statistics | `models/bayesian_network/categorical/cpd.rs` |
| `CatPhi` | Categorical potential for inference | `models/bayesian_network/categorical/potential.rs` |
| `GaussBN` | Gaussian Bayesian network | `models/bayesian_network/gaussian/bn.rs` |
| `GaussCPD` | Gaussian CPD (regression) | `models/bayesian_network/gaussian/cpd.rs` |
| `GaussCPDP` | GaussCPD parameters (coeffs, intercept, cov) | `models/bayesian_network/gaussian/cpd.rs` |
| `GaussCPDS` | GaussCPD sufficient statistics | `models/bayesian_network/gaussian/sufficient_statistics.rs` |
| `GaussPhiK`, `GaussPhi` | Gaussian potential (canonical form) | `models/bayesian_network/gaussian/potential.rs` |
| `MixedBN` | Mixed (categorical+gaussian) BN | `models/bayesian_network/mixed/model.rs` |
| `MixedCPD`, `MixedCPDS`, `MixedSample` | Enum-based CPD dispatchers | `models/bayesian_network/mixed/parameters.rs` |
| `MixedEv`, `MixedTable`, `MixedIncTable`, `MixedWtdTable` | Enum-based data dispatchers | `models/bayesian_network/mixed/model.rs` |
| `CatCTBN` | Categorical CTBN | `models/continuous_time_bayesian_network/categorical/ctbn.rs` |
| `CatCIM` | Categorical CIM (3D rate matrix) | `models/continuous_time_bayesian_network/categorical/cim.rs` |
| `CatCIMS` | CatCIM sufficient statistics | `models/continuous_time_bayesian_network/categorical/cim.rs` |

### Dataset Types (`datasets/`)

| Struct | Description |
|--------|-------------|
| `CatTable` | Categorical complete dataset (Array2<u8>) |
| `CatIncTable` | Categorical incomplete dataset (Option<u8>) |
| `CatWtdTable` | Categorical weighted dataset |
| `CatEv` | Categorical evidence (label→value map) |
| `GaussTable` | Gaussian complete dataset (Array2<f64>) |
| `GaussIncTable` | Gaussian incomplete dataset (Option<f64>) |
| `GaussWtdTable` | Gaussian weighted dataset |
| `GaussEv` | Gaussian evidence (label→value map) |
| `CatTrj` | Single categorical trajectory |
| `CatTrjs` | Multiple categorical trajectories |
| `CatTrjEv` | Categorical trajectory evidence (event-type enum) |
| `CatTrjsEv` | Multiple trajectory evidence |
| `CatWtdTrj`, `CatWtdTrjs` | Weighted trajectories |
| `MissingMechanism` | Missing data mechanism (MCAR/MAR/MNAR) |
| `MissingTable` | Table with missingness metadata |

### Estimator Traits & Implementations (`estimators/`)

| Trait | Purpose |
|-------|---------|
| `CSSEstimator<T>` | CPD sufficient statistics estimation |
| `ParCSSEstimator<T>` | Parallel variant |
| `CPDEstimator<T>` | CPD parameter estimation |
| `ParCPDEstimator<T>` | Parallel variant |
| `BNEstimator<T>` / `ParBNEstimator<T>` | BN fitting (blanket via CPDEstimator) |
| `CTBNEstimator<T>` / `ParCTBNEstimator<T>` | CTBN fitting (blanket via CPDEstimator) |

| Struct | Description |
|--------|-------------|
| `MLE<'a, D>` | Maximum likelihood estimator |
| `BE<'a, D, T>` | Bayesian estimator (Dirichlet/Normal-Wishart priors) |
| `SSE<'a, D>` | Sufficient statistics estimator (weighted/IPW/AIPW/PW) |
| `EM<'a, M, E, ...>` | EM algorithm iterator (CatCTBN) |
| `EMBuilder<'a, M, E, ...>` | EM builder pattern |
| `RAWE<'a, R, E, D>` | Raw evidence estimator for CTBN |
| `ChiSquaredTest<'a, E>` | χ² conditional independence test |
| `FTest<'a, E>` | F-test conditional independence test |
| `CTPC<'a, T, S>` | Continuous-time PC algorithm |
| `CTHC<'a, S>` | Continuous-time hill climbing |
| `PK` | Prior knowledge (forbidden/required/temporal) |
| `LL`, `AIC`, `AICC`, `BIC`, `BICC`, `HQC` | Scoring criteria for structure learning |

### Inference (`inference/`)

| Struct | Trait(s) | Description |
|--------|----------|-------------|
| `ApproximateInference<'a, R, M, F>` | `BNInference`, `ParBNInference` | Monte Carlo approximate inference (sequential & parallel) |
| `CausalInference<'a, E>` | `BNCausalInference`, `ParBNCausalInference` | PACE/CPACE causal effect estimation |
| — | `BackdoorCriterion` | Backdoor adjustment set operations |
| — | `GraphicalSeparation` | d-separation, minimal separator sets |
| — | `TopologicalOrder` | Topological ordering of DAGs |
| — | `VStructures` | V-structure identification |

### Samplers (`samplers/`)

| Struct | Trait(s) | Description |
|--------|----------|-------------|
| `ForwardSampler<'a, R, M>` | `BNSampler`, `ParBNSampler`, `CTBNSampler`, `ParCTBNSampler` | Ancestral sampling |
| `ImportanceSampler<'a, R, M, E>` | (CTBN only) | Importance sampling with evidence |

### Random Generators (`random/`)

All implement `Random` (and some `ParRandom`). Used for controlled, reproducible generation.

| Struct | Generates |
|--------|-----------|
| `RngDiGraph<'a, R>` | Random directed graph |
| `RngDag<'a, R>` | Random DAG |
| `RngUnGraph<'a, R>` | Random undirected graph |
| `RngMissingMechanism<'a, R>` | Random missing data (MCAR/MAR/MNAR) |
| `RngCatBN<'a, R>` | Random CatBN with tunable density |
| `RngCatCPD<'a, R>` | Random CatCPD (Dirichlet-sampled) |
| `RngGaussBN<'a, R>` | Random GaussBN |
| `RngGaussCPD<'a, R>` | Random GaussCPD |
| `RngCatIncTable<'a, R>` | Random incomplete categorical table |
| `RngGaussIncTable<'a, R>` | Random incomplete Gaussian table |
| `RngCatTrjEv<'a, R, D>` | Random trajectory evidence |

### I/O (`io/`)

| Trait | Format | Implementors |
|-------|--------|--------------|
| `BifIO` | BIF (Bayesian Interchange Format, PEG-parsed) | CatBN |
| `JsonIO` | JSON (with JSON Schema validation) | CatBN, GaussBN, CatCTBN, DiGraph, UnGraph |
| `CsvIO` | CSV (string + file) | CatTable, CatIncTable, GaussTable |

### Type Aliases & Support Types (`types/`)

| Type | Definition |
|------|------------|
| `Map<K, V>` | `IndexMap<K, V, FxBuildHasher>` |
| `Set<T>` | `IndexSet<T, FxBuildHasher>` |
| `Labels` | `Set<String>` |
| `States` | `Map<String, Set<String>>` |
| `Result<T>` | `std::result::Result<T, Error>` |
| `Error` | Error struct with `kind: ErrorKind` + `location: &'static Location<'static>` |
| `ErrorKind` | 30+ variants (InvalidParameter, LabelsMismatch, etc.) |
| `MI` | Multi-dimensional index (ravel/unravel) |
| `PseudoInverse` trait | Pseudo-inverse computation for matrices |

### Error Handling

- All errors use `ErrorKind` enum (30+ variants) with `#[track_caller]` location tracking.
- Use `Err(Error::new(ErrorKind::InvalidParameter("param", "reason")))` instead of panics.
- The `Error` type implements `Into<PyErr>` at the FFI boundary for Python exception conversion.
- Tests verify error kinds via `matches!(err, Err(Error { kind: ErrorKind::..., .. }))`.

### JSON Schema Validation

JSON I/O is validated against 9 embedded JSON schemas loaded by `InMemoryRetriever` at startup. Schema files are stored in `assets/json_schema/`.

---

## 4. Allowed Changes

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

## 5. Dependency Governance

- **Rust (Backend):**
  - Manage via `causal-hub/Cargo.toml`.
  - Critical: `openblas` (computational backend), `ndarray` (+approx, +rayon, +serde), `pyo3`.
  - BLAS feature flags: `openblas-static` (default), `openblas-system`, `accelerate-system`.
  - Scientific: `statrs` (χ²/F distributions), `ndarray-linalg` (Cholesky/SVD), `ndarray-stats`.
  - Parsing: `pest`/`pest_derive` (PEG for BIF), `csv`, `jsonschema`.
  - Data structures: `indexmap`, `fxhash`, `itertools`, `paste`, `dry`.
  - Testing: `criterion` (benchmarks), `tempfile`, `rand_xoshiro`.
- **Python (Frontend):**
  - Manage via `pyproject.toml` and `python/Cargo.toml`.
  - Critical: `numpy`, `pandas`, `scipy`, `networkx`.
  - Stub generation: `pyo3-stub-gen`, `pyo3-stub-gen-derive`.

---

## 6. Code Modification Protocol

Follow this workflow when making changes. Steps are guidelines—use judgment on scope:

1. **Analyze:** Trace the relevant call path (Python call → PyO3 wrapper → Rust implementation). For model changes, check `causal-hub/src/models`, `python/src/models`, and `python/causal_hub/*.pyi`.
2. **Apply:** Make the code changes. Remember: if a struct field is added, update the `#[pyclass]` definition and `__init__` constructor.
3. **Verify (when appropriate):**
    - Rust integration tests: `cargo test --test mod --features openblas-system`
    - All Rust tests (including unit): `cargo test --workspace --features openblas-system`
    - Python tests: `pytest python/tests -v`
    - Linters: `pre-commit run --all-files`

---

## 7. Feature Addition Protocol

- **Rust Backend:**
  - New logic belongs in `causal-hub/src/`.
  - Create: Struct/Trait impl -> Unit Test -> `pub mod` export.
- **Python Bindings:**
  - Expose via `python/src/`.
  - Create: `#[pyclass]` or `#[pyfunction]` -> Register in `lib.rs` -> Add `.pyi` stub -> Add `pytest` case.
- **MixedBN Specifics (if extending):**
  - Add a variant to each non-exhaustive enum (`MixedCPD`, `MixedCPDS`, `MixedSample`, `MixedEv`, `MixedTable`, `MixedIncTable`, `MixedWtdTable`).
  - Implement `From<NewType>` for each enum.
  - Update `CPD` for `MixedCPD` to dispatch the new variant in `pf`, `sample`, `fitted_statistics`, etc.
  - Update `BN` for `MixedBN` validation if new variant imposes new constraints.

---

## 8. Refactoring Rules

- **Preserve Traits:** Do not break key traits like `Graph`, `CPD`, `CIM`, `Phi`, `BN`, `CTBN`, `Estimator`, `BNInference` which facilitate polymorphism.
- **Generics:** Maintain generic implementations over floating point types (`F: Float`) to support both `f32` and `f64`.
- **Builder Pattern:** Estimators (`MLE`, `BE`, `SSE`, `EM`, `CTPC`, `CTHC`) use a builder pattern with `new()` returning a builder and `.fit()`/`.par_fit()` consuming it.
- **Strict Typing:** Maintain strict Rust typing. Minimize `Box<dyn Any>` usage.

---

## 9. Testing Obligations

- **Rust Integration Tests** (`causal-hub/tests/`):
  - `#[test]` functions for all new logic are mandatory.
  - Categorize under `tests/models/bayesian_network/<type>/` for model tests, `tests/estimators/` for estimator tests, etc.
  - Test edge cases: empty graphs, singular matrices, missing data, label mismatches.
  - Use `assert_relative_eq!` with appropriate epsilon (1e-2 to 5e-2 for stochastic tests, 1e-8 for deterministic).
  - Use deterministic RNG: `Xoshiro256PlusPlus::seed_from_u64(42)`.
  - Mark slow tests with `#[ignore = "slow; run manually in release mode"]`.
  - Test error kinds via pattern matching (`matches!(err, Error { kind: ErrorKind::..., .. })`).
- **Python Integration Tests** (`python/tests/`):
  - Must verify data round-trips correctly between Python and Rust (pandas/polars ↔ native types).
  - Test for correct exception raising (e.g., passing invalid headers to a dataset loader).
  - Validate PyData integration (NetworkX interop, pandas/polars I/O).

---

## 10. Performance & Scalability Constraints

- **Memory:** Avoid cloning large vectors/matrices (`Vec`, `Array2`) unless necessary. Use views/slices.
- **Parallelism:** Use `rayon` for data parallelism in estimators, inference, and samplers, but ensure it doesn't conflict with BLAS threading configurations. Release GIL (`py.detach()`) before entering parallel regions in the bridge layer.
- **Builds:** Ensure code compiles without warnings to keep CI times low and artifacts clean.

---

## 11. Security & Safety Constraints

- **Input Validation:**
  - Panic on invalid input is forbidden in `pub` functions. Return `Result`.
  - Validate array dimensions before performing matrix operations (label counts, parent counts, etc.).
- **FFI Safety:**
  - Ensure objects passed from Python (like `PyList`, `PyArray`) are checked for type and consistency before casting to Rust types.
- **No `unsafe`:** the codebase must remain `unsafe`-free. If unavoidable for FFI, document safety invariants and get approval.

---

## 12. Observability Rules

- **Logging:**
  - Rust: Use `log` crate macros (`info!`, `warn!`, `debug!`).
  - Python: Use `pyo3-log` so Rust logs appear in standard Python logging streams.
- **Errors:**
  - Error messages must be descriptive (e.g., "Matrix is not positive definite" vs. "Math error").

---

## 13. Documentation Synchronization

- **Rust Docs:** Update `///` documentation comments for any changed public API.
- **Python Docs:** Update `"""Docstrings"""` in the PyO3 wrappers.
- **Stubs:** `*.pyi` files are the source of truth for the user's IDE. They must exactly match the implementation. Regenerate with `cargo run --bin stub_gen` in `python/`.

---

## 14. Multi-Agent Coordination Rules

- **Cross-Language Atomicity:** A PR that adds a feature to Rust but fails to expose it in Python (or vice-versa) is incomplete.
- **Stub First:** Define the desired Python API signature (in `.pyi` or pseudo-code) *before* implementing the Rust binding logic.

---

## 15. Change Risk Classification

- **Low Risk:** Doc string updates, adding a helper function in `utils`, adding a test, adding a benchmark.
- **Medium Risk:** Modifying an estimator's algorithm, updating a dependency version, adding a new MixedBN enum variant.
- **High Risk:** Changing `Cargo.toml` features, modifying `unsafe` blocks, changing the memory layout of core structs (`BN`, `CTBN`, `DiGraph`, `Phi`). — **REQUIRES EXTENSIVE TESTING.**

---

## 16. Development Workflow

1. **Write code** in the appropriate Rust backend (`causal-hub/src/`) or Python bridge (`python/src/`).
2. **Add tests** for all new logic in `causal-hub/tests/` (Rust) and/or `python/tests/` (Python).
3. **Run linters and formatters** before committing:
   - Rust: `cargo fmt --all` (formatting), `cargo clippy --workspace` (linting)
   - Python: `ruff check` and `ruff format`
4. **Run pre-commit** to verify consistency across all staged files:

   ```bash
   pre-commit run --all-files
   ```

   This enforces `clippy`, `ruff`, formatting, and other checks defined in `.pre-commit-config.yaml`.
5. **Run tests:**
   - Rust integration tests: `cargo test --test mod --features openblas-system`
   - Python tests: `uv run maturin develop && uv run pytest` (run from project root)
6. **Regenerate `.pyi` type stubs** if Python API changed:

   ```bash
   cd python && cargo run --bin stub_gen
   ```

7. **Commit** using conventional commit style that matches the repo.

---

## 17. Stop Conditions

**If any of the following occur, stop the current task and report the issue to the user before continuing:**

1. `cargo check` or `cargo clippy` produces errors you cannot resolve.
2. After a reasonable search, you cannot locate the Rust struct backing a Python class.
3. You find yourself reaching for `unwrap()` to work around a type error.
4. Pre-existing tests fail and the cause is unclear.
5. A change would require manually editing `Cargo.lock`.
6. A change would require adding `unsafe` code or relaxing `clippy`/`ruff` lint strictness.
