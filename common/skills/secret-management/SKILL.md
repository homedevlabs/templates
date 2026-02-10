---
name: secret-management
description: Secure handling of credentials and keys
---

# Secret Management

## Golden Rules

1.  **No Secrets in Code**: Absolutely zero API keys, passwords, or certificates in Git.
2.  **No .env in Production**: Do not rely on copying `.env` files to servers. Inject secrets as environment variables via the platform (Kubernetes Secrets, Vault).

## Rotation
Secrets should be rotated regularly. Automated rotation (e.g., AWS Secrets Manager) is preferred.

## Development
Use tools like `dotenv` for local development, providing a `.env.example` template with safe defaults.
Never commit `.env` (add to `.gitignore`).

## Leak Detection
Implement pre-commit hooks (e.g., `gitleaks`, `trufflehog`) to scan for accidental secret commits before push.
