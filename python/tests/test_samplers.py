import numpy as np
import pandas as pd
from causal_hub.assets import load_eating
from causal_hub.datasets import CatTable, CatTrjs, GaussTable
from causal_hub.estimators import EstimatorMethod
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
    model = CatBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)

    # 4. Sample
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42)

    assert isinstance(sampled_data, CatTable)
    assert sampled_data.sample_size() == n_samples
    assert set(sampled_data.labels()) == {"A", "B"}

    # Check if samples respect the perfect correlation approx
    sdf = sampled_data.to_pandas()
    assert (sdf["A"] == sdf["B"]).all()


def test_cat_bn_sample_sequential() -> None:
    """Test sequential sampling for Categorical Bayesian Network."""
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    df = pd.DataFrame({"A": a}).astype("category")
    dataset = CatTable.from_pandas(df)
    graph = DiGraph.empty(["A"])
    model = CatBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42, parallel=False)
    assert isinstance(sampled_data, CatTable)
    assert sampled_data.sample_size() == n_samples


def test_cat_bn_sample_parallel_sequential_consistency() -> None:
    """Test that parallel and sequential sampling both produce valid results."""
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()
    df = pd.DataFrame({"A": a, "B": b}).astype("category")
    dataset = CatTable.from_pandas(df)
    graph = DiGraph.empty(["A", "B"])
    graph.add_edge("A", "B")
    model = CatBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)
    n_samples = 200
    par = model.sample(n=n_samples, seed=42, parallel=True)
    seq = model.sample(n=n_samples, seed=42, parallel=False)
    assert par.sample_size() == n_samples
    assert seq.sample_size() == n_samples
    assert set(par.labels()) == set(seq.labels())


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
    model = GaussBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)

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


def test_gauss_bn_sample_sequential() -> None:
    """Test sequential sampling for Gaussian Bayesian Network."""
    size = 100
    x = np.random.normal(0, 1, size)
    df = pd.DataFrame({"X": x})
    dataset = GaussTable.from_pandas(df)
    graph = DiGraph.empty(["X"])
    model = GaussBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)
    n_samples = 50
    sampled_data = model.sample(n=n_samples, seed=42, parallel=False)
    assert isinstance(sampled_data, GaussTable)
    assert sampled_data.sample_size() == n_samples


def test_gauss_bn_sample_parallel_sequential_consistency() -> None:
    """Test parallel/sequential consistency for Gaussian BN."""
    size = 200
    x = np.random.normal(0, 1, size)
    y = 2 * x + np.random.normal(0, 0.1, size)
    df = pd.DataFrame({"X": x, "Y": y})
    dataset = GaussTable.from_pandas(df)
    graph = DiGraph.empty(["X", "Y"])
    graph.add_edge("X", "Y")
    model = GaussBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)
    n_samples = 100
    par = model.sample(n=n_samples, seed=42, parallel=True)
    seq = model.sample(n=n_samples, seed=42, parallel=False)
    assert par.sample_size() == n_samples
    assert seq.sample_size() == n_samples
    assert set(par.labels()) == set(seq.labels())


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
    model = CatCTBN.fit(dataset, graph, estimator_method=EstimatorMethod.MLE)

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


def test_cat_ctbn_sample_by_length() -> None:
    """Test CTBN sampling with max_len."""
    eating = load_eating()
    sampled = eating.sample(n=2, max_len=5, seed=42)
    assert isinstance(sampled, CatTrjs)
    for trj_df in sampled.to_pandas():
        assert len(trj_df) <= 5


def test_cat_ctbn_sample_sequential() -> None:
    """Test sequential CTBN sampling."""
    from causal_hub.assets import load_eating

    eating = load_eating()
    sampled = eating.sample(n=2, max_time=5.0, seed=42, parallel=False)
    assert isinstance(sampled, CatTrjs)
    assert len(sampled.to_pandas()) == 2
