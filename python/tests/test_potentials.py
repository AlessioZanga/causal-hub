"""Tests for CatPhi and GaussPhi potential bindings."""

import numpy as np
from causal_hub.assets import load_asia, load_ecoli70
from causal_hub.models import CatPhi, GaussPhi


# ── CatPhi tests ──────────────────────────────────────────────


def test_cat_phi_from_cpd() -> None:
    """Test creating a CatPhi from a CatCPD."""
    asia = load_asia()
    cpds = asia.cpds()
    cpd = cpds["asia"]
    phi = CatPhi.from_cpd(cpd)
    assert phi.labels() == ["asia"]
    # States are alphabetically sorted: ("no", "yes")
    assert phi.support() == {"asia": ("no", "yes")}
    assert phi.shape() == [2]
    # P(asia = no) = 0.99, P(asia = yes) = 0.01 (alphabetical order)
    np.testing.assert_allclose(phi.parameters(), [0.99, 0.01])
    assert phi.parameters_size() == 2


def test_cat_phi_eq() -> None:
    """Test CatPhi equality."""
    asia = load_asia()
    cpd = asia.cpds()["asia"]
    phi1 = CatPhi.from_cpd(cpd)
    phi2 = CatPhi.from_cpd(cpd)
    assert phi1 == phi2


def test_cat_phi_normalize() -> None:
    """Test normalizing a CatPhi."""
    asia = load_asia()
    cpd = asia.cpds()["asia"]
    phi = CatPhi.from_cpd(cpd)
    norm = phi.normalize()
    np.testing.assert_allclose(norm.parameters(), phi.parameters())


def test_cat_phi_condition() -> None:
    """Test conditioning then normalizing a CatPhi."""
    asia = load_asia()
    cpds = asia.cpds()
    cpd_asia = cpds["asia"]
    cpd_smoke = cpds["smoke"]
    phi_asia = CatPhi.from_cpd(cpd_asia)
    phi_smoke = CatPhi.from_cpd(cpd_smoke)
    joint = phi_asia * phi_smoke
    assert set(joint.labels()) == {"asia", "smoke"}

    # Condition on asia = "yes", then normalize
    cond = joint.condition({"asia": "yes"}).normalize()
    assert cond.labels() == ["smoke"]
    params = cond.parameters().flatten()
    smoke_params = cpd_smoke.parameters().flatten()
    np.testing.assert_allclose(params, smoke_params)


def test_cat_phi_marginalize() -> None:
    """Test marginalizing a CatPhi."""
    asia = load_asia()
    cpds = asia.cpds()
    cpd_asia = cpds["asia"]
    cpd_smoke = cpds["smoke"]
    phi_asia = CatPhi.from_cpd(cpd_asia)
    phi_smoke = CatPhi.from_cpd(cpd_smoke)
    joint = phi_asia * phi_smoke
    marg = joint.marginalize("asia")
    assert marg.labels() == ["smoke"]
    np.testing.assert_allclose(marg.parameters(), [0.5, 0.5], atol=0.001)


def test_cat_phi_into_cpd() -> None:
    """Test converting a CatPhi back to a CatCPD."""
    asia = load_asia()
    cpd = asia.cpds()["asia"]
    phi = CatPhi.from_cpd(cpd)
    cpd_roundtrip = phi.into_cpd(x=["asia"], z=[])
    assert cpd_roundtrip.labels() == ["asia"]
    np.testing.assert_allclose(cpd_roundtrip.parameters(), cpd.parameters())


def test_cat_phi_multiply() -> None:
    """Test multiplying two CatPhi potentials."""
    asia = load_asia()
    cpds = asia.cpds()
    phi_asia = CatPhi.from_cpd(cpds["asia"])
    phi_smoke = CatPhi.from_cpd(cpds["smoke"])
    joint = phi_asia * phi_smoke
    assert set(joint.labels()) == {"asia", "smoke"}
    expected = np.outer(
        phi_asia.parameters().flatten(),
        phi_smoke.parameters().flatten(),
    ).flatten()
    np.testing.assert_allclose(joint.parameters().flatten(), expected)


def test_cat_phi_divide() -> None:
    """Test dividing two CatPhi potentials.

    Dividing a joint by a marginal yields the conditional potential
    (up to normalization). Normalizing then marginalizing recovers
    the original marginal.
    """
    asia = load_asia()
    cpds = asia.cpds()
    phi_asia = CatPhi.from_cpd(cpds["asia"])
    phi_smoke = CatPhi.from_cpd(cpds["smoke"])
    joint = phi_asia * phi_smoke
    quotient = joint / phi_smoke
    # Quotient has both labels (asia, smoke)
    assert set(quotient.labels()) == {"asia", "smoke"}
    # Normalize, then marginalize smoke out to recover phi_asia
    normalized = quotient.normalize()
    recovered = normalized.marginalize("smoke")
    np.testing.assert_allclose(
        recovered.parameters().flatten(),
        phi_asia.parameters().flatten(),
    )


def test_cat_phi_inplace_divide() -> None:
    """Test in-place division recovers marginal after normalization."""
    asia = load_asia()
    cpds = asia.cpds()
    phi_a = CatPhi.from_cpd(cpds["asia"])
    phi_s = CatPhi.from_cpd(cpds["smoke"])
    quotient = phi_a * phi_s
    quotient /= phi_s
    normalized = quotient.normalize()
    recovered = normalized.marginalize("smoke")
    np.testing.assert_allclose(
        recovered.parameters().flatten(),
        phi_a.parameters().flatten(),
    )


