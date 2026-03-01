# Markdown Boxes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a web-based gallery of shareable markdown templates with form-based customization and export.

**Architecture:** Leptos CSR (Rust/WASM) static site. Templates are `.md` files with YAML frontmatter, parsed at build time by `build.rs` into JSON, embedded in the binary via `include_str!`. No runtime YAML parsing — only `serde_json` in WASM. Comrak renders live markdown preview.

**Tech Stack:** Leptos 0.8 CSR, comrak 0.50 (default-features=false), serde/serde_json, Trunk build tool

**Key Design Decision:** YAML frontmatter is parsed at build time only (native Rust in `build.rs`). A `generated/templates.json` file is produced and embedded via `include_str!`. This avoids YAML parsing in WASM entirely.

---

### Task 1: Project Scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`
- Create: `Trunk.toml`
- Create: `index.html`

**Step 1: Create Cargo.toml**

```toml
[package]
name = "markdown-boxes"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
leptos_meta = "0.8"
leptos_router = "0.8"
console_log = "1"
log = "0.4"
console_error_panic_hook = "0.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
comrak = { version = "0.50", default-features = false }
web-sys = { version = "0.3", features = ["Navigator", "Clipboard", "Blob", "Url", "HtmlAnchorElement", "BlobPropertyBag"] }
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"

[build-dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yml = "0.0"
glob = "0.3"

[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = "abort"
```

**Step 2: Create src/main.rs**

```rust
use markdown_boxes::App;
use leptos::prelude::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
```

**Step 3: Create src/lib.rs**

```rust
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app">
                <header>
                    <h1>"Markdown Boxes"</h1>
                </header>
                <main>
                    <Routes fallback=|| view! { <p>"Not found"</p> }>
                        <Route path=path!("/") view=|| view! { <p>"Gallery coming soon"</p> }/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
```

**Step 4: Create Trunk.toml**

```toml
[build]
target = "index.html"
dist = "dist"

[serve]
address = "127.0.0.1"
port = 3000
```

**Step 5: Create index.html**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Markdown Boxes</title>
  <link data-trunk rel="rust" data-wasm-opt="z" />
  <style>
    :root {
      --bg: #0f0f0f;
      --surface: #1a1a1a;
      --surface-2: #242424;
      --accent: #6366f1;
      --accent-hover: #818cf8;
      --text: #e8e8e8;
      --text-muted: #888888;
      --border: #2a2a2a;
      --radius: 8px;
    }
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
    }
    .app { max-width: 1200px; margin: 0 auto; padding: 0 24px; }
    header { padding: 24px 0; border-bottom: 1px solid var(--border); margin-bottom: 32px; }
    header h1 { font-size: 1.5rem; font-weight: 600; }
  </style>
