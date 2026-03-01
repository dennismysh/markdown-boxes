use crate::models::template::{Category, Template};

static TEMPLATES_JSON: &str = include_str!("../generated/templates.json");

pub fn load_templates() -> Vec<Template> {
    serde_json::from_str(TEMPLATES_JSON).expect("valid templates JSON")
}

pub fn get_template(templates: &[Template], slug: &str) -> Option<Template> {
    templates.iter().find(|t| t.slug == slug).cloned()
}

pub fn filter_by_category(templates: &[Template], category: &Category) -> Vec<Template> {
    templates
        .iter()
        .filter(|t| &t.category == category)
        .cloned()
        .collect()
}

pub fn search_templates(templates: &[Template], query: &str) -> Vec<Template> {
    if query.is_empty() {
        return templates.to_vec();
    }
    let query = query.to_lowercase();
    templates
        .iter()
        .filter(|t| {
            t.title.to_lowercase().contains(&query)
                || t.description.to_lowercase().contains(&query)
                || t.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_embedded_templates() {
        let templates = load_templates();
        assert_eq!(templates.len(), 3);
    }

    #[test]
    fn find_template_by_slug() {
        let templates = load_templates();
        let result = get_template(&templates, "auth-flow");
        assert!(result.is_some());
        assert_eq!(result.unwrap().title, "Authentication Flow");
    }

    #[test]
    fn find_nonexistent_template() {
        let templates = load_templates();
        assert!(get_template(&templates, "nonexistent").is_none());
    }

    #[test]
    fn filter_by_category() {
        let templates = load_templates();
        let results = super::filter_by_category(&templates, &Category::ImplementationPlan);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].slug, "auth-flow");
    }

    #[test]
    fn search_by_title() {
        let templates = load_templates();
        let results = search_templates(&templates, "auth");
        assert!(results.iter().any(|t| t.slug == "auth-flow"));
    }

    #[test]
    fn search_by_tag() {
        let templates = load_templates();
        let results = search_templates(&templates, "jwt");
        assert!(results.iter().any(|t| t.slug == "auth-flow"));
    }

    #[test]
    fn empty_search_returns_all() {
        let templates = load_templates();
        let results = search_templates(&templates, "");
        assert_eq!(results.len(), templates.len());
    }
}
