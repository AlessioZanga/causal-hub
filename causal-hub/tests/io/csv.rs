#[cfg(test)]
mod tests {
    use std::io::Write;

    use causal_hub::{
        datasets::{CatIncTable, CatTable, Dataset, GaussTable, IncDataset},
        io::CsvIO,
        labels,
        models::Labelled,
        types::Result,
    };
    use ndarray::prelude::*;
    use tempfile::NamedTempFile;

    mod table {
        use super::*;

        mod categorical {
            use super::*;

            mod complete {
                use super::*;

                #[test]
                fn from_csv_reader() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,no,yes\n",
                        "no,yes,yes\n",
                        "yes,yes,yes"
                    );
                    let dataset = CatTable::from_csv_string(csv)?;

                    let values = array![[0, 0, 0], [0, 0, 1], [0, 1, 1], [1, 1, 1]];

                    assert_eq!(dataset.labels(), &labels!["A", "B", "C"]);
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

                    Ok(())
                }

                #[test]
                fn from_csv_reader_malformed_record() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,no\n",
                        "no,yes,yes\n",
                        "yes,yes,yes"
                    );
                    assert!(CatTable::from_csv_string(csv).is_err());

                    Ok(())
                }

                #[test]
                fn from_csv_reader_missing_value() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,,no\n",
                        "no,yes,yes\n",
                        "yes,yes,yes"
                    );
                    assert!(CatTable::from_csv_string(csv).is_err());

                    Ok(())
                }

                #[test]
                fn to_csv_writer() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,no,yes\n",
                        "no,yes,yes\n",
                        "yes,yes,yes\n"
                    );
                    let dataset = CatTable::from_csv_string(csv)?;

                    // Create a buffer to write the CSV data.
                    let mut buffer = Vec::new();
                    dataset.to_csv_writer(&mut buffer)?;

                    // Assert that the written CSV matches the original.
                    assert_eq!(String::from_utf8(buffer)?, csv);

                    Ok(())
                }
            }

            mod incomplete {
                use super::*;

                const M: <CatIncTable as IncDataset>::Missing = CatIncTable::MISSING;

                #[test]
                fn from_csv_reader() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,,yes\n",
                        ",yes,yes\n",
                        "yes,yes,yes"
                    );
                    let dataset = CatIncTable::from_csv_string(csv)?;

                    let values = array![[0, 0, 0], [0, M, 1], [M, 1, 1], [1, 1, 1]];

                    assert_eq!(dataset.labels(), &labels!["A", "B", "C"]);
                    assert!(
                        dataset
                            .states()
                            .values()
                            .all(|x| x.iter().eq(["no", "yes"]))
                    );
                    assert_eq!(dataset.values(), &values);

                    Ok(())
                }

                #[test]
                fn from_csv_reader_malformed_record() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,no\n",
                        ",yes,yes\n",
                        "yes,yes,yes"
                    );
                    assert!(CatIncTable::from_csv_string(csv).is_err());

                    Ok(())
                }

                #[test]
                fn to_csv_writer() -> Result<()> {
                    let csv = concat!(
                        "A,B,C\n",
                        "no,no,no\n",
                        "no,,yes\n",
                        ",yes,yes\n",
                        "yes,yes,yes\n"
                    );
                    let dataset = CatIncTable::from_csv_string(csv)?;

                    // Create a buffer to write the CSV data.
                    let mut buffer = Vec::new();
                    dataset.to_csv_writer(&mut buffer)?;

                    // Assert that the written CSV matches the original.
                    assert_eq!(String::from_utf8(buffer)?, csv);

                    Ok(())
                }
            }
        }

        mod gaussian {
            use super::*;

            #[test]
            fn from_csv_reader() -> Result<()> {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5,6\n", //
                    "7,8,9\n"
                );
                let dataset = GaussTable::from_csv_string(csv)?;

                let values = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];

                assert_eq!(dataset.labels(), &labels!["X", "Y", "Z"]);
                assert_eq!(dataset.values(), &values);

                Ok(())
            }

            #[test]
            fn from_csv_reader_malformed_record() -> Result<()> {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5\n",   //
                    "7,8,9\n"
                );
                assert!(GaussTable::from_csv_string(csv).is_err());

                Ok(())
            }

            #[test]
            fn from_csv_reader_missing_value() -> Result<()> {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,,6\n",  //
                    "7,8,9\n"
                );
                assert!(GaussTable::from_csv_string(csv).is_err());

                Ok(())
            }

            #[test]
            fn to_csv_writer() -> Result<()> {
                let csv = concat!(
                    "X,Y,Z\n", //
                    "1,2,3\n", //
                    "4,5,6\n", //
                    "7,8,9\n"
                );
                let dataset = GaussTable::from_csv_string(csv)?;

                // Create a buffer to write the CSV data.
                let mut buffer = Vec::new();
                dataset.to_csv_writer(&mut buffer)?;

                // Assert that the written CSV matches the original.
                assert_eq!(String::from_utf8(buffer)?, csv);

                Ok(())
            }
        }
    }

    // Tests for file-based CSV operations.
    mod file_operations {
        use super::*;

        #[test]
        fn from_csv_file_categorical() -> Result<()> {
            // Create a temporary file with CSV content.
            let mut temp_file = NamedTempFile::new()?;
            let csv_content = concat!("A,B,C\n", "yes,no,no\n", "no,yes,yes\n", "yes,yes,no\n");
            temp_file.write_all(csv_content.as_bytes())?;

            // Read from the file.
            let path = temp_file.path().to_str().unwrap();
            let dataset = CatTable::from_csv_file(path)?;

            assert_eq!(dataset.labels(), &labels!["A", "B", "C"]);
            assert_eq!(dataset.values().nrows(), 3);

            Ok(())
        }

        #[test]
        fn to_csv_file_categorical() -> Result<()> {
            let csv = concat!("X,Y\n", "a,b\n", "c,d\n");
            let dataset = CatTable::from_csv_string(csv)?;

            // Create a temporary file path.
            let temp_file = NamedTempFile::new()?;
            let path = temp_file.path().to_str().unwrap();

            // Write to the file.
            dataset.to_csv_file(path)?;

            // Read back and verify.
            let restored = CatTable::from_csv_file(path)?;
            assert_eq!(restored.labels(), dataset.labels());
            assert_eq!(restored.values(), dataset.values());

            Ok(())
        }

        #[test]
        fn from_csv_file_gaussian() -> Result<()> {
            // Create a temporary file with CSV content.
            let mut temp_file = NamedTempFile::new()?;
            let csv_content = concat!("X,Y,Z\n", "1.5,2.5,3.5\n", "4.5,5.5,6.5\n");
            temp_file.write_all(csv_content.as_bytes())?;

            // Read from the file.
            let path = temp_file.path().to_str().unwrap();
            let dataset = GaussTable::from_csv_file(path)?;

            assert_eq!(dataset.labels(), &labels!["X", "Y", "Z"]);
            assert_eq!(dataset.values().nrows(), 2);

            Ok(())
        }

        #[test]
        fn to_csv_file_gaussian() -> Result<()> {
            let csv = concat!("A,B\n", "1.0,2.0\n", "3.0,4.0\n");
            let dataset = GaussTable::from_csv_string(csv)?;

            // Create a temporary file path.
            let temp_file = NamedTempFile::new()?;
            let path = temp_file.path().to_str().unwrap();

            // Write to the file.
            dataset.to_csv_file(path)?;

            // Read back and verify.
            let restored = GaussTable::from_csv_file(path)?;
            assert_eq!(restored.labels(), dataset.labels());
            assert_eq!(restored.values(), dataset.values());

            Ok(())
        }

        #[test]
        fn from_csv_file_incomplete() -> Result<()> {
            // Create a temporary file with CSV content.
            let mut temp_file = NamedTempFile::new()?;
            let csv_content = concat!("A,B\n", "yes,no\n", ",yes\n", "no,\n");
            temp_file.write_all(csv_content.as_bytes())?;

            // Read from the file.
            let path = temp_file.path().to_str().unwrap();
            let dataset = CatIncTable::from_csv_file(path)?;

            assert_eq!(dataset.labels(), &labels!["A", "B"]);
            assert_eq!(dataset.values().nrows(), 3);

            Ok(())
        }

        #[test]
        fn to_csv_file_incomplete() -> Result<()> {
            let csv = concat!("X,Y\n", "a,\n", ",b\n");
            let dataset = CatIncTable::from_csv_string(csv)?;

            // Create a temporary file path.
            let temp_file = NamedTempFile::new()?;
            let path = temp_file.path().to_str().unwrap();

            // Write to the file.
            dataset.to_csv_file(path)?;

            // Read back and verify.
            let restored = CatIncTable::from_csv_file(path)?;
            assert_eq!(restored.labels(), dataset.labels());

            Ok(())
        }

        #[test]
        fn from_csv_file_not_found() -> Result<()> {
            let res = CatTable::from_csv_file("/nonexistent/path/to/file.csv");
            assert!(res.is_err());
            Ok(())
        }

        #[test]
        fn to_csv_string() -> Result<()> {
            let csv = concat!("A,B,C\n", "x,y,z\n", "a,b,c\n");
            let dataset = CatTable::from_csv_string(csv)?;

            let output = dataset.to_csv_string()?;
            assert_eq!(output, csv);

            Ok(())
        }

        #[test]
        fn to_csv_string_gaussian() -> Result<()> {
            let csv = concat!("X,Y\n", "1.5,2.5\n", "3.5,4.5\n");
            let dataset = GaussTable::from_csv_string(csv)?;

            let output = dataset.to_csv_string()?;
            assert_eq!(output, csv);

            Ok(())
        }

        #[test]
        fn roundtrip_csv_string() -> Result<()> {
            let original_csv = concat!("P,Q,R\n", "a,b,c\n", "d,e,f\n", "g,h,i\n");
            let dataset = CatTable::from_csv_string(original_csv)?;
            let exported_csv = dataset.to_csv_string()?;
            let restored = CatTable::from_csv_string(&exported_csv)?;

            assert_eq!(restored.labels(), dataset.labels());
            assert_eq!(restored.values(), dataset.values());

            Ok(())
        }

        #[test]
        fn roundtrip_csv_file() -> Result<()> {
            let original_csv = concat!("Alpha,Beta,Gamma\n", "1.1,2.2,3.3\n", "4.4,5.5,6.6\n");
            let dataset = GaussTable::from_csv_string(original_csv)?;

            // Create a temporary file path.
            let temp_file = NamedTempFile::new()?;
            let path = temp_file.path().to_str().unwrap();

            // Write to file and read back.
            dataset.to_csv_file(path)?;
            let restored = GaussTable::from_csv_file(path)?;

            assert_eq!(restored.labels(), dataset.labels());
            assert_eq!(restored.values(), dataset.values());

            Ok(())
        }
    }
}
