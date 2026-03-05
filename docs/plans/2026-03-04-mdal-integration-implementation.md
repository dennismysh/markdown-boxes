# MDAL Integration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Evolve markdown-boxes toward the MDAL spec by adding variable filters, typed sections, conditionals/loops, styling spec, and dual preview toggle.

**Architecture:** Extend the existing build-time template parsing pipeline and runtime substitution engine. New models (Filter, Section, StyleSpec) are parsed in `build.rs` and serialized into `templates.json`. A new `engine.rs` handles conditionals/loops at runtime. The Leptos UI gets form validation, section grouping, and a rendered/source preview toggle.

**Tech Stack:** Rust, Leptos 0.8 (CSR), comrak, serde, Trunk (WASM)

---

## Phase 1: File Format Migration

### Task 1: Rename template files from .md to .mdal

**Files:**
- Rename: `templates/implementation-plans/auth-flow.md` → `templates/implementation-plans/auth-flow.mdal`
- Rename: `templates/implementation-plans/ci-cd-pipeline.md` → `templates/implementation-plans/ci-cd-pipeline.mdal`
- Rename: `templates/design-prompts/dashboard.md` → `templates/design-prompts/dashboard.mdal`
- Rename: `templates/design-prompts/landing-page.md` → `templates/design-prompts/landing-page.mdal`
- Rename: `templates/ui-components/data-table.md` → `templates/ui-components/data-table.mdal`
- Rename: `templates/ui-components/form-wizard.md` → `templates/ui-components/form-wizard.mdal`
- Rename: `templates/full-stack-flows/crud-api.md` → `templates/full-stack-flows/crud-api.mdal`
- Rename: `templates/backend-patterns/background-jobs.md` → `templates/backend-patterns/background-jobs.mdal`

**Step 1: Rename all template files**

```bash
cd templates
for f in $(find . -name "*.md"); do mv "$f" "${f%.md}.mdal"; done
```

**Step 2: Update build.rs glob pattern**

In `build.rs`, change the glob from `*.md` to `*.mdal`:

```rust
// Change this line:
for entry in glob::glob("templates/**/*.md").expect("glob pattern") {
// To:
for entry in glob::glob("templates/**/*.mdal").expect("glob pattern") {
```

**Step 3: Update .gitignore if needed**

Check that `.gitignore` doesn't exclude `.mdal` files.

**Step 4: Build and run tests**

Run: `cargo test`
Expected: All 16 tests pass (templates load from regenerated JSON)

**Step 5: Commit**

```bash
git add -A
git commit -m "refactor: rename template files from .md to .mdal"
```

---

### Task 2: Extend frontmatter with MDAL fields

**Files:**
- Modify: `build.rs` (Frontmatter struct, lines ~45-55)
- Modify: `src/models/template.rs` (Template struct, lines ~5-15)

**Step 1: Write failing test for new frontmatter fields**

Add to `src/models/template.rs` tests:

```rust
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
```

**Step 2: Run test to verify it fails**

Run: `cargo test deserialize_mdal_frontmatter_fields`
Expected: FAIL — fields don't exist on Template

**Step 3: Add MDAL fields to Template struct**

In `src/models/template.rs`, update the Template struct:

```rust
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
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutputTarget {
    pub format: String,
    pub target: String,
}
```

**Step 4: Run test to verify it passes**

Run: `cargo test deserialize_mdal_frontmatter_fields`
Expected: PASS

**Step 5: Update build.rs to parse and emit new fields**

In `build.rs`, add the new fields to `Frontmatter`, `TemplateData`, and the `OutputData` struct:

```rust
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
    placeholders: Vec<PlaceholderData>,
}
```

Update `parse_template` to pass through the new fields:

```rust
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
    placeholders: frontmatter.placeholders,
    body,
})
```

**Step 6: Run all tests**

Run: `cargo test`
Expected: All tests pass (existing templates have no new fields — they default to None/empty)

**Step 7: Commit**

```bash
git add src/models/template.rs build.rs
git commit -m "feat: add MDAL frontmatter fields (type, version, author, outputs)"
```

---

### Task 3: Add MDAL frontmatter to template files

**Files:**
- Modify: All 8 `.mdal` template files

**Step 1: Update auth-flow.mdal frontmatter**

Add MDAL fields to the existing frontmatter (insert after the opening `---`):

```yaml
---
type: application
name: "Authentication Flow"
version: 1.0.0
author: "@system/templates"
title: "Authentication Flow"
category: implementation-plan
tags: [auth, jwt, backend, security]
preview: auth-flow-hero.svg
description: "Step-by-step plan for building JWT-based authentication with login, registration, and token refresh."
outputs:
  - format: html
    target: web
placeholders:
  # ... existing placeholders unchanged ...
---
```

**Step 2: Repeat for remaining 7 templates**

Add the same pattern to each file:
- `type: application` for implementation plans and full-stack flows
- `type: content` for design prompts
- `type: application` for UI components and backend patterns
- `version: 1.0.0`, `author: "@system/templates"` for all
- `outputs: [{format: html, target: web}]` for all

Note: Keep `title` field — it's used by the existing system. The `name` field from MDAL spec is a synonym; use `title` as the canonical field.

**Step 3: Build and run tests**

