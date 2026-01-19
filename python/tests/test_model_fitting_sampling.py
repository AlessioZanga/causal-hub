import numpy as np
import pandas as pd
from causal_hub.datasets import CatTable, CatTrjs, GaussTable
from causal_hub.models import CatBN, CatCTBN, DiGraph, GaussBN


def test_cat_bn_fit_sample() -> None:
    """Test fitting and sampling for Categorical Bayesian Network."""
    # 1. Create Data
    # 2 variables A, B. A->B.
    # A ~ unif("0", "1")
    # B = A
    # If A=0, B=0. If A=1, B=1.
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
    # Shape A->B, |B|=2, |A|=2 => (2, 2)
    # Ordering of states is alphabetical/sorted usually: "0", "1".
    # Col 0 (A="0"): P(B="0"|A="0")=1, P(B="1"|A="0")=0
    # Col 1 (A="1"): P(B="0"|A="1")=0, P(B="1"|A="1")=1
    # Expected: [[1, 0], [0, 1]]
    expected_b = np.array([[1.0, 0.0], [0.0, 1.0]])
    np.testing.assert_allclose(params_b, expected_b, atol=0.1)

    # 4. Sample
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42)

    assert isinstance(sampled_data, CatTable)
    assert sampled_data.sample_size() == n_samples
    assert set(sampled_data.labels()) == {"A", "B"}

    # Check if samples respect the perfect correlation approx
    sdf = sampled_data.to_pandas()
    # Check consistency
    # With MLE and perfect data, P(B=x|A=x)=1.
    # So sampled data should also be perfect copies.
    assert (sdf["A"] == sdf["B"]).all()


def test_gauss_bn_fit_sample() -> None:
    """Test fitting and sampling for Gaussian Bayesian Network."""
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
    # Y = 2*X + eps.
    # Coeffs: [2.0]
    # Intercept: 0.0
    # Covariance: 0.01

    coeffs = params_y["coefficients"]
    intercept = params_y["intercept"]
    cov = params_y["covariance"]

    np.testing.assert_allclose(coeffs, [[2.0]], atol=0.2)
    np.testing.assert_allclose(intercept, [0.0], atol=0.2)
    np.testing.assert_allclose(cov, [[0.01]], atol=0.05)

    # 4. Sample
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42)

    assert isinstance(sampled_data, GaussTable)
    assert sampled_data.sample_size() == n_samples
    assert set(sampled_data.labels()) == {"X", "Y"}

    # Check basic statistics in sampled data
    sdf = sampled_data.to_pandas()
    # Correlation should be high
    corr = sdf.corr().loc["X", "Y"]
    assert corr > 0.9


def test_cat_ctbn_fit_sample() -> None:
    """Test fitting and sampling for Categorical Continuous Time Bayesian Network."""
    # 1. Create Data (Trajectories)
    # 1 Trajectory, 2 params
    # Simple case: constant states for some time
    # Just ensure it runs for now

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

    # 4. Sample
    # Sample 2 trajectories, max time 10.0
    sampled = model.sample(n=2, max_time=5.0, seed=42)

    assert isinstance(sampled, CatTrjs)
    # Convert to pandas to check
    sdfs = sampled.to_pandas()
    assert len(sdfs) == 2
    for df in sdfs:
        assert "A" in df.columns
        assert "B" in df.columns
        assert "time" in df.columns
        assert df["time"].max() <= 5.0
