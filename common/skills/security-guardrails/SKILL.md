---
name: security-guardrails
description: Essential security practices for backend services
---

# Security Guardrails

## Input Validation

1.  **Never Trust Input**: Validate all incoming data (body, query params, headers) against a strict schema (Zod, Pydantic, Joi).
2.  **Sanitization**: Sanitize inputs to prevent XSS and Injection attacks, even if using an ORM.

## Authentication & Authorization

-   **Secrets**: Never commit secrets to code. Use environment variables.
-   **Least Privilege**: Database users and API tokens should have the minimum permissions necessary.
-   **Token Handling**: Validate JWT signature and expiration on every authenticated request.

## Headers

-   **Helmet**: Use security headers (HSTS, X-Frame-Options, X-Content-Type-Options).
-   **CORS**: Configure CORS strictly. Do not use `Access-Control-Allow-Origin: *` in production.

## Rate Limiting

-   Implement rate limiting (e.g., Redis-based) to prevent abuse and DDoS attacks.
-   Return `429 Too Many Requests` when limits are exceeded.