Run: `cargo test`
Expected: All tests pass

**Step 4: Commit**

```bash
git add templates/
git commit -m "feat: add MDAL frontmatter to all template files"
```

---

## Phase 2: Variable Filters

### Task 4: Add Filter enum and parsing

**Files:**
- Create: `src/models/filter.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/models/placeholder.rs`

**Step 1: Write failing tests for Filter**

Create `src/models/filter.rs`:

```rust
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
            // Parse [a, b, c] syntax
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
        assert_eq!(filters, vec![Filter::Options(vec!["React".into(), "Vue".into(), "Svelte".into()])]);
    }

    #[test]
    fn parse_multiple_filters() {
        let (name, filters) = parse_inline_filters("name | required | max_length: 50 | default: \"Untitled\"");
        assert_eq!(name, "name");
        assert_eq!(filters.len(), 3);
        assert_eq!(filters[0], Filter::Required);
        assert_eq!(filters[1], Filter::MaxLength(50));
        assert_eq!(filters[2], Filter::Default("Untitled".to_string()));
    }
}
```

**Step 2: Add module to mod.rs**

In `src/models/mod.rs`:

```rust
pub mod filter;
pub mod placeholder;
pub mod template;
```

**Step 3: Run tests**

Run: `cargo test parse_`
Expected: All 6 filter parsing tests pass

**Step 4: Commit**

```bash
git add src/models/filter.rs src/models/mod.rs
git commit -m "feat: add Filter enum with inline filter syntax parser"
```

---

### Task 5: Add filters field to Placeholder

**Files:**
- Modify: `src/models/placeholder.rs`

**Step 1: Write failing test**

Add to `src/models/placeholder.rs` tests:

```rust
#[test]
fn deserialize_placeholder_with_filters() {
    let json = r#"{"key":"name","label":"Name","type":"text","options":null,"filters":[{"required":null},{"max_length":100}]}"#;
    let p: Placeholder = serde_json::from_str(json).unwrap();
    assert_eq!(p.filters.len(), 2);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test deserialize_placeholder_with_filters`
Expected: FAIL — no `filters` field

**Step 3: Add filters to Placeholder struct**

In `src/models/placeholder.rs`:

```rust
use serde::{Deserialize, Serialize};
use super::filter::Filter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Placeholder {
    pub key: String,
    pub label: String,
    #[serde(rename = "type")]
    pub kind: PlaceholderType,
    pub options: Option<Vec<String>>,
    #[serde(default)]
    pub filters: Vec<Filter>,
}
```

**Step 4: Run all tests**

Run: `cargo test`
Expected: All tests pass (existing placeholders have no `filters` — defaults to empty vec)

**Step 5: Commit**

```bash
git add src/models/placeholder.rs
git commit -m "feat: add filters field to Placeholder struct"
```

---

### Task 6: Parse filters from frontmatter in build.rs

**Files:**
- Modify: `build.rs`

**Step 1: Add filters to PlaceholderData and Frontmatter parsing**

In `build.rs`, update `PlaceholderData`:

```rust
#[derive(Debug, Deserialize, Serialize)]
struct PlaceholderData {
    key: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    options: Option<Vec<String>>,
    #[serde(default)]
    filters: Vec<serde_json::Value>,
}
```

The MDAL frontmatter `filters` field is a list of objects like `- required: true` or `- max_length: 100`. These are parsed as generic JSON values and serialized into `templates.json` for the runtime to interpret.

Alternatively, keep it simpler — parse MDAL filter syntax in the frontmatter as a list of strings or key-value pairs:

```yaml
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
    filters:
      - required
      - max_length: 100
      - default: "My Project"
```

To handle this mixed format (bare strings and key-value maps), use an untagged enum in `build.rs`:

```rust
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum FilterData {
    Flag(String),
    KeyValue(std::collections::HashMap<String, serde_json::Value>),
}

#[derive(Debug, Deserialize, Serialize)]
struct PlaceholderData {
    key: String,
    label: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    options: Option<Vec<String>>,
    #[serde(default)]
    filters: Vec<FilterData>,
}
```

Then in `parse_template`, convert `FilterData` to the runtime `Filter` JSON format before writing to `templates.json`. Add a conversion function:

```rust
fn convert_filters(filter_data: &[FilterData]) -> Vec<serde_json::Value> {
    filter_data.iter().map(|f| match f {
        FilterData::Flag(s) if s == "required" => {
            serde_json::json!({"required": null})
        }
        FilterData::KeyValue(map) => {
            if let Some(v) = map.get("max_length") {
                serde_json::json!({"max_length": v})
            } else if let Some(v) = map.get("default") {
                serde_json::json!({"default": v})
            } else if let Some(v) = map.get("options") {
                serde_json::json!({"options": v})
            } else if let Some(v) = map.get("required") {
                let _ = v;
                serde_json::json!({"required": null})
            } else {
                serde_json::json!(null)
            }
        }
        _ => serde_json::json!(null),
    }).collect()
}
```

Update `TemplateData` to use `Vec<serde_json::Value>` for filters, and update `parse_template` to call `convert_filters`.

**Step 2: Build and run tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Commit**

```bash
git add build.rs
git commit -m "feat: parse MDAL filter syntax from frontmatter in build.rs"
```

