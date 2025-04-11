#[cfg(test)]
mod tests {
    use causal_hub_next::{
        data::{CategoricalData, Data},
        io::FromCsvReader,
    };
    use csv::ReaderBuilder;
    use ndarray::prelude::*;

    #[test]
    fn test_from_csv_reader() {
        let reader = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,no,yes\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader.as_bytes());
        let data = CategoricalData::from_csv_reader(reader);

        let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];

        assert!(data.labels().iter().eq(["A", "B", "C"]));
        assert!(data.states().values().all(|x| x.iter().eq(["no", "yes"])));
        assert_eq!(data.values(), &values);

        assert_eq!(
            data.to_string(),
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
        let reader = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,no\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader.as_bytes());
        let _ = CategoricalData::from_csv_reader(reader);
    }

    #[test]
    #[should_panic(expected = "Missing value on line 2.")]
    fn test_from_csv_reader_missing_value() {
        let reader = concat!(
            "A,B,C\n",
            "no,no,no\n",
            "no,,no\n",
            "no,yes,yes\n",
            "yes,yes,yes"
        );
        let reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader.as_bytes());
        let _ = CategoricalData::from_csv_reader(reader);
    }
}
