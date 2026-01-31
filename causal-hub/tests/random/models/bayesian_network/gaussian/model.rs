use causal_hub::{
    labels,
    random::{Random, RngGaussBN},
};
use rand::prelude::*;

#[test]
fn rng_gauss_bn_new() {
    let mut rng = StdRng::seed_from_u64(42);
    let labels = labels!["X1", "X2", "X3"];
    let (s_a, s_b, e, p) = (1.0, 1.0, 1e-6, 0.5);

    let rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p);
    assert!(rng_bn.is_ok());

    let rng_bn = RngGaussBN::new(&mut rng, &labels, 0.0, s_b, e, p);
    assert!(rng_bn.is_err());

    let rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, 0.0, e, p);
    assert!(rng_bn.is_err());

    let rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, 0.0, p);
    assert!(rng_bn.is_err());

    let rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, -0.1);
    assert!(rng_bn.is_err());

    let rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, 1.1);
    assert!(rng_bn.is_err());
}

#[test]
fn rng_gauss_bn_random() -> causal_hub::types::Result<()> {
    let mut rng = StdRng::seed_from_u64(42);
    let labels = labels!["X1", "X2", "X3"];
    let (s_a, s_b, e, p) = (1.0, 1.0, 1e-6, 0.5);

    let mut rng_bn = RngGaussBN::new(&mut rng, &labels, s_a, s_b, e, p)?;
    let bn = rng_bn.random();
    assert!(bn.is_ok());

    Ok(())
}
