---
name: flutter-clean-arch
description: Clean Architecture layers for Flutter applications
---

# Flutter Clean Architecture

## Layers
Strictly separate concerns into three layers:

1.  **Presentation** (Widgets, BLoC, State)
2.  **Domain** (Entities, UseCases, Repository Interfaces)
    -   *Rule*: Pure Dart. No Flutter dependencies. No Data dependencies.
3.  **Data** (Models, Repository Implementations, Data Sources)

## Dependency Flow
`Presentation` -> `Domain` <- `Data`

The Domain layer knows nothing about the outer layers. Data layer implements interfaces defined in Domain.

## Data Sources
Split data access into two:
-   **RemoteDataSource**: API calls (Dio/Retrofit)
-   **LocalDataSource**: Cache/Storage (Hive/SharedPreferences)

## Dependency Injection
Use `get_it` and `injectable` to manage dependencies.
