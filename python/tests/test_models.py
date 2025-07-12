from causal_hub import load_asia, load_eating


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


def test_load_eating() -> None:
    # Load the Eating BN.
    eating = load_eating()
    # Get the graph of the BN.
    graph = eating.graph()

    # Check the vertices labels.
    vertices = ["Eating", "FullStomach", "Hungry",]
    assert graph.vertices() == vertices, "Wrong vertices labels."
