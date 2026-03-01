use std::collections::HashMap;

pub fn substitute(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = body.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{{{key}}}}}");
        result = result.replace(&placeholder, value);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_single_placeholder() {
        let body = "Hello {{name}}!";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "World".to_string());
        assert_eq!(substitute(body, &values), "Hello World!");
    }

    #[test]
    fn substitute_multiple_placeholders() {
        let body = "{{greeting}} {{name}}, welcome to {{place}}.";
        let mut values = HashMap::new();
        values.insert("greeting".to_string(), "Hi".to_string());
        values.insert("name".to_string(), "Alice".to_string());
        values.insert("place".to_string(), "Rust".to_string());
        assert_eq!(substitute(body, &values), "Hi Alice, welcome to Rust.");
    }

    #[test]
    fn repeated_placeholder() {
        let body = "{{x}} and {{x}} again";
        let mut values = HashMap::new();
        values.insert("x".to_string(), "yes".to_string());
        assert_eq!(substitute(body, &values), "yes and yes again");
    }

    #[test]
    fn missing_value_leaves_placeholder() {
        let body = "Hello {{name}}!";
        let values = HashMap::new();
        assert_eq!(substitute(body, &values), "Hello {{name}}!");
    }

    #[test]
    fn empty_value_substitutes_empty() {
        let body = "Hello {{name}}!";
        let mut values = HashMap::new();
        values.insert("name".to_string(), String::new());
        assert_eq!(substitute(body, &values), "Hello !");
    }
}
