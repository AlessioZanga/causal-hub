#[cfg(test)]
mod attributes {
    include!("./attributes.rs");
}

#[cfg(test)]
mod parser {
    use causal_hub::{
        io::{File, DOT},
        prelude::*,
    };

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/dot")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| x.extension().unwrap().eq("dot"))
            .for_each(|x| {
                let dot = DOT::read(&x);
                assert!(dot.is_ok(), "{}: {:?}", x.display(), dot.err());
            });
    }

    #[test]
    fn from_graph() {
        let dot = DOT::read("tests/assets/dot/1658.dot").unwrap();
        let g = Graph::from(dot);

        assert!(L!(g).eq([
            "0", "1", "2", "3", "ADC1", "ADC2", "ADC3", "Charger", "GND", "IHall1", "IHall2",
            "IHall3", "OpAmp1", "OpAmp2", "OpAmp3", "Temp1", "Temp2", "Temp3", "V1", "V2", "V3"
        ]));
    }

    #[test]
    fn from_digraph() {
        let dot = DOT::read("tests/assets/dot/14.dot").unwrap();
        let g = DiGraph::from(dot);

        assert!(L!(g).eq(["a", "b"]));
    }

    #[test]
    fn from_partiallydirectedgraph() {
        let dot = DOT::read("tests/assets/dot/1999.dot").unwrap();
        let g = PDGraph::from(dot);
        assert!(L!(g).eq(["0", "1", "2", "3", "4"]));
    }
}

#[cfg(test)]
mod plot {
    use std::path::Path;

    use causal_hub::{
        io::{File, DOT},
        prelude::*,
    };

    #[test]
    fn plot() {
        let path = Path::new("tests/assets/dot/1880.dot");
        let dot = DOT::read(path).unwrap();
        let dot = dot.plot(Path::new("tests/assets/dot/1880.pdf"));
        assert!(dot.is_ok(), "{}: {:?}", path.display(), dot.err());
    }
}
