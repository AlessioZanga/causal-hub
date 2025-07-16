import numpy as np
import networkx as nx
from causal_hub import DiGraph


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
