pub trait JsonIO {
    fn from_json(json: &str) -> Self;
    fn to_json(&self) -> String;
    fn read_json(path: &str) -> Self;
    fn write_json(&self, path: &str);
}

#[macro_export]
macro_rules! impl_json_io {
    ($type:ty) => {
        impl $crate::io::JsonIO for $type {
            fn from_json(json: &str) -> Self {
                serde_json::from_str(json).unwrap()
            }

            fn to_json(&self) -> String {
                serde_json::to_string(self).unwrap()
            }

            fn read_json(path: &str) -> Self {
                use std::{fs::File, io::BufReader};
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap()
            }

            fn write_json(&self, path: &str) {
                use std::{fs::File, io::BufWriter};
                let file = File::create(path).unwrap();
                let writer = BufWriter::new(file);
                serde_json::to_writer(writer, self).unwrap()
            }
        }
    };
}
