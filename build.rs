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
    sections: Vec<SectionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style_spec: Option<StyleSpecData>,
    body: String,
}

#[derive(Debug, Serialize)]
struct StyleSpecData {
    name: String,
    approach: Option<String>,
    colors: std::collections::HashMap<String, String>,
    typography: std::collections::HashMap<String, String>,
    effects: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize)]
struct SectionData {
    name: String,
    section_type: String,
    properties: std::collections::HashMap<String, String>,
    content: String,
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

fn parse_sections(body: &str) -> Vec<SectionData> {
    let mut sections = Vec::new();
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        if lines[i].starts_with("## Section:") {
            let name = lines[i].trim_start_matches("## Section:").trim().to_string();
            let mut properties = std::collections::HashMap::new();
            let mut section_type = "component".to_string();
            i += 1;

            // Parse dash-prefixed property lines
            while i < lines.len() && lines[i].starts_with("- ") {
                let prop_line = lines[i].trim_start_matches("- ");
                if let Some((key, val)) = prop_line.split_once(':') {
                    let key = key.trim().to_string();
                    let val = val.trim().to_string();
                    if key == "type" {
                        section_type = val;
                    } else {
                        properties.insert(key, val);
                    }
                }
                i += 1;
            }

            // Collect content until next ## or end
            let mut content = String::new();
            while i < lines.len() && !lines[i].starts_with("## ") {
                content.push_str(lines[i]);
                content.push('\n');
                i += 1;
            }

            sections.push(SectionData {
                name,
                section_type,
                properties,
                content: content.trim().to_string(),
            });
        } else {
            i += 1;
        }
    }

    sections
}

fn parse_style_spec(body: &str) -> Option<StyleSpecData> {
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        if lines[i].starts_with("## Styling:") {
            let name = lines[i].trim_start_matches("## Styling:").trim().to_string();
            let mut approach = None;
            let mut colors = std::collections::HashMap::new();
            let mut typography = std::collections::HashMap::new();
            let mut effects = std::collections::HashMap::new();
            i += 1;

            // Parse top-level properties
            while i < lines.len() && lines[i].starts_with("- ") {
                let prop = lines[i].trim_start_matches("- ");
                if let Some((k, v)) = prop.split_once(':') {
                    if k.trim() == "approach" {
                        approach = Some(v.trim().to_string());
                    }
                }
                i += 1;
            }

            // Parse subsections
            let mut current_map: Option<&mut std::collections::HashMap<String, String>> = None;
            while i < lines.len() && !lines[i].starts_with("## ") {
                let line = lines[i];
                if line.starts_with("### Colors") {
                    current_map = Some(&mut colors);
                } else if line.starts_with("### Typography") {
                    current_map = Some(&mut typography);
                } else if line.starts_with("### Effects") {
                    current_map = Some(&mut effects);
                } else if line.starts_with("- ") {
                    if let Some(ref mut map) = current_map {
                        let prop = line.trim_start_matches("- ");
                        if let Some((k, v)) = prop.split_once(':') {
                            map.insert(k.trim().to_string(), v.trim().to_string());
                        }
                    }
                }
                i += 1;
            }

            return Some(StyleSpecData {
                name,
                approach,
                colors,
                typography,
                effects,
            });
        }
        i += 1;
    }
    None
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

    let sections = parse_sections(&body);
    let style_spec = parse_style_spec(&body);

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
        sections,
        style_spec,
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