</head>
<body></body>
</html>
```

**Step 6: Create placeholder build.rs and generated/ directory**

```rust
// build.rs
fn main() {
    // Template index generation will go here
}
```

Also create `generated/templates.json`:
```json
[]
```

**Step 7: Verify it builds and runs**

Run: `trunk serve --open`
Expected: Browser opens to localhost:3000, shows "Markdown Boxes" header and "Gallery coming soon"

**Step 8: Commit**

```bash
git add Cargo.toml Cargo.lock src/main.rs src/lib.rs Trunk.toml index.html build.rs generated/templates.json
git commit -m "feat: scaffold Leptos CSR project with trunk"
```

---

### Task 2: Template Data Model

**Files:**
- Create: `src/models/mod.rs`
- Create: `src/models/template.rs`
- Create: `src/models/placeholder.rs`
- Modify: `src/lib.rs` (add mod declaration)

**Step 1: Write failing tests for Template deserialization**

Create `src/models/template.rs`:

```rust
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
    use crate::models::placeholder::{Placeholder, PlaceholderType};

    #[test]
    fn deserialize_template_from_json() {
        let json = r#"{
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
        }"#;

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
```

**Step 2: Write failing tests for Placeholder types**

Create `src/models/placeholder.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Placeholder {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub kind: PlaceholderType,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PlaceholderType {
    Text,
    Select,
    Multiline,
    Boolean,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_text_placeholder() {
        let json = r#"{"key":"name","label":"Name","type":"text","options":null}"#;
        let p: Placeholder = serde_json::from_str(json).unwrap();
        assert_eq!(p.kind, PlaceholderType::Text);
        assert!(p.options.is_none());
    }

    #[test]
    fn deserialize_select_placeholder() {
        let json = r#"{"key":"framework","label":"Framework","type":"select","options":["React","Vue"]}"#;
        let p: Placeholder = serde_json::from_str(json).unwrap();
        assert_eq!(p.kind, PlaceholderType::Select);
        assert_eq!(p.options.as_ref().unwrap().len(), 2);
    }
}
```

**Step 3: Create mod.rs and wire into lib.rs**

Create `src/models/mod.rs`:
```rust
pub mod placeholder;
pub mod template;
```

Add to `src/lib.rs`:
```rust
pub mod models;
```

**Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: All 4 tests pass

**Step 5: Commit**

```bash
git add src/models/
git commit -m "feat: add Template and Placeholder data models with tests"
```

---

### Task 3: Frontmatter Parser (build-time only)

**Files:**
- Create: `build_support/mod.rs` — NO, build.rs can't use local modules easily. Instead, put parser logic directly in `build.rs`.

Actually, since the frontmatter parser runs only in `build.rs` (native Rust, not WASM), we'll put the parsing logic there. But we also need the same `Template` struct shape. We'll define a parallel `BuildTemplate` struct in `build.rs` that serializes to the same JSON the runtime `Template` deserializes.

**Files:**
- Modify: `build.rs`

**Step 1: Write build.rs with frontmatter parsing and tests**

Replace `build.rs` with:

```rust
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_template() {
        let content = r#"---
title: "Test Plan"
category: implementation-plan
tags: [rust, testing]
description: "A test template"
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
---

# {{project_name}} Test Plan

Write tests for {{project_name}}."#;

        let result = parse_template(content, "test-plan");
        assert!(result.is_some());
        let t = result.unwrap();
        assert_eq!(t.slug, "test-plan");
        assert_eq!(t.title, "Test Plan");
        assert_eq!(t.category, "implementation-plan");
        assert_eq!(t.tags, vec!["rust", "testing"]);
        assert_eq!(t.placeholders.len(), 1);
        assert!(t.body.contains("{{project_name}}"));
    }

    #[test]
    fn skip_template_without_frontmatter() {
        let content = "# Just a heading\nSome text";
        let result = parse_template(content, "bad");
        assert!(result.is_none());
    }

    #[test]
    fn skip_template_with_invalid_yaml() {
        let content = "---\n[invalid yaml\n---\nBody";
        let result = parse_template(content, "bad");
        assert!(result.is_none());
    }
}
```

**Step 2: Run build.rs tests**

Note: `build.rs` tests can't be run with `cargo test` directly — they're a separate compilation unit. To test the parsing logic, we'll extract it into a small helper crate or test it manually. For pragmatism, verify by creating a template and running `cargo build`.

Run: `cargo build`
Expected: Compiles, `generated/templates.json` contains `[]` (no templates yet)

**Step 3: Commit**

```bash
git add build.rs
git commit -m "feat: add build.rs with YAML frontmatter parser"
```

---

### Task 4: Sample Templates

**Files:**
- Create: `templates/implementation-plans/auth-flow.md`
- Create: `templates/design-prompts/landing-page.md`
- Create: `templates/ui-components/data-table.md`

**Step 1: Create auth-flow template**

Create `templates/implementation-plans/auth-flow.md`:

```markdown
---
title: "Authentication Flow"
category: implementation-plan
tags: [auth, jwt, backend, security]
description: "Step-by-step plan for building JWT-based authentication with login, registration, and token refresh."
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
  - key: auth_provider
    label: "Auth Provider"
    type: select
    options: [Firebase, Supabase, Custom JWT, Auth0]
  - key: framework
    label: "Backend Framework"
    type: select
    options: [Express, FastAPI, Axum, Django, Rails]
  - key: additional_requirements
    label: "Additional Requirements"
    type: multiline
---

# {{project_name}} — Authentication Flow

## Overview

Build a JWT-based auth system for {{project_name}} using {{auth_provider}} with {{framework}}.

## Step 1: User Registration

Create the registration endpoint that accepts email and password:
- Hash passwords with bcrypt (cost factor 12)
- Validate email format and password strength
- Store user in database
- Return JWT access token + refresh token

