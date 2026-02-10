---
name: container-hardening
description: Security best practices for Docker containers
---

# Container Hardening

## Base Images

1.  **Minimal**: Use minimal base images like `alpine` or `distroless` (Google) to reduce attack surface.
    -   *Good*: `gcr.io/distroless/nodejs:18`
    -   *Bad*: `node:18` (contains unnecessary tools like curl, bash)
2.  **Pinned Versions**: Always pin specific tag versions or SHA digests. Avoid `latest`.

## Runtime Security

-   **Non-Root User**: NEVER run containers as root.
    ```dockerfile
    USER node
    ```
-   **Read-Only Filesystem**: Mount root filesystem as read-only wherever possible.

## Scanning
Scan images for vulnerabilities (CVEs) during the CI/CD pipeline (e.g., using Trivy or Grype). Block build on High/Critical severities.
