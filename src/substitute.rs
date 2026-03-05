use std::collections::HashMap;
use crate::models::filter::{parse_inline_filters, Filter};

pub fn substitute(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(body.len());
    let mut remaining = body;

    while let Some(start) = remaining.find("{{") {
        // Skip {{# and {{/ block tags — those are handled by the engine
        let after_braces = &remaining[start + 2..];
        if after_braces.starts_with('#') || after_braces.starts_with('/') {
            result.push_str(&remaining[..start + 2]);
            remaining = after_braces;
            continue;
        }

        result.push_str(&remaining[..start]);

        if let Some(end) = after_braces.find("}}") {
            let expr = &remaining[start + 2..start + 2 + end];
            let (var_name, filters) = parse_inline_filters(expr);

            if let Some(val) = values.get(&var_name) {
                if !val.is_empty() {
                    result.push_str(val);
                } else if let Some(default) = get_default(&filters) {
                    result.push_str(&default);
                }
            } else if let Some(default) = get_default(&filters) {
                result.push_str(&default);
            } else {
                // Show clean placeholder without filters
                result.push_str(&format!("{{{{{var_name}}}}}"));
            }

            remaining = &remaining[start + 2 + end + 2..];
        } else {
            result.push_str(&remaining[start..]);
            remaining = "";
        }
    }
    result.push_str(remaining);
    result
}

fn get_default(filters: &[Filter]) -> Option<String> {
    filters.iter().find_map(|f| match f {
        Filter::Default(val) => Some(val.clone()),
        _ => None,
    })
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

    #[test]
    fn substitute_strips_filter_syntax() {
        let body = "Hello {{name | required}}!";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "World".to_string());
        assert_eq!(substitute(body, &values), "Hello World!");
    }

    #[test]
    fn substitute_strips_multiple_filters() {
        let body = "Project: {{title | required | max_length: 100}}";
        let mut values = HashMap::new();
        values.insert("title".to_string(), "MyApp".to_string());
        assert_eq!(substitute(body, &values), "Project: MyApp");
    }

    #[test]
    fn substitute_applies_default_filter() {
        let body = "Hello {{name | default: \"World\"}}!";
        let values = HashMap::new();
        assert_eq!(substitute(body, &values), "Hello World!");
    }

    #[test]
    fn substitute_value_overrides_default() {
        let body = "Hello {{name | default: \"World\"}}!";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());
        assert_eq!(substitute(body, &values), "Hello Alice!");
    }

    #[test]
    fn substitute_unfilled_filter_placeholder_shows_var_name() {
        let body = "Hello {{name | required}}!";
        let values = HashMap::new();
        assert_eq!(substitute(body, &values), "Hello {{name}}!");
    }

    #[test]
    fn substitute_skips_block_tags() {
        let body = "{{#if show}}visible{{/if}}";
        let values = HashMap::new();
        assert_eq!(substitute(body, &values), "{{#if show}}visible{{/if}}");
    }
}
