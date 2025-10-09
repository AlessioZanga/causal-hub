import tempfile

import networkx as nx
import numpy as np

from causal_hub.assets import load_asia, load_eating, load_ecoli70
from causal_hub.models import CatBN, CatCTBN, DiGraph, GaussBN


def test_digraph_from_networkx() -> None:
    # Define vertices and edges for a simple directed graph.
    vertices = ["A", "B", "C", "D"]
    edges = [("A", "B"), ("B", "C"), ("C", "D")]

    # Create a simple directed graph using NetworkX.
    G = nx.DiGraph()
    G.add_edges_from(edges)

    # Convert the NetworkX graph to a DiGraph.
    graph = DiGraph.from_networkx(G)

    # Check the vertices and edges.
    assert graph.vertices() == vertices, "Wrong vertices in the graph."
    assert graph.edges() == edges, "Wrong edges in the graph."


def test_digraph_to_networkx() -> None:
    # Define vertices and edges for a simple directed graph.
    vertices = ["A", "B", "C", "D"]
    edges = [("A", "B"), ("B", "C"), ("C", "D")]

    # Create a simple directed graph using NetworkX.
    G = nx.DiGraph()
    G.add_edges_from(edges)
    # Create a DiGraph.
    graph = DiGraph.from_networkx(G)

    # Convert the DiGraph to a NetworkX graph.
    G = graph.to_networkx()

    # Check the vertices and edges in the NetworkX graph.
    assert list(G.nodes) == vertices, "Wrong vertices in the NetworkX graph."
    assert list(G.edges) == edges, "Wrong edges in the NetworkX graph."


def test_digraph_graphical_separation() -> None:
    # Get the BN from the assets.
    bn = load_asia()
    # Get the graph from the BN.
    g = bn.graph()
    # Get the vertices of the graph.
    v = set(g.vertices())
    # For each vertex ...
    for x in v:
        # Get the parents of the vertex.
        pa_x = set(g.parents(x))
        # Get the descendants of the vertex.
        de_x = set(g.descendants(x))
        # Get the non-descendants of the vertex: V - De(x) - Pa(x) - {x}.
        non_de_x = v - de_x - pa_x - {x}
        # Assert every vertex is d-separated from its non-descendants given its parents.
        assert not non_de_x or g.is_separator_set(
            [x], non_de_x, pa_x
        ), f"Vertex {x} is not d-separated from its non-descendants given its parents."


