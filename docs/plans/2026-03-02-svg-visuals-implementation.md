# SVG Visuals Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add diagram-style SVG visuals to templates — hero SVGs on gallery cards and detail headers, plus inline SVGs in markdown content.

**Architecture:** Hybrid approach — separate hero SVG files served as static assets for gallery cards and detail view headers, plus inline `<svg>` blocks in markdown bodies rendered by comrak's HTML passthrough. Build pipeline copies hero SVGs to `dist/previews/`, template card component prepends BASE_PATH to image src.

**Tech Stack:** Rust/Leptos 0.8, Trunk (static asset hooks), comrak (unsafe HTML rendering), GitHub Actions (CI/CD)

---

### Task 1: Configure Trunk to copy hero SVGs to dist

**Files:**
- Modify: `Trunk.toml`

**Step 1: Add Trunk copy hook for preview assets**

Add a `[[hooks]]` section to `Trunk.toml` that copies hero SVGs into `dist/previews/` after build. Trunk supports `stage = "post_build"` hooks.

```toml
[build]
target = "index.html"
dist = "dist"

[serve]
address = "127.0.0.1"
port = 3000

[[hooks]]
stage = "post_build"
command = "sh"
command_arguments = ["-c", "mkdir -p dist/previews && find templates -name '*-hero.svg' -exec cp {} dist/previews/ \\;"]
```

**Step 2: Verify the hook works**

Run: `trunk build`
Expected: `dist/previews/` directory is created (empty for now since no SVGs exist yet)

**Step 3: Commit**

```bash
git add Trunk.toml
git commit -m "feat: add Trunk post-build hook to copy hero SVGs to dist/previews"
```

---

### Task 2: Fix BASE_PATH on card preview image src

**Files:**
- Modify: `src/components/template_card.rs:12`

The card component currently generates `/previews/{p}` without BASE_PATH. This breaks on GitHub Pages where the app is served at `/markdown-boxes/`.

**Step 1: Update the preview_src to include BASE_PATH**

In `src/components/template_card.rs`, change line 12 from:

```rust
let preview_src = template.preview.map(|p| format!("/previews/{p}"));
```

to:

```rust
let preview_src = template.preview.map(|p| format!("{}/previews/{p}", crate::BASE_PATH));
```

**Step 2: Verify it compiles**

Run: `cargo check --target wasm32-unknown-unknown`
Expected: compiles with no errors

**Step 3: Commit**

```bash
git add src/components/template_card.rs
git commit -m "fix: prepend BASE_PATH to card preview image src"
```

---

### Task 3: Add hero diagram to detail view preview pane

**Files:**
- Modify: `src/pages/template_view.rs:25-68`

Show the hero SVG image at the top of the preview pane, above the rendered markdown, when the template has a `preview` field.

**Step 1: Add hero image rendering**

In `src/pages/template_view.rs`, inside the `if let Some(tmpl) = template()` block, extract the preview field and render it. After line 28 (`let placeholders = tmpl.placeholders.clone();`), add:

```rust
let preview_src = tmpl.preview.map(|p| format!("{}/previews/{p}", crate::BASE_PATH));
```

Then inside the `template-preview-pane` div (between the `<h3>"Preview"</h3>` and `<MarkdownPreview .../>` on lines 66-67), add the hero image:

```rust
<div class="template-preview-pane">
    <h3>"Preview"</h3>
    {preview_src.map(|src| view! {
        <div class="hero-diagram">
            <img src=src alt="Template diagram" />
        </div>
    })}
    <MarkdownPreview content=preview_content />
</div>
```

**Step 2: Verify it compiles**

Run: `cargo check --target wasm32-unknown-unknown`
Expected: compiles with no errors

**Step 3: Commit**

```bash
git add src/pages/template_view.rs
git commit -m "feat: show hero diagram in template detail view"
```

---

### Task 4: Add CSS for hero diagram and inline SVGs

**Files:**
- Modify: `index.html:167-178` (after prose styles)

**Step 1: Add hero diagram styles**

After the `.prose strong` rule (line 178 in `index.html`), add:

```css
    .hero-diagram { margin-bottom: 16px; padding-bottom: 16px; border-bottom: 1px solid var(--border); }
    .hero-diagram img { width: 100%; height: auto; border-radius: 6px; }
    .prose svg { max-width: 100%; height: auto; }
```

