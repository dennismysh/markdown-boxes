---
title: "Authentication Flow"
category: implementation-plan
tags: [auth, jwt, backend, security]
preview: auth-flow-hero.svg
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

<svg viewBox="0 0 480 80" xmlns="http://www.w3.org/2000/svg" style="background:#242424;border-radius:6px;padding:8px">
  <defs>
    <marker id="a" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto">
      <path d="M0,0 L8,4 L0,8 Z" fill="#6366f1"/>
    </marker>
  </defs>
  <rect x="10" y="20" width="80" height="40" rx="6" fill="#1a1a1a" stroke="#2a2a2a" stroke-width="1"/>
  <text x="50" y="44" text-anchor="middle" fill="#e8e8e8" font-family="sans-serif" font-size="10">Client</text>
  <rect x="130" y="20" width="100" height="40" rx="6" fill="#1a1a1a" stroke="#6366f1" stroke-width="1"/>
  <text x="180" y="44" text-anchor="middle" fill="#e8e8e8" font-family="sans-serif" font-size="10">/refresh</text>
  <rect x="270" y="20" width="90" height="40" rx="6" fill="#1a1a1a" stroke="#2a2a2a" stroke-width="1"/>
  <text x="315" y="44" text-anchor="middle" fill="#e8e8e8" font-family="sans-serif" font-size="10">Validate</text>
  <rect x="400" y="20" width="70" height="40" rx="6" fill="#1a1a1a" stroke="#6366f1" stroke-width="1"/>
  <text x="435" y="38" text-anchor="middle" fill="#e8e8e8" font-family="sans-serif" font-size="10">New</text>
  <text x="435" y="50" text-anchor="middle" fill="#888888" font-family="sans-serif" font-size="9">Tokens</text>
  <line x1="90" y1="40" x2="125" y2="40" stroke="#6366f1" stroke-width="1.5" marker-end="url(#a)"/>
  <line x1="230" y1="40" x2="265" y2="40" stroke="#6366f1" stroke-width="1.5" marker-end="url(#a)"/>
  <line x1="360" y1="40" x2="395" y2="40" stroke="#6366f1" stroke-width="1.5" marker-end="url(#a)"/>
</svg>

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