## Step 2: Login Flow

Implement the login endpoint:
- Verify credentials against stored hash
- Generate short-lived access token (15 min)
- Generate long-lived refresh token (7 days)
- Set refresh token as httpOnly cookie

## Step 3: Token Refresh

Create the refresh endpoint:
- Validate refresh token from cookie
- Issue new access + refresh token pair
- Rotate refresh tokens (invalidate old one)

## Step 4: Protected Routes

Add auth middleware:
- Extract JWT from Authorization header
- Verify signature and expiration
- Attach user context to request
- Return 401 for invalid/expired tokens

## Step 5: Logout

Implement logout:
- Invalidate refresh token server-side
- Clear httpOnly cookie
- Client clears stored access token

## Additional Notes

{{additional_requirements}}
```

**Step 2: Create landing-page design prompt template**

Create `templates/design-prompts/landing-page.md`:

```markdown
---
title: "Landing Page Design"
category: design-prompt
tags: [design, landing-page, ui, marketing]
description: "AI prompt for designing a modern landing page with hero, features, and CTA sections."
placeholders:
  - key: product_name
    label: "Product Name"
    type: text
  - key: tagline
    label: "Tagline"
    type: text
  - key: product_description
    label: "Product Description"
    type: multiline
  - key: color_scheme
    label: "Color Scheme"
    type: select
    options: [Dark minimal, Light clean, Vibrant gradient, Corporate blue]
  - key: num_features
    label: "Number of Features to Highlight"
    type: select
    options: ["3", "4", "6"]
---

# Design Prompt: {{product_name}} Landing Page

## Context

Design a modern landing page for **{{product_name}}**.

Tagline: "{{tagline}}"

Product description: {{product_description}}

## Design Requirements

**Color scheme:** {{color_scheme}}

### Hero Section
- Large headline with the tagline
- Brief subtitle expanding on the value proposition
- Primary CTA button ("Get Started" or similar)
- Hero image or illustration on the right

### Features Section
- {{num_features}} feature cards in a grid
- Each card: icon, title, 1-2 sentence description
- Clean spacing, consistent alignment

### Social Proof
- Testimonial quotes or company logos
- Usage statistics if available

### Footer CTA
- Repeated call to action
- Secondary link to documentation or pricing

## Technical Constraints
- Responsive: mobile-first, works at 320px-1440px+
- Performance: no layout shift, lazy-load images
- Accessibility: WCAG 2.1 AA compliant
```

**Step 3: Create data-table UI component template**

Create `templates/ui-components/data-table.md`:

```markdown
---
title: "Data Table Component"
category: ui-component
tags: [table, data, component, ui]
description: "Prompt for building a sortable, filterable data table component with pagination."
placeholders:
  - key: framework
    label: "UI Framework"
    type: select
    options: [React, Vue, Svelte, Solid]
  - key: styling
    label: "Styling Approach"
    type: select
    options: [Tailwind CSS, CSS Modules, Styled Components, Plain CSS]
  - key: columns
    label: "Column Definitions (comma-separated)"
    type: text
  - key: has_pagination
    label: "Include Pagination"
    type: boolean
---

# Data Table Component

## Prompt

Build a reusable data table component in {{framework}} with {{styling}}.

### Columns

The table should display these columns: {{columns}}

### Features

- **Sorting:** Click column headers to sort ascending/descending
- **Filtering:** Text input above the table for global search across all columns
- **Selection:** Checkbox column for row selection with select-all header
- **Pagination:** {{has_pagination}}
- **Empty state:** Show a centered message when no data matches filters

### API

```
Props:
  data: array of row objects
  columns: array of { key, label, sortable? }
  onSelectionChange: callback with selected row IDs
  pageSize: number (default 20)
```

### Responsive Behavior

- Desktop: full table with all columns
- Tablet: horizontally scrollable
- Mobile: card layout, one card per row
```

**Step 4: Verify build generates index**

Run: `cargo build`
Expected: `generated/templates.json` contains 3 templates with parsed frontmatter and bodies

Run: `cat generated/templates.json | head -20`
Expected: JSON array with template objects

**Step 5: Commit**

```bash
git add templates/ generated/templates.json
git commit -m "feat: add 3 seed templates (auth, landing page, data table)"
```

---

### Task 5: Template Store

**Files:**
- Create: `src/store.rs`
- Modify: `src/lib.rs`

**Step 1: Write tests for template store**

Create `src/store.rs`:

```rust
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
        assert!(!templates.is_empty());
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
```

**Step 2: Wire into lib.rs**

Add to `src/lib.rs`:
```rust
pub mod store;
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All store tests pass (templates are embedded from generated/templates.json)

