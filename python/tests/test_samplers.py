import numpy as np
import pandas as pd
from causal_hub.datasets import CatTable, CatTrjs, GaussTable
from causal_hub.models import CatBN, CatCTBN, DiGraph, GaussBN


def test_cat_bn_sample() -> None:
    """Test sampling for Categorical Bayesian Network."""
    # 1. Create Data for fitting (or load one)
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()
    df = pd.DataFrame({"A": a, "B": b}).astype("category")
    dataset = CatTable.from_pandas(df)

    # 2. Define Structure
    graph = DiGraph.empty(["A", "B"])
    graph.add_edge("A", "B")

    # 3. Fit Model
    model = CatBN.fit(dataset, graph, method="mle")

    # 4. Sample
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42)

    assert isinstance(sampled_data, CatTable)
    assert sampled_data.sample_size() == n_samples
    assert set(sampled_data.labels()) == {"A", "B"}

    # Check if samples respect the perfect correlation approx
    sdf = sampled_data.to_pandas()
    assert (sdf["A"] == sdf["B"]).all()


def test_gauss_bn_sample() -> None:
    """Test sampling for Gaussian Bayesian Network."""
    # 1. Create Data
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

    # 4. Sample
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42)

    assert isinstance(sampled_data, GaussTable)
    assert sampled_data.sample_size() == n_samples
    assert set(sampled_data.labels()) == {"X", "Y"}

    # Check basic statistics in sampled data
    sdf = sampled_data.to_pandas()
    corr = sdf.corr().loc["X", "Y"]
    assert corr > 0.9


def test_cat_ctbn_sample() -> None:
    """Test sampling for Categorical Continuous Time Bayesian Network."""
    # 1. Create Data (Trajectories)
    dfs = []
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

    # 4. Sample
    sampled = model.sample(n=2, max_time=5.0, seed=42)

    assert isinstance(sampled, CatTrjs)
    sdfs = sampled.to_pandas()
    assert len(sdfs) == 2
    for df in sdfs:
        assert "A" in df.columns
        assert "B" in df.columns
        assert "time" in df.columns
        assert df["time"].max() <= 5.0
