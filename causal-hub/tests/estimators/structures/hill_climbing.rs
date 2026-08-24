#[cfg(test)]
mod tests {
    use causal_hub::{
        assets::load_asia,
        estimators::{BIC, HC, MLE, PK, ScoringCriterion},
        inference::TopologicalOrder,
        models::{BN, CatBN, DiGraph, GaussBN, Graph, HasLabels},
        random::{Random, RngGaussBN},
        samplers::{ForwardSampler, ParBNSampler},
        set,
        types::{Cache, Error, ErrorKind, Labels, Result, Set},
    };
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256PlusPlus;

    /// Computes the total score of a graph given the scoring criterion.
    fn total_score<S>(graph: &DiGraph, score: &S) -> Result<f64>
    where
        S: ScoringCriterion,
    {
        let mut total = 0.;
        // Sum the local scores of each vertex given its parents.
        for y in graph.vertices() {
            let pa: Set<usize> = graph.parents(&set![y])?;
            total += score.call(&set![y], &pa)?;
        }
        Ok(total)
    }

    #[test]
    fn hc_fit() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Initialize the HC algorithm with the default empty initial graph.
        let hc = HC::new(&bic);
        // Run the HC algorithm.
        let fitted_model: CatBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph is acyclic ...
        assert!(fitted_graph.topological_order().is_some());
        // ... and that its score matches the score of the true graph,
        // i.e., the learned structure is Markov-equivalent to the true one.
        let fitted_score = total_score(fitted_graph, &bic)?;
        let true_score = total_score(model.graph(), &bic)?;
        assert!(
            (fitted_score - true_score).abs() < 1e-6,
            "Fitted score {fitted_score} differs from true score {true_score}"
        );

