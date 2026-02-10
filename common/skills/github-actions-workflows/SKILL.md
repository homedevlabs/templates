---
name: github-actions-workflows
description: Guidelines for CI/CD pipelines with GitHub Actions
---

# GitHub Actions Workflows

## Pipeline Structure

1.  **Triggers**: Run workflows on `push` to main/develop branches and `pull_request` to ensure code quality before merge.
2.  **Concurrency**: Use `concurrency` groups to cancel outdated runs on the same PR to save resources.

## Jobs

-   **Lint & Format**: Fail fast if code style is incorrect.
-   **Test**: Run unit tests.
-   **Build**: Build Docker image or artifacts.
-   **Deploy**: Deploy only if all previous steps pass.

## Optimization

-   **Caching**: Cache dependencies (`node_modules`, `~/.cargo`) to speed up workflow execution significantly.
-   **Matrix Strategy**: Use matrix builds to test across multiple versions if building a library.

## Security

-   **Secrets**: Never hardcode credentials. Use `${{ secrets.MY_SECRET }}` from repository settings.
-   **Permissions**: Define least-privilege permissions at the workflow or job level (`contents: read`, `id-token: write`).