---

### Task 7: Strip filter syntax in substitution

**Files:**
- Modify: `src/substitute.rs`

**Step 1: Write failing tests**

Add to `src/substitute.rs` tests:

```rust
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
```

**Step 2: Run tests to verify they fail**

Run: `cargo test substitute_strips`
Expected: FAIL — current substitute doesn't understand filter syntax

**Step 3: Update substitute function**

Replace the `substitute` function in `src/substitute.rs`:

```rust
use std::collections::HashMap;
use crate::models::filter::parse_inline_filters;

pub fn substitute(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::with_capacity(body.len());
    let mut remaining = body;

    while let Some(start) = remaining.find("{{") {
        result.push_str(&remaining[..start]);

        if let Some(end) = remaining[start..].find("}}") {
            let expr = &remaining[start + 2..start + end];
            let (var_name, filters) = parse_inline_filters(expr);

            if let Some(val) = values.get(&var_name) {
                if !val.is_empty() {
                    result.push_str(val);
                } else {
                    // Empty value: check for default
                    if let Some(default) = get_default(&filters) {
                        result.push_str(&default);
                    }
                }
            } else {
                // No value provided: check for default
                if let Some(default) = get_default(&filters) {
                    result.push_str(&default);
                } else {
                    // Show clean placeholder without filters
                    result.push_str(&format!("{{{{{var_name}}}}}"));
                }
            }

            remaining = &remaining[start + end + 2..];
        } else {
            // No closing }}, push rest and break
            result.push_str(&remaining[start..]);
            remaining = "";
        }
    }
    result.push_str(remaining);
    result
}

fn get_default(filters: &[crate::models::filter::Filter]) -> Option<String> {
    use crate::models::filter::Filter;
    filters.iter().find_map(|f| match f {
        Filter::Default(val) => Some(val.clone()),
        _ => None,
    })
}
```

**Step 4: Run tests to verify they pass**

Run: `cargo test`
Expected: All substitute tests pass, including new ones

**Step 5: Commit**

```bash
git add src/substitute.rs
git commit -m "feat: strip filter syntax during substitution, support default values"
```

---

### Task 8: Form validation from filters

**Files:**
- Modify: `src/components/form_field.rs`
- Modify: `index.html` (CSS for validation states)

**Step 1: Update FormField to read filters**

The `Placeholder` now has a `filters` field. Update `FormField` to show:
- A red asterisk and `required` class for `Required` filter
- A character counter for `MaxLength` filter
- Pre-fill the field value for `Default` filter (this happens in `template_view.rs`)

In `src/components/form_field.rs`, add filter-aware rendering:

```rust
use leptos::prelude::*;
use crate::models::placeholder::{Placeholder, PlaceholderType};
use crate::models::filter::Filter;

#[component]
pub fn FormField(
    placeholder: Placeholder,
    value: ReadSignal<String>,
    on_change: WriteSignal<String>,
) -> impl IntoView {
    let label_text = placeholder.label.clone();
    let key = placeholder.key.clone();
    let input_label = placeholder.label.clone();

    let is_required = placeholder.filters.iter().any(|f| matches!(f, Filter::Required));
    let max_length = placeholder.filters.iter().find_map(|f| match f {
        Filter::MaxLength(n) => Some(*n),
        _ => None,
    });

    let label_suffix = if is_required { " *" } else { "" };
    let full_label = format!("{label_text}{label_suffix}");

    let input_view = match placeholder.kind {
        PlaceholderType::Text => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            let ml = max_length;
            view! {
                <input
                    type="text"
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                    maxlength=ml.map(|n| n.to_string()).unwrap_or_default()
                />
                {ml.map(|max| {
                    view! {
                        <span class="char-counter">
                            {move || format!("{}/{}", value.get().len(), max)}
                        </span>
                    }
                })}
            }.into_any()
        },
        PlaceholderType::Multiline => {
            let ph = format!("Enter {}...", input_label);
            let k = key.clone();
            let ml = max_length;
            view! {
                <textarea
                    id=k
                    prop:value=value
                    on:input=move |ev| on_change.set(event_target_value(&ev))
                    placeholder=ph
                    rows=4
                    maxlength=ml.map(|n| n.to_string()).unwrap_or_default()
                />
                {ml.map(|max| {
                    view! {
                        <span class="char-counter">
                            {move || format!("{}/{}", value.get().len(), max)}
                        </span>
                    }
                })}
            }.into_any()
        },
        PlaceholderType::Select => {
            // Options can come from filters or the legacy options field
            let filter_options = placeholder.filters.iter().find_map(|f| match f {
                Filter::Options(opts) => Some(opts.clone()),
                _ => None,
            });
            let options = filter_options.or(placeholder.options).unwrap_or_default();
            let k = key.clone();
            view! {
                <select
                    id=k
                    on:change=move |ev| on_change.set(event_target_value(&ev))
                >
                    <option value="" disabled selected>"Select..."</option>
                    {options.into_iter().map(|opt| {
                        let val = opt.clone();
                        view! {
                            <option value=val>{opt}</option>
                        }
                    }).collect_view()}
                </select>
            }.into_any()
        },
        PlaceholderType::Boolean => {
            view! {
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
            }.into_any()
        },
    };

    let field_class = if is_required { "form-field required" } else { "form-field" };

    view! {
        <div class=field_class>
            <label for=key>{full_label}</label>
            {input_view}
        </div>
    }
}
```