        Ok(())
    }

    #[test]
    fn hc_par_fit() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Initialize the HC algorithm with the default empty initial graph.
        let hc = HC::new(&bic);
        // Run the HC algorithm in parallel.
        let fitted_model: CatBN = hc.par_fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph is acyclic ...
        assert!(fitted_graph.topological_order().is_some());
        // ... and that its score matches the score of the true graph,
        // i.e., the learned structure is Markov-equivalent to the true one.
        let fitted_score = total_score(fitted_graph, &bic)?;
        let true_score = total_score(model.graph(), &bic)?;
        assert!(
            (fitted_score - true_score).abs() < 1e-6,
            "Fitted score {fitted_score} differs from true score {true_score}"
        );

        Ok(())
    }

    #[test]
    fn hc_with_initial_graph() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Get an edge of the true graph as the initial graph.
        let Some((x, y)) = model.graph().edges().into_iter().next() else {
            panic!("The true graph must have at least one edge");
        };
        let mut initial_graph = DiGraph::empty(dataset.labels())?;
        assert!(initial_graph.add_edge(x, y)?);

        // Initialize the HC algorithm with the custom initial graph.
        let hc = HC::new(&bic).with_initial_graph(&initial_graph)?;
        // Run the HC algorithm.
        let fitted_model: CatBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph score matches the score of the true graph.
        let fitted_score = total_score(fitted_graph, &bic)?;
        let true_score = total_score(model.graph(), &bic)?;
        assert!(
            (fitted_score - true_score).abs() < 1e-6,
            "Fitted score {fitted_score} differs from true score {true_score}"
        );

        Ok(())
    }

    #[test]
    fn hc_with_max_parents() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Initialize the HC algorithm with maximum one parent per vertex.
        let hc = HC::new(&bic).with_max_parents(1);
        // Run the HC algorithm.
        let fitted_model: CatBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that every vertex has at most one parent.
        for x in fitted_graph.vertices() {
            assert!(fitted_graph.parents(&set![x])?.len() <= 1);
        }

        Ok(())
    }

    #[test]
    fn hc_prior_knowledge_forbidden_edge() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Get an edge of the true graph to be forbidden.
        let Some((x, y)) = model.graph().edges().into_iter().next() else {
            panic!("The true graph must have at least one edge");
        };
        // Initialize empty prior knowledge with the forbidden edge.
        let prior_knowledge = PK::new(
            dataset.labels().clone(),
            [(x, y)],
            [],
            Vec::<Vec<usize>>::new(),
        )?;

        // Initialize the HC algorithm.
        let hc = HC::new(&bic).with_prior_knowledge(&prior_knowledge)?;
        // Run the HC algorithm.
        let fitted_model: CatBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the forbidden edge is not in the fitted graph.
        assert!(!fitted_graph.has_edge(x, y)?);

        Ok(())
    }

    #[test]
    fn hc_prior_knowledge_required_edge() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Get a pair of vertices not connected in the true graph to be required.
        let Some((x, y)) = model
            .graph()
            .vertices()
            .into_iter()
            .flat_map(|i| model.graph().vertices().into_iter().map(move |j| (i, j)))
            .find(|&(i, j)| i != j && !model.graph().has_edge(i, j).unwrap_or(true))
        else {
            panic!("The true graph complement must have at least one edge");
        };
        // Initialize empty prior knowledge with the required edge.
        let prior_knowledge = PK::new(
            dataset.labels().clone(),
            [],
            [(x, y)],
            Vec::<Vec<usize>>::new(),
        )?;

        // Initialize the HC algorithm.
        let hc = HC::new(&bic).with_prior_knowledge(&prior_knowledge)?;
        // Run the HC algorithm.
        let fitted_model: CatBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the required edge is in the fitted graph.
        assert!(fitted_graph.has_edge(x, y)?);

        Ok(())
    }

    #[test]
    fn hc_label_mismatch() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Set an initial graph with mismatching labels.
        let initial_graph = DiGraph::empty(["A", "B", "C"])?;

        // Match error kind.
        match HC::new(&bic).with_initial_graph(&initial_graph) {
            Err(err) => assert!(matches!(
                err,
                Error {
                    kind: ErrorKind::LabelMismatch(_, _),
                    ..
                }
            )),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn hc_prior_knowledge_conflict() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Get an edge of the true graph to be both in the initial graph and forbidden.
        let Some((x, y)) = model.graph().edges().into_iter().next() else {
            panic!("The true graph must have at least one edge");
        };
        // Initialize prior knowledge forbidding an edge present in the initial graph.
        let prior_knowledge = PK::new(
            dataset.labels().clone(),
            [(x, y)],
            [],
            Vec::<Vec<usize>>::new(),
        )?;

        // Set an initial graph containing the forbidden edge.
        let mut initial_graph = DiGraph::empty(dataset.labels())?;
        assert!(initial_graph.add_edge(x, y)?);

        // Match error kind.
        match HC::new(&bic)
            .with_initial_graph(&initial_graph)
            .and_then(|hc| hc.with_prior_knowledge(&prior_knowledge))
        {
            Err(err) => {
                assert!(matches!(
                    err,
                    Error {
                        kind: ErrorKind::PriorKnowledgeConflict(_),
                        ..
                    }
                ))
            }
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn hc_not_a_dag() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load Asia.
        let model = load_asia()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Set an initial graph with a cycle.
        let mut initial_graph = DiGraph::empty(dataset.labels())?;
        assert!(initial_graph.add_edge(0, 1)?);
        assert!(initial_graph.add_edge(1, 2)?);
        assert!(initial_graph.add_edge(2, 0)?);

        // Initialize the HC algorithm.
        let hc = HC::new(&bic).with_initial_graph(&initial_graph)?;

        // Fit the model.
        let fitted_model: Result<CatBN> = hc.fit();

        // Match error kind.
        match fitted_model {
            Err(err) => assert!(matches!(
                err,
                Error {
                    kind: ErrorKind::NotADag,
                    ..
                }
            )),
            _ => panic!("Should be error"),
        };

        Ok(())
    }

    #[test]
    fn hc_gaussian_fit() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Sample a random Gaussian BN with ten vertices.
        let labels: Labels = (0..10).map(|i| format!("X{i}")).collect();
        let model = RngGaussBN::new(&mut rng, &labels, 1.0, 1.0, 1e-6, 0.3)?.random()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Initialize the HC algorithm with the default empty initial graph.
        let hc = HC::new(&bic);
        // Run the HC algorithm.
        let fitted_model: GaussBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph is acyclic.
        assert!(fitted_graph.topological_order().is_some());

        // Assert that the skeleton of the fitted graph recalls most of the
        // true dependencies and does not add many spurious edges.
        let true_skeleton: Set<(usize, usize)> = model
            .graph()
            .edges()
            .into_iter()
            .map(|(x, y)| if x <= y { (x, y) } else { (y, x) })
            .collect();
        let fitted_skeleton: Set<(usize, usize)> = fitted_graph
            .edges()
            .into_iter()
            .map(|(x, y)| if x <= y { (x, y) } else { (y, x) })
            .collect();
        let true_positives = fitted_skeleton.intersection(&true_skeleton).count() as f64;
        let recall = true_positives / true_skeleton.len() as f64;
        let precision = true_positives / fitted_skeleton.len() as f64;
        assert!(recall >= 0.8, "Skeleton recall {recall} is too low");
        assert!(
            precision >= 0.8,
            "Skeleton precision {precision} is too low"
        );

        Ok(())
    }

    #[test]
    fn hc_gaussian_par_fit() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Sample a random Gaussian BN with ten vertices.
        let labels: Labels = (0..10).map(|i| format!("X{i}")).collect();
        let model = RngGaussBN::new(&mut rng, &labels, 1.0, 1.0, 1e-6, 0.3)?.random()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Initialize the HC algorithm with the default empty initial graph.
        let hc = HC::new(&bic);
        // Run the HC algorithm in parallel.
        let fitted_model: GaussBN = hc.par_fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph is acyclic.
        assert!(fitted_graph.topological_order().is_some());

        // Assert that the skeleton of the fitted graph recalls most of the
        // true dependencies and does not add many spurious edges.
        let true_skeleton: Set<(usize, usize)> = model
            .graph()
            .edges()
            .into_iter()
            .map(|(x, y)| if x <= y { (x, y) } else { (y, x) })
            .collect();
        let fitted_skeleton: Set<(usize, usize)> = fitted_graph
            .edges()
            .into_iter()
            .map(|(x, y)| if x <= y { (x, y) } else { (y, x) })
            .collect();
        let true_positives = fitted_skeleton.intersection(&true_skeleton).count() as f64;
        let recall = true_positives / true_skeleton.len() as f64;
        let precision = true_positives / fitted_skeleton.len() as f64;
        assert!(recall >= 0.8, "Skeleton recall {recall} is too low");
        assert!(
            precision >= 0.8,
            "Skeleton precision {precision} is too low"
        );

        Ok(())
    }

    #[test]
    #[ignore = "slow; run manually in release mode"]
    fn hc_ecoli70_fit() -> Result<()> {
        // Initialize a random number generator.
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(42);

        // Load EColi70.
        let model = causal_hub::assets::load_ecoli70()?;
        // Initialize a sampler.
        let forward = ForwardSampler::new(&mut rng, &model)?;
        // Sample 10000 samples.
        let dataset = forward.par_sample_n(10_000)?;

        // Initialize a parameter estimator.
        let estimator = MLE::new(&dataset);
        // Cache the parameter estimator.
        let cache = Cache::new(&estimator);
        // Initialize the scoring criterion.
        let bic = BIC::new(&cache);

        // Set the empty graph as the score baseline.
        let initial_graph = DiGraph::empty(dataset.labels())?;
        // Initialize the HC algorithm with the default empty initial graph.
        let hc = HC::new(&bic);
        // Run the HC algorithm.
        let fitted_model: GaussBN = hc.fit()?;
        let fitted_graph = fitted_model.graph();

        // Assert that the fitted graph is acyclic and spans the same variables.
        assert!(fitted_graph.topological_order().is_some());
        assert_eq!(fitted_graph.vertices(), model.graph().vertices());
        // Assert that the search improved over the empty graph.
        assert!(total_score(fitted_graph, &bic)? > total_score(&initial_graph, &bic)?);

        Ok(())
    }
}
