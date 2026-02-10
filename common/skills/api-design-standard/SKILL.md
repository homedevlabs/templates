---
name: api-design-standard
description: Standards for designing RESTful APIs
---

# API Design Standards

## Principles

1.  **Resource-Oriented**: URLs should represent resources (nouns), not actions (verbs).
    -   *Good*: `POST /users`, `GET /orders/123`
    -   *Bad*: `POST /createUser`, `GET /getOrderDetails`
2.  **Stateless**: Each request must contain all information necessary to understand the request.
3.  **Versioning**: Version your API via URL path (`/v1/users`) or Accept header.

## Responses

-   **Status Codes**: Use correct HTTP status codes.
    -   `200 OK`: Success (GET, PUT)
    -   `201 Created`: Success (POST)
    -   `204 No Content`: Success (DELETE)
    -   `400 Bad Request`: Client error (validation)
    -   `401 Unauthorized`: Missing or invalid token
    -   `403 Forbidden`: Valid token but insufficient permissions
    -   `404 Not Found`: Resource does not exist
    -   `500 Internal Server Error`: Server bug
-   **Envelope**: Standardize JSON response structure.
    ```json
    {
      "data": { ... },
      "meta": { "page": 1, "total": 100 },
      "error": null
    }
    ```

## Filtering & Pagination

-   **Pagination**: Accept `page` and `limit` (or `cursor`). Always return pagination metadata in response.
-   **Sorting**: Use `sort=-createdAt` (descending) or `sort=name` (ascending).
