use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Filter {
    Required,
    Default(String),
    MaxLength(usize),
    Options(Vec<String>),
}

/// Parse inline filter syntax from a placeholder expression like
/// "project_name | required | max_length: 100 | default: \"My Project\""
/// Returns (variable_name, Vec<Filter>)
pub fn parse_inline_filters(expr: &str) -> (String, Vec<Filter>) {
    let parts: Vec<&str> = expr.split('|').map(|s| s.trim()).collect();
    let var_name = parts[0].to_string();
    let mut filters = Vec::new();

    for part in &parts[1..] {
        let part = part.trim();
        if part == "required" {
            filters.push(Filter::Required);
        } else if let Some(val) = part.strip_prefix("default:") {
            let val = val.trim().trim_matches('"');
            filters.push(Filter::Default(val.to_string()));
        } else if let Some(val) = part.strip_prefix("max_length:") {
            if let Ok(n) = val.trim().parse::<usize>() {
                filters.push(Filter::MaxLength(n));
            }
        } else if let Some(val) = part.strip_prefix("options:") {
            let val = val.trim();
            let inner = val.trim_start_matches('[').trim_end_matches(']');
            let options: Vec<String> = inner.split(',').map(|s| s.trim().to_string()).collect();
            filters.push(Filter::Options(options));
        }
    }

    (var_name, filters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_filters() {
        let (name, filters) = parse_inline_filters("project_name");
        assert_eq!(name, "project_name");
        assert!(filters.is_empty());
    }

    #[test]
    fn parse_required_filter() {
        let (name, filters) = parse_inline_filters("project_name | required");
        assert_eq!(name, "project_name");
        assert_eq!(filters, vec![Filter::Required]);
    }

    #[test]
    fn parse_default_filter() {
        let (name, filters) = parse_inline_filters("project_name | default: \"My Project\"");
        assert_eq!(name, "project_name");
        assert_eq!(filters, vec![Filter::Default("My Project".to_string())]);
    }

    #[test]
    fn parse_max_length_filter() {
        let (name, filters) = parse_inline_filters("title | max_length: 100");
        assert_eq!(name, "title");
        assert_eq!(filters, vec![Filter::MaxLength(100)]);
    }

    #[test]
    fn parse_options_filter() {
        let (name, filters) = parse_inline_filters("framework | options: [React, Vue, Svelte]");
        assert_eq!(name, "framework");
        assert_eq!(
            filters,
            vec![Filter::Options(vec![
                "React".into(),
                "Vue".into(),
                "Svelte".into()
            ])]
        );
    }

    #[test]
    fn parse_multiple_filters() {
        let (name, filters) =
            parse_inline_filters("name | required | max_length: 50 | default: \"Untitled\"");
        assert_eq!(name, "name");
        assert_eq!(filters.len(), 3);
        assert_eq!(filters[0], Filter::Required);
        assert_eq!(filters[1], Filter::MaxLength(50));
        assert_eq!(filters[2], Filter::Default("Untitled".to_string()));
    }
}