**Step 2: Pre-fill default values in template_view.rs**

In `src/pages/template_view.rs`, when creating field signals, check for a `Default` filter and use it as the initial value:

```rust
// In the field_signals creation, change:
let (read, write) = signal(String::new());
// To:
let default_val = p.filters.iter().find_map(|f| match f {
    crate::models::filter::Filter::Default(v) => Some(v.clone()),
    _ => None,
}).unwrap_or_default();
let (read, write) = signal(default_val);
```

**Step 3: Add CSS for validation states**

In `index.html`, add after the existing `.form-field` styles:

```css
.form-field.required label::after { content: ""; }
.char-counter { display: block; font-size: 0.75rem; color: var(--text-muted); text-align: right; margin-top: 2px; }
```

**Step 4: Build and verify**

Run: `cargo test`
Expected: All tests pass

Run: `trunk serve` and manually verify a template with filters shows the asterisk and character counter.

**Step 5: Commit**

```bash
git add src/components/form_field.rs src/pages/template_view.rs index.html
git commit -m "feat: form validation from filters (required, max_length, default)"
```

---

### Task 9: Add showcase filter syntax to auth-flow template

**Files:**
- Modify: `templates/implementation-plans/auth-flow.mdal`

**Step 1: Update auth-flow placeholders to use filters**

```yaml
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
    filters:
      - required
      - max_length: 50
      - default: "My App"
  - key: auth_provider
    label: "Auth Provider"
    type: select
    filters:
      - required
      - options: [Firebase, Supabase, Custom JWT, Auth0]
  - key: framework
    label: "Backend Framework"
    type: select
    filters:
      - required
      - options: [Express, FastAPI, Axum, Django, Rails]
  - key: additional_requirements
    label: "Additional Requirements"
    type: multiline
    filters:
      - max_length: 500
```

Remove the old `options` fields from the select placeholders since they're now in `filters`.

**Step 2: Build and test**

Run: `cargo test`
Expected: All tests pass

**Step 3: Commit**

```bash
git add templates/implementation-plans/auth-flow.mdal
git commit -m "feat: showcase MDAL filter syntax in auth-flow template"
```

---

## Phase 3: Typed Sections

### Task 10: Add Section model

**Files:**
- Create: `src/models/section.rs`
- Modify: `src/models/mod.rs`

**Step 1: Create Section struct with tests**

Create `src/models/section.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Section {
    pub name: String,
    pub section_type: SectionType,
    pub properties: HashMap<String, String>,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SectionType {
    Component,
    Layout,
    Logic,
    Data,
    Style,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_section() {
        let json = r#"{
            "name": "Hero",
            "section_type": "component",
            "properties": {"layout": "centered", "required": "true"},
            "content": "### Content\n- headline: {{title}}"
        }"#;
        let s: Section = serde_json::from_str(json).unwrap();
        assert_eq!(s.name, "Hero");
        assert_eq!(s.section_type, SectionType::Component);
        assert_eq!(s.properties.get("layout").unwrap(), "centered");
    }
}
```

**Step 2: Add to mod.rs**

In `src/models/mod.rs`:

```rust
pub mod filter;
pub mod placeholder;
pub mod section;
pub mod template;
```

**Step 3: Run tests**

Run: `cargo test deserialize_section`
Expected: PASS

**Step 4: Commit**

```bash
git add src/models/section.rs src/models/mod.rs
git commit -m "feat: add Section model with SectionType enum"
```

---

### Task 11: Parse sections in build.rs

**Files:**
- Modify: `build.rs`
- Modify: `src/models/template.rs`

**Step 1: Add sections field to Template**

In `src/models/template.rs`, add to Template struct:

```rust
use super::section::Section;

// In Template struct, add:
#[serde(default)]
pub sections: Vec<Section>,
```

**Step 2: Add section parsing to build.rs**

Add a `SectionData` struct and parsing function to `build.rs`:

```rust
#[derive(Debug, Serialize)]
struct SectionData {
    name: String,
    section_type: String,
    properties: std::collections::HashMap<String, String>,
    content: String,
}
```

Add `sections: Vec<SectionData>` to `TemplateData`.

Add a function to parse sections from the template body:

```rust
fn parse_sections(body: &str) -> (Vec<SectionData>, String) {
    let mut sections = Vec::new();
    let mut cleaned_body = String::new();
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("## Section:") {
            let name = line.trim_start_matches("## Section:").trim().to_string();
            let mut properties = std::collections::HashMap::new();
            let mut content = String::new();
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
            cleaned_body.push_str(line);
            cleaned_body.push('\n');
            i += 1;
        }
    }

    (sections, cleaned_body.trim().to_string())
}
```

Call `parse_sections` in `parse_template` and include sections in the output:

