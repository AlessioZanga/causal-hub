use approx::assert_relative_eq;
use causal_hub::{
    datasets::Dataset,
    estimators::{BNEstimator, CPDEstimator, MLE, ParCPDEstimator},
    inference::{ApproximateInference, BNInference, ParBNInference},
    io::JsonIO,
    labels,
    models::{
        BN, CPD, CatCPD, DiGraph, GaussCPD, GaussCPDP, Graph, HasLabels, MixedBN, MixedCPD,
        MixedSupport, MixedTable,
    },
    random::Random,
    samplers::{BNSampler, ForwardSampler, ParBNSampler},
    set, support,
    types::{Map, Result},
};
use ndarray::prelude::*;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;

// ── Helpers ─────────────────────────────────────────────────────

fn make_cat_model() -> Result<MixedBN> {
    let mut graph = DiGraph::empty(["A", "B", "C"])?;
    graph.add_edge(0, 1)?;
    graph.add_edge(0, 2)?;
    graph.add_edge(1, 2)?;

    let cpds = [
        MixedCPD::from(CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.1, 0.9]],
        )?),
        MixedCPD::from(CatCPD::new(
            support![("B", ["no", "yes"])],
            support![("A", ["no", "yes"])],
            array![[0.2, 0.8], [0.4, 0.6]],
        )?),
        MixedCPD::from(CatCPD::new(
            support![("C", ["no", "yes"])],
            support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
            array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4]],
        )?),
    ];

    MixedBN::new(graph, cpds)
}

fn make_gauss_model() -> Result<MixedBN> {
    let mut graph = DiGraph::empty(["X", "Y", "Z"])?;
    graph.add_edge(0, 1)?;
    graph.add_edge(0, 2)?;

    let cpds = [
        MixedCPD::from(GaussCPD::new(
            labels!["X"],
            labels![],
            GaussCPDP::new(Array2::zeros((1, 0)), array![0.0], array![[1.0]])?,
        )?),
        MixedCPD::from(GaussCPD::new(
            labels!["Y"],
            labels!["X"],
            GaussCPDP::new(array![[0.5]], array![1.0], array![[0.1]])?,
        )?),
        MixedCPD::from(GaussCPD::new(
            labels!["Z"],
            labels!["X"],
            GaussCPDP::new(array![[-0.3]], array![0.5], array![[0.2]])?,
        )?),
    ];

    MixedBN::new(graph, cpds)
}

// ── JSON I/O ───────────────────────────────────────────────────

#[test]
fn json_roundtrip_categorical() -> Result<()> {
    let model = make_cat_model()?;
    let json = model.to_json_string()?;
    let recovered = MixedBN::from_json_string(&json)?;
    assert_relative_eq!(model, recovered);
    Ok(())
}

#[test]
fn json_roundtrip_gaussian() -> Result<()> {
    let model = make_gauss_model()?;
    let json = model.to_json_string()?;
    let recovered = MixedBN::from_json_string(&json)?;
    assert_relative_eq!(model, recovered);
    Ok(())
}

#[test]
fn json_roundtrip_with_optionals() -> Result<()> {
    let (name, desc) = ("test_model", "A test model");
    let mut graph = DiGraph::empty(["A", "B"])?;
    graph.add_edge(0, 1)?;
    let cpds = [
        MixedCPD::from(CatCPD::new(
            support![("A", ["no", "yes"])],
            support![],
            array![[0.5, 0.5]],
        )?),
        MixedCPD::from(CatCPD::new(
            support![("B", ["no", "yes"])],
            support![("A", ["no", "yes"])],
            array![[0.2, 0.8], [0.4, 0.6]],
        )?),
    ];
    let model = MixedBN::with_optionals(Some(name.into()), Some(desc.into()), graph, cpds)?;

    let json = model.to_json_string()?;
    let recovered = MixedBN::from_json_string(&json)?;
    assert_eq!(recovered.name(), Some(name));
    assert_eq!(recovered.description(), Some(desc));
    assert_relative_eq!(model, recovered);
    Ok(())
}

#[test]
fn json_invalid_type_rejected() -> Result<()> {
    let result = MixedBN::from_json_string(r#"{"graph":{},"cpds":[],"type":"catbn"}"#);
    assert!(result.is_err());
    Ok(())
}

// ── Forward Sampling ───────────────────────────────────────────

#[test]
fn sample_categorical_mixedbn() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::new(&mut rng, &model)?;
    let sample = sampler.sample()?;

    match sample {
        causal_hub::models::MixedSample::Categorical(stats) => {
            assert_eq!(stats.len(), 3);
            assert!(stats.iter().all(|&v| v == 0 || v == 1));
        }
        _ => panic!("expected categorical sample"),
    }
    Ok(())
}