**Step 2: Verify styles render correctly**

Run: `trunk serve`
Navigate to a template detail view. The hero-diagram section should exist in DOM (empty for now). The `.prose svg` rule handles future inline SVGs.

**Step 3: Commit**

```bash
git add index.html
git commit -m "feat: add CSS for hero diagrams and inline SVGs"
```

---

### Task 5: Update GitHub Actions to copy hero SVGs

**Files:**
- Modify: `.github/workflows/deploy.yml:42-43`

The Trunk post-build hook handles local builds, but we should verify it also runs in CI. Trunk hooks run automatically during `trunk build`, so no explicit CI changes should be needed. However, verify by checking the workflow output.

**Step 1: Verify trunk build runs the hook in CI**

The `[[hooks]]` section in `Trunk.toml` runs automatically when Trunk builds. No changes needed to `deploy.yml`. The `trunk build --release --public-url /markdown-boxes/` command on line 42 will execute the post_build hook, creating `dist/previews/`.

**Step 2: Commit** (no changes — this is a verification task)

If the hook runs correctly in CI, no commit needed. If not, add an explicit step to deploy.yml after the Build step:

```yaml
      - name: Copy preview assets
        run: mkdir -p dist/previews && find templates -name '*-hero.svg' -exec cp {} dist/previews/ \;
```

---

### Task 6: Create a sample hero SVG for one template

**Files:**
- Create: `templates/implementation-plans/auth-flow-hero.svg`
- Modify: `templates/implementation-plans/auth-flow.md` (frontmatter only)

**Step 1: Create a simple auth-flow diagram SVG**

Create `templates/implementation-plans/auth-flow-hero.svg` with a diagram-style SVG showing a simplified auth flow (User → Login → Auth Server → Token → App). Use the dark theme colors from the app (background `#1a1a1a`, text `#e8e8e8`, accent `#6366f1`).

**Step 2: Update the auth-flow template frontmatter**

Add `preview: auth-flow-hero.svg` to the YAML frontmatter in `templates/implementation-plans/auth-flow.md`.

**Step 3: Build and verify**

Run: `trunk build && ls dist/previews/`
Expected: `auth-flow-hero.svg` appears in `dist/previews/`

Run: `trunk serve`
Navigate to gallery. Auth-flow card should show the SVG diagram instead of the category placeholder. Click into it — hero diagram should appear above the markdown preview.

**Step 4: Commit**

```bash
git add templates/implementation-plans/auth-flow-hero.svg templates/implementation-plans/auth-flow.md
git commit -m "feat: add sample hero SVG for auth-flow template"
```

---

### Task 7: Add inline SVG to a template body

**Files:**
- Modify: `templates/implementation-plans/auth-flow.md` (body content)

**Step 1: Add an inline SVG diagram in the markdown body**

Add a small inline SVG diagram within the markdown body at an appropriate location (e.g. after the architecture section). The SVG should illustrate a specific concept from the template — e.g. the token refresh flow.

Example placement in the markdown:

```markdown
## Token Refresh Flow

<svg viewBox="0 0 400 100" xmlns="http://www.w3.org/2000/svg">
  <!-- simple diagram showing token refresh -->
</svg>
```

**Step 2: Build and verify**

Run: `trunk serve`
Navigate to the auth-flow template. The inline SVG should render within the markdown preview, styled responsively by the `.prose svg` CSS rule.

**Step 3: Commit**

```bash
git add templates/implementation-plans/auth-flow.md
git commit -m "feat: add inline SVG diagram to auth-flow template body"
```

---

### Task 8: Verify full end-to-end flow

**Step 1: Run full build**

Run: `trunk build --release --public-url /markdown-boxes/`
Expected: clean build, `dist/previews/auth-flow-hero.svg` exists

**Step 2: Test gallery card rendering**

Verify auth-flow card shows hero SVG, other cards show category placeholder.

**Step 3: Test detail view**

Verify hero diagram appears above markdown preview. Verify inline SVG renders within markdown content. Verify SVGs are responsive (resize browser window).

**Step 4: Test export**

Copy markdown to clipboard. Verify inline SVG is included. Verify hero SVG is NOT included (it's metadata).

**Step 5: Push and verify CI**

```bash
git push
```

Check GitHub Actions workflow runs successfully and deploys to Pages with the preview assets.
