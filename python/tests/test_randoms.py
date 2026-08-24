import networkx as nx
import pandas as pd
import pytest
from causal_hub import Error
from causal_hub.datasets import (
    CatIncTable,
    CatTable,
    CatTrj,
    CatTrjEv,
    CatTrjs,
    CatTrjsEv,
    GaussIncTable,
    GaussTable,
    MissingMechanism,
    MissingType,
)
from causal_hub.models import CatBN, CatCPD, DiGraph, GaussBN, GaussCPD


def test_digraph_random() -> None:
    """Test generating a random Directed Graph."""
    # Define vertices.
    vertices = ["A", "B", "C", "D"]
    # Generate a random graph.
    graph = DiGraph.random(vertices, p=0.5, seed=42)

    # Check the vertices.
    assert graph.vertices() == vertices, "Wrong vertices in the graph."


def test_digraph_random_dag() -> None:
    """Test generating a random Directed Acyclic Graph."""
    # Define vertices.
    vertices = ["A", "B", "C", "D"]
    # Generate a random graph.
    graph = DiGraph.random_dag(vertices, p=0.5, seed=42)

    # Check the vertices.
    assert graph.vertices() == vertices, "Wrong vertices in the graph."
    # Check if it is a DAG.
    assert nx.is_directed_acyclic_graph(graph.to_networkx()), "The graph is not a DAG."


def test_cat_cpd_random() -> None:
    """Test generating a random Categorical CPD."""
    # Define states and conditioning states.
    states = {"A": ("0", "1")}
    conditioning_support = {"B": ("0", "1"), "C": ("0", "1", "2")}
    # Generate a random CPD.
    cpd = CatCPD.random(states, conditioning_support, alpha=1.0, seed=42)

    # Check the states.
    assert cpd.support() == states, "Wrong states in the CPD."
    assert (
        cpd.conditioning_support() == conditioning_support
    ), "Wrong conditioning states in the CPD."


def test_gauss_cpd_random() -> None:
    """Test generating a random Gaussian CPD."""
    # Define labels and conditioning labels.
    labels = ["A", "B"]
    conditioning_labels = ["C", "D", "E"]
    # Generate a random CPD.
    cpd = GaussCPD.random(
        labels, conditioning_labels, s_a=1.0, s_b=1.0, e=1e-6, seed=42
    )

    # Check the labels.
    assert cpd.labels() == labels, "Wrong labels in the CPD."
    assert (
        cpd.conditioning_labels() == conditioning_labels
    ), "Wrong conditioning labels in the CPD."


def test_cat_bn_random() -> None:
    """Test generating a random Categorical BN."""
    # Define states.
    states = {"A": ("0", "1"), "B": ("0", "1"), "C": ("0", "1", "2")}
    # Generate a random BN.
    bn = CatBN.random(states, p=0.5, alpha=1.0, seed=42)

    # Check the vertices.
    assert sorted(bn.graph().vertices()) == sorted(
        states.keys()
    ), "Wrong vertices in the BN."


def test_gauss_bn_random() -> None:
    """Test generating a random Gaussian BN."""
    # Define labels.
    labels = ["A", "B", "C"]
    # Generate a random BN.
    bn = GaussBN.random(labels, p=0.5, s_a=1.0, s_b=1.0, e=1e-6, seed=42)

    # Check the vertices.
    assert sorted(bn.graph().vertices()) == sorted(labels), "Wrong vertices in the BN."

    # Invalid Parameters
    with pytest.raises(Error):
        GaussBN.random(labels, p=0.5, s_a=0.0, s_b=1.0, e=1e-6)
    with pytest.raises(Error):
        GaussBN.random(labels, p=0.5, s_a=1.0, s_b=0.0, e=1e-6)
    with pytest.raises(Error):
        GaussBN.random(labels, p=0.5, s_a=1.0, s_b=1.0, e=0.0)
    with pytest.raises(Error):
        GaussBN.random(labels, p=-0.1, s_a=1.0, s_b=1.0, e=1e-6)
    with pytest.raises(Error):
        GaussBN.random(labels, p=1.1, s_a=1.0, s_b=1.0, e=1e-6)


def test_cat_trj_ev_random() -> None:
    """Test generating a random Categorical Trajectory Evidence."""
    # Create a sample DataFrame with a time column and categorical columns.
    df = pd.DataFrame(
        {
            "time": [0, 1, 2, 3, 4],
            "column_1": ["A", "A", "B", "C", "C"],
            "column_2": ["X", "Y", "Y", "Y", "Z"],
        }
    )
    # Set data type for time column.
    df["time"] = df["time"].astype("float64")
    # Set data types for categorical columns.
    columns = list(set(df.columns) - {"time"})
    df[columns] = df[columns].astype("category")
    # Create a CatTrj object.
    trj = CatTrj.from_pandas(df)

    # Generate random evidence.
    evidence = CatTrjEv.random(trj, p=0.5, seed=42)

    # Check the labels.
    assert evidence.labels() == trj.labels(), "Wrong labels in the evidence."

    # Invalid Parameters
    with pytest.raises(Error):
        CatTrjEv.random(trj, p=-0.1)
    with pytest.raises(Error):
        CatTrjEv.random(trj, p=1.1)