**Step 4: Commit**

```bash
git add src/store.rs src/lib.rs
git commit -m "feat: add template store with search and filter"
```

---

### Task 6: Placeholder Substitution

**Files:**
- Create: `src/substitute.rs`
- Modify: `src/lib.rs`

**Step 1: Write failing tests for substitution**

Create `src/substitute.rs`:

```rust
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
```

**Step 2: Wire into lib.rs**

Add to `src/lib.rs`:
```rust
pub mod substitute;
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All substitution tests pass

**Step 4: Commit**

```bash
git add src/substitute.rs src/lib.rs
git commit -m "feat: add placeholder substitution with tests"
```

---

### Task 7: Gallery Page

**Files:**
- Create: `src/pages/mod.rs`
- Create: `src/pages/gallery.rs`
- Create: `src/components/mod.rs`
- Create: `src/components/template_card.rs`
- Create: `src/components/search_bar.rs`
- Create: `src/components/category_filter.rs`
- Modify: `src/lib.rs`

**Step 1: Create the TemplateCard component**

Create `src/components/template_card.rs`:

```rust
use leptos::prelude::*;
use crate::models::template::Template;

#[component]
pub fn TemplateCard(template: Template) -> impl IntoView {
    let slug = template.slug.clone();
    let href = format!("/template/{slug}");

    view! {
        <a href=href class="template-card">
            <div class="card-preview">
                {template.preview.map(|p| view! {
                    <img src=format!("/previews/{p}") alt="" />
                }.into_any()).unwrap_or_else(|| view! {
                    <div class="card-preview-placeholder">
                        <span>{template.category.label()}</span>
                    </div>
                }.into_any())}
            </div>
            <div class="card-body">
                <span class="card-category">{template.category.label()}</span>
                <h3 class="card-title">{template.title}</h3>
                <p class="card-description">{template.description}</p>
                <div class="card-tags">
                    {template.tags.into_iter().map(|tag| view! {
                        <span class="tag">{tag}</span>
                    }).collect_view()}
                </div>
            </div>
        </a>
    }
}
```

**Step 2: Create SearchBar component**

Create `src/components/search_bar.rs`:

```rust
use leptos::prelude::*;

#[component]
pub fn SearchBar(
    value: ReadSignal<String>,
    on_input: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="search-bar">
            <input
                type="text"
                placeholder="Search templates..."
                prop:value=value
                on:input=move |ev| {
                    on_input.set(event_target_value(&ev));
                }
            />
        </div>
    }
}
```

**Step 3: Create CategoryFilter component**

Create `src/components/category_filter.rs`:

```rust
use leptos::prelude::*;
use crate::models::template::Category;