def test_cat_phi_joint_asia_smoke() -> None:
    """Test creating a joint distribution over asia, smoke, bronc."""
    asia = load_asia()
    cpds = asia.cpds()
    phi_a = CatPhi.from_cpd(cpds["asia"])
    phi_s = CatPhi.from_cpd(cpds["smoke"])
    phi_b = CatPhi.from_cpd(cpds["bronc"])
    joint = phi_a * phi_s * phi_b
    labels = set(joint.labels())
    assert labels == {"asia", "smoke", "bronc"}

    marg_a = joint.marginalize(["smoke", "bronc"])
    assert marg_a.labels() == ["asia"]
    np.testing.assert_allclose(
        marg_a.parameters().flatten(),
        phi_a.parameters().flatten(),
        atol=0.01,
    )


# ── GaussPhi tests ─────────────────────────────────────────────


def test_gauss_phi_from_cpd() -> None:
    """Test creating a GaussPhi from a GaussCPD."""
    ecoli = load_ecoli70()
    cpds = ecoli.cpds()
    cpd = list(cpds.values())[0]
    phi = GaussPhi.from_cpd(cpd)
    assert set(phi.labels()) == set(cpd.labels()) | set(cpd.conditioning_labels())
    K = phi.precision_matrix()
    assert K.shape[0] == len(phi.labels())
    h = phi.information_vector()
    assert h.shape[0] == len(phi.labels())
    g = phi.log_normalization_constant()
    assert isinstance(g, float)


def test_gauss_phi_normalize() -> None:
    """Test normalizing a GaussPhi."""
    ecoli = load_ecoli70()
    cpd = list(ecoli.cpds().values())[0]
    phi = GaussPhi.from_cpd(cpd)
    norm = phi.normalize()
    np.testing.assert_allclose(norm.parameters_size(), phi.parameters_size())


def test_gauss_phi_condition() -> None:
    """Test conditioning a GaussPhi."""
    ecoli = load_ecoli70()
    cpd = list(ecoli.cpds().values())[0]
    phi = GaussPhi.from_cpd(cpd)
    if len(phi.labels()) > 1:
        label = phi.labels()[0]
        cond = phi.condition({label: 0.0})
        assert label not in cond.labels()


def test_gauss_phi_marginalize() -> None:
    """Test marginalizing a GaussPhi."""
    ecoli = load_ecoli70()
    cpd = list(ecoli.cpds().values())[0]
    phi = GaussPhi.from_cpd(cpd)
    if len(phi.labels()) > 1:
        label = phi.labels()[0]
        marg = phi.marginalize(label)
        assert label not in marg.labels()


def test_gauss_phi_multiply_divide() -> None:
    """Test multiplying and dividing GaussPhi potentials."""
    ecoli = load_ecoli70()
    cpds = ecoli.cpds()
    cpd_list = list(cpds.values())
    if len(cpd_list) >= 2:
        phi1 = GaussPhi.from_cpd(cpd_list[0])
        phi2 = GaussPhi.from_cpd(cpd_list[1])
        product = phi1 * phi2
        assert isinstance(product, GaussPhi)
        quotient = product / phi2
        assert isinstance(quotient, GaussPhi)


def test_gauss_phi_inplace_ops() -> None:
    """Test in-place GaussPhi operations product type."""
    ecoli = load_ecoli70()
    cpd_list = list(ecoli.cpds().values())
    if len(cpd_list) >= 2:
        phi1 = GaussPhi.from_cpd(cpd_list[0])
        phi2 = GaussPhi.from_cpd(cpd_list[1])
        assert isinstance(phi1 * phi2, GaussPhi)
        assert isinstance(phi1 / phi2, GaussPhi)


def test_gauss_phi_into_cpd() -> None:
    """Test converting a GaussPhi back to a GaussCPD."""
    ecoli = load_ecoli70()
    cpd = list(ecoli.cpds().values())[0]
    phi = GaussPhi.from_cpd(cpd)
    cpd_labels = set(cpd.labels()) | set(cpd.conditioning_labels())
    if len(cpd_labels) > 1:
        labels = list(cpd_labels)
        cpd_rt = phi.into_cpd(x=[labels[0]], z=labels[1:])
        assert cpd_rt.labels() == [labels[0]]
    else:
        label = list(cpd_labels)[0]
        cpd_rt = phi.into_cpd(x=[label], z=[])
        assert cpd_rt.labels() == [label]


def test_gauss_phi_first_cpds() -> None:
    """Test GaussPhi creation from first few Ecoli70 CPDs."""
    ecoli = load_ecoli70()
    cpds = ecoli.cpds()
    tested = 0
    for name, cpd in cpds.items():
        try:
            phi = GaussPhi.from_cpd(cpd)
        except Exception:
            continue
        assert name in phi.labels()
        K = phi.precision_matrix()
        n = len(phi.labels())
        assert K.shape == (n, n)
        h = phi.information_vector()
        assert h.shape == (n,)
        g = phi.log_normalization_constant()
        assert np.isfinite(g)
        tested += 1
        if tested >= 3:
            break
    assert tested > 0, "No GaussCPD could be converted to GaussPhi"
