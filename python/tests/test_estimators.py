import numpy as np
import pandas as pd
from causal_hub.datasets import CatTable, CatTrjs, CatTrjsEv, GaussTable
from causal_hub.estimators import PK, em, sem
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
    model = CatBN.fit(dataset, graph, method="mle")

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
    model = GaussBN.fit(dataset, graph, method="mle")

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
    model = CatCTBN.fit(dataset, graph, method="mle")
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
    # sem(evidence, prior_knowledge, algorithm, max_iter, seed, kwargs)
    # Algorithm "cthc" (Continuous Time Hill Climbing)
    # kwargs might refer to estimator args or search args.
    # Usually "score" is needed for HC.
    result = sem(evidence, pk, "cthc", max_iter=2, seed=42, score="BIC")

    assert isinstance(result, dict)
    assert "models" in result
    assert "expectations" in result
    assert len(result["models"]) > 0


def test_prior_knowledge() -> None:
    """Test PriorKnowledge object creation."""
    labels = ["X", "Y", "Z"]
    forbidden = [("X", "Y")]
    required = [("Y", "Z")]
    temporal_order = [["X", "Y"], ["Z"]]  # X,Y before Z

    pk = PK(labels, forbidden, required, temporal_order)

    assert pk is not None
