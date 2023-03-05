#[cfg(test)]
mod parser {
    use causal_hub::io::{BIF, File};

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/bif")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| x.extension().unwrap().eq("bif"))
            .for_each(|x| {
                let bif = BIF::read(&x);
                assert!(bif.is_ok(), "{}: {:?}", x.display(), bif.err());
            });
    }
}
