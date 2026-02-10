---
name: react-query-patterns
description: Best practices for data fetching with TanStack Query (React Query)
---

# React Query Patterns

## Query Keys

1.  **Factory Pattern**: Always use a query key factory to maintain consistency and type safety.
    ```typescript
    const todoKeys = {
      all: ['todos'] as const,
      lists: () => [...todoKeys.all, 'list'] as const,
      list: (filters: string) => [...todoKeys.lists(), { filters }] as const,
      details: () => [...todoKeys.all, 'detail'] as const,
      detail: (id: number) => [...todoKeys.details(), id] as const,
    }
    ```
2.  **Hierarchy**: Organize keys from general to specific to allow granular cache invalidation.

## Data Fetching

-   **Separation of Concerns**: Create custom hooks for all queries and mutations. Do not call `useQuery` directly in components.
    -   *Good*: `useTodo(id)`
    -   *Bad*: `useQuery(['todos', id], fetchTodo)` inside component.
-   **Error Handling**: Handle errors at the global level (QueryClient) or specifically in the hook, not in the UI component unless necessary.

## Mutating Data

-   **Optimistic Updates**: Implement optimistic UI for better user experience on mutations (like toggling a todo).
-   **Invalidation**: Always invalidate relevant query keys after a successful mutation to refetch fresh data.

## Performance

-   **Stale Time**: Configure `staleTime` appropriately. Data that rarely changes should not be refetched immediately on window focus.
-   **Prefetching**: Use `queryClient.prefetchQuery` on hover or route change for instant data availability.
