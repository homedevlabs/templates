---
name: logging-observability
description: Guidelines for structured logging, metrics, and tracing
---

# Logging & Observability Standards

## Structured Logging

1.  **JSON Format**: All logs in production must be output as JSON lines to allow easy parsing by log aggregators (Elasticsearch, Loki).
2.  **Context**: Always include context objects, not just messages.
    -   *Bad*: `logger.info("User logged in " + userId)`
    -   *Good*: `logger.info("User logged in", { userId, ipAddress })`
3.  **Correlation IDs**: Ensure every request has a unique `X-Request-ID` or `traceId` that is propagated to downstream services and included in every log statement.

## Log Levels

-   **ERROR**: System is in distress, human intervention required.
-   **WARN**: Something unexpected happened, but system can continue (e.g., rate limit hit).
-   **INFO**: Standard operational events (startup, shutdown, key business events).
-   **DEBUG**: Detailed information for troubleshooting (disabled in production by default).

## Metrics & Tracing

-   **Prometheus**: Expose standard metrics (request count, latency, error rate) on `/metrics`.
-   **OpenTelemetry**: Instrument critical paths with spans. Use standard attributes for HTTP and DB operations.