#[test]
fn sample_gaussian_mixedbn() -> Result<()> {
    let model = make_gauss_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::new(&mut rng, &model)?;
    let sample = sampler.sample()?;

    match sample {
        causal_hub::models::MixedSample::Gaussian(stats) => {
            assert_eq!(stats.len(), 3);
            assert!(stats.iter().all(|&v| v.is_finite()));
        }
        _ => panic!("expected gaussian sample"),
    }
    Ok(())
}

#[test]
fn sample_n_categorical_mixedbn() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::new(&mut rng, &model)?;
    let dataset = sampler.sample_n(100)?;

    match &dataset {
        MixedTable::Categorical(t) => {
            assert_eq!(t.values().shape(), &[100, 3]);
        }
        _ => panic!("expected categorical table"),
    }
    Ok(())
}

#[test]
fn sample_n_gaussian_mixedbn() -> Result<()> {
    let model = make_gauss_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::new(&mut rng, &model)?;
    let dataset = sampler.sample_n(100)?;

    match &dataset {
        MixedTable::Gaussian(t) => {
            assert_eq!(t.labels().len(), 3);
        }
        _ => panic!("expected gaussian table"),
    }
    Ok(())
}

#[test]
fn par_sample_n_categorical_mixedbn() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::<Xoshiro256PlusPlus, _>::new(&mut rng, &model)?;
    let dataset = sampler.par_sample_n(100)?;

    match &dataset {
        MixedTable::Categorical(t) => {
            assert_eq!(t.values().shape(), &[100, 3]);
        }
        _ => panic!("expected categorical table"),
    }
    Ok(())
}

#[test]
fn par_sample_n_gaussian_mixedbn() -> Result<()> {
    let model = make_gauss_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let sampler = ForwardSampler::<Xoshiro256PlusPlus, _>::new(&mut rng, &model)?;
    let dataset = sampler.par_sample_n(100)?;

    match &dataset {
        MixedTable::Gaussian(t) => {
            assert_eq!(t.labels().len(), 3);
        }
        _ => panic!("expected gaussian table"),
    }
    Ok(())
}

// ── Estimator Fitting ──────────────────────────────────────────

#[test]
fn mle_fit_categorical_mixedbn() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let dataset = ForwardSampler::new(&mut rng, &model)?.sample_n(1000)?;
    let graph = model.graph().clone();

    let estimator = MLE::new(&dataset);
    let fitted: MixedBN = BNEstimator::fit(&estimator, graph)?;

    assert_eq!(fitted.labels(), model.labels());
    // P(A=no) should be ≈ 0.1
    let p_a0 = match &fitted.cpds()["A"] {
        MixedCPD::Categorical(c) => c.parameters()[(0, 0)],
        _ => panic!("expected categorical"),
    };
    assert_relative_eq!(p_a0, 0.1, epsilon = 0.05);
    Ok(())
}

#[test]
fn mle_fit_gaussian_mixedbn() -> Result<()> {
    let model = make_gauss_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let dataset = ForwardSampler::new(&mut rng, &model)?.sample_n(1000)?;
    let graph = model.graph().clone();

    let estimator = MLE::new(&dataset);
    let fitted: MixedBN = BNEstimator::fit(&estimator, graph)?;

    assert_eq!(fitted.labels(), model.labels());
    Ok(())
}

#[test]
fn mle_cpd_fit_categorical() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let dataset = ForwardSampler::new(&mut rng, &model)?.sample_n(100)?;

    let estimator = MLE::new(&dataset);
    let distribution = CPDEstimator::fit(&estimator, &set![0], &set![])?;

    match distribution {
        MixedCPD::Categorical(_) => assert_eq!(distribution.labels(), &labels!["A"]),
        _ => panic!("expected categorical CPD"),
    }
    Ok(())
}

#[test]
fn mle_cpd_par_fit_categorical() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let dataset = ForwardSampler::new(&mut rng, &model)?.sample_n(100)?;

    let estimator = MLE::new(&dataset);
    let distribution = ParCPDEstimator::par_fit(&estimator, &set![0], &set![])?;

    match distribution {
        MixedCPD::Categorical(_) => assert_eq!(distribution.labels(), &labels!["A"]),
        _ => panic!("expected categorical CPD"),
    }
    Ok(())
}

// ── Approximate Inference ──────────────────────────────────────

#[test]
fn approximate_inference_categorical() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let engine = ApproximateInference::new(&mut rng, &model);
    let distribution = engine.estimate(&set![2], &set![0], None)?;

    match &distribution {
        MixedCPD::Categorical(c) => {
            // P(C=no | A=no) = 0.1*0.2 + 0.3*0.8 = 0.26
            let probability = c.parameters()[(0, 0)];
            assert_relative_eq!(probability, 0.26, epsilon = 0.05);
        }
        _ => panic!("expected categorical"),
    }
    Ok(())
}

