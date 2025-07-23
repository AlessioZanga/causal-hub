import numpy as np
import networkx as nx
from causal_hub import DiGraph, load_asia


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
    # Create an adjacency matrix for the directed graph.
    adjacency_matrix = np.zeros((len(vertices), len(vertices)), dtype=int)
    # Fill the adjacency matrix based on the edges.
    for edge in edges:
        i = vertices.index(edge[0])
        j = vertices.index(edge[1])
        adjacency_matrix[i, j] = 1

    # Create a DiGraph.
    graph = DiGraph.from_adjacency_matrix(vertices, adjacency_matrix)

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
        assert not non_de_x or g.is_separator_set([x], non_de_x, pa_x), \
            f"Vertex {x} is not d-separated from its non-descendants given its parents."
