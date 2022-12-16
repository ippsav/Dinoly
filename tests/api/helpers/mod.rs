use serde_json::Value;

mod json;
pub mod server;

pub use json::*;

pub struct TestCase<'a> {
    pub input: &'a Value,
    pub error: &'a Value,
}
impl<'a> TestCase<'a> {
    pub fn gen_test_cases(inputs_with_error: &'a [(Value, Value)]) -> Vec<Self> {
        let mut test_cases = Vec::new();
        for (input, error) in inputs_with_error.into_iter() {
            let test_case = TestCase { input, error };
            test_cases.push(test_case);
        }

        test_cases
    }
}
