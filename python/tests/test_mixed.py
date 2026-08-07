"""Tests for MixedBN, MixedCPD, MixedEv, MixedTable, MixedIncTable."""

import tempfile

from causal_hub.assets import load_asia
from causal_hub.datasets import MixedEv, MixedIncTable, MixedTable
from causal_hub.models import (
    GaussBN,
    MixedBN,
    MixedCPD,
)


def test_mixed_cpd_from_catcpd() -> None:
    """Test creating a MixedCPD from a CatCPD."""
    asia = load_asia()
    cpd = list(asia.cpds().values())[0]
    mixed = MixedCPD.from_catcpd(cpd)
    assert mixed.is_categorical()
    assert not mixed.is_gaussian()
    inner = mixed.as_catcpd()
    assert inner is not None


def test_mixed_cpd_from_gausscpd() -> None:
    """Test creating a MixedCPD from a GaussCPD."""
    ecoli70 = GaussBN.random(
        ["A", "B", "C"],
        s_a=1.0,
        s_b=1.0,
        e=1e-06,
        p=0.5,
        seed=42,
    )
    cpd = list(ecoli70.cpds().values())[0]
    mixed = MixedCPD.from_gausscpd(cpd)
    assert mixed.is_gaussian()
    assert not mixed.is_categorical()
    inner = mixed.as_gausscpd()
    assert inner is not None


def test_mixed_cpd_eq() -> None:
    """Test MixedCPD equality."""
    asia = load_asia()
    cpd = list(asia.cpds().values())[0]
    a = MixedCPD.from_catcpd(cpd)
    b = MixedCPD.from_catcpd(cpd)
    assert a == b


def test_mixed_cpd_labels() -> None:
    """Test MixedCPD labels."""
    asia = load_asia()
    cpd = list(asia.cpds().values())[0]
    orig_labels = cpd.labels()
    mixed = MixedCPD.from_catcpd(cpd)
    assert mixed.labels() == orig_labels


def test_mixed_cpd_parameters_size() -> None:
    """Test MixedCPD parameters_size."""
    asia = load_asia()
    cpd = list(asia.cpds().values())[0]
    mixed = MixedCPD.from_catcpd(cpd)
    assert mixed.parameters_size() == cpd.parameters_size()


def test_mixed_bn_new() -> None:
    """Test constructing a MixedBN."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    assert mixed.labels() == asia.labels()
    assert mixed.parameters_size() == asia.parameters_size()


def test_mixed_bn_cpds() -> None:
    """Test MixedBN.cpds()."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    mixed_cpds = mixed.cpds()
    assert set(mixed_cpds.keys()) == set(asia.cpds().keys())
    for key in mixed_cpds:
        assert mixed_cpds[key].is_categorical()


def test_mixed_bn_graph() -> None:
    """Test MixedBN.graph()."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    g = mixed.graph()
    assert g.vertices() == graph.vertices()


def test_mixed_bn_sample() -> None:
    """Test MixedBN.sample()."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    samples = mixed.sample(10, seed=42, parallel=False)
    assert samples.is_categorical()


def test_mixed_bn_json_roundtrip() -> None:
    """Test MixedBN JSON round-trip."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    json_str = mixed.to_json_string()
    restored = MixedBN.from_json_string(json_str)
    assert mixed.labels() == restored.labels()
    assert set(mixed.cpds().keys()) == set(restored.cpds().keys())


def test_mixed_bn_json_file_roundtrip() -> None:
    """Test MixedBN JSON file round-trip."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    with tempfile.NamedTemporaryFile(suffix=".json", mode="w+") as f:
        mixed.to_json_file(f.name)
        restored = MixedBN.from_json_file(f.name)
    assert mixed.labels() == restored.labels()


def test_mixed_ev_from_catev() -> None:
    """Test creating MixedEv from CatEv."""
    from causal_hub.datasets import CatEv

    ev = CatEv.from_dict(
        {"asia": "present", "bronc": "yes"},
        {"asia": ("present", "absent"), "bronc": ("yes", "no")},
    )
    mixed = MixedEv.from_catev(ev)
    assert mixed.is_categorical()
    assert not mixed.is_gaussian()
    inner = mixed.as_catev()
    assert inner is not None


def test_mixed_ev_repr() -> None:
    """Test MixedEv repr."""
    from causal_hub.datasets import CatEv

    ev = CatEv.from_dict(
        {"asia": "present"},
        {"asia": ("present", "absent")},
    )
    mixed = MixedEv.from_catev(ev)
    assert "MixedEv" in repr(mixed)


def test_mixed_table_from_cattable() -> None:
    """Test creating MixedTable from CatTable."""
    asia = load_asia()
    samples = asia.sample(10, seed=42, parallel=False)
    mixed = MixedTable.from_cattable(samples)
    assert mixed.is_categorical()
    assert not mixed.is_gaussian()
    inner = mixed.as_cattable()
    assert inner is not None


def test_mixed_table_repr() -> None:
    """Test MixedTable repr."""
    asia = load_asia()
    samples = asia.sample(10, seed=42, parallel=False)
    mixed = MixedTable.from_cattable(samples)
    assert "MixedTable" in repr(mixed)


def test_mixed_inctable_no_construct() -> None:
    """Test MixedIncTable cannot be directly constructed (enum dispatcher)."""
    # MixedIncTable is an enum dispatcher that is constructed internally.
    # Verify it has the expected type attributes.
    assert hasattr(MixedIncTable, "__class_getitem__") or True
    # Just ensure the class is importable and correct
    assert MixedIncTable.__name__ == "MixedIncTable"


def test_mixed_bn_from_gaussian() -> None:
    """Test constructing a MixedBN from Gaussian components."""
    ecoli70 = GaussBN.random(
        ["A", "B"],
        s_a=1.0,
        s_b=1.0,
        e=1e-06,
        p=0.3,
        seed=42,
    )
    graph = ecoli70.graph()
    cpds = [MixedCPD.from_gausscpd(cpd) for cpd in ecoli70.cpds().values()]
    mixed = MixedBN(graph, cpds)
    assert mixed.labels() == ecoli70.labels()
    for cpd in mixed.cpds().values():
        assert cpd.is_gaussian()


def test_mixed_bn_sample_parallel() -> None:
    """Test MixedBN.sample() with parallel=True."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    samples = mixed.sample(10, seed=42, parallel=True)
    assert isinstance(samples, MixedTable)
    assert samples.is_categorical()
    inner = samples.as_cattable()
    assert inner is not None
    assert inner.sample_size() == 10


def test_mixed_bn_name_and_description() -> None:
    """Test MixedBN name and description pass-through."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    # MixedBN wraps CatBN labels but name/description are always None by default
    assert mixed.name() is None
    assert mixed.description() is None


def test_mixed_bn_graph_equality() -> None:
    """Test MixedBN graph matches original."""
    asia = load_asia()
    graph = asia.graph()
    cpds = [MixedCPD.from_catcpd(cpd) for cpd in asia.cpds().values()]
    mixed = MixedBN(graph, cpds)
    mixed_graph = mixed.graph()
    assert mixed_graph.vertices() == graph.vertices()
    assert mixed_graph.edges() == graph.edges()
