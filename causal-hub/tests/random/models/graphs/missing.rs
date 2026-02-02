use causal_hub::{
    datasets::{MissingMechanism, MissingType},
    labels, map,
    models::{DiGraph, Graph},
    random::{Random, RngMissingMechanism},
    set,
    types::{Error, Result},
};
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

#[test]
fn new() {
    let labels = labels!("X", "Y");
    let pr = map![(0, set![1])];
    let mechanism = MissingMechanism::new(labels, pr);
    assert!(mechanism.is_ok());
}

#[test]
fn new_out_of_bounds_key() {
    let labels = labels!("X", "Y");
    let pr = map![(2, set![1])];
    let mechanism = MissingMechanism::new(labels, pr);
    assert!(matches!(mechanism, Err(Error::VertexOutOfBounds(2))));
}

#[test]
fn new_out_of_bounds_value() {
    let labels = labels!("X", "Y");
    let pr = map![(0, set![2])];
    let mechanism = MissingMechanism::new(labels, pr);
    assert!(matches!(mechanism, Err(Error::VertexOutOfBounds(2))));
}

#[test]
fn rng_pr_mcar() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let labels = labels!["X", "Y", "Z"];
    let mut g = DiGraph::empty(labels)?;
    g.add_edge(0, 1)?;
    g.add_edge(2, 1)?;

    let mut sampler = RngMissingMechanism::new(&mut rng, &g, MissingType::MCAR, 0.5)?;
    let pr = sampler.random()?;

    // With 3 variables and p=0.5, round(3*0.5) = round(1.5) = 2 variables should be missing.
    assert_eq!(pr.len(), 2);
    for v in pr.values() {
        assert!(v.is_empty());
    }

    Ok(())
}

#[test]
fn rng_pr_mar() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let labels = labels!["X", "Y", "Z"];
    let mut g = DiGraph::empty(labels)?;
    g.add_edge(0, 1)?; // X -> Y
    g.add_edge(2, 1)?; // Z -> Y
    // V-structure: X -> Y <- Z

    let mut sampler = RngMissingMechanism::new(&mut rng, &g, MissingType::MAR, 0.5)?;
    let pr = sampler.random()?;

    assert_eq!(pr.len(), 2);
    // In MAR with v-structure X -> Y <- Z, it should prefer making X or Z missing with Y as cause.
    // round(3 * 0.5) = 2.
    // The v-structure is (0, 1, 2) i.e. X -> Y <- Z.
    // So X or Z missing, Y observed.
    for (x, pa) in pr {
        assert!(!pa.is_empty());
        for z in pa {
            assert_ne!(x, z);
        }
    }

    Ok(())
}

#[test]
fn rng_pr_mnar() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let labels = labels!["X", "Y", "Z"];
    let mut g = DiGraph::empty(labels)?;
    g.add_edge(0, 1)?;
    g.add_edge(2, 1)?;

    let mut sampler = RngMissingMechanism::new(&mut rng, &g, MissingType::MNAR, 0.5)?;
    let pr = sampler.random()?;

    assert_eq!(pr.len(), 2);

    Ok(())
}
