---
title: "Data Table Component"
category: ui-component
tags: [table, data, component, ui]
description: "Prompt for building a sortable, filterable data table component with pagination."
placeholders:
  - key: framework
    label: "UI Framework"
    type: select
    options: [React, Vue, Svelte, Solid]
  - key: styling
    label: "Styling Approach"
    type: select
    options: [Tailwind CSS, CSS Modules, Styled Components, Plain CSS]
  - key: columns
    label: "Column Definitions (comma-separated)"
    type: text
  - key: has_pagination
    label: "Include Pagination"
    type: boolean
---

# Data Table Component

## Prompt

Build a reusable data table component in {{framework}} with {{styling}}.

### Columns

The table should display these columns: {{columns}}

### Features

- **Sorting:** Click column headers to sort ascending/descending
- **Filtering:** Text input above the table for global search across all columns
- **Selection:** Checkbox column for row selection with select-all header
- **Pagination:** {{has_pagination}}
- **Empty state:** Show a centered message when no data matches filters

### API

```
Props:
  data: array of row objects
  columns: array of { key, label, sortable? }
  onSelectionChange: callback with selected row IDs
  pageSize: number (default 20)
```

### Responsive Behavior

- Desktop: full table with all columns
- Tablet: horizontally scrollable
- Mobile: card layout, one card per row
