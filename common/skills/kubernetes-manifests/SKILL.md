---
name: kubernetes-manifests
description: Guidelines for production-ready Kubernetes configuration
---

# Kubernetes Manifests Best Practices

## Resources

1.  **Requests & Limits**: Always define `requests` (for scheduling) and `limits` (for OOM killing) for CPU and Memory.
    -   *Example*: `requests: cpu: 100m, memory: 128Mi`
2.  **QoS**: Prefer Guaranteed QoS (request == limit) for mission-critical apps to avoid eviction under pressure.

## Health Checks

-   **Liveness Probe**: Checks if the app is crashed or deadlocked. Restarts container on failure.
-   **Readiness Probe**: Checks if the app is ready to serve traffic (e.g., DB connected). Removes from service endpoints on failure.
-   **Startup Probe**: For slow-starting apps, prevents liveness probe from killing it prematurely.

## Deployment Strategy

-   **RollingUpdate**: Use `maxSurge` (e.g., 25%) and `maxUnavailable` (e.g., 0) to ensure zero-downtime deployments.
-   **Replicas**: Run at least 2 replicas for high availability (HA).

## Network

-   **Service**: Use `ClusterIP` for internal communication.
-   **Ingress**: Use Ingress for external access, configured with proper TLS termination.
