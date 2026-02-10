---
name: tailwind-expert
description: Best practices for styling with Tailwind CSS
---

# Tailwind CSS Expert Guidelines

## Core Principles

1.  **Utility-First**: Always prefer standard utility classes over custom CSS or `@apply`.
    -   *Good*: `class="px-4 py-2 bg-blue-500 rounded"`
    -   *Bad*: `.btn { @apply px-4 py-2 ... }` (unless widely reused)
2.  **Mobile-First**: Design for mobile first, then add breakpoints for larger screens.
    -   *Example*: `w-full md:w-1/2 lg:w-1/3` (starts full width, becomes smaller on desktop)
3.  **Constraint-Based**: Stick to the design tokens defined in `tailwind.config.js`. Do not use arbitrary values like `w-[123px]` unless absolutely necessary.

## Component Structure

-   Keep className strings clean. Extract complex logic into `clsx` or `classnames` utilities.
-   Order classes logically: Layout -> Spacing -> Sizing -> Typography -> Visuals -> Interaction.

## Theming

-   Extend the theme in `tailwind.config.js` rather than overwriting it.
-   Use CSS variables for dynamic themes (Dark Mode).

## Performance

-   Ensure PurgeCSS (built-in) is correctly configured to scan all source files.
-   Avoid dynamic class names that cannot be statically analyzed (e.g., `bg-${color}-500`). Instead, map props to complete class strings.
