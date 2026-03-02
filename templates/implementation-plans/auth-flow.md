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
