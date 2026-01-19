import pandas as pd
from causal_hub.datasets import CatTrjsEv
from causal_hub.estimators import PK, em, sem
from causal_hub.models import DiGraph


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
