# GitHub Pages Deployment — Design Document

## Goal

Auto-deploy Markdown Boxes to `dennismysh.github.io/markdown-boxes` on every push to main.

## Approach

GitHub Actions workflow builds the Leptos CSR app with trunk and deploys the static `dist/` output to GitHub Pages.

## GitHub Actions Workflow

A single file `.github/workflows/deploy.yml`:

1. Trigger on push to `main`
2. Install Rust stable + `wasm32-unknown-unknown` target
3. Install trunk
4. Run `trunk build --release --public-url /markdown-boxes/`
5. Copy `dist/index.html` to `dist/404.html` (SPA routing fix)
6. Deploy `dist/` to GitHub Pages via `actions/deploy-pages`

The `--public-url /markdown-boxes/` flag ensures all asset URLs include the repo subpath prefix, since GitHub Pages serves project sites at `username.github.io/repo-name/`.

## SPA Routing Fix

GitHub Pages returns a real 404 for paths like `/markdown-boxes/template/auth-flow`. Since Leptos handles routing client-side, we copy `index.html` to `404.html` after the build. GitHub Pages serves `404.html` for any unknown path, which bootstraps the WASM app, which then handles the route.

## Repo Settings

Enable GitHub Pages with source set to "GitHub Actions" (not branch-based).

## What Changes

- Create: `.github/workflows/deploy.yml`
- No changes to existing source code