#[test]
fn par_approximate_inference_categorical() -> Result<()> {
    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let engine = ApproximateInference::new(&mut rng, &model);
    let distribution = engine.par_estimate(&set![2], &set![0], None)?;

    match &distribution {
        MixedCPD::Categorical(c) => {
            let probability = c.parameters()[(0, 0)];
            assert_relative_eq!(probability, 0.26, epsilon = 0.05);
        }
        _ => panic!("expected categorical"),
    }
    Ok(())
}

#[test]
fn approximate_inference_gaussian() -> Result<()> {
    let model = make_gauss_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let engine = ApproximateInference::new(&mut rng, &model);
    let distribution = engine.estimate(&set![1], &set![0], None)?;

    match &distribution {
        MixedCPD::Gaussian(graph) => {
            // Y = 1.0 + 0.5*X + eps, so P(Y | X=0) has mean ≈ 1.0
            let intercept = graph.parameters().intercept()[0];
            assert_relative_eq!(intercept, 1.0, epsilon = 0.1);
        }
        _ => panic!("expected gaussian"),
    }
    Ok(())
}

#[test]
fn inference_evidence_rejected() -> Result<()> {
    use causal_hub::{datasets::CatEvT, models::MixedEv};

    let model = make_cat_model()?;
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let engine: ApproximateInference<'_, Xoshiro256PlusPlus, MixedBN, ()> =
        ApproximateInference::new(&mut rng, &model);

    let ev = MixedEv::from(causal_hub::datasets::CatEv::new(
        support![("A", ["no", "yes"])],
        vec![CatEvT::CertainPositive { event: 0, state: 0 }],
    )?);

    let result = engine.estimate(&set![2], &set![0], Some(&ev));
    assert!(result.is_err());
    Ok(())
}

// ── Random Generation ─────────────────────────────────────────

#[test]
fn rng_mixedbn_categorical() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let mut support: Map<String, MixedSupport> = Map::default();
    support.insert(
        "A".into(),
        MixedSupport::Categorical(support![("A", ["no", "yes"])]),
    );
    support.insert(
        "B".into(),
        MixedSupport::Categorical(support![("B", ["no", "yes"])]),
    );

    let labels = support.keys().cloned().collect();
    let mut generator =
        causal_hub::random::RngMixedBN::new(&mut rng, &labels, &support, 1.0, 1.0, 1.0, 0.01, 0.5)?;
    let model = generator.random()?;

    assert_eq!(model.labels().len(), 2);
    for distribution in model.cpds().values() {
        assert!(matches!(distribution, MixedCPD::Categorical(_)));
    }
    Ok(())
}

#[test]
fn rng_mixedbn_gaussian() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

    let mut support: Map<String, MixedSupport> = Map::default();
    support.insert("X".into(), MixedSupport::Gaussian(Default::default()));
    support.insert("Y".into(), MixedSupport::Gaussian(Default::default()));

    let labels = support.keys().cloned().collect();
    let mut generator =
        causal_hub::random::RngMixedBN::new(&mut rng, &labels, &support, 1.0, 1.0, 1.0, 0.01, 0.5)?;
    let model = generator.random()?;

    assert_eq!(model.labels().len(), 2);
    for distribution in model.cpds().values() {
        assert!(matches!(distribution, MixedCPD::Gaussian(_)));
    }
    Ok(())
}

#[test]
fn rng_mixedbn_validation() -> Result<()> {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);
    let support = Map::<String, MixedSupport>::default();
    let labels = support.keys().cloned().collect();

    assert!(
        causal_hub::random::RngMixedBN::new(&mut rng, &labels, &support, -1.0, 1.0, 1.0, 0.01, 0.5)
            .is_err()
    );
    Ok(())
}

// ── Backward Compat: MixedBN JSON Roundtrip ──────────────────

#[test]
fn from_catbn_equivalent_roundtrip() -> Result<()> {
    let mixed_bn = {
        let mut graph = DiGraph::empty(["A", "B", "C"])?;
        graph.add_edge(0, 1)?;
        graph.add_edge(0, 2)?;
        graph.add_edge(1, 2)?;
        let cpds = [
            MixedCPD::from(CatCPD::new(
                support![("A", ["no", "yes"])],
                support![],
                array![[0.5, 0.5]],
            )?),
            MixedCPD::from(CatCPD::new(
                support![("B", ["no", "yes"])],
                support![("A", ["no", "yes"])],
                array![[0.2, 0.8], [0.4, 0.6]],
            )?),
            MixedCPD::from(CatCPD::new(
                support![("C", ["no", "yes"])],
                support![("A", ["no", "yes"]), ("B", ["no", "yes"])],
                array![[0.1, 0.9], [0.3, 0.7], [0.5, 0.5], [0.6, 0.4]],
            )?),
        ];
        MixedBN::new(graph, cpds)?
    };

    let json = mixed_bn.to_json_string()?;
    let recovered = MixedBN::from_json_string(&json)?;
    assert_relative_eq!(mixed_bn, recovered);
    Ok(())
}