```rust
let (sections, _) = parse_sections(&body);
// Keep original body intact — sections are metadata, body stays for rendering
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 4: Commit**

```bash
git add build.rs src/models/template.rs
git commit -m "feat: parse typed sections from template body in build.rs"
```

---

### Task 12: Display sections in template view

**Files:**
- Modify: `src/pages/template_view.rs`
- Modify: `index.html`

**Step 1: Group form fields by section**

In `src/pages/template_view.rs`, when the template has sections, group placeholders by which section they appear in. For now, show section names as headers above their form fields.

Add section badges above the form fields:

```rust
// After the <h3>"Customize"</h3>, add:
{if !tmpl_sections.is_empty() {
    Some(view! {
        <div class="section-badges">
            {tmpl_sections.iter().map(|s| {
                let name = s.name.clone();
                let stype = format!("{:?}", s.section_type).to_lowercase();
                view! {
                    <span class="section-badge">
                        <span class="section-badge-type">{stype}</span>
                        {name}
                    </span>
                }
            }).collect_view()}
        </div>
    })
} else {
    None
}}
```

**Step 2: Add CSS for section badges**

In `index.html`:

```css
.section-badges { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 16px; }
.section-badge { font-size: 0.75rem; padding: 3px 10px; background: var(--surface-2); border-radius: 4px; color: var(--text-muted); }
.section-badge-type { color: var(--accent); margin-right: 4px; text-transform: uppercase; font-size: 0.625rem; }
```

**Step 3: Build and verify**

Run: `cargo test`
Run: `trunk serve` — verify section badges appear for templates that have sections

**Step 4: Commit**

```bash
git add src/pages/template_view.rs index.html
git commit -m "feat: display typed section badges in template detail view"
```

---

## Phase 4: Conditionals & Loops

### Task 13: Create engine.rs with conditional evaluation

**Files:**
- Create: `src/engine.rs`
- Modify: `src/lib.rs`

**Step 1: Write failing tests for conditionals**

Create `src/engine.rs`:

```rust
use std::collections::HashMap;

