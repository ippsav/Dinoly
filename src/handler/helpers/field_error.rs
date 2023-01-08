use std::collections::HashMap;

use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub fields: Option<HashMap<String, String>>,
}

impl From<ValidationErrors> for ResponseError {
    fn from(v: ValidationErrors) -> Self {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        v.field_errors().into_iter().for_each(|(k, v)| {
            let msg = format!("invalid {}", v[0].code);

            hash_map.insert(k.into(), msg);
        });
        Self {
            fields: Some(hash_map),
        }
    }
}
