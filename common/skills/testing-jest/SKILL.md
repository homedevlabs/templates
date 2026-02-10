---
name: testing-jest
description: Best practices for unit testing with Jest
---

# Jest Testing Guidelines

## Structure

1.  **Co-location**: Keep test files next to the source file (`MyComponent.test.tsx` next to `MyComponent.tsx`) or in a `__tests__` directory.
2.  **Description**: Use descriptive names for `describe` blocks (component/function name) and `it` blocks (behavior).
    -   *Good*: `it('should render the user name when loaded')`
    -   *Bad*: `it('works')`

## Isolation

-   **Mocking**: Mock external dependencies (API calls, databases) to ensure tests are fast and deterministic.
-   **Clean State**: Use `beforeEach` or `afterEach` to reset mocks and clean up DOM to avoid side effects between tests.

## Coverage

-   **Thresholds**: Maintain a reasonable coverage threshold (e.g., 80% lines/functions).
-   **Critical Paths**: Prioritize testing business logic and critical UI paths over 100% trivial coverage.

## Assertions

-   **Specificity**: Use specific matchers/assertions.
    -   *Good*: `expect(result).toBe(5)`
    -   *Bad*: `expect(result).toBeTruthy()`
