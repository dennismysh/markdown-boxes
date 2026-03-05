# MDAL Integration Design

**Date:** 2026-03-04
**Status:** Approved
**Goal:** Evolve markdown-boxes toward the MDAL spec, with the long-term vision of becoming the reference MDAL executor.

---

## Strategy

Incrementally adopt MDAL features into the existing Leptos/WASM template system. Templates switch to `.mdal` file extension and richer frontmatter immediately. New language features are added in phases: variable filters, typed sections, conditionals/loops, styling spec. The existing gallery, form generation, and live preview architecture remain — they get upgraded, not replaced.

**Build order:** Filters → Sections → Conditionals → Styling

---

## 1. File Format & Frontmatter

Templates switch from `.md` to `.mdal`. Frontmatter adopts the MDAL schema.

### New frontmatter fields

```yaml
---
type: application | workflow | content | system
name: "Template Name"
version: 1.0.0
author: "@system/templates"
category: implementation-plan | design-prompt | ui-component | full-stack-flow | backend-pattern
tags: [tag1, tag2]
preview: hero.svg
description: "Brief description"
outputs:
  - format: html
    target: web
dependencies: []
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
    filters:
      - required: true
      - max_length: 100
      - default: "My Project"
---
```

### Changes from current format

- `type`, `version`, `author`, `outputs`, `dependencies` added from MDAL spec
- `placeholders[].options` replaced by `placeholders[].filters` array
- `build.rs` scans for `*.mdal` instead of `*.md`
- `category` stays (markdown-boxes specific, maps to MDAL's type subtypes)

---

## 2. Variable Filters & Constraints

Extend the `{{placeholder}}` syntax with inline filters.

### Syntax

```markdown
{{project_name | required}}
{{project_name | default: "My Project"}}
{{project_name | max_length: 100}}
{{tech_stack | options: [React, Vue, Svelte]}}
{{project_name | required | max_length: 100}}
```

### Implementation

- New `Filter` enum in `models/placeholder.rs`: `Required`, `Default(String)`, `MaxLength(usize)`, `Options(Vec<String>)`
- `build.rs` parses filters from both frontmatter `placeholders` block AND inline `{{var | filter}}` syntax in the body
- Inline filters merge with frontmatter definitions (frontmatter takes precedence on conflicts)
- Filters map to form validation:
  - `required` → field marked required
  - `max_length` → character counter
  - `options` → dropdown
  - `default` → pre-filled value
- Substitution engine strips filter syntax before output

### Migration

Existing `type: select` + `options: [...]` continues to work. Filter syntax is additive.

---

## 3. Typed Sections

Template bodies gain semantic structure with `## Section:` headings.

### Syntax

```markdown
## Section: Hero
- type: component
- layout: centered
- required: true

### Content
- headline: {{user.headline | required}}
- cta_text: {{cta | default: "Learn More"}}
```

### Implementation

- New `Section` struct in `models/`: `name`, `section_type` (component/layout/logic/data), `properties` (key-value pairs), `content` (raw markdown body)
- `build.rs` parses `## Section:` headings and extracts dash-prefixed metadata lines
- Regular `##` headings (without `Section:`) remain plain markdown — backward compatible
- Sections appear in template detail view as collapsible groups with type badges
- Form groups placeholder fields by section
- Stored in `templates.json` as `sections: Vec<Section>` on each template

### Scope limits

Sections are structural metadata only. No layout rendering (we don't interpret `layout: centered` visually yet). Rendering comes later when the executor matures.

---

## 4. Conditionals & Loops

Template bodies support `{{#if}}` and `{{#each}}` blocks.

### Syntax

```markdown
{{#if user.projects.length > 0}}
## Projects
{{#each user.projects as project}}
- **{{project.title}}**: {{project.description}}
{{/each}}
{{else}}
_No projects yet._
{{/if}}
```

### Implementation

- New `src/engine.rs` module for MDAL expression evaluation (separate from `substitute.rs`)
- `{{#if condition}}...{{else}}...{{/if}}` — truthiness: non-empty strings true, empty/missing false, supports `> 0`, `== "value"`, bare variable checks
- `{{#each collection as item}}...{{/each}}` — iterates arrays, exposes `{{item.property}}`
- Blocks can nest
- Engine processes blocks first (expand/collapse), then runs variable substitution
- Form fields for array-type placeholders get "add item" button

### Scope limits

- No complex expressions (no `&&`, `||`, arithmetic)
- No `{{#unless}}` — use `{{#if}}` with `{{else}}`
- Array items are flat objects (no nested arrays)
- Conditions limited to: truthy check, `== value`, `> number`, `.length > number`

### Preview

Conditionals evaluate live as user fills in fields — sections appear/disappear in real time.

---

## 5. Styling Specification

Templates declare their own design system via `## Styling:` sections.

### Syntax

```markdown
## Styling: DesignSystem
- type: style
- approach: brutalist

### Colors
- primary: {{accent_color | default: "#000000"}}
- background: #ffffff
- text: #111111

### Typography
- headings: JetBrains Mono
- body: Inter
- base_size: 16px

### Effects
- borders: 2px solid
- radius: 0px
```

### Implementation

- New `StyleSpec` struct in `models/`: `approach` (enum: brutalist/minimalist/playful/professional), `colors` (HashMap), `typography` (HashMap), `effects` (HashMap)
- `build.rs` parses `## Styling:` sections into `templates.json`
- Rendered preview applies styling as CSS custom properties: `--mdal-primary`, `--mdal-bg`, `--mdal-font-heading`, etc.
- Variables in styling section are reactive — changing color in form updates preview instantly

### Scope limits

Does not generate standalone CSS or themed outputs. Styles the preview pane only. Full output generation is a future executor feature.

---

## 6. Dual Preview Toggle

Template detail view gets a toggle between two modes:

- **Rendered** (default): Substituted markdown rendered as HTML with styling spec applied
- **Source**: Raw `.mdal` content with substituted values — filled placeholders in green, unfilled in yellow, filter syntax dimmed

Toggle button in preview header: `[Rendered] [Source]`

---

## 7. Migration & Template Updates

### File changes

- `templates/**/*.md` → `templates/**/*.mdal`
- `build.rs` scans `*.mdal` instead of `*.md`
- All frontmatter extended with `type`, `version`, `author`, `outputs`
- At least 2 templates upgraded to showcase new features (sections, conditionals, filters)
- Remaining templates get minimal frontmatter additions and work as-is

### Example migration

```yaml
# Before (auth-flow.md)
---
title: "Authentication Flow"
category: implementation-plan
tags: [auth, jwt, security]
placeholders:
  - key: auth_method
    type: select
    options: [JWT, Session, OAuth]
---

# After (auth-flow.mdal)
---
type: application
name: "Authentication Flow"
version: 1.0.0
author: "@system/templates"
category: implementation-plan
tags: [auth, jwt, security]
outputs:
  - format: html
    target: web
placeholders:
  - key: auth_method
    type: select
    filters:
      - required: true
      - options: [JWT, Session, OAuth]
---
```

---

## 8. Error Handling

- **Invalid filter syntax** → warning in console, placeholder renders without filter
- **Missing required field** → form field highlighted red, export blocked with message
- **Malformed `{{#if}}`/`{{#each}}`** → block rendered as literal text with warning badge
- **Unknown section type** → treated as plain markdown, no crash

---

## Architecture Summary

### New files

- `src/engine.rs` — MDAL expression evaluator (conditionals, loops)
- `src/models/section.rs` — Section struct
- `src/models/style.rs` — StyleSpec struct
- `src/models/filter.rs` — Filter enum and parsing

### Modified files

- `build.rs` — scan `.mdal`, parse sections/styling/filters
- `src/models/placeholder.rs` — add `filters: Vec<Filter>`
- `src/models/template.rs` — add MDAL frontmatter fields, sections, style_spec
- `src/substitute.rs` — strip filter syntax during substitution
- `src/pages/template_view.rs` — section grouping, dual preview toggle, validation UI
- `src/components/form_field.rs` — validation from filters (required, max_length, char counter)
- `src/components/markdown_preview.rs` — apply style spec as CSS vars
- `index.html` — CSS for validation states, source view highlighting, toggle button

### No new dependencies

All features implemented with existing crate set (comrak, serde, leptos).
