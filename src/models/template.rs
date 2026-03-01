use serde::{Deserialize, Serialize};
use super::placeholder::Placeholder;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    pub slug: String,
    pub title: String,
    pub category: Category,
    pub tags: Vec<String>,
    pub preview: Option<String>,
    pub description: String,
    pub placeholders: Vec<Placeholder>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    ImplementationPlan,
    DesignPrompt,
    UiComponent,
    FullStackFlow,
    BackendPattern,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::ImplementationPlan => "Implementation Plan",
            Category::DesignPrompt => "Design Prompt",
            Category::UiComponent => "UI Component",
            Category::FullStackFlow => "Full-Stack Flow",
            Category::BackendPattern => "Backend Pattern",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deserialize_template_from_json() {
        let json = r##"{
            "slug": "auth-flow",
            "title": "Auth Flow Implementation Plan",
            "category": "full-stack-flow",
            "tags": ["auth", "jwt"],
            "preview": null,
            "description": "Build JWT auth",
            "placeholders": [
                {
                    "key": "project_name",
                    "label": "Project Name",
                    "type": "text",
                    "options": null
                }
            ],
            "body": "# {{project_name}}"
        }"##;

        let template: Template = serde_json::from_str(json).unwrap();
        assert_eq!(template.slug, "auth-flow");
        assert_eq!(template.category, Category::FullStackFlow);
        assert_eq!(template.placeholders.len(), 1);
        assert_eq!(template.placeholders[0].key, "project_name");
    }

    #[test]
    fn category_label() {
        assert_eq!(Category::ImplementationPlan.label(), "Implementation Plan");
        assert_eq!(Category::FullStackFlow.label(), "Full-Stack Flow");
    }
}
