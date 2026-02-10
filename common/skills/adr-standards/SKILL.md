---
name: adr-standards
description: Guideline for writing and maintaining Architecture Decision Records (ADRs)
---

# Architecture Decision Records (ADR) Standards

## Why ADRs?
To document significant architectural decisions in a way that provides context, consequences, and history for future team members.

## Structure
Every ADR should follow this Markdown structure:

1.  **Title**: Short noun phrase (e.g., `Use BullMQ for Background Jobs`).
2.  **Status**: `Proposed`, `Accepted`, `Deprecated`, or `Superseded`.
3.  **Context**: What is the issue we're seeing? What are the constraints?
4.  **Decision**: The change we are proposing or the technology we are choosing.
5.  **Consequences**: The results of this decision (good, bad, and neutral).

## Location
Store all ADRs in `docs/adr/XXXX-title.md`, where `XXXX` is a sequential number (e.g., `0001-record-architecture.md`).

## Tools
Use `adr-tools` CLI to manage them easily:
`adr new Use Postgres as Database`
