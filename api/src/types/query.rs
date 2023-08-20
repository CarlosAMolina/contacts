use handle_errors::Error;
use std::collections::HashMap;

/// Extract query from params.
pub fn extract_query(params: HashMap<String, String>) -> Result<String, Error> {
    if params.contains_key("query") {
        let query = params.get("query").unwrap().clone();
        return Ok(query);
    } else {
        return Err(Error::MissingParameters);
    };
}

#[cfg(test)]
mod tests_extract_query{
    use super::*;

    #[test]
    fn test_extract_query_if_valid_parameters() {
        let mut params = HashMap::new();
        params.insert(String::from("query"), String::from("john"));
        let result = extract_query(params);
        let expected_result = "john".to_string();
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_extract_query_if_missing_parameters() {
        let params = HashMap::new();
        let result = format!("{}", extract_query(params).unwrap_err());
        let expected_result = format!("{}", Error::MissingParameters);
        assert_eq!(result, expected_result);
    }

}
