import numpy as np
import pandas as pd
import pytest
from causal_hub.assets import load_eating
from causal_hub.datasets import (
    CatIncTable,
    CatTable,
    CatTrjs,
    CatTrjsEv,
    CatWtdTable,
    CatWtdTrjs,
    GaussIncTable,
    GaussTable,
    MissingMechanism,
)
from causal_hub.datasets import MissingMethod as MM
from causal_hub.datasets import MissingType as MT
from causal_hub.estimators import (
    PK,
    EstimatorMethod,
    ScorerMethod,
    cthc,
    ctpc,
    em,
    hc,
    sem,
)
from causal_hub.models import CatBN, CatCTBN, DiGraph, GaussBN


def test_cat_bn_fit() -> None:
    """Test fitting for Categorical Bayesian Network."""
    # 1. Create Data
    # 2 variables A, B. A->B.
    # A ~ unif("0", "1")
    # B = A
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    # 2. Define Structure
    graph = DiGraph.empty(["A", "B"])
    graph.add_edge("A", "B")

    # 3. Fit Model
    model = CatBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)

    assert isinstance(model, CatBN)
    assert set(model.labels()) == {"A", "B"}

    # Check fitted parameters
    cpds = model.cpds()
    params_b = cpds["B"].parameters()
    # Expected: [[1, 0], [0, 1]]
    expected_b = np.array([[1.0, 0.0], [0.0, 1.0]])
    np.testing.assert_allclose(params_b, expected_b, atol=0.1)


def test_gauss_bn_fit() -> None:
    """Test fitting for Gaussian Bayesian Network."""
    # 1. Create Data
    # X ~ N(0, 1)
    # Y = 2*X + N(0, 0.01)
    size = 200
    x = np.random.normal(0, 1, size)
    y = 2 * x + np.random.normal(0, 0.1, size)

    df = pd.DataFrame({"X": x, "Y": y})
    dataset = GaussTable.from_pandas(df)

    # 2. Define Structure
    graph = DiGraph.empty(["X", "Y"])
    graph.add_edge("X", "Y")

    # 3. Fit
    model = GaussBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)

    assert isinstance(model, GaussBN)

    # Check fitted parameters
    cpds = model.cpds()
    params_y = cpds["Y"].parameters()

    coeffs = params_y["coefficients"]
    intercept = params_y["intercept"]
    cov = params_y["covariance"]

    np.testing.assert_allclose(coeffs, [[2.0]], atol=0.2)
    np.testing.assert_allclose(intercept, [0.0], atol=0.2)
    np.testing.assert_allclose(cov, [[0.01]], atol=0.05)


def test_cat_ctbn_fit() -> None:
    """Test fitting for Categorical Continuous Time Bayesian Network."""
    # 1. Create Data (Trajectories)
    dfs = []
    # 5 trajectories
    for i in range(5):
        if i % 2 == 0:
            # Type 1
            df = pd.DataFrame(
                {
                    "time": [0.0, 1.0, 2.0, 3.0, 4.0],
                    "A": ["0", "1", "1", "0", "0"],
                    "B": ["0", "0", "1", "1", "0"],
                }
            )
        else:
            # Type 2: B flips while A is constant, then A flips
            df = pd.DataFrame(
                {
                    "time": [0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                    "A": ["0", "0", "0", "1", "1", "1"],
                    "B": ["0", "1", "0", "0", "1", "0"],
                }
            )

        # Set types
        df["time"] = df["time"].astype("float64")
        df["A"] = df["A"].astype("category")
        df["B"] = df["B"].astype("category")
        dfs.append(df)

    dataset = CatTrjs.from_pandas(dfs)

    # 2. Graph
    graph = DiGraph.empty(["A", "B"])
    graph.add_edge("A", "B")

    # 3. Fit
    model = CatCTBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)
    assert isinstance(model, CatCTBN)


