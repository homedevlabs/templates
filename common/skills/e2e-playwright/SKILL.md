---
name: e2e-playwright
description: Best practices for End-to-End testing with Playwright
---

# Playwright E2E Testing

## Selectors

1.  **Resilience**: Prefer user-facing selectors like `getByRole`, `getByText`, `getByLabel`.
2.  **Data Attributes**: When semantic selectors are hard to use, use `data-testid="my-element"` and `getByTestId()`. Avoid relying on CSS classes or XPath which are brittle.

## Test Isolation

-   **Independent Tests**: Each test must be able to run independently. Do not rely on state created by a previous test.
-   **Fixtures**: Use fixtures to set up the environment (login, seed data) before tests run.

## Performance & Stability

-   **Auto-waiting**: Rely on Playwright's auto-waiting mechanism rather than manual `page.waitForTimeout()`.
-   **Parallelism**: Configure tests to run in parallel to reduce CI execution time.

## Artifacts

-   **Screenshots/Video**: Configure reports to capture screenshots and video on failure for easier debugging.
