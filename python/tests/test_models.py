import tempfile

from causal_hub.assets import load_asia, load_eating, load_ecoli70
from causal_hub.models import CatBN, CatCTBN, GaussBN


def test_asia() -> None:
    # Load the Asia BN.
    asia = load_asia()
    # Get the graph of the BN.
    graph = asia.graph()

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
    # Load the Eating BN.
    eating = load_eating()
    # Get the graph of the BN.
    graph = eating.graph()

    # Check the vertices labels.
    vertices = [
        "Eating",
        "FullStomach",
        "Hungry",
    ]
    assert graph.vertices() == vertices, "Wrong vertices labels."


def test_eating_read_write_json() -> None:
    # Load the Eating BN.
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
