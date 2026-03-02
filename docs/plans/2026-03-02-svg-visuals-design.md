# SVG Visuals for Templates

## Problem

Templates in the gallery lack visual context. Users browse text-only cards and can't quickly grasp what a template produces without reading through it.

## Decision: Hybrid Approach

Two types of SVG diagrams, stored and served differently:

1. **Hero SVGs** (separate files) — high-level diagram showing what the template produces. Displayed on gallery cards and at the top of the detail view's preview pane.
2. **Inline SVGs** (in markdown body) — contextual diagrams embedded where they're relevant within the template content. Rendered by comrak's HTML passthrough.

### Why hybrid over fully self-contained

At scale (100k+ templates), self-contained files bloat `templates.json` and the WASM binary. Hero SVGs served as separate static assets are independently cacheable, resizable, and map cleanly to a future backend where gallery metadata and full content are served by different API endpoints.

## File Structure

```
templates/
  design/
    auth-flow.md
    auth-flow-hero.svg
    dashboard.md
    dashboard-hero.svg
  backend/
    api-spec.md              # no hero — shows category placeholder
```

Convention: `{slug}-hero.svg` alongside the `.md` file. Referenced in frontmatter via existing `preview` field.

## Build Pipeline

- `build.rs` reads `preview` from frontmatter as it does today (no change)
- Trunk copies hero SVGs to `dist/previews/` via a `[[hooks]]` section in `Trunk.toml`
- Inline SVGs in markdown need no build changes — comrak with unsafe HTML passes `<svg>` blocks through

## UI Rendering

### Gallery cards
Existing card component already handles the `preview` field — renders `<img src="{BASE_PATH}/previews/{filename}">` when present, category placeholder when absent. No component changes needed.

### Detail view — hero
Show hero SVG at the top of the preview pane (above rendered markdown) via `<img>` tag. Only rendered when `preview` is `Some`.

### Detail view — inline
Rendered automatically by comrak. Add CSS for `.prose svg { max-width: 100%; height: auto; }` to ensure responsive sizing.

### Export
Inline SVGs are included in the exported markdown. Hero diagram is not — it's metadata, not template content.

## SVG Style

Diagram-style illustrations (flowcharts, architecture diagrams, wireframes) that visually represent what the template produces. Hand-crafted per template.
