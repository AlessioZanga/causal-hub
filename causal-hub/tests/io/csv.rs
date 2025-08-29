#[cfg(test)]
mod tests {
    use causal_hub::{
        datasets::{CatData, Dataset},
        io::CsvIO,
    };
    use ndarray::prelude::*;

    #[test]
    fn test_from_csv_reader() {
        let csv = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,no,yes\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let dataset = CatData::from_csv(csv);

        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];

        assert!(dataset.labels().iter().eq(["A", "B", "C"]));
        assert!(
            dataset
                .states()
                .values()
                .all(|x| x.iter().eq(["no", "yes"]))
        );
        assert_eq!(dataset.values(), &values);

        assert_eq!(
            dataset.to_string(),
            concat!(
                "-------------------\n",
                "| A   | B   | C   |\n",
                "| --- | --- | --- |\n",
                "| no  | no  | no  |\n",
                "| no  | no  | yes |\n",
                "| no  | yes | yes |\n",
                "| yes | yes | yes |\n",
                "-------------------\n",
            )
        );
    }

    #[test]
    #[should_panic(expected = "Malformed record on line 2.")]
    fn test_from_csv_reader_malformed_record() {
        let csv = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,no\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let _ = CatData::from_csv(csv);
    }

    #[test]
    #[should_panic(expected = "Missing value on line 2.")]
    fn test_from_csv_reader_missing_value() {
        let csv = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,,no\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let _ = CatData::from_csv(csv);
    }
}