def test_asia() -> None:
    # Load the Asia BN.
    asia = load_asia()
    # Get the graph of the BN.
    graph = asia.graph()

    # Check the name.
    assert asia.name() == "asia", "Wrong name."
    # Check the description.
    assert asia.description() == None, "Wrong description."
    # Check the vertices labels.
    vertices = ["asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_asia_sample() -> None:
    # Load the Asia BN.
    asia = load_asia()
    # Sample 1000 data points from the BN.
    sample = asia.sample(1000, seed=42)

    # Check the labels of the sample.
    labels = ["asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"]
    assert sample.labels() == labels, "Wrong sample labels."
    # Check the shape of the sample.
    assert sample.values().shape == (1000, 8), "Wrong sample shape."
    # Check the sample size.
    assert sample.sample_size() == 1000, "Wrong sample size."


def test_asia_fit() -> None:
    # Load the Asia BN.
    asia = load_asia()
    # Sample 1000 data points from the BN.
    sample = asia.sample(1000, seed=42)
    # Fit a new BN to the sample.
    asia_fitted = CatBN.fit(sample, asia.graph(), method="be")

    # Check the labels of the fitted BN.
    assert asia_fitted.labels() == asia.labels(), "Wrong fitted BN labels."
    # Check the graph of the fitted BN.
    assert asia_fitted.graph() == asia.graph(), "Wrong fitted BN graph."


def test_asia_read_write_json() -> None:
    # Load the Asia BN.
    asia = load_asia()

    # Get a named temp file for the JSON.
    path = tempfile.NamedTemporaryFile().name
    # Write to a JSON file.
    asia.write_json(path)
    # Read from the JSON file.
    asia_from_json = CatBN.read_json(path)

    # Check the labels after read/write.
    assert asia.labels() == asia_from_json.labels(), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert asia.graph() == asia_from_json.graph(), "Wrong graph after read/write."
    # Check the CPDs after read/write.
    assert asia.cpds() == asia_from_json.cpds(), "Wrong CPDs after read/write."


def test_ecoli70() -> None:
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Get the graph of the BN.
    graph = ecoli70.graph()

    # Check the name.
    assert ecoli70.name() == "ecoli70", "Wrong name."
    # Check the description.
    assert ecoli70.description() == None, "Wrong description."
    # Check the vertices labels.
    vertices = [
        "aceB",
        "asnA",
        "atpD",
        "atpG",
        "b1191",
        "b1583",
        "b1963",
        "cchB",
        "cspA",
        "cspG",
        "dnaG",
        "dnaJ",
        "dnaK",
        "eutG",
        "fixC",
        "flgD",
        "folK",
        "ftsJ",
        "gltA",
        "hupB",
        "ibpB",
        "icdA",
        "lacA",
        "lacY",
        "lacZ",
        "lpdA",
        "mopB",
        "nmpC",
        "nuoM",
        "pspA",
        "pspB",
        "sucA",
        "sucD",
        "tnaA",
        "yaeM",
        "yceP",
        "ycgX",
        "yecO",
        "yedE",
        "yfaD",
        "yfiA",
        "ygbD",
        "ygcE",
        "yhdM",
        "yheI",
        "yjbO",
    ]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_ecoli70_sample() -> None:
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Sample 1000 data points from the BN.
    sample = ecoli70.sample(1000, seed=42)

    # Check the labels of the sample.
    labels = [
        "aceB",
        "asnA",
        "atpD",
        "atpG",
        "b1191",
        "b1583",
        "b1963",
        "cchB",
        "cspA",
        "cspG",
        "dnaG",
        "dnaJ",
        "dnaK",
        "eutG",
        "fixC",
        "flgD",
        "folK",
        "ftsJ",
        "gltA",
        "hupB",
        "ibpB",
        "icdA",
        "lacA",
        "lacY",
        "lacZ",
        "lpdA",
        "mopB",
        "nmpC",
        "nuoM",
        "pspA",
        "pspB",
        "sucA",
        "sucD",
        "tnaA",
        "yaeM",
        "yceP",
        "ycgX",
        "yecO",
        "yedE",
        "yfaD",
        "yfiA",
        "ygbD",
        "ygcE",
        "yhdM",
        "yheI",
        "yjbO",
    ]
    assert sample.labels() == labels, "Wrong sample labels."
    # Check the shape of the sample.
    assert sample.values().shape == (1000, 46), "Wrong sample shape."
    # Check the sample size.
    assert sample.sample_size() == 1000, "Wrong sample size."


def test_ecoli70_fit() -> None:
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Sample 1000 data points from the BN.
    sample = ecoli70.sample(1000, seed=42)
    # Fit a new BN to the sample.
    ecoli70_fitted = GaussBN.fit(sample, ecoli70.graph())

    # Check the labels of the fitted BN.
    assert ecoli70_fitted.labels() == ecoli70.labels(), "Wrong fitted BN labels."
    # Check the graph of the fitted BN.
    assert ecoli70_fitted.graph() == ecoli70.graph(), "Wrong fitted BN graph."


def test_ecoli70_read_write_json() -> None:
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()

    # Get a named temp file for the JSON.
    path = tempfile.NamedTemporaryFile().name
    # Write to a JSON file.
    ecoli70.write_json(path)
    # Read from the JSON file.
    ecoli70_from_json = GaussBN.read_json(path)

    # Check the labels after read/write.
    assert (
        ecoli70.labels() == ecoli70_from_json.labels()
    ), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert ecoli70.graph() == ecoli70_from_json.graph(), "Wrong graph after read/write."
    # Check the CPDs after read/write.
    assert ecoli70.cpds() == ecoli70_from_json.cpds(), "Wrong CPDs after read/write."


def test_eating() -> None:
    # Load the Eating CTBN.
    eating = load_eating()
    # Get the graph of the CTBN.
    graph = eating.graph()

    # Check the name.
    assert eating.name() == "eating", "Wrong name."
    # Check the description.
    assert (
        eating.description()
        == 'See: U. Nodelman, C.R. Shelton, and D. Koller (2003). "Learning Continuous Time Bayesian Networks." Proc. Nineteenth Conference on Uncertainty in Artificial Intelligence (UAI) (pp. 451-458).'
    ), "Wrong description."
    # Check the vertices labels.
    vertices = [
        "Eating",
        "FullStomach",
        "Hungry",
    ]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_eating_sample() -> None:
    # Load the Eating CTBN.
    eating = load_eating()
    # Sample 1000 trajectories from the CTBN.
    sample = eating.sample(1000, max_time=10.0, seed=42)

    # Check the labels of the sample.
    labels = [
        "Eating",
        "FullStomach",
        "Hungry",
    ]
    assert sample.labels() == labels, "Wrong sample labels."


def test_eating_fit() -> None:
    # Load the Eating CTBN.
    eating = load_eating()
    # Sample 1000 trajectories from the CTBN.
    sample = eating.sample(1000, max_time=10.0, seed=42)
    # Fit a new CTBN to the sample.
    eating_fitted = CatCTBN.fit(sample, eating.graph(), method="be")

    # Check the labels of the fitted CTBN.
    assert eating_fitted.labels() == eating.labels(), "Wrong fitted CTBN labels."
    # Check the graph of the fitted CTBN.
    assert eating_fitted.graph() == eating.graph(), "Wrong fitted CTBN graph."


def test_eating_read_write_json() -> None:
    # Load the Eating CTBN.
    eating = load_eating()

    # Get a named temp file for the JSON.
    path = tempfile.NamedTemporaryFile().name
    # Write to a JSON file.
    eating.write_json(path)
    # Read from the JSON file.
    eating_from_json = CatCTBN.read_json(path)

    # Check the labels after read/write.
    assert (
        eating.labels() == eating_from_json.labels()
    ), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert eating.graph() == eating_from_json.graph(), "Wrong graph after read/write."
    # Check the CIMs after read/write.
    assert eating.cims() == eating_from_json.cims(), "Wrong CIMs after read/write."
