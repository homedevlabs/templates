---
name: accessibility-a11y
description: Guidelines for building accessible web applications (WCAG 2.1 AA+)
---

# Accessibility (A11y) Expert Guidelines

## Semantic HTML

1.  **Use Native Elements**: Always prefer native HTML elements (`<button>`, `<a>`, `<input>`) over `<div>` or `<span>` with click handlers. Native elements give you keyboard support and focus management for free.
2.  **Headings Hierarchy**: Ensure `<h1>` through `<h6>` are used in logical order. Never skip levels for visual reasons (use CSS classes instead).

## ARIA & Labelling

-   **Labels**: All form inputs must have a visible `<label>` or `aria-label`.
-   **ARIA Usage**: Use ARIA attributes (`aria-expanded`, `aria-hidden`) ONLY when native HTML cannot express the state or semantic required.
-   **Images**: All `<img>` tags must have an `alt` attribute. Use `alt=""` for purely decorative images.

## Interaction

1.  **Keyboard Navigation**: Every interactive element must be reachable and usable via Tab key.
2.  **Focus States**: Never remove outline (`outline: none`) without providing a clearly visible alternative focus state.
3.  **Skip Links**: Include a "Skip to Main Content" link at the top of the page.

## Visuals

-   **Color Contrast**: Ensure text has a contrast ratio of at least 4.5:1 against the background (WCAG AA).
-   **Motion**: Respect `prefers-reduced-motion` media query for animations.
