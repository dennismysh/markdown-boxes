---
title: "CI/CD Pipeline Setup"
category: implementation-plan
tags: [ci, cd, devops, testing, deployment, github-actions]
description: "Step-by-step plan for setting up a CI/CD pipeline with testing, linting, building, and automated deployment."
placeholders:
  - key: project_name
    label: "Project Name"
    type: text
  - key: language
    label: "Language / Runtime"
    type: select
    options: [TypeScript / Node.js, Python, Rust, Go, Java / Kotlin]
  - key: ci_platform
    label: "CI/CD Platform"
    type: select
    options: [GitHub Actions, GitLab CI, CircleCI]
  - key: deploy_target
    label: "Deployment Target"
    type: select
    options: [AWS (ECS/Lambda), Vercel, Netlify, Docker + VPS, Kubernetes]
  - key: test_strategy
    label: "Testing Strategy"
    type: select
    options: [Unit + Integration, Unit + Integration + E2E, Unit only]
---

# {{project_name}} — CI/CD Pipeline

## Overview

Set up automated CI/CD for {{project_name}} ({{language}}) using {{ci_platform}}, deploying to {{deploy_target}}.

## Pipeline Stages

### 1. Lint & Format Check
- Run linter (eslint / ruff / clippy / golangci-lint)
- Check code formatting (prettier / black / rustfmt / gofmt)
- Fail fast — don't run tests if lint fails

### 2. Test
- Strategy: {{test_strategy}}
- Run tests in parallel where possible
- Generate coverage report
- Fail if coverage drops below threshold (80%)

### 3. Build
- Compile / bundle the application
- Verify build output is valid
- Cache dependencies between runs

### 4. Deploy
- Target: {{deploy_target}}
- Deploy to staging on push to `develop`
- Deploy to production on push to `main`
- Require manual approval for production (optional)

## Branch Strategy

| Branch | Trigger | Deploy To |
|--------|---------|-----------|
| Feature branches | PR opened/updated | — (CI only) |
| develop | Push / merge | Staging |
| main | Push / merge | Production |

## Environment Variables

- Store secrets in CI platform's secret store
- Never commit .env files
- Separate configs for staging vs production

## Notifications

- Slack/Discord notification on deploy success/failure
- PR status checks block merge on failure
- Weekly summary of deployment frequency + failure rate

## Rollback Plan

- Keep last 3 successful deployments available
- One-click rollback to previous version
- Database migrations must be backwards-compatible
