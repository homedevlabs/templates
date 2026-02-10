---
name: flutter-bloc-pattern
description: State management implementation using BLoC/Cubit
---

# Flutter BLoC Pattern

## Core Principle
Separate the presentation layer from business logic using streams (Events in, States out).

## Structure
Each feature should have its own BLoC/Cubit:

```
feature/
  bloc/
    feature_bloc.dart
    feature_event.dart
    feature_state.dart
  view/
    feature_page.dart
```

## Guidelines

1.  **Cubit over BLoC**: Prefer `Cubit` for simple state simple features (less boilerplate). Use `BLoC` only for complex event-driven logic (debounce, transform).
2.  **Immutability**: All States and Events must be immutable (use `Equatable` or `Freezed`).
3.  **No Logic in UI**: The View should only subscribe to state changes (`BlocBuilder`) and dispatch events.

## Freezed
Use `freezed` package to generate union types and copyWith methods for states.
