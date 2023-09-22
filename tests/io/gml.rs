#[cfg(test)]
mod parser {
    use causal_hub::{
        io::{File, GML},
        prelude::*,
    };
    use itertools::Itertools;

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/gml")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| x.extension().unwrap().eq("gml"))
            .for_each(|x| {
                let gml = GML::read(&x);
                assert!(gml.is_ok(), "{}: {:?}", x.display(), gml.err());
            });
    }

    #[test]
    fn gml_to_graph() {
        let gml = GML::read("tests/assets/gml/0.gml").unwrap();
        let g = Graph::from(gml);

        assert_eq!(L!(g).collect_vec(), ["13", "5"]);
        assert_eq!(E!(g).collect_vec(), [(0, 1)]);
    }

    #[test]
    fn graph_to_gml() {
        let g = Graph::new(["A", "B", "C"], [("A", "B"), ("B", "C")]);
        let gml_string: String = GML::from(g).try_into().unwrap();

        assert_eq!(
            gml_string,
            "graph [\n\tgraphType \"graph\"\n\tnode [\n\t\tid 0\n\t\tlabel \"A\"\n\t]\n\tnode [\n\t\tid 1\n\t\tlabel \"B\"\n\t]\n\tnode [\n\t\tid 2\n\t\tlabel \"C\"\n\t]\n\tedge [\n\t\tsource 0\n\t\ttarget 1\n\t]\n\tedge [\n\t\tsource 1\n\t\ttarget 2\n\t]\n]\n"
        );
    }

    #[test]
    fn gml_to_digraph() {
        let gml = GML::read("tests/assets/gml/1.gml").unwrap();
        let g = DiGraph::from(gml);

        assert_eq!(L!(g).collect_vec(), ["Node 1", "Node 2", "Node 3"]);
        assert_eq!(E!(g).collect_vec(), [(0, 1), (1, 2), (2, 0)]);
    }

    #[test]
    fn digraph_to_gml() {
        let g = DiGraph::new(["A", "B", "C"], [("A", "B"), ("B", "C")]);
        let gml_string: String = GML::from(g).try_into().unwrap();

        assert_eq!(
            gml_string,
            "graph [\n\tdirected 1\n\tnode [\n\t\tid 0\n\t\tlabel \"A\"\n\t]\n\tnode [\n\t\tid 1\n\t\tlabel \"B\"\n\t]\n\tnode [\n\t\tid 2\n\t\tlabel \"C\"\n\t]\n\tedge [\n\t\tsource 0\n\t\ttarget 1\n\t]\n\tedge [\n\t\tsource 1\n\t\ttarget 2\n\t]\n]\n"
        );
    }
}
