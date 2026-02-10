---
name: feature-flags
description: Managing feature toggles and gradual rollouts
---

# Feature Flags & Toggles

## Principles

1.  **Decouple Deploy from Release**: Deploy code to production hidden behind a flag. Enable feature when ready.
2.  **Short-lived**: Most flags should be removed after the feature is stable to avoid technical debt.
3.  **Context-aware**: Flags should support targeting (e.g., enable for internal users first, then 10% of traffic).

## Implementation
Avoid simple `if (env.ENABLE_FEATURE)` checks. Use a robust client/service like Unleash or LaunchDarkly.

```javascript
if (flags.isEnabled('new-checkout-flow', userContext)) {
  renderNewCheckout();
} else {
  renderLegacyCheckout();
}
```

## Naming
Be explicit: `enable_new_dashboard_v2` is better than `dashboard_refactor`.
