#[cfg(test)]
mod tests {
    use causal_hub::io::DOT;

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/dot")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| !x.extension().unwrap().eq("ignore"))
            .for_each(|x| {
                let dot = DOT::read(&x);
                assert!(dot.is_ok(), "{}: {:?}", x.display(), dot.err());
            });
    }
}
