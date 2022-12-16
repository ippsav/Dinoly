use serde::Deserialize;
use serde_json::Value;
use std::fs::File;

#[derive(Deserialize)]
pub struct TestCase {
    pub input: Value,
    pub error: Value,
}

impl TestCase {
    pub fn gen_test_cases_from_file(file_name: &'static str) -> Vec<Self> {
        // Getting path to input file
        let mut current_dir = std::env::current_dir().expect("couldn't get current directory");
        let path_to_file = &format!("tests/api/inputs/{}", file_name);
        current_dir.push(path_to_file);
        current_dir.set_extension("json");

        // Open file
        let file = File::open(current_dir).expect("couldn't open file");

        // Deserialize from file
        let test_cases: Vec<TestCase> =
            serde_json::from_reader(file).expect("couldn't deserialize json data from reader");

        test_cases
    }
}
