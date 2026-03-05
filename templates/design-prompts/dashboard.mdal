---
title: "Analytics Dashboard"
category: design-prompt
tags: [design, dashboard, charts, data-visualization, ui]
description: "AI prompt for designing an analytics dashboard with charts, KPIs, and data filters."
placeholders:
  - key: product_name
    label: "Product Name"
    type: text
  - key: metrics
    label: "Key Metrics (comma-separated, e.g. revenue, users, conversion rate)"
    type: text
  - key: chart_types
    label: "Primary Chart Type"
    type: select
    options: [Line charts, Bar charts, Mix of chart types]
  - key: time_range
    label: "Default Time Range"
    type: select
    options: [Last 7 days, Last 30 days, Last 90 days, Custom range]
  - key: theme
    label: "Visual Theme"
    type: select
    options: [Dark professional, Light minimal, Colorful modern]
---

# Design Prompt: {{product_name}} Dashboard

## Context

Design an analytics dashboard for **{{product_name}}** that displays these key metrics: {{metrics}}.

## Layout

### Top Bar
- Product logo + name on the left
- Time range selector: {{time_range}} (with custom date picker option)
- Refresh button with last-updated timestamp

### KPI Cards Row
- One card per key metric
- Each shows: current value, trend (up/down arrow + percentage), sparkline
- Cards should be scannable at a glance

### Charts Section
- Primary visualization: {{chart_types}}
- Each chart has a title, legend, and tooltip on hover
- Charts should be responsive and stack vertically on mobile

### Data Table
- Below charts, a sortable table with the raw data
- Columns match the key metrics
- Search/filter capability
- Export to CSV button

## Visual Style

Theme: {{theme}}

### Design Principles
- Data-ink ratio: maximize information, minimize decoration
- Consistent color coding across all charts and KPIs
- Clear visual hierarchy: KPIs > charts > table
- Use whitespace to separate sections, not heavy borders

## Technical Notes
- Charts should animate on load (subtle, not distracting)
- Skeleton loading states while data fetches
- Responsive: works from 375px to 2560px
- Accessible: all chart data available in table form
