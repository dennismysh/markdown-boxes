use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct TemplateData {
    slug: String,
    title: String,
    category: String,
    tags: Vec<String>,
    preview: Option<String>,
    description: String,
    placeholders: Vec<PlaceholderData>,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct PlaceholderData {
    key: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    options: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    category: String,
    tags: Vec<String>,
    #[serde(default)]
    preview: Option<String>,
    description: String,
    #[serde(default)]
    placeholders: Vec<PlaceholderData>,
}

fn parse_template(content: &str, slug: &str) -> Option<TemplateData> {
    let content = content.trim();
    if !content.starts_with("---") {
        eprintln!("Warning: {slug} missing frontmatter, skipping");
        return None;
    }

    let after_first = &content[3..];
    let end = after_first.find("---")?;
    let yaml_str = &after_first[..end];
    let body = after_first[end + 3..].trim().to_string();

    let frontmatter: Frontmatter = match serde_yml::from_str(yaml_str) {
        Ok(fm) => fm,
        Err(e) => {
            eprintln!("Warning: {slug} has invalid frontmatter: {e}");
            return None;
        }
    };

    Some(TemplateData {
        slug: slug.to_string(),
        title: frontmatter.title,
        category: frontmatter.category,
        tags: frontmatter.tags,
        preview: frontmatter.preview,
        description: frontmatter.description,
        placeholders: frontmatter.placeholders,
        body,
    })
}

fn main() {
    println!("cargo::rerun-if-changed=templates/");

    let templates_dir = Path::new("templates");
    let mut templates = Vec::new();

    if templates_dir.exists() {
        for entry in glob::glob("templates/**/*.md").expect("glob pattern") {
            if let Ok(path) = entry {
                let content = fs::read_to_string(&path).expect("read template");
                let slug = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                if let Some(template) = parse_template(&content, &slug) {
                    templates.push(template);
                }
            }
        }
    }

    templates.sort_by(|a, b| a.title.cmp(&b.title));

    let json = serde_json::to_string_pretty(&templates).expect("serialize templates");
    let out_dir = Path::new("generated");
    fs::create_dir_all(out_dir).expect("create generated/");
    fs::write(out_dir.join("templates.json"), json).expect("write templates.json");
}
