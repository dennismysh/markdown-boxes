use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct TemplateData {
    slug: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    mdal_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    author: Option<String>,
    category: String,
    tags: Vec<String>,
    preview: Option<String>,
    description: String,
    outputs: Vec<OutputData>,
    placeholders: Vec<PlaceholderData>,
    body: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct OutputData {
    format: String,
    target: String,
}

#[derive(Debug, Deserialize)]
struct PlaceholderInput {
    key: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    options: Option<Vec<String>>,
    #[serde(default)]
    filters: Vec<FilterInput>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FilterInput {
    Flag(String),
    KeyValue(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Debug, Serialize)]
struct PlaceholderData {
    key: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<Vec<String>>,
    filters: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    #[serde(default, rename = "type")]
    mdal_type: Option<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    author: Option<String>,
    category: String,
    tags: Vec<String>,
    #[serde(default)]
    preview: Option<String>,
    description: String,
    #[serde(default)]
    outputs: Vec<OutputData>,
    #[serde(default)]
    placeholders: Vec<PlaceholderInput>,
}

fn convert_placeholder(input: PlaceholderInput) -> PlaceholderData {
    let filters = convert_filters(&input.filters);
    PlaceholderData {
        key: input.key,
        label: input.label,
        kind: input.kind,
        options: input.options,
        filters,
    }
}

fn convert_filters(inputs: &[FilterInput]) -> Vec<serde_json::Value> {
    inputs
        .iter()
        .filter_map(|f| match f {
            FilterInput::Flag(s) if s == "required" => Some(serde_json::json!({"required": null})),
            FilterInput::KeyValue(map) => {
                if let Some(v) = map.get("max_length") {
                    Some(serde_json::json!({"max_length": v}))
                } else if let Some(v) = map.get("default") {
                    Some(serde_json::json!({"default": v}))
                } else if let Some(v) = map.get("options") {
                    Some(serde_json::json!({"options": v}))
                } else if map.contains_key("required") {
                    Some(serde_json::json!({"required": null}))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect()
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
        mdal_type: frontmatter.mdal_type,
        version: frontmatter.version,
        author: frontmatter.author,
        category: frontmatter.category,
        tags: frontmatter.tags,
        preview: frontmatter.preview,
        description: frontmatter.description,
        outputs: frontmatter.outputs,
        placeholders: frontmatter.placeholders.into_iter().map(convert_placeholder).collect(),
        body,
    })
}

fn main() {
    println!("cargo::rerun-if-changed=templates/");

    let templates_dir = Path::new("templates");
    let mut templates = Vec::new();

    if templates_dir.exists() {
        for entry in glob::glob("templates/**/*.mdal").expect("glob pattern") {
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

    // Collect hero SVGs into public/previews/ for Trunk to copy into dist
    let previews_dir = Path::new("public/previews");
    fs::create_dir_all(previews_dir).expect("create public/previews/");
    for entry in glob::glob("templates/**/*-hero.svg").expect("glob pattern") {
        if let Ok(path) = entry {
            let filename = path.file_name().unwrap();
            fs::copy(&path, previews_dir.join(filename)).expect("copy hero SVG");
        }
    }
}