def test_parameter_learning_em() -> None:
    """Test parameter learning using Expectation-Maximization (EM)."""
    # Create evidence data
    # 2 variables: A, B. States: A={0,1}, B={0,1}
    # Trajectory 1:
    # A=0 from 0 to 1
    # B=0 from 0 to 1
    df1 = pd.DataFrame(
        {
            "event": ["A", "B", "A", "B"],
            "state": ["0", "0", "1", "1"],
            "start_time": [0.0, 0.0, 1.0, 1.0],
            "end_time": [1.0, 1.0, 2.0, 2.0],
        }
    )

    # Ensure types
    df1["start_time"] = df1["start_time"].astype("float64")
    df1["end_time"] = df1["end_time"].astype("float64")

    # Needs explicit states to ensure consistency
    states = {
        "A": ("0", "1"),
        "B": ("0", "1"),
    }

    # Create CatTrjsEv
    evidence = CatTrjsEv.from_pandas([df1], with_states=states)

    # Create Initial Graph
    graph = DiGraph.empty(["A", "B"])
    graph.add_edge("A", "B")

    # Call EM
    # em(evidence, graph, max_iter=2, seed=42)
    # It returns a dict of params
    result = em(evidence, graph, max_iter=2, seed=42)

    assert isinstance(result, dict)
    # Check keys
    assert "models" in result
    assert "expectations" in result
    assert len(result["models"]) > 0
    assert len(result["expectations"]) > 0

    # Exercise weighted trajectory wrappers.
    exp0 = result["expectations"][0]
    assert isinstance(exp0, CatWtdTrjs)
    assert exp0.labels() == ["A", "B"]
    assert set(exp0.support().keys()) == {"A", "B"}

    weighted_trjs = exp0.values()
    assert len(weighted_trjs) > 0
    w0 = weighted_trjs[0]
    assert w0.labels() == ["A", "B"]
    assert set(w0.support().keys()) == {"A", "B"}
    assert w0.weight() > 0.0

    trj0 = w0.trajectory()
    assert trj0.labels() == ["A", "B"]
    assert trj0.values().shape[1] == 2
    np.testing.assert_array_equal(trj0.times(), w0.times())


def test_structure_learning_sem() -> None:
    """Test structure learning using Structural Expectation-Maximization (SEM)."""
    # Reuse evidence setup
    df1 = pd.DataFrame(
        {
            "event": ["A", "B"],
            "state": ["0", "0"],
            "start_time": [0.0, 0.0],
            "end_time": [1.0, 1.0],
        }
    )
    df1["start_time"] = df1["start_time"].astype("float64")
    df1["end_time"] = df1["end_time"].astype("float64")

    states = {
        "A": ("0", "1"),
        "B": ("0", "1"),
    }
    evidence = CatTrjsEv.from_pandas([df1], with_states=states)

    # Create Prior Knowledge
    labels = ["A", "B"]
    forbidden = []
    required = []
    temporal_order = []

    pk = PK(labels, forbidden, required, temporal_order)

    # Call SEM
    # sem(evidence, algorithm, max_iter, seed, f_test, c_test, kwargs)
    # Algorithm "cthc" (Continuous Time Hill Climbing)
    # kwargs might refer to estimator args or search args.
    # Usually "score" is needed for HC.
    result = sem(evidence, "cthc", max_iter=2, seed=42, prior_knowledge=pk)

    assert isinstance(result, dict)
    assert "models" in result
    assert "expectations" in result
    assert len(result["models"]) > 0
    assert len(result["expectations"]) > 0

    # Exercise weighted trajectory wrappers.
    sem_step0 = result["expectations"][0]
    assert isinstance(sem_step0, dict)
    assert "expectations" in sem_step0
    assert len(sem_step0["expectations"]) > 0

    exp0 = sem_step0["expectations"][0]
    assert isinstance(exp0, CatWtdTrjs)
    assert exp0.labels() == ["A", "B"]
    assert set(exp0.support().keys()) == {"A", "B"}

    weighted_trjs = exp0.values()
    assert len(weighted_trjs) > 0
    w0 = weighted_trjs[0]
    assert w0.labels() == ["A", "B"]
    assert w0.weight() > 0.0


def test_structure_learning_hc() -> None:
    """Test structure learning using Hill Climbing (HC)."""
    # 1. Create Data
    # 2 variables A, B. A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    # 2. Define Prior Knowledge
    pk = PK(["A", "B"], [], [], [])

    # 3. Learn Structure
    fitted_model = hc(dataset, prior_knowledge=pk)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"A", "B"}
    # The learned edge is Markov-equivalent in both directions.
    assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_hc_no_prior_knowledge() -> None:
    """Test structure learning using Hill Climbing (HC) without prior knowledge."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    fitted_model = hc(dataset)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"A", "B"}
    assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_hc_initial_graph() -> None:
    """Test structure learning using Hill Climbing (HC) with an initial graph."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    initial_graph = DiGraph.empty(["A", "B"])

    fitted_model = hc(dataset, initial_graph=initial_graph)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"A", "B"}
    assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_hc_initial_graph_mismatch() -> None:
    """Test that HC raises an error for an initial graph with mismatching labels."""  # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    initial_graph = DiGraph.empty(["A", "C"])

    with pytest.raises(Exception) as excinfo:
        hc(dataset, initial_graph=initial_graph)
    assert "label" in str(excinfo.value).lower()


