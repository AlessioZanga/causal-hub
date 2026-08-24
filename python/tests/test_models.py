import tempfile

import networkx as nx
import numpy as np
import pandas as pd
import pytest
from causal_hub.assets import (
    load_asia,
    load_cancer,
    load_earthquake,
    load_eating,
    load_ecoli70,
    load_sachs,
    load_survey,
)
from causal_hub.datasets import CatTable, CatTrjs, GaussIncTable, GaussTable
from causal_hub.estimators import (
    FitMethod,
    ParametersEstimator,
    StructureEstimator,
)
from causal_hub.models import CatBN, CatCTBN, DiGraph, GaussBN, UnGraph


def test_digraph_from_networkx() -> None:
    """Test creating a Directed Graph from NetworkX graph."""
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
    """Test converting a Directed Graph to NetworkX graph."""
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


def test_digraph_gml_round_trip() -> None:
    """Test GML read/write round-trip on a DiGraph."""
    # Define vertices and edges for a simple directed graph.
    vertices = ["A", "B", "C", "D"]
    edges = [("A", "B"), ("B", "C"), ("C", "D")]

    # Create a simple directed graph.
    graph = DiGraph.empty(vertices)
    for x, y in edges:
        graph.add_edge(x, y)

    # Serialize to a GML string and parse it back.
    gml = graph.to_gml_string()
    parsed = DiGraph.from_gml_string(gml)

    # Check the vertices and edges are preserved.
    assert parsed.vertices() == vertices, "Wrong vertices after GML round-trip."
    assert parsed.edges() == edges, "Wrong edges after GML round-trip."

    # Check file round-trip.
    with tempfile.NamedTemporaryFile(suffix=".gml") as tmp:
        path = tmp.name
        graph.to_gml_file(path)
        parsed = DiGraph.from_gml_file(path)
    assert parsed.vertices() == vertices, "Wrong vertices after GML file round-trip."
    assert parsed.edges() == edges, "Wrong edges after GML file round-trip."


def test_digraph_dot_round_trip() -> None:
    """Test DOT read/write round-trip on a DiGraph."""
    # Define vertices and edges for a simple directed graph.
    vertices = ["A", "B", "C", "D"]
    edges = [("A", "B"), ("B", "C"), ("C", "D")]

    # Create a simple directed graph.
    graph = DiGraph.empty(vertices)
    for x, y in edges:
        graph.add_edge(x, y)

    # Serialize to a DOT string and parse it back.
    dot = graph.to_dot_string()
    parsed = DiGraph.from_dot_string(dot)

    # Check the vertices and edges are preserved.
    assert parsed.vertices() == vertices, "Wrong vertices after DOT round-trip."
    assert parsed.edges() == edges, "Wrong edges after DOT round-trip."

    # Check file round-trip.
    with tempfile.NamedTemporaryFile(suffix=".dot") as tmp:
        path = tmp.name
        graph.to_dot_file(path)
        parsed = DiGraph.from_dot_file(path)
    assert parsed.vertices() == vertices, "Wrong vertices after DOT file round-trip."
    assert parsed.edges() == edges, "Wrong edges after DOT file round-trip."