#[component]
pub fn CategoryFilter(
    selected: ReadSignal<Option<Category>>,
    on_select: WriteSignal<Option<Category>>,
) -> impl IntoView {
    let categories = vec![
        Category::ImplementationPlan,
        Category::DesignPrompt,
        Category::UiComponent,
        Category::FullStackFlow,
        Category::BackendPattern,
    ];

    view! {
        <div class="category-filter">
            <button
                class=move || if selected.get().is_none() { "filter-btn active" } else { "filter-btn" }
                on:click=move |_| on_select.set(None)
            >
                "All"
            </button>
            {categories.into_iter().map(|cat| {
                let cat_clone = cat.clone();
                let label = cat.label();
                view! {
                    <button
                        class=move || {
                            if selected.get().as_ref() == Some(&cat_clone) { "filter-btn active" } else { "filter-btn" }
                        }
                        on:click={
                            let cat = cat_clone.clone();
                            move |_| on_select.set(Some(cat.clone()))
                        }
                    >
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
```

**Step 4: Create Gallery page**

Create `src/pages/gallery.rs`:

```rust
use leptos::prelude::*;
use crate::components::category_filter::CategoryFilter;
use crate::components::search_bar::SearchBar;
use crate::components::template_card::TemplateCard;
use crate::models::template::Category;
use crate::store;

#[component]
pub fn Gallery() -> impl IntoView {
    let all_templates = store::load_templates();
    let (search_query, set_search_query) = signal(String::new());
    let (selected_category, set_selected_category) = signal(Option::<Category>::None);

    let filtered = move || {
        let mut templates = store::search_templates(&all_templates, &search_query.get());
        if let Some(ref cat) = selected_category.get() {
            templates = store::filter_by_category(&templates, cat);
        }
        templates
    };

    view! {
        <div class="gallery">
            <div class="gallery-header">
                <h2>"Templates"</h2>
                <p class="gallery-subtitle">"Browse proven templates for AI-driven workflows"</p>
            </div>
            <SearchBar value=search_query on_input=set_search_query />
            <CategoryFilter selected=selected_category on_select=set_selected_category />
            <div class="gallery-grid">
                {move || filtered().into_iter().map(|t| view! {
                    <TemplateCard template=t />
                }).collect_view()}
            </div>
            {move || {
                if filtered().is_empty() {
                    Some(view! {
                        <p class="empty-state">"No templates match your search."</p>
                    })
                } else {
                    None
                }
            }}
        </div>
    }
}
```

**Step 5: Create mod files and wire into lib.rs**

Create `src/components/mod.rs`:
```rust
pub mod category_filter;
pub mod search_bar;
pub mod template_card;
```

Create `src/pages/mod.rs`:
```rust
pub mod gallery;
```

Update `src/lib.rs` to add modules and route:
```rust
pub mod components;
pub mod models;
pub mod pages;
pub mod store;
pub mod substitute;

use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use pages::gallery::Gallery;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="app">
                <header>
                    <a href="/" class="logo">"Markdown Boxes"</a>
                </header>
                <main>
                    <Routes fallback=|| view! { <p class="not-found">"Not found"</p> }>
                        <Route path=path!("/") view=Gallery/>
                    </Routes>
                </main>
            </div>
        </Router>
    }
}
```

**Step 6: Add gallery CSS to index.html**

Append to the `<style>` block in `index.html`:

```css
/* Gallery */
.gallery { padding: 24px 0; }
.gallery-header { margin-bottom: 24px; }
.gallery-header h2 { font-size: 1.75rem; font-weight: 600; margin-bottom: 4px; }
.gallery-subtitle { color: var(--text-muted); }
.gallery-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
  margin-top: 20px;
}
.empty-state { text-align: center; color: var(--text-muted); padding: 48px 0; }

/* Template Card */
.template-card {
  display: block;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  overflow: hidden;
  text-decoration: none;
  color: inherit;
  transition: border-color 0.15s, transform 0.15s;
}
.template-card:hover { border-color: var(--accent); transform: translateY(-2px); }
.card-preview { height: 160px; background: var(--surface-2); display: flex; align-items: center; justify-content: center; }
.card-preview img { width: 100%; height: 100%; object-fit: cover; }
.card-preview-placeholder { color: var(--text-muted); font-size: 0.875rem; }
.card-body { padding: 16px; }
.card-category { font-size: 0.75rem; color: var(--accent); text-transform: uppercase; letter-spacing: 0.05em; }
.card-title { font-size: 1.125rem; font-weight: 600; margin: 4px 0 8px; }
.card-description { font-size: 0.875rem; color: var(--text-muted); line-height: 1.5; }
.card-tags { display: flex; flex-wrap: wrap; gap: 6px; margin-top: 12px; }
.tag { font-size: 0.75rem; padding: 2px 8px; background: var(--surface-2); border-radius: 4px; color: var(--text-muted); }

/* Search */
.search-bar { margin-bottom: 16px; }
.search-bar input {
  width: 100%;
  padding: 10px 16px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  color: var(--text);
  font-size: 0.9375rem;
  outline: none;
}
.search-bar input:focus { border-color: var(--accent); }
.search-bar input::placeholder { color: var(--text-muted); }

/* Category Filter */
.category-filter { display: flex; flex-wrap: wrap; gap: 8px; margin-bottom: 8px; }
.filter-btn {
  padding: 6px 14px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: 20px;
  color: var(--text-muted);
  font-size: 0.8125rem;
  cursor: pointer;
  transition: all 0.15s;
}
.filter-btn:hover { border-color: var(--accent); color: var(--text); }
.filter-btn.active { background: var(--accent); border-color: var(--accent); color: white; }
```

**Step 7: Verify gallery renders**

Run: `trunk serve`
Expected: Gallery page shows 3 template cards with titles, descriptions, category badges, and tags. Search and category filter buttons are visible.

**Step 8: Commit**

```bash
git add src/components/ src/pages/ src/lib.rs index.html
git commit -m "feat: add gallery page with search and category filter"
```

---

### Task 8: Template View Page

**Files:**
- Create: `src/pages/template_view.rs`
- Create: `src/components/form_field.rs`
- Create: `src/components/markdown_preview.rs`
- Modify: `src/pages/mod.rs`
- Modify: `src/components/mod.rs`
- Modify: `src/lib.rs` (add route)

**Step 1: Create FormField component**

Create `src/components/form_field.rs`:

```rust
use leptos::prelude::*;
use crate::models::placeholder::{Placeholder, PlaceholderType};

#[component]
pub fn FormField(
    placeholder: Placeholder,
    value: ReadSignal<String>,
    on_change: WriteSignal<String>,
) -> impl IntoView {
    let label = placeholder.label.clone();
    let key = placeholder.key.clone();

    let input_view = match placeholder.kind {
        PlaceholderType::Text => view! {
            <input
                type="text"
                id=key.clone()
                prop:value=value
                on:input=move |ev| on_change.set(event_target_value(&ev))
                placeholder=format!("Enter {label}...")
            />
        }.into_any(),
        PlaceholderType::Multiline => view! {
            <textarea
                id=key.clone()
                prop:value=value
                on:input=move |ev| on_change.set(event_target_value(&ev))
                placeholder=format!("Enter {label}...")
                rows=4
            />
        }.into_any(),
        PlaceholderType::Select => {
            let options = placeholder.options.unwrap_or_default();
            view! {
                <select
                    id=key.clone()
                    on:change=move |ev| on_change.set(event_target_value(&ev))
                >
                    <option value="" disabled selected>"Select..."</option>
                    {options.into_iter().map(|opt| view! {
                        <option value=opt.clone()>{opt}</option>
                    }).collect_view()}
                </select>
            }.into_any()
        },
        PlaceholderType::Boolean => view! {
            <label class="toggle-label">
                <input
                    type="checkbox"
                    on:change=move |ev| {
                        let checked = event_target_checked(&ev);
                        on_change.set(if checked { "Yes".to_string() } else { "No".to_string() });
                    }
                />
                <span>{move || value.get()}</span>
            </label>
        }.into_any(),
    };

    view! {
        <div class="form-field">
            <label for=key>{placeholder.label}</label>
            {input_view}
        </div>
    }
}
```

**Step 2: Create MarkdownPreview component**

Create `src/components/markdown_preview.rs`:

```rust
use leptos::prelude::*;
use comrak::{parse_document, Arena, Options};

fn render_markdown(input: &str) -> String {
    let arena = Arena::new();
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.render.r#unsafe = true;

    let root = parse_document(&arena, input, &options);
    let mut html = String::new();
    comrak::format_html(root, &options, &mut html).unwrap();
    html
}

#[component]
pub fn MarkdownPreview(content: Signal<String>) -> impl IntoView {
    let html = move || render_markdown(&content.get());

    view! {
        <div class="markdown-preview prose" inner_html=html />
    }
}
```

**Step 3: Create TemplateView page**

Create `src/pages/template_view.rs`:

```rust
use std::collections::HashMap;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::components::form_field::FormField;
use crate::components::markdown_preview::MarkdownPreview;
use crate::models::placeholder::Placeholder;
use crate::store;
use crate::substitute::substitute;

#[component]
pub fn TemplateView() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug").unwrap_or_default();

    let all_templates = store::load_templates();

    let template = move || store::get_template(&all_templates, &slug());

    view! {
        {move || {
            if let Some(tmpl) = template() {
                let title = tmpl.title.clone();
                let body = tmpl.body.clone();
                let placeholders = tmpl.placeholders.clone();

                // Create a signal for each placeholder value
                let field_signals: Vec<(Placeholder, ReadSignal<String>, WriteSignal<String>)> =
                    placeholders.into_iter().map(|p| {
                        let (read, write) = signal(String::new());
                        (p, read, write)
                    }).collect();

                let field_signals_for_preview = field_signals.clone();

                let preview_content = Signal::derive(move || {
                    let mut values = HashMap::new();
                    for (p, read, _) in &field_signals_for_preview {
                        let val = read.get();
                        if !val.is_empty() {
                            values.insert(p.key.clone(), val);
                        }
                    }
                    substitute(&body, &values)
                });

                view! {
                    <div class="template-view">
                        <div class="template-header">
                            <a href="/" class="back-link">"← Back to gallery"</a>
                            <h2>{title}</h2>
                        </div>
                        <div class="template-layout">
                            <div class="template-form">
                                <h3>"Customize"</h3>
                                {field_signals.into_iter().map(|(p, read, write)| {
                                    view! { <FormField placeholder=p value=read on_change=write /> }
                                }).collect_view()}
                                <div class="export-buttons">
                                    <ExportButtons content=preview_content />
                                </div>
                            </div>
                            <div class="template-preview-pane">
                                <h3>"Preview"</h3>
                                <MarkdownPreview content=preview_content />
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! {
                    <div class="not-found">
                        <h2>"Template not found"</h2>
                        <a href="/">"← Back to gallery"</a>
                    </div>
                }.into_any()
            }
        }}
    }
}

#[component]
fn ExportButtons(content: Signal<String>) -> impl IntoView {
    let (copied, set_copied) = signal(false);

    let copy_to_clipboard = move |_| {
        let text = content.get();
        let window = web_sys::window().unwrap();
        let navigator = window.navigator();
        let clipboard = navigator.clipboard();
        let _ = clipboard.write_text(&text);
        set_copied.set(true);
        // Reset after 2 seconds
        set_timeout(move || set_copied.set(false), std::time::Duration::from_secs(2));
    };

    let download = move |_| {
        let text = content.get();
        let blob_parts = js_sys::Array::new();
        blob_parts.push(&wasm_bindgen::JsValue::from_str(&text));
        let mut opts = web_sys::BlobPropertyBag::new();
        opts.type_("text/markdown");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(&blob_parts, &opts).unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

        let document = web_sys::window().unwrap().document().unwrap();
        let a = document.create_element("a").unwrap();
        let a: web_sys::HtmlAnchorElement = a.unchecked_into();
        a.set_href(&url);
        a.set_download("template.md");
        a.click();
        web_sys::Url::revoke_object_url(&url).unwrap();
    };

    view! {
        <button class="btn btn-primary" on:click=copy_to_clipboard>
            {move || if copied.get() { "Copied!" } else { "Copy to Clipboard" }}
        </button>
        <button class="btn btn-secondary" on:click=download>
            "Download .md"
        </button>
    }
}
```

**Step 4: Update mod files**

Update `src/pages/mod.rs`:
```rust
pub mod gallery;
pub mod template_view;
```

Update `src/components/mod.rs`:
```rust
pub mod category_filter;
pub mod form_field;
pub mod markdown_preview;
pub mod search_bar;
pub mod template_card;
```

**Step 5: Add template view route to lib.rs**

Update `src/lib.rs` routes section:
```rust
use pages::template_view::TemplateView;

// In the Routes block:
<Route path=path!("/template/:slug") view=TemplateView/>
```

**Step 6: Add template view CSS to index.html**

Append to `<style>` in `index.html`:

```css
/* Template View */
.template-view { padding: 24px 0; }
.template-header { margin-bottom: 24px; }
.back-link { color: var(--accent); text-decoration: none; font-size: 0.875rem; }
.back-link:hover { text-decoration: underline; }
.template-header h2 { margin-top: 8px; font-size: 1.5rem; }

.template-layout {
  display: grid;
  grid-template-columns: 360px 1fr;
  gap: 24px;
  align-items: start;
}
@media (max-width: 768px) {
  .template-layout { grid-template-columns: 1fr; }
}

.template-form {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 20px;
  position: sticky;
  top: 20px;
}
.template-form h3 { font-size: 1rem; margin-bottom: 16px; }

.template-preview-pane {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 20px;
  min-height: 400px;
}
.template-preview-pane h3 { font-size: 1rem; margin-bottom: 16px; border-bottom: 1px solid var(--border); padding-bottom: 12px; }

/* Form Fields */
.form-field { margin-bottom: 16px; }
.form-field label { display: block; font-size: 0.8125rem; color: var(--text-muted); margin-bottom: 4px; }
.form-field input, .form-field select, .form-field textarea {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 0.875rem;
  font-family: inherit;
  outline: none;
}
.form-field input:focus, .form-field select:focus, .form-field textarea:focus { border-color: var(--accent); }
.form-field textarea { resize: vertical; }
.toggle-label { display: flex; align-items: center; gap: 8px; cursor: pointer; }

/* Export Buttons */
.export-buttons { display: flex; gap: 8px; margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--border); }
.btn {
  padding: 8px 16px;
  border-radius: 6px;
  font-size: 0.8125rem;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: opacity 0.15s;
}
.btn:hover { opacity: 0.9; }
.btn-primary { background: var(--accent); color: white; }
.btn-secondary { background: var(--surface-2); color: var(--text); border: 1px solid var(--border); }

/* Markdown Preview Prose */
.prose { line-height: 1.7; color: var(--text); }
.prose h1 { font-size: 1.5rem; margin: 24px 0 12px; }
.prose h2 { font-size: 1.25rem; margin: 20px 0 10px; }
.prose h3 { font-size: 1.1rem; margin: 16px 0 8px; }
.prose p { margin: 8px 0; }
.prose ul, .prose ol { padding-left: 24px; margin: 8px 0; }
.prose li { margin: 4px 0; }
.prose code { background: var(--surface-2); padding: 2px 6px; border-radius: 4px; font-size: 0.875em; }
.prose pre { background: var(--surface-2); padding: 16px; border-radius: 6px; overflow-x: auto; margin: 12px 0; }
.prose pre code { background: none; padding: 0; }

/* Not Found */
.not-found { text-align: center; padding: 64px 0; }
.not-found h2 { margin-bottom: 16px; }
.not-found a { color: var(--accent); }
```

**Step 7: Verify template view works**

Run: `trunk serve`
Expected: Click a template card → navigates to `/template/<slug>` → shows form on left with fields for each placeholder → shows live markdown preview on right that updates as you type → Copy and Download buttons work

**Step 8: Commit**

```bash
git add src/pages/ src/components/ src/lib.rs index.html
git commit -m "feat: add template view with form fields, live preview, and export"
```

---

### Task 9: Polish and Final Integration

**Files:**
- Modify: `index.html` (responsive tweaks)
- Modify: `src/lib.rs` (any final wiring)

**Step 1: Add responsive and accessibility tweaks**

Add to `<style>` in `index.html`:

```css
/* Header */
header {
  display: flex;
  align-items: center;
  padding: 16px 0;
  border-bottom: 1px solid var(--border);
  margin-bottom: 32px;
}
.logo {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text);
  text-decoration: none;
}
.logo:hover { color: var(--accent); }

/* Responsive */
@media (max-width: 640px) {
  .app { padding: 0 16px; }
  .gallery-grid { grid-template-columns: 1fr; }
  .category-filter { overflow-x: auto; flex-wrap: nowrap; }
}
```

**Step 2: Verify full flow end-to-end**

Run: `trunk serve`
Test:
1. Gallery loads with 3 cards
2. Search filters cards as you type
3. Category buttons filter correctly
4. Click card → template view
5. Fill in form → preview updates live
6. Copy to clipboard works
7. Download produces a `.md` file
8. Back link returns to gallery
9. Direct URL `/template/auth-flow` works
10. Unknown slug shows "Template not found"

**Step 3: Commit**

```bash
git add index.html src/
git commit -m "feat: polish responsive layout and complete integration"
```

---

## Summary

| Task | Description | Key Files |
|------|------------|-----------|
| 1 | Project scaffold | Cargo.toml, main.rs, lib.rs, Trunk.toml, index.html |
| 2 | Data model + tests | models/template.rs, models/placeholder.rs |
| 3 | Frontmatter parser | build.rs |
| 4 | Sample templates | templates/**/*.md |
| 5 | Template store + tests | store.rs |
| 6 | Placeholder substitution + tests | substitute.rs |
| 7 | Gallery page + components | pages/gallery.rs, components/*.rs |
| 8 | Template view + form + preview + export | pages/template_view.rs, components/*.rs |
| 9 | Polish + integration | index.html, lib.rs |