def test_structure_learning_hc_prior_knowledge() -> None:
    """Test structure learning using Hill Climbing (HC) with prior knowledge."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    # Forbid the edge A -> B.
    pk = PK(["A", "B"], [("A", "B")], [], [])

    fitted_model = hc(dataset, prior_knowledge=pk, max_parents=1)

    assert isinstance(fitted_model, CatBN)
    assert not fitted_model.graph().has_edge("A", "B")
    # Every vertex has at most one parent.
    for v in fitted_model.graph().vertices():
        assert len(fitted_model.graph().parents(v)) <= 1


def test_structure_learning_hc_invalid_dataset() -> None:
    """Test that HC raises an error for unsupported dataset types."""
    df = pd.DataFrame(
        {
            "time": [0.0, 1.0, 2.0],
            "A": ["0", "1", "1"],
            "B": ["0", "0", "0"],
        }
    )
    df["time"] = df["time"].astype("float64")
    df["A"] = df["A"].astype("category")
    df["B"] = df["B"].astype("category")

    dataset = CatTrjs.from_pandas([df])
    pk = PK(["A", "B"], [], [], [])

    with pytest.raises(TypeError) as excinfo:
        hc(dataset, prior_knowledge=pk)
    assert "CatTable" in str(excinfo.value) or "GaussTable" in str(excinfo.value)


def test_structure_learning_hc_incomplete_dataset() -> None:
    """Test structure learning using Hill Climbing (HC) on an incomplete dataset."""
    states = {"X": ["0", "1"], "Y": ["0", "1"]}
    model = CatBN.random(states, p=0.5, seed=42)

    cat_table = model.sample(50, seed=42)
    mechanism = MissingMechanism.random(model.graph(), MT.MCAR, 1.0, seed=42)
    inc_table = CatIncTable.random(cat_table, mechanism, 0.1, 0.5, seed=42)

    fitted_model = hc(inc_table)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"X", "Y"}


def test_structure_learning_hc_weighted_dataset() -> None:
    """Test structure learning using Hill Climbing (HC) on a weighted dataset."""
    # A = B.
    size = 50
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    cat_table = CatTable.from_pandas(df)
    dataset = CatWtdTable(cat_table, np.ones(size))

    fitted_model = hc(dataset)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"A", "B"}
    assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_hc_gauss() -> None:
    """Test structure learning using Hill Climbing (HC) for Gaussian BNs."""
    # X ~ N(0, 1), Y = 2*X + noise.
    size = 200
    x = np.random.normal(0, 1, size)
    y = 2 * x + np.random.normal(0, 0.1, size)

    df = pd.DataFrame({"X": x, "Y": y})
    dataset = GaussTable.from_pandas(df)

    pk = PK(["X", "Y"], [], [], [])

    fitted_model = hc(dataset, prior_knowledge=pk)

    assert isinstance(fitted_model, GaussBN)
    assert set(fitted_model.graph().vertices()) == {"X", "Y"}
    assert set(fitted_model.graph().edges()) in [{("X", "Y")}, {("Y", "X")}]


def test_structure_learning_ctpc() -> None:
    """Test structure learning using Continuous Time Peter-Clark (CTPC)."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    pk = PK(model.labels(), [], [], [])

    fitted_model = ctpc(dataset, prior_knowledge=pk, f_test=0.01, c_test=0.01)

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    # CTPC recovers the true structure on this dataset.
    assert set(fitted_model.graph().edges()) == set(model.graph().edges())


def test_structure_learning_ctpc_no_prior_knowledge() -> None:
    """Test structure learning using Continuous Time Peter-Clark (CTPC) without prior knowledge."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    fitted_model = ctpc(dataset)

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    assert set(fitted_model.graph().edges()) == set(model.graph().edges())


def test_structure_learning_hc_scores() -> None:
    """Test structure learning using Hill Climbing (HC) with all scoring criteria."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    pk = PK(["A", "B"], [], [], [])

    for scorer_method in [
        ScorerMethod.LL,
        ScorerMethod.AIC,
        ScorerMethod.AICC,
        ScorerMethod.BIC,
        ScorerMethod.BICC,
        ScorerMethod.HQC,
    ]:
        fitted_model = hc(dataset, prior_knowledge=pk, scorer_method=scorer_method)

        assert isinstance(fitted_model, CatBN)
        assert set(fitted_model.graph().vertices()) == {"A", "B"}
        # The learned edge is Markov-equivalent in both directions.
        assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_hc_gauss_score() -> None:
    """Test structure learning using Hill Climbing (HC) with a scoring criterion for Gaussian BNs."""
    # X ~ N(0, 1), Y = 2*X + noise.
    size = 200
    x = np.random.normal(0, 1, size)
    y = 2 * x + np.random.normal(0, 0.1, size)

    df = pd.DataFrame({"X": x, "Y": y})
    dataset = GaussTable.from_pandas(df)

    pk = PK(["X", "Y"], [], [], [])

    fitted_model = hc(dataset, prior_knowledge=pk, scorer_method=ScorerMethod.AIC)

    assert isinstance(fitted_model, GaussBN)
    assert set(fitted_model.graph().vertices()) == {"X", "Y"}
    assert set(fitted_model.graph().edges()) in [{("X", "Y")}, {("Y", "X")}]


