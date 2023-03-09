#[cfg(test)]
mod parser {
    use causal_hub::{
        io::{File, BIF},
        prelude::*,
    };

    #[test]
    fn read() {
        // Test for each scenario.
        std::fs::read_dir("tests/assets/bif")
            .expect("No such file or directory")
            .map(|x| x.unwrap().path())
            .filter(|x| x.extension().unwrap().eq("bif"))
            .map(|x| {
                let bif = BIF::read(&x);
                assert!(bif.is_ok(), "{}: {:?}", x.display(), bif.err());
                bif.unwrap()
            })
            .for_each(|bif| {
                let b: DiscreteBayesianNetwork = bif.into();
                let (g, t) = (b.graph(), b.parameters());
                // Assert vertices labels are coherent.
                assert!(g.labels().eq(t.keys()));
                // Assert parents labels are coherent.
                for x in V!(g) {
                    // Get parents labels from graph.
                    let g_pa_x = Pa!(g, x).map(|y| g.label(y));
                    // Get parents labels from parameters.
                    let t_pa_x = t[g.label(x)].scope().filter(|&z| z != g.label(x));

                    assert!(g_pa_x.eq(t_pa_x));
                }
            });
    }

    #[test]
    #[ignore]
    fn write() {
        // FIXME:
        println!(
            "{}",
            Into::<String>::into(BIF::read("tests/assets/bif/asia.bif").unwrap())
        );
        panic!();
    }
}