/// Evaluate MDAL expression blocks (conditionals and loops) in template body.
/// Processes {{#if}}/{{else}}/{{/if}} and {{#each}} blocks.
/// Returns the body with blocks expanded/collapsed based on values.
pub fn evaluate_blocks(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut remaining = body;

    while let Some(start) = remaining.find("{{#if ") {
        result.push_str(&remaining[..start]);

        let after_tag = &remaining[start + 6..];
        let cond_end = match after_tag.find("}}") {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };
        let condition = after_tag[..cond_end].trim();

        let block_start = start + 6 + cond_end + 2;
        let rest = &remaining[block_start..];

        // Find matching {{/if}}
        let endif_tag = "{{/if}}";
        let else_tag = "{{else}}";

        let endif_pos = match rest.find(endif_tag) {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let (if_block, else_block) = if let Some(else_pos) = rest[..endif_pos].find(else_tag) {
            (
                rest[..else_pos].trim(),
                rest[else_pos + else_tag.len()..endif_pos].trim(),
            )
        } else {
            (rest[..endif_pos].trim(), "")
        };

        let truthy = evaluate_condition(condition, values);

        if truthy {
            result.push_str(if_block);
        } else {
            result.push_str(else_block);
        }

        remaining = &rest[endif_pos + endif_tag.len()..];
    }

    result.push_str(remaining);
    result
}

fn evaluate_condition(condition: &str, values: &HashMap<String, String>) -> bool {
    let condition = condition.trim();

    // Check for comparison operators
    if let Some((left, right)) = condition.split_once("==") {
        let left = left.trim();
        let right = right.trim().trim_matches('"');
        let val = values.get(left).map(|s| s.as_str()).unwrap_or("");
        return val == right;
    }

    if let Some((left, right)) = condition.split_once('>') {
        let left = left.trim();
        let right = right.trim();

        // Handle .length
        if left.ends_with(".length") {
            let var_name = left.trim_end_matches(".length");
            let val = values.get(var_name).map(|s| s.as_str()).unwrap_or("");
            let count = if val.is_empty() { 0 } else {
                // Rough count: split by comma for arrays
                val.split(',').count()
            };
            if let Ok(n) = right.parse::<usize>() {
                return count > n;
            }
        }
    }

    // Bare variable — truthy if non-empty
    let val = values.get(condition).map(|s| s.as_str()).unwrap_or("");
    !val.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_truthy_variable() {
        let body = "before {{#if name}}Hello {{name}}{{/if}} after";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Alice".to_string());
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before Hello {{name}} after");
    }

    #[test]
    fn if_falsy_variable() {
        let body = "before {{#if name}}Hello{{/if}} after";
        let values = HashMap::new();
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before  after");
    }

    #[test]
    fn if_else_truthy() {
        let body = "{{#if name}}Hello {{name}}{{else}}No name{{/if}}";
        let mut values = HashMap::new();
        values.insert("name".to_string(), "Bob".to_string());
        assert_eq!(evaluate_blocks(body, &values), "Hello {{name}}");
    }

    #[test]
    fn if_else_falsy() {
        let body = "{{#if name}}Hello{{else}}No name{{/if}}";
        let values = HashMap::new();
        assert_eq!(evaluate_blocks(body, &values), "No name");
    }

    #[test]
    fn if_equality_check() {
        let body = r#"{{#if role == "admin"}}Admin panel{{else}}User view{{/if}}"#;
        let mut values = HashMap::new();
        values.insert("role".to_string(), "admin".to_string());
        assert_eq!(evaluate_blocks(body, &values), "Admin panel");
    }

    #[test]
    fn if_equality_check_fails() {
        let body = r#"{{#if role == "admin"}}Admin panel{{else}}User view{{/if}}"#;
        let mut values = HashMap::new();
        values.insert("role".to_string(), "user".to_string());
        assert_eq!(evaluate_blocks(body, &values), "User view");
    }

    #[test]
    fn no_conditionals_passthrough() {
        let body = "Just regular text with {{placeholder}}";
        let values = HashMap::new();
        assert_eq!(evaluate_blocks(body, &values), body);
    }

    #[test]
    fn malformed_if_left_as_literal() {
        let body = "before {{#if name after";
        let values = HashMap::new();
        let result = evaluate_blocks(body, &values);
        assert_eq!(result, "before {{#if name after");
    }
}
```

**Step 2: Add module to lib.rs**

In `src/lib.rs`:

```rust
pub mod engine;
```

**Step 3: Run tests**

Run: `cargo test -p markdown-boxes -- engine`
Expected: All 8 engine tests pass

**Step 4: Commit**

```bash
git add src/engine.rs src/lib.rs
git commit -m "feat: add MDAL conditional expression evaluator (engine.rs)"
```

---

### Task 14: Add {{#each}} loop support

**Files:**
- Modify: `src/engine.rs`

**Step 1: Write failing tests for each loops**

Add to `src/engine.rs` tests:

```rust
#[test]
fn each_loop_basic() {
    let body = "{{#each items as item}}- {{item}}\n{{/each}}";
    let mut values = HashMap::new();
    values.insert("items".to_string(), "Apple,Banana,Cherry".to_string());
    let result = evaluate_blocks(body, &values);
    assert_eq!(result, "- Apple\n- Banana\n- Cherry\n");
}

#[test]
fn each_loop_empty() {
    let body = "{{#each items as item}}- {{item}}\n{{/each}}";
    let values = HashMap::new();
    let result = evaluate_blocks(body, &values);
    assert_eq!(result, "");
}
```

**Step 2: Run tests to verify they fail**

Run: `cargo test each_loop`
Expected: FAIL — `{{#each` is not yet processed

**Step 3: Add each loop processing**

In `src/engine.rs`, update `evaluate_blocks` to also handle `{{#each}}` before or after `{{#if}}` processing. Add a second pass:

```rust
pub fn evaluate_blocks(body: &str, values: &HashMap<String, String>) -> String {
    let after_ifs = evaluate_ifs(body, values);
    evaluate_eachs(&after_ifs, values)
}

fn evaluate_ifs(body: &str, values: &HashMap<String, String>) -> String {
    // ... move the existing if/else logic here ...
}

fn evaluate_eachs(body: &str, values: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut remaining = body;

    while let Some(start) = remaining.find("{{#each ") {
        result.push_str(&remaining[..start]);

        let after_tag = &remaining[start + 8..];
        let tag_end = match after_tag.find("}}") {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let expr = &after_tag[..tag_end]; // e.g., "items as item"
        let parts: Vec<&str> = expr.split(" as ").collect();
        let collection_name = parts[0].trim();
        let item_name = if parts.len() > 1 { parts[1].trim() } else { "item" };

        let block_start_in_rest = tag_end + 2;
        let rest = &after_tag[block_start_in_rest..];

        let end_tag = "{{/each}}";
        let end_pos = match rest.find(end_tag) {
            Some(i) => i,
            None => {
                result.push_str(&remaining[start..]);
                remaining = "";
                break;
            }
        };

        let loop_body = &rest[..end_pos];

        // Get collection value — comma-separated string
        let items_str = values.get(collection_name).map(|s| s.as_str()).unwrap_or("");
        if !items_str.is_empty() {
            for item in items_str.split(',') {
                let item = item.trim();
                let expanded = loop_body.replace(&format!("{{{{{item_name}}}}}"), item);
                result.push_str(&expanded);
            }
        }

        remaining = &rest[end_pos + end_tag.len()..];
    }

    result.push_str(remaining);
    result
}
```

**Step 4: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 5: Commit**

```bash
git add src/engine.rs
git commit -m "feat: add {{#each}} loop support to MDAL engine"
```

---

### Task 15: Wire engine into live preview

**Files:**
- Modify: `src/pages/template_view.rs`

**Step 1: Update preview_content to use engine**

In `src/pages/template_view.rs`, update the `preview_content` signal to run `evaluate_blocks` before `substitute`:

```rust
use crate::engine::evaluate_blocks;

// In the preview_content Signal::derive, change to:
let preview_content = Signal::derive(move || {
    let mut values = HashMap::new();
    for (p, read, _) in &field_signals_for_preview {
        let val = read.get();
        if !val.is_empty() {
            values.insert(p.key.clone(), val);
        }
    }
    let after_blocks = evaluate_blocks(&body, &values);
    substitute(&after_blocks, &values)
});
```

**Step 2: Build and verify**

Run: `cargo test`
Run: `trunk serve` — verify conditionals in templates expand/collapse in real time

**Step 3: Commit**

```bash
git add src/pages/template_view.rs
git commit -m "feat: wire MDAL engine into live preview (conditionals + loops)"
```

---

## Phase 5: Styling Specification

### Task 16: Add StyleSpec model

**Files:**
- Create: `src/models/style.rs`
- Modify: `src/models/mod.rs`
- Modify: `src/models/template.rs`

**Step 1: Create StyleSpec with tests**

Create `src/models/style.rs`:

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StyleSpec {
    pub name: String,
    pub approach: Option<String>,
    pub colors: HashMap<String, String>,
    pub typography: HashMap<String, String>,
    pub effects: HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_style_spec() {
        let json = r#"{
            "name": "DesignSystem",
            "approach": "brutalist",
            "colors": {"primary": "#000000", "background": "#ffffff"},
            "typography": {"headings": "JetBrains Mono"},
            "effects": {"borders": "2px solid"}
        }"#;
        let s: StyleSpec = serde_json::from_str(json).unwrap();
        assert_eq!(s.approach.unwrap(), "brutalist");
        assert_eq!(s.colors.get("primary").unwrap(), "#000000");
    }
}
```

**Step 2: Add to mod.rs and Template**

In `src/models/mod.rs`:

```rust
pub mod filter;
pub mod placeholder;
pub mod section;
pub mod style;
pub mod template;
```

In `src/models/template.rs`, add:

```rust
use super::style::StyleSpec;

// In Template struct, add:
#[serde(default)]
pub style_spec: Option<StyleSpec>,
```

**Step 3: Run tests**

Run: `cargo test`
Expected: All tests pass

**Step 4: Commit**

```bash
git add src/models/style.rs src/models/mod.rs src/models/template.rs
git commit -m "feat: add StyleSpec model for MDAL styling sections"
```

---

### Task 17: Parse styling sections in build.rs

**Files:**
- Modify: `build.rs`

**Step 1: Add styling section parsing**

In `build.rs`, extend `parse_sections` (or add a new function) to detect `## Styling:` headings. When found, parse the subsections (Colors, Typography, Effects) into a `StyleSpecData`:

```rust
#[derive(Debug, Serialize)]
struct StyleSpecData {
    name: String,
    approach: Option<String>,
    colors: std::collections::HashMap<String, String>,
    typography: std::collections::HashMap<String, String>,
    effects: std::collections::HashMap<String, String>,
}
```

Add `style_spec: Option<StyleSpecData>` to `TemplateData`.

Parse `## Styling:` sections similarly to `## Section:` but route them to the style spec instead of the sections vec. Parse `### Colors`, `### Typography`, `### Effects` subsections by reading `- key: value` lines under each.

```rust
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

            return Some(StyleSpecData { name, approach, colors, typography, effects });
        }
        i += 1;
    }
    None
}
```

Call in `parse_template`:

```rust
let style_spec = parse_style_spec(&body);
```

**Step 2: Build and run tests**

Run: `cargo test`
Expected: All tests pass

**Step 3: Commit**

```bash
git add build.rs
git commit -m "feat: parse ## Styling: sections from MDAL templates"
```

---

### Task 18: Apply style spec as CSS custom properties

**Files:**
- Modify: `src/components/markdown_preview.rs`
- Modify: `src/pages/template_view.rs`

**Step 1: Pass style spec to preview**

In `src/pages/template_view.rs`, pass the template's `style_spec` to the preview pane. Generate a CSS `style` string from the StyleSpec:

```rust
let style_css = tmpl.style_spec.as_ref().map(|s| {
    let mut css = String::new();
    for (k, v) in &s.colors {
        css.push_str(&format!("--mdal-color-{k}: {v}; "));
    }
    for (k, v) in &s.typography {
        css.push_str(&format!("--mdal-font-{k}: {v}; "));
    }
    css
}).unwrap_or_default();
```

Apply the style string to the preview pane wrapper:

```rust
<div class="template-preview-pane" style=style_css.clone()>
```

**Step 2: Use CSS vars in preview styling**

In `index.html`, add fallback-aware custom property usage:

```css
.template-preview-pane {
    /* existing styles... */
    color: var(--mdal-color-text, var(--text));
    background: var(--mdal-color-background, var(--surface));
}
.template-preview-pane .prose h1,
.template-preview-pane .prose h2,
.template-preview-pane .prose h3 {
    font-family: var(--mdal-font-headings, inherit);
}
.template-preview-pane .prose {
    font-family: var(--mdal-font-body, inherit);
}
```

**Step 3: Build and verify**

Run: `cargo test`
Run: `trunk serve` — verify a template with `## Styling:` section shows custom colors/fonts in preview

**Step 4: Commit**

```bash
git add src/pages/template_view.rs src/components/markdown_preview.rs index.html
git commit -m "feat: apply MDAL style spec as CSS custom properties in preview"
```

---

## Phase 6: Dual Preview Toggle

### Task 19: Add rendered/source preview toggle

**Files:**
- Modify: `src/pages/template_view.rs`
- Modify: `index.html`

**Step 1: Add preview mode signal**

In `src/pages/template_view.rs`, add a toggle signal:

```rust
let (preview_mode, set_preview_mode) = signal("rendered".to_string());
```

**Step 2: Add toggle buttons**

Before the preview content, add toggle buttons:

```rust
<div class="preview-toggle">
    <button
        class=move || if preview_mode.get() == "rendered" { "toggle-btn active" } else { "toggle-btn" }
        on:click=move |_| set_preview_mode.set("rendered".to_string())
    >
        "Rendered"
    </button>
    <button
        class=move || if preview_mode.get() == "source" { "toggle-btn active" } else { "toggle-btn" }
        on:click=move |_| set_preview_mode.set("source".to_string())
    >
        "Source"
    </button>
</div>
```

**Step 3: Conditional rendering based on mode**

Show either the MarkdownPreview or a raw source view:

```rust
{move || {
    if preview_mode.get() == "rendered" {
        view! { <MarkdownPreview content=preview_content /> }.into_any()
    } else {
        // Source mode: show raw MDAL with substituted values
        let source = preview_content.get();
        view! {
            <pre class="mdal-source"><code>{source}</code></pre>
        }.into_any()
    }
}}
```

**Step 4: Add CSS for toggle and source view**

In `index.html`:

```css
.preview-toggle { display: flex; gap: 4px; margin-bottom: 16px; }
.toggle-btn {
    padding: 4px 12px;
    background: var(--surface-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
}
.toggle-btn.active { background: var(--accent); border-color: var(--accent); color: white; }
.mdal-source {
    background: var(--bg);
    padding: 16px;
    border-radius: 6px;
    font-size: 0.875rem;
    line-height: 1.6;
    overflow-x: auto;
    white-space: pre-wrap;
    color: var(--text);
}
```

**Step 5: Build and verify**

Run: `cargo test`
Run: `trunk serve` — toggle between Rendered and Source views

**Step 6: Commit**

```bash
git add src/pages/template_view.rs index.html
git commit -m "feat: add rendered/source dual preview toggle"
```

---

### Task 20: Update download button for .mdal

**Files:**
- Modify: `src/pages/template_view.rs`

**Step 1: Change download filename**

In the `ExportButtons` component, change the download filename:

```rust
a.set_download("template.mdal");
```

Also update the button label:

```rust
"Download .mdal"
```

**Step 2: Build and verify**

Run: `cargo test`

**Step 3: Commit**

```bash
git add src/pages/template_view.rs
git commit -m "feat: update export to download .mdal files"
```

---

### Task 21: Add showcase template with all MDAL features

**Files:**
- Create: `templates/design-prompts/portfolio.mdal`

**Step 1: Create a template that uses sections, conditionals, filters, and styling**

```markdown
---
type: application
name: "Portfolio Website"
version: 1.0.0
author: "@system/templates"
title: "Portfolio Website"
category: design-prompt
tags: [portfolio, design, personal, website]
description: "MDAL specification for a personal portfolio website with sections, conditionals, and styling."
outputs:
  - format: html
    target: web
placeholders:
  - key: name
    label: "Your Name"
    type: text
    filters:
      - required
      - max_length: 50
  - key: tagline
    label: "Tagline"
    type: text
    filters:
      - required
      - max_length: 150
      - default: "Building things that matter"
  - key: style
    label: "Design Style"
    type: select
    filters:
      - required
      - options: [brutalist, minimalist, playful, professional]
  - key: show_projects
    label: "Show Projects Section"
    type: boolean
  - key: accent_color
    label: "Accent Color"
    type: text
    filters:
      - default: "#6366f1"
---

# {{name | required}}'s Portfolio

## Section: Hero
- type: component
- layout: centered

### Content
- headline: {{name | required}}
- tagline: {{tagline | default: "Building things that matter"}}

---

## Section: About
- type: component
- layout: two_column

A brief introduction about {{name}}.

---

{{#if show_projects}}
## Section: Projects
- type: component
- layout: grid

Showcase your best work here.
{{else}}
_Projects section hidden._
{{/if}}

---

## Section: Contact
- type: component
- layout: centered

Get in touch with {{name}}.

---

## Styling: DesignSystem
- type: style
- approach: {{style | options: [brutalist, minimalist, playful, professional]}}

### Colors
- primary: {{accent_color | default: "#6366f1"}}
- background: #0f0f0f
- text: #e8e8e8

### Typography
- headings: JetBrains Mono
- body: Inter
- base_size: 16px

### Effects
- borders: 2px solid
- radius: 0px
```

**Step 2: Build and verify**

Run: `cargo test`
Run: `trunk serve` — verify the new template appears in gallery with all MDAL features working

**Step 3: Commit**

```bash
git add templates/design-prompts/portfolio.mdal
git commit -m "feat: add portfolio template showcasing all MDAL features"
```

---

### Task 22: Final integration test

**Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass (16+ existing + new filter/section/engine tests)

**Step 2: Build release**

Run: `trunk build --release --public-url /markdown-boxes/`
Expected: Clean build, no warnings

**Step 3: Manual verification**

Run `trunk serve` and verify:
- [ ] Gallery shows all templates (now loaded from `.mdal` files)
- [ ] Template cards display correctly
- [ ] Auth-flow template shows required asterisks and character counters
- [ ] Portfolio template has section badges, conditional projects section, and styled preview
- [ ] Rendered/Source toggle works
- [ ] Copy to clipboard exports clean markdown (no filter syntax)
- [ ] Download saves as `.mdal`

**Step 4: Commit if any fixups needed**

```bash
git add -A
git commit -m "fix: integration fixups for MDAL features"
```
