use serde::{Deserialize, Serialize};
use super::placeholder::Placeholder;
use super::section::Section;
use super::style::StyleSpec;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    pub slug: String,
    pub title: String,
    #[serde(default)]
    pub mdal_type: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    pub category: Category,
    pub tags: Vec<String>,
    pub preview: Option<String>,
    pub description: String,
    #[serde(default)]
    pub outputs: Vec<OutputTarget>,
    pub placeholders: Vec<Placeholder>,
    #[serde(default)]
    pub sections: Vec<Section>,
    #[serde(default)]
    pub style_spec: Option<StyleSpec>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputTarget {
    pub format: String,
    pub target: String,
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
    fn deserialize_mdal_frontmatter_fields() {
        let json = r##"{
            "slug": "test",
            "title": "Test",
            "mdal_type": "application",
            "version": "1.0.0",
            "author": "@system/templates",
            "category": "design-prompt",
            "tags": [],
            "preview": null,
            "description": "Test template",
            "outputs": [{"format": "html", "target": "web"}],
            "placeholders": [],
            "body": "# Test"
        }"##;
        let t: Template = serde_json::from_str(json).unwrap();
        assert_eq!(t.mdal_type.unwrap(), "application");
        assert_eq!(t.version.unwrap(), "1.0.0");
        assert_eq!(t.author.unwrap(), "@system/templates");
        assert_eq!(t.outputs.len(), 1);
    }

    #[test]
    fn category_label() {
        assert_eq!(Category::ImplementationPlan.label(), "Implementation Plan");
        assert_eq!(Category::FullStackFlow.label(), "Full-Stack Flow");
    }
}