def test_structure_learning_hc_rejects_unknown_kwarg() -> None:
    """Test that HC rejects unknown keyword arguments with a TypeError."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    pk = PK(["A", "B"], [], [], [])

    with pytest.raises(TypeError) as excinfo:
        hc(dataset, prior_knowledge=pk, estimator=EstimatorMethod.MLE)
    assert "estimator" in str(excinfo.value)


def test_structure_learning_hc_invalid_score() -> None:
    """Test that HC raises an error for unsupported score types."""
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    pk = PK(["A", "B"], [], [], [])

    with pytest.raises(TypeError):
        hc(dataset, prior_knowledge=pk, scorer_method="BIC")


def test_structure_learning_cthc() -> None:
    """Test structure learning using Continuous Time Hill Climbing (CTHC)."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    pk = PK(model.labels(), [], [], [])

    fitted_model = cthc(dataset, prior_knowledge=pk, max_parents=2)

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    # Every vertex has at most two parents.
    for v in fitted_model.graph().vertices():
        assert len(fitted_model.graph().parents(v)) <= 2


def test_structure_learning_cthc_no_prior_knowledge() -> None:
    """Test structure learning using Continuous Time Hill Climbing (CTHC) without prior knowledge."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    fitted_model = cthc(dataset)

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())


def test_structure_learning_cthc_initial_graph() -> None:
    """Test structure learning using Continuous Time Hill Climbing (CTHC) with an initial graph."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    initial_graph = DiGraph.empty(model.labels())

    fitted_model = cthc(dataset, initial_graph=initial_graph, max_parents=2)

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    # Every vertex has at most two parents.
    for v in fitted_model.graph().vertices():
        assert len(fitted_model.graph().parents(v)) <= 2


def test_structure_learning_sequential() -> None:
    """Test structure learning running HC and CTHC sequentially."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    pk = PK(["A", "B"], [], [], [])

    fitted_model = hc(dataset, prior_knowledge=pk, parallel=False)

    assert isinstance(fitted_model, CatBN)
    assert set(fitted_model.graph().vertices()) == {"A", "B"}
    assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]

    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    trajectories = model.sample(100, max_len=100, seed=42)

    fitted_model = cthc(
        trajectories, prior_knowledge=PK(model.labels(), [], [], []), parallel=False
    )

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())


def test_structure_learning_cthc_score() -> None:
    """Test structure learning using Continuous Time Hill Climbing (CTHC) with a scoring criterion."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    pk = PK(model.labels(), [], [], [])

    fitted_model = cthc(
        dataset, prior_knowledge=pk, scorer_method=ScorerMethod.AICC, max_parents=2
    )

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    # Every vertex has at most two parents.
    for v in fitted_model.graph().vertices():
        assert len(fitted_model.graph().parents(v)) <= 2


