---
name: techdocs-structure
description: Standard structure for Backstage TechDocs integration
---

# TechDocs Standard Structure

## Philosophy
Documentation should live with the code ("Docs-like-code") and be built automatically.

## Directory Layout
Every repository must have a `docs/` folder and a `mkdocs.yml` at the root.

```
/
  mkdocs.yml
  docs/
    index.md         # Landing page (Overview)
    architecture/    # Diagrams (C4), ADRs
    deployment/      # How to build/deploy/run
    api/             # API Reference (OpenAPI)
    runbooks/        # Troubleshooting guides
```

## mkdocs.yml Template
Ensure at minimum:

```yaml
site_name: ${{ parameters.name }}
nav:
  - Home: index.md
  - Architecture: architecture/
  - Deployment: deployment/
  - API: api/
plugins:
  - techdocs-core
```

## Content Guidelines
-   **Index**: Brief overview, owner contact, link to slack channel.
-   **Diagrams**: Use MermaidJS for live diagrams within markdown.
