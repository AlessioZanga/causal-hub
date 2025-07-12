import tempfile
from causal_hub import load_asia, load_eating, CatCTBN, CatBN


def test_load_asia() -> None:
    # Load the Asia BN.
    asia = load_asia()
    # Get the graph of the BN.
    graph = asia.graph()

    # Check the vertices labels.
    vertices = [
        "asia", "bronc", "dysp", "either", "lung", "smoke", "tub", "xray"
    ]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_load_asia_read_write_json() -> None:
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


def test_load_eating() -> None:
    # Load the Eating BN.
    eating = load_eating()
    # Get the graph of the BN.
    graph = eating.graph()

    # Check the vertices labels.
    vertices = ["Eating", "FullStomach", "Hungry",]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_load_eating_read_write_json() -> None:
    # Load the Eating BN.
    eating = load_eating()

    # Get a named temp file for the JSON.
    path = tempfile.NamedTemporaryFile().name
    # Write to a JSON file.
    eating.write_json(path)
    # Read from the JSON file.
    eating_from_json = CatCTBN.read_json(path)

    # Check the labels after read/write.
    assert eating.labels() == eating_from_json.labels(), "Wrong labels after read/write."
    # Check the graph after read/write.
    assert eating.graph() == eating_from_json.graph(), "Wrong graph after read/write."
    # Check the CIMs after read/write.
    assert eating.cims() == eating_from_json.cims(), "Wrong CIMs after read/write."