def test_cat_trjs_ev_random() -> None:
    """Test generating random Categorical Trajectories Evidences."""
    # Create two sample DataFrames with a time column and categorical columns.
    dfs = [
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
        pd.DataFrame(
            {
                "time": [0, 1, 2, 3, 4],
                "column_1": ["A", "A", "B", "C", "C"],
                "column_2": ["X", "Y", "Y", "Y", "Z"],
            }
        ),
    ]
    # Set data type for time column and categorical columns.
    for df in dfs:
        df["time"] = df["time"].astype("float64")
        columns = list(set(df.columns) - {"time"})
        df[columns] = df[columns].astype("category")

    # Create a CatTrjs object.
    trjs = CatTrjs.from_pandas(dfs)

    # Generate random evidence.
    evidences = CatTrjsEv.random(trjs, p=0.5, seed=42)

    # Check the labels.
    assert evidences.labels() == trjs.labels(), "Wrong labels in the evidence."

    # Invalid Parameters
    with pytest.raises(Error):
        CatTrjsEv.random(trjs, p=-0.1)
    with pytest.raises(Error):
        CatTrjsEv.random(trjs, p=1.1)


def test_missing_mechanism_random() -> None:
    labels = ["X", "Y", "Z"]
    graph = DiGraph.empty(labels)
    graph.add_edge("X", "Y")
    graph.add_edge("Y", "Z")

    # Random MCAR
    mechanism = MissingMechanism.random(graph, MissingType.MCAR, 0.5, seed=42)
    assert isinstance(mechanism, MissingMechanism)
    assert mechanism.labels() == labels

    # Random MAR
    mechanism = MissingMechanism.random(graph, MissingType.MAR, 0.5, seed=42)
    assert isinstance(mechanism, MissingMechanism)
    assert mechanism.labels() == labels

    # Random MNAR
    mechanism = MissingMechanism.random(graph, MissingType.MNAR, 0.5, seed=42)
    assert isinstance(mechanism, MissingMechanism)
    assert mechanism.labels() == labels


def test_cat_inc_table_random() -> None:
    """Test generating a random Categorical Incomplete Table."""
    df = pd.DataFrame(
        {
            "A": ["0", "1", "0", "1"],
            "B": ["0", "1", "1", "0"],
        }
    ).astype("category")
    dataset = CatTable.from_pandas(df)
    graph = DiGraph.empty(dataset.labels())
    graph.add_edge("A", "B")
    mechanism = MissingMechanism.random(graph, MissingType.MAR, 0.5, seed=42)

    # Valid Generation
    sample = CatIncTable.random(dataset, mechanism, p_min=0.1, p_max=0.2, seed=42)
    assert sample.labels() == dataset.labels()
    assert sample.support() == dataset.support()

    # Invalid Parameters
    with pytest.raises(Error):
        CatIncTable.random(dataset, mechanism, p_min=-0.1, p_max=0.2)
    with pytest.raises(Error):
        CatIncTable.random(dataset, mechanism, p_min=1.1, p_max=0.2)
    with pytest.raises(Error):
        CatIncTable.random(dataset, mechanism, p_min=0.1, p_max=-0.1)
    with pytest.raises(Error):
        CatIncTable.random(dataset, mechanism, p_min=0.1, p_max=1.2)
    with pytest.raises(Error):
        CatIncTable.random(dataset, mechanism, p_min=0.5, p_max=0.2)


def test_gauss_inc_table_random() -> None:
    """Test generating a random Gaussian Incomplete Table."""
    df = pd.DataFrame(
        {
            "A": [0.0, 1.0, 2.0, 3.0],
            "B": [0.5, 1.5, 2.5, 3.5],
        }
    )
    dataset = GaussTable.from_pandas(df)
    graph = DiGraph.empty(dataset.labels())
    graph.add_edge("A", "B")
    mechanism = MissingMechanism.random(graph, MissingType.MAR, 0.5, seed=42)

    # Valid Generation
    sample = GaussIncTable.random(dataset, mechanism, p_min=0.1, p_max=0.2, seed=42)
    assert sample.labels() == dataset.labels()

    # Invalid Parameters
    with pytest.raises(Error):
        GaussIncTable.random(dataset, mechanism, p_min=-0.1, p_max=0.2)
    with pytest.raises(Error):
        GaussIncTable.random(dataset, mechanism, p_min=1.1, p_max=0.2)
    with pytest.raises(Error):
        GaussIncTable.random(dataset, mechanism, p_min=0.1, p_max=-0.1)
    with pytest.raises(Error):
        GaussIncTable.random(dataset, mechanism, p_min=0.1, p_max=1.2)
    with pytest.raises(Error):
        GaussIncTable.random(dataset, mechanism, p_min=0.5, p_max=0.2)
