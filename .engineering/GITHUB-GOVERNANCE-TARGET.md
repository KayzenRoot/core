# CORE GitHub Governance Target

Status: `TARGET_NOT_ENFORCED`

## Purpose

Define the GitHub-hosted protection profile CORE should use after repository-administration write capability is available. This file is policy intent and MUST NOT be interpreted as proof that GitHub currently enforces the rules.

## Current evidence

At bootstrap audit time the GitHub Rulesets API returned an empty ruleset collection for `KayzenRoot/core`.

## Target `main` policy

- Changes reach `main` through pull requests.
- Force pushes are blocked.
- Branch deletion is blocked.
- The exact candidate head must pass the `Governance` required check.
- Required review/audit evidence must bind the exact candidate head.
- Unresolved HIGH or CRITICAL findings block merge.
- Conversation resolution is required when review threads exist.
- No bypass actor is assumed by default.
- Squash merge is preferred for governed Work Orders unless a future ADR selects another history policy.
- Auto-merge may be used only after required checks and governed review conditions are satisfied.

## Portability

GitHub is an acceleration/evidence platform. CORE product runtime must not silently require GitHub unless future product Scope explicitly admits that dependency.

## Activation gate

Changing repository rulesets is an administration mutation. Activate this target only through an authorized GitHub administration surface, then capture the returned ruleset id/configuration and update this document from `TARGET_NOT_ENFORCED` to an evidence-backed state.
