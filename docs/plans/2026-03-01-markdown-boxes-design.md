# Markdown Boxes — Design Document

## Product Overview

Markdown Boxes is a web-based gallery of shareable markdown templates for AI-driven workflows. Templates cover implementation plans, design prompts, UI component specs, and full-stack patterns.

Users browse the gallery, pick a template, customize it through a form interface, and export the completed markdown (copy to clipboard or download as `.md`) to use with AI tools like Claude, ChatGPT, Cursor, etc.

The core problem: people struggle with how to word prompts and plans to produce specific results. Markdown Boxes provides proven, shareable starting points with an easy customization experience.

## Core User Flow

1. Browse gallery (visual previews, categories, search)
2. Select a template
3. See the template with editable sections highlighted — explicit `{{placeholders}}` plus optional AI-suggested additional editable areas
4. Fill in the form fields
5. Preview the completed markdown in real-time
6. Copy to clipboard or download as `.md`

## Tech Stack

- **Frontend:** Leptos CSR (Rust/WASM)
- **Hosting:** Static (Netlify, GitHub Pages, or similar)
- **Templates:** Flat `.md` files in the git repo
- **Markdown rendering:** comrak compiled to WASM
- **Search:** Client-side fuzzy search over a build-generated index
- **AI enhancement:** External API call (user provides own API key) for suggesting additional editable sections

## Template Format

Each template is a markdown file with YAML frontmatter:

```markdown
---
title: "Auth Flow Implementation Plan"
category: full-stack-flow
tags: [auth, jwt, backend, security]
preview: auth-flow-preview.png
description: "Step-by-step plan for building JWT-based authentication"
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
  - key: auth_provider
    label: "Auth Provider"
    type: select
    options: [Firebase, Supabase, Custom JWT, OAuth2]
  - key: framework
    label: "Backend Framework"
    type: select
    options: [Express, FastAPI, Axum, Django]
---

# {{project_name}} — Authentication Flow

## Overview
Build a JWT-based auth system using {{auth_provider}} with {{framework}}.
```

### Placeholder Types

- `text` — single-line text input
- `select` — dropdown with predefined options
- `multiline` — multi-line text area
- `boolean` — toggle/checkbox

### Template Categories

- `implementation-plan`
- `design-prompt`
- `ui-component`
- `full-stack-flow`
- `backend-pattern`

### Content Source

Curated seed collection created by the project maintainer, with community contributions accepted via pull requests over time.

## Architecture

```
markdown-boxes/
├── src/
│   ├── app.rs                  # Root app + router
│   ├── pages/
│   │   ├── gallery.rs          # Gallery grid with filters
│   │   ├── template_view.rs    # Single template editor/form
│   │   └── about.rs
│   ├── components/
│   │   ├── template_card.rs    # Gallery card with preview image
│   │   ├── form_field.rs       # Dynamic form field (text/select/multiline)
│   │   ├── markdown_preview.rs # Live rendered markdown preview
│   │   ├── search_bar.rs
│   │   └── category_filter.rs
│   └── models/
│       ├── template.rs         # Template struct (parsed from frontmatter)
│       └── placeholder.rs      # Placeholder types and values
├── templates/                  # Markdown template files
│   ├── implementation-plans/
│   ├── design-prompts/
│   ├── ui-components/
│   └── full-stack-flows/
├── build.rs                    # Generates templates-index.json at build time
├── static/
│   ├── previews/               # Template preview images
│   └── templates-index.json    # Build-generated index
└── Cargo.toml
```

### Data Flow

1. **Build time:** `build.rs` scans `templates/`, parses frontmatter, generates `templates-index.json` (title, category, tags, description, preview path — no body content)
2. **Gallery load:** WASM app fetches `templates-index.json`, renders cards with preview images
3. **Template open:** App fetches the individual `.md` file, parses frontmatter + body, renders form fields from `placeholders`
4. **Live editing:** As user fills form fields, `{{placeholder}}` values are substituted in real-time in the preview pane
5. **Export:** User copies completed markdown or downloads as `.md`

### AI Suggestion Feature

Optional enhancement — user clicks "Suggest editable areas" on any template:

- Requires user's own API key (stored in localStorage)
- Calls an external AI API with the template content
- AI identifies additional paragraphs/sections that could be customized
- These appear as highlighted, optional form fields alongside explicit placeholders
- Fully functional without AI — placeholders alone provide the core experience

## Error Handling

**Template parsing:**
- Malformed frontmatter → skip template from index, log warning at build time
- Missing required fields (title, category) → build error
- Missing preview image → show default placeholder in gallery

**Form editing:**
- Empty required placeholders → inline validation, disable export until filled
- `{{placeholder}}` in body with no frontmatter definition → treat as literal text

**AI suggestion:**
- No API key configured → hide "Suggest editable areas" button
- API call fails → dismissible error toast, form still works without AI
- Nonsensical AI suggestions → user can dismiss individual suggestions

**Static hosting:**
- Template fetch 404 → "Template not found" with link back to gallery
- Index fetch fails → error state with retry button

## Testing Strategy

- **Unit tests:** Template parsing (frontmatter extraction, placeholder substitution), search/filter logic
- **Build tests:** Verify `build.rs` generates valid index JSON, catches malformed templates
- **No E2E tests initially** — prioritize unit tests on core logic
