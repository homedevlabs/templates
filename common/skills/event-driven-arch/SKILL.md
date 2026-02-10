---
name: event-driven-arch
description: Patterns for asynchronous communication between services
---

# Event-Driven Architecture

## Core Concepts

1.  **Events**: Notifications of facts that happened (e.g., `OrderPlaced`, `UserRegistered`). Immutable.
2.  **Commands**: Instructions to do something (e.g., `ProcessPayment`). Can fail.

## Loose Coupling
Producers of events should not know who consumes them.
-   *Good*: `AuthService` emits `UserRegistered`. EmailService listens to `UserRegistered`.
-   *Bad*: `AuthService` calls `EmailService.sendWelcomeEmail()`.

## Idempotency
Consumers must handle duplicate events gracefully.
-   Store processed event IDs in a unique index.
-   Design operations to be repeatable (e.g., `set status = APPROVED` is safe to run twice).

## Dead Letter Queues (DLQ)
Always configure a DLQ for events that fail processing after N retries to prevent blocking the stream.
