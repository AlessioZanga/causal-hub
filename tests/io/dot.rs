#[cfg(test)]
mod attributes {
    include!("./attributes.rs");
}

#[cfg(test)]
mod parser {
    use causal_hub::io::{File, DOT};

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