def test_digraph_graphical_separation() -> None:
    """Test graphical separation (d-separation) on a known network (Asia)."""
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
    """Test loading and properties of the Asia network."""
    # Load the Asia BN.
    asia = load_asia()
    # Get the graph of the BN.
    graph = asia.graph()

    # Check the name.
    assert asia.name() == "asia", "Wrong name."
    # Check the description.
    assert asia.description() is None, "Wrong description."
    # Check the vertices labels.
    vertices = ["asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_asia_sample() -> None:
    """Test sampling from the Asia network."""
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
    """Test fitting the Asia network structure to sampled data."""
    # Load the Asia BN.
    asia = load_asia()
    # Sample 1000 data points from the BN.
    sample = asia.sample(1000, seed=42)
    # Fit a new BN to the sample.
    asia_fitted = CatBN.fit_parameters(
        sample, asia.graph(), parameters_estimator=ParametersEstimator.BE
    )

    # Check the labels of the fitted BN.
    assert asia_fitted.labels() == asia.labels(), "Wrong fitted BN labels."
    # Check the graph of the fitted BN.
    assert asia_fitted.graph() == asia.graph(), "Wrong fitted BN graph."


def test_asia_read_to_json_file() -> None:
    """Test JSON serialization/deserialization for Asia network."""
    # Load the Asia BN.
    asia = load_asia()

    # Get a named temp file for the JSON.
    with tempfile.NamedTemporaryFile(suffix=".json") as tmp:
        path = tmp.name
    # Write to a JSON file.
    asia.to_json_file(path)
    # Read from the JSON file.
    asia_from_json = CatBN.from_json_file(path)

    # Check the labels after read/write.
    assert asia.labels() == asia_from_json.labels(), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert asia.graph() == asia_from_json.graph(), "Wrong graph after read/write."
    # Check the CPDs after read/write.
    assert asia.cpds() == asia_from_json.cpds(), "Wrong CPDs after read/write."


def test_ecoli70() -> None:
    """Test loading and properties of the Ecoli70 network."""
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Get the graph of the BN.
    graph = ecoli70.graph()

    # Check the name.
    assert ecoli70.name() == "ecoli70", "Wrong name."
    # Check the description.
    assert ecoli70.description() is None, "Wrong description."
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
    """Test sampling from the Ecoli70 network."""
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
    """Test fitting the Ecoli70 network."""
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Sample 1000 data points from the BN.
    sample = ecoli70.sample(1000, seed=42)
    # Fit a new BN to the sample.
    ecoli70_fitted = GaussBN.fit_parameters(sample, ecoli70.graph())

    # Check the labels of the fitted BN.
    assert ecoli70_fitted.labels() == ecoli70.labels(), "Wrong fitted BN labels."
    # Check the graph of the fitted BN.
    assert ecoli70_fitted.graph() == ecoli70.graph(), "Wrong fitted BN graph."


def test_ecoli70_read_to_json_file() -> None:
    """Test JSON serialization/deserialization for Ecoli70 network."""
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()

    # Get a named temp file for the JSON.
    with tempfile.NamedTemporaryFile(suffix=".json") as tmp:
        path = tmp.name
    # Write to a JSON file.
    ecoli70.to_json_file(path)
    # Read from the JSON file.
    ecoli70_from_json = GaussBN.from_json_file(path)

    # Check the labels after read/write.
    assert (
        ecoli70.labels() == ecoli70_from_json.labels()
    ), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert ecoli70.graph() == ecoli70_from_json.graph(), "Wrong graph after read/write."
    # Check the CPDs after read/write.
    assert ecoli70.cpds() == ecoli70_from_json.cpds(), "Wrong CPDs after read/write."


def test_eating() -> None:
    """Test loading and properties of the Eating network."""
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
    """Test sampling from the Eating network."""
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
    """Test fitting the Eating network."""
    # Load the Eating CTBN.
    eating = load_eating()
    # Sample 1000 trajectories from the CTBN.
    sample = eating.sample(1000, max_time=10.0, seed=42)
    # Fit a new CTBN to the sample.
    eating_fitted = CatCTBN.fit_parameters(
        sample, eating.graph(), parameters_estimator=ParametersEstimator.BE
    )

    # Check the labels of the fitted CTBN.
    assert eating_fitted.labels() == eating.labels(), "Wrong fitted CTBN labels."
    # Check the graph of the fitted CTBN.
    assert eating_fitted.graph() == eating.graph(), "Wrong fitted CTBN graph."


def test_eating_read_to_json_file() -> None:
    """Test JSON serialization/deserialization for Eating network."""
    # Load the Eating CTBN.
    eating = load_eating()

    # Get a named temp file for the JSON.
    with tempfile.NamedTemporaryFile(suffix=".json") as tmp:
        path = tmp.name
    # Write to a JSON file.
    eating.to_json_file(path)
    # Read from the JSON file.
    eating_from_json = CatCTBN.from_json_file(path)

    # Check the labels after read/write.
    assert (
        eating.labels() == eating_from_json.labels()
    ), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert eating.graph() == eating_from_json.graph(), "Wrong graph after read/write."
    # Check the CIMs after read/write.
    assert eating.cims() == eating_from_json.cims(), "Wrong CIMs after read/write."


def test_categorical_bayesian_network_fit_incomplete() -> None:
    """Test fitting a Categorical BN from incomplete data using MLE."""
    import numpy as np
    import pandas as pd
    from causal_hub.datasets import CatIncTable

    # Define the DataFrame with missing values.
    data = {
        "A": ["X", "Y", "X", "Y", "X"],
        "B": ["X", "Y", "X", np.nan, "X"],
    }
    df = pd.DataFrame(data)
    df["A"] = df["A"].astype("category")
    df["B"] = df["B"].astype("category")

    # Construct the dataset.
    dataset = CatIncTable.from_pandas(df)

    # Define the graph.
    G = nx.DiGraph([("A", "B")])
    graph = DiGraph.from_networkx(G)

    # Fit the model.
    model = CatBN.fit_parameters(
        dataset, graph, parameters_estimator=ParametersEstimator.MLE
    )

    # Check the labels and graph.
    assert model.labels() == ["A", "B"]
    assert model.graph().edges() == [("A", "B")]

    # Check the parameters.
    cpds = model.cpds()
    # P(A)
    # A values: X, Y, X, Y, X. P(A=X) = 3/5 = 0.6, P(A=Y) = 2/5 = 0.4
    np.testing.assert_allclose(cpds["A"].parameters().flatten(), [0.6, 0.4])

    # P(B | A)
    # When A=X: B is X (3 times). P(B=X | A=X) = 1.0, P(B=Y | A=X) = 0.0
    # When A=Y: B is Y (1 time), NaN (1 time). P(B=X | A=Y) = 0.0, P(B=Y | A=Y) = 1.0
    # Parameters for B are organized as [P(B=X|A=X), P(B=Y|A=X), P(B=X|A=Y), P(B=Y|A=Y)]
    np.testing.assert_allclose(cpds["B"].parameters().flatten(), [1.0, 0.0, 0.0, 1.0])


def test_gaussian_bayesian_network_fit_numerical() -> None:
    """Test fitting a Gaussian BN from numerical data using MLE."""
    import numpy as np
    import pandas as pd
    from causal_hub.datasets import GaussTable

    # Define the DataFrame with numerical values.
    # B = 2*A + 1 + epsilon, epsilon ~ N(0, 0.1^2)
    np.random.seed(42)
    A = np.random.normal(0, 1, 1000)
    B = 2 * A + 1 + np.random.normal(0, 0.1, 1000)
    df = pd.DataFrame({"A": A, "B": B})

    # Construct the dataset.
    dataset = GaussTable.from_pandas(df)

    # Define the graph.
    graph = DiGraph.from_networkx(nx.DiGraph([("A", "B")]))

    # Fit the model.
    model = GaussBN.fit_parameters(
        dataset, graph, parameters_estimator=ParametersEstimator.MLE
    )

    # Check the parameters.
    cpds = model.cpds()

    # Check P(A) -> Mean ~ 0, Variance ~ 1
    params_A = cpds["A"].parameters()
    np.testing.assert_allclose(params_A["intercept"], [0.0], atol=0.1)
    np.testing.assert_allclose(params_A["covariance"], [[1.0]], atol=0.1)

    # Check P(B | A) -> Intercept ~ 1, Coeff ~ 2, Variance ~ 0.01 (0.1^2)
    params_B = cpds["B"].parameters()
    np.testing.assert_allclose(params_B["intercept"], [1.0], atol=0.05)
    np.testing.assert_allclose(params_B["coefficients"], [[2.0]], atol=0.05)
    np.testing.assert_allclose(params_B["covariance"], [[0.01]], atol=0.01)


def test_ecoli70_fit_incomplete() -> None:
    """Test fitting the Ecoli70 network from incomplete data."""
    # Load the Ecoli70 BN.
    ecoli70 = load_ecoli70()
    # Sample 1000 data points from the BN.
    sample = ecoli70.sample(1000, seed=42)
    # Get values.
    values = sample.values()
    # Introduce missing values.
    # Set 10% of values to NaN.
    rng = np.random.default_rng(42)
    mask = rng.random(values.shape) < 0.1
    values[mask] = np.nan

    # Create incomplete dataset.
    df = pd.DataFrame(values, columns=sample.labels())
    dataset = GaussIncTable.from_pandas(df)

    # Fit a new BN to the sample.
    fitted = GaussBN.fit_parameters(dataset, ecoli70.graph())

    # Check the fit.
    assert fitted.labels() == ecoli70.labels()
    # TODO: Check parameters.


@pytest.mark.parametrize(
    "loader, x, z, target, expected, tol",
    [
        (load_asia, ["lung"], {"smoke": "yes"}, {"lung": "yes"}, 0.1000, 0.05),
        (
            load_asia,
            ["lung", "bronc"],
            {"smoke": "yes", "asia": "yes"},
            {"lung": "yes", "bronc": "yes"},
            0.0600,
            0.05,
        ),
        (load_cancer, ["Cancer"], {"Smoker": "True"}, {"Cancer": "True"}, 0.0320, 0.02),
        (
            load_cancer,
            ["Cancer", "Dyspnoea"],
            {"Smoker": "True", "Pollution": "low"},
            {"Cancer": "True", "Dyspnoea": "True"},
            0.0195,
            0.02,
        ),
        (
            load_earthquake,
            ["Alarm"],
            {"Burglary": "True"},
            {"Alarm": "True"},
            0.84,
            0.1,
        ),
        (
            load_earthquake,
            ["MaryCalls", "JohnCalls"],
            {"Alarm": "True", "Earthquake": "True"},
            {"MaryCalls": "True", "JohnCalls": "True"},
            0.6300,
            0.05,
        ),
        (load_sachs, ["Raf"], {"PKA": "LOW"}, {"Raf": "LOW"}, 0.1146, 0.05),
        (
            load_sachs,
            ["Akt", "Erk"],
            {"PKA": "LOW", "PKC": "LOW"},
            {"Akt": "LOW", "Erk": "LOW"},
            0.1903,
            0.05,
        ),
        (load_survey, ["E"], {"R": "big"}, {"E": "high"}, 0.7330, 0.05),
        (
            load_survey,
            ["T", "O"],
            {"A": "adult", "S": "M"},
            {"T": "car", "O": "emp"},
            0.5279,
            0.05,
        ),
    ],
)
def test_inference_accuracy(loader, x, z, target, expected, tol) -> None:
    """Test inference accuracy against precomputed values."""
    # Load the model.
    bn = loader()

    # Estimate the query.
    # We use a fixed seed for reproducibility.
    # Since 'estimate' uses approximate inference by default (sampling),
    # we need to ensure reasonable convergence or sample size.
    # The default checks usually run with reasonable defaults.
    est_cpd = bn.estimate(x=x, z=list(z.keys()), seed=42)

    # Get the parameters.
    params = est_cpd.parameters()

    # Helper to map configuration to index
    def map_config(config, variables, shapes, states_dict):
        idx = 0
        stride = 1
        # Reverse iteration for C-order (last dim varies fastest)
        for i in range(len(variables) - 1, -1, -1):
            var = variables[i]
            val = config[var]
            state_idx = states_dict[var].index(val)
            idx += state_idx * stride
            stride *= shapes[i]
        return idx

    # Get states mapping
    t_states = est_cpd.support()
    c_states = est_cpd.conditioning_support()

    # Calculate indices
    # Conditioning index (rows)
    # Handle empty evidence case
    if not z:
        z_idx = 0
    else:
        # Note: estimate() uses conditioning_labels() which might define the order.
        z_idx = map_config(
            z,
            est_cpd.conditioning_labels(),
            est_cpd.conditioning_shape(),
            c_states,
        )

    # Target index (cols)
    x_idx = map_config(target, est_cpd.labels(), est_cpd.shape(), t_states)

    # Get probability
    # Layout matches: (Conditioning, Target)
    if params.ndim == 2:
        prob = params[z_idx, x_idx]
    else:
        # Should be 2D even if singleton dims?
        # If 1D, it might be flattened.
        # Check layout.

        # Let's assume 2D as seen in inspection.
        # But for empty Z, shape was (1, 2).
        prob = params[z_idx, x_idx]

    print(f"Computed: {prob}, Expected: {expected}")
    assert abs(prob - expected) < tol


def test_cat_ctbn_sample_by_length_and_time() -> None:
    """Test CTBN sampling with both max_len and max_time."""
    from causal_hub.assets import load_eating

    eating = load_eating()
    sampled = eating.sample(n=2, max_len=10, max_time=5.0, seed=42)
    assert isinstance(sampled, CatTrjs)
    sdfs = sampled.to_pandas()
    assert len(sdfs) == 2
    for trj in sdfs:
        assert len(trj) <= 10
        assert trj["time"].max() <= 5.0


def test_cat_ctbn_sample_error_no_limit() -> None:
    """Test CTBN sample errors when neither max_len nor max_time is set."""
    import pytest
    from causal_hub.assets import load_eating

    eating = load_eating()
    with pytest.raises(ValueError, match="At least one"):
        eating.sample(n=2, seed=42)


def test_inference_with_evidence_dict() -> None:
    """Test estimate/do_estimate with evidence dictionaries."""
    bn = load_earthquake()

    est = bn.estimate(
        x=["MaryCalls"],
        z=["Alarm"],
        w={"Earthquake": "True"},
        seed=42,
    )
    assert est is not None

    est_do = bn.do_estimate(
        x=["Alarm"],
        y=["MaryCalls"],
        z=[],
        w={"Earthquake": "True"},
        seed=42,
    )
    assert est_do is not None


@pytest.mark.parametrize("graph_type", [DiGraph, UnGraph])
def test_graph_add_vertex(graph_type) -> None:
    """Test adding a new vertex to a graph."""
    # Create an empty graph.
    graph = graph_type.empty(["A", "C"])

    # Add a vertex that does not exist.
    i = graph.add_vertex("B")
    assert i == 1, "Wrong index for the new vertex."
    assert graph.vertices() == ["A", "B", "C"], "Wrong vertices after addition."

    # Adding a vertex that already exists returns its index.
    assert graph.add_vertex("A") == 0, "Wrong index for the existing vertex."
    assert graph.vertices() == ["A", "B", "C"], "Vertices changed by existing addition."


@pytest.mark.parametrize("graph_type", [DiGraph, UnGraph])
def test_graph_add_vertex_preserves_edges(graph_type) -> None:
    """Test that edges are preserved when adding a vertex in between."""
    # Create a graph with an edge.
    graph = graph_type.empty(["A", "C"])
    graph.add_edge("A", "C")

    # Add a vertex in between.
    graph.add_vertex("B")

    # The former edge is preserved.
    assert graph.edges() == [("A", "C")], "Edge lost by vertex addition."


@pytest.mark.parametrize("graph_type", [DiGraph, UnGraph])
def test_graph_del_vertex(graph_type) -> None:
    """Test deleting a vertex from a graph."""
    # Create a graph.
    graph = graph_type.empty(["A", "B", "C"])

    # Delete an existing vertex.
    assert graph.del_vertex("B") is True, "Vertex deletion failed."
    assert graph.vertices() == ["A", "C"], "Wrong vertices after deletion."

    # Deleting a vertex that does not exist returns False.
    assert (
        graph.del_vertex("B") is False
    ), "Deletion of missing vertex must return False."
    assert (
        graph.del_vertex("Z") is False
    ), "Deletion of unknown vertex must return False."
    assert graph.vertices() == ["A", "C"], "Vertices changed by failed deletion."


@pytest.mark.parametrize("graph_type", [DiGraph, UnGraph])
def test_graph_del_vertex_removes_incident_edges(graph_type) -> None:
    """Test that incident edges are removed when deleting a vertex."""
    # Create a graph with edges.
    graph = graph_type.empty(["A", "B", "C"])
    graph.add_edge("A", "B")
    graph.add_edge("B", "C")

    # Delete the middle vertex.
    assert graph.del_vertex("B") is True, "Vertex deletion failed."

    # The incident edges are gone.
    assert graph.edges() == [], "Incident edges not removed."
    assert graph.vertices() == ["A", "C"], "Wrong vertices after deletion."


def test_cat_bn_fit() -> None:
    """Test CatBN.fit dispatching between parameter fitting and structure learning."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)
    graph = DiGraph.empty(["A", "B"])

    # FitMethod.Parameters requires a graph ...
    with pytest.raises(ValueError, match="graph is required"):
        CatBN.fit(dataset, None, FitMethod.Parameters)

    # ... and fits the CPDs over it.
    model = CatBN.fit(dataset, graph, FitMethod.Parameters)
    assert isinstance(model, CatBN)

    # The default fit method is Parameters.
    model = CatBN.fit(dataset, graph)
    assert isinstance(model, CatBN)


def test_cat_bn_fit_structure() -> None:
    """Test CatBN.fit with FitMethod.Structure ignoring the graph."""
    # A = B.
    size = 100
    a = np.random.choice(["0", "1"], size=size)
    b = a.copy()

    df = pd.DataFrame({"A": a, "B": b})
    df = df.astype("category")

    dataset = CatTable.from_pandas(df)
    graph = DiGraph.empty(["A", "C"])

    # FitMethod.Structure learns the structure from data, ignoring any given graph.
    model = CatBN.fit(dataset, graph, FitMethod.Structure)

    assert isinstance(model, CatBN)
    assert set(model.graph().vertices()) == {"A", "B"}
    # The learned edge is Markov-equivalent in both directions.
    assert set(model.graph().edges()) in [{("A", "B")}, {("B", "A")}]

    # Passing no graph at all is supported as well.
    model = CatBN.fit(dataset, None, FitMethod.Structure)

    assert isinstance(model, CatBN)


def test_gauss_bn_fit_structure() -> None:
    """Test GaussBN.fit with FitMethod.Structure."""
    # X ~ N(0, 1), Y = 2*X + noise.
    size = 200
    x = np.random.normal(0, 1, size)
    y = 2 * x + np.random.normal(0, 0.1, size)

    df = pd.DataFrame({"X": x, "Y": y})
    dataset = GaussTable.from_pandas(df)

    model = GaussBN.fit(dataset, None, FitMethod.Structure)

    assert isinstance(model, GaussBN)
    assert set(model.graph().vertices()) == {"X", "Y"}
    assert set(model.graph().edges()) in [{("X", "Y")}, {("Y", "X")}]


def test_ctbn_fit_structure() -> None:
    """Test CatCTBN.fit with FitMethod.Structure."""
    # Load the Eating CTBN and sample trajectories from it.
    eating = load_eating()
    sample = eating.sample(50, max_len=50, seed=42)

    # FitMethod.Structure learns the structure from the trajectories.
    model = CatCTBN.fit(sample, None, FitMethod.Structure)

    assert isinstance(model, CatCTBN)
    assert set(model.graph().vertices()) == set(eating.labels())


def test_ctbn_fit_structure_ctpc() -> None:
    """Test CatCTBN.fit_structure with the CTPC algorithm."""
    # Load the Eating CTBN and sample trajectories from it.
    eating = load_eating()
    sample = eating.sample(100, max_len=100, seed=42)

    model = CatCTBN.fit_structure(sample, StructureEstimator.CTPC)

    assert isinstance(model, CatCTBN)
    assert set(model.graph().vertices()) == set(eating.labels())


def test_ctbn_fit_structure_dispatch() -> None:
    """Test CatCTBN.fit dispatching to structure learning with both algorithms."""
    # Load the Eating CTBN and sample trajectories from it.
    eating = load_eating()
    sample = eating.sample(100, max_len=100, seed=42)

    for structure_estimator in [
        StructureEstimator.CTHC,
        StructureEstimator.CTPC,
    ]:
        model = CatCTBN.fit(
            sample,
            None,
            FitMethod.Structure,
            structure_estimator=structure_estimator,
            parallel=False,
        )

        assert isinstance(model, CatCTBN)
        assert set(model.graph().vertices()) == set(eating.labels())