def test_structure_learning_hc_estimator() -> None:
    """Test structure learning using Hill Climbing (HC) with different parameter estimators."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)

    pk = PK(["A", "B"], [], [], [])

    for estimator in [EstimatorMethod.MLE, EstimatorMethod.BE]:
        fitted_model = hc(dataset, prior_knowledge=pk, estimator_method=estimator)

        assert isinstance(fitted_model, CatBN)
        assert set(fitted_model.graph().vertices()) == {"A", "B"}
        assert set(fitted_model.graph().edges()) in [{("A", "B")}, {("B", "A")}]


def test_structure_learning_cthc_estimator() -> None:
    """Test structure learning using Continuous Time Hill Climbing (CTHC) with parameter estimators."""
    # Load the Eating model and sample trajectories from it.
    model = load_eating()
    dataset = model.sample(100, max_len=100, seed=42)

    pk = PK(model.labels(), [], [], [])

    fitted_model = cthc(
        dataset, prior_knowledge=pk, max_parents=2, estimator_method=EstimatorMethod.MLE
    )

    assert isinstance(fitted_model, CatCTBN)
    assert set(fitted_model.graph().vertices()) == set(model.labels())
    # Every vertex has at most two parents.
    for v in fitted_model.graph().vertices():
        assert len(fitted_model.graph().parents(v)) <= 2


def test_prior_knowledge() -> None:
    """Test PriorKnowledge object creation."""
    labels = ["X", "Y", "Z"]
    forbidden = [("X", "Y")]
    required = [("Y", "Z")]
    temporal_order = [["X", "Y"], ["Z"]]  # X,Y before Z

    pk = PK(labels, forbidden, required, temporal_order)

    assert pk is not None


@pytest.mark.parametrize("missing_type", [MT.MCAR, MT.MAR, MT.MNAR])
@pytest.mark.parametrize(
    "missing_method",
    [MM.LW, MM.PW, MM.IPW, MM.AIPW],
)
def test_cat_bn_missing_data_flow(missing_method, missing_type):
    """Test CatBN with all missing methods and mechanisms."""
    states = {"X": ["0", "1"], "Y": ["0", "1"], "Z": ["0", "1"]}
    model = CatBN.random(states, p=0.5, seed=42)
    graph = model.graph()

    cat_table = model.sample(500, seed=42)
    mechanism = MissingMechanism.random(graph, missing_type, 1.0, seed=42)
    inc_table = CatIncTable.random(cat_table, mechanism, 0.1, 0.5, seed=42)

    new_model = CatBN.fit(
        inc_table,
        graph,
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )
    assert new_model is not None

    est = new_model.estimate(
        ["Y"],
        ["X"],
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )
    assert est is not None

    new_model.do_estimate(
        ["X"],
        ["Y"],
        [],
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )


def test_cat_bn_invalid_mechanism_validation():
    """Test that passing a mechanism for LW/PW raises an error."""
    states = {"X": ["0", "1"], "Y": ["0", "1"]}
    model = CatBN.random(states, p=1.0, seed=42)
    graph = model.graph()
    cat_table = model.sample(10, seed=42)
    mechanism = MissingMechanism.random(graph, MT.MCAR, 1.0, seed=42)
    inc_table = CatIncTable.random(cat_table, mechanism, 0.1, 0.5, seed=42)

    with pytest.raises(Exception) as excinfo:
        CatBN.fit(
            inc_table,
            graph,
            missing_method=MM.LW,
            missing_mechanism=mechanism,
        )
    assert "must be None if missing_method is LW or PW" in str(excinfo.value)


@pytest.mark.parametrize("missing_type", [MT.MCAR, MT.MAR, MT.MNAR])
@pytest.mark.parametrize("missing_method", [MM.LW, MM.PW, MM.IPW, MM.AIPW])
def test_gauss_bn_missing_data_flow(missing_method, missing_type):
    """Test GaussBN with all missing methods and mechanisms."""
    labels = ["X", "Y", "Z"]
    model = GaussBN.random(labels, p=0.5, seed=42)
    labels = model.labels()
    graph = model.graph()

    gauss_table = model.sample(300, seed=42)
    mechanism = MissingMechanism.random(graph, missing_type, 1.0, seed=42)
    inc_table = GaussIncTable.random(gauss_table, mechanism, 0.1, 0.5, seed=42)

    new_model = GaussBN.fit(
        inc_table,
        graph,
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )
    assert new_model is not None

    est = new_model.estimate(
        ["Y"],
        ["X"],
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )
    assert est is not None

    new_model.do_estimate(
        ["X"],
        ["Y"],
        [],
        missing_method=missing_method,
        missing_mechanism=(mechanism if missing_method in [MM.IPW, MM.AIPW] else None),
    )


def test_gauss_bn_invalid_mechanism_validation():
    """Test that passing a mechanism for LW/PW raises an error."""
    labels = ["X", "Y", "Z"]
    model = GaussBN.random(labels, p=1.0, seed=42)
    graph = model.graph()
    gauss_table = model.sample(10, seed=42)
    mechanism = MissingMechanism.random(graph, MT.MCAR, 1.0, seed=42)
    inc_table = GaussIncTable.random(gauss_table, mechanism, 0.1, 0.5, seed=42)

    with pytest.raises(Exception) as excinfo:
        GaussBN.fit(
            inc_table,
            graph,
            missing_method=MM.LW,
            missing_mechanism=mechanism,
        )
    assert "must be None if missing_method is LW or PW" in str(excinfo.value)
