# CORE GitHub Governance Target

Status: `ENFORCED_PENDING_REVIEW`

## Purpose

Define the GitHub-hosted protection profile CORE uses after repository-administration write capability is available. The current state below is evidence-backed by the GitHub API readback recorded in the CORE-GOV-002-C01 Evidence Bundle; it is not an independent review or merge approval.

## Current evidence

At the pre-mutation audit, the GitHub Rulesets API returned an empty ruleset collection for `KayzenRoot/core`.

Post-activation readback at `2026-09-21T13:22:47Z`:

- repository: `KayzenRoot/core`, default branch: `main`;
- ruleset: `CORE main protection`, numeric id `23769853`, source `KayzenRoot/core`, enforcement `active`;
- target: `~DEFAULT_BRANCH`, with no bypass actors;
- deletion and non-fast-forward rules are active;
- pull requests are required, review threads must be resolved, stale reviews are dismissed on push, approvals required: `0`, CODEOWNERS approval: `false`, last-push approval: `false`, unattributed-Copilot extra approval: `false`;
- allowed ruleset merge methods: `rebase`, `squash`;
- required status checks are `Governance`, `M01 (ubuntu-latest)`, `M01 (windows-latest)`, `M01 fuzz campaign`, `M02 workspace adapter (ubuntu-latest)`, `M02 workspace adapter (windows-latest)`, and `M02 bounded fuzz campaign`, with strict freshness and enforcement on branch creation;
- repository settings after mutation: squash merge `true`, merge commit `false`, rebase merge `true`, auto-merge `true`, update branch `true`, delete branch on merge `true`, squash title `PR_TITLE`, squash message `PR_BODY`.

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
- Auto-merge may be used only after all required checks and governed review conditions are satisfied.

## Enforcement boundary

The active GitHub ruleset mechanically enforces the repository-hosted controls: pull-request entry to `main`, deletion/non-fast-forward protection, strict required status checks, review-thread resolution, no bypass actor and allowed merge methods.

The GEF independent audit verdict, exact-head review record and the rule that unresolved HIGH/CRITICAL findings block promotion remain process-level governance gates. They are not represented by a separate GitHub-native approval/status context in CORE-GOV-002-C01. Because the repository is currently operated through a solo maintainer identity, the ruleset approval count is intentionally zero rather than creating an impossible self-approval requirement. Auto-merge MUST NOT be enabled on a PR until the governed audit verdict permits promotion.

## Safe executor tool bootstrap

When a required secure executor CLI is unavailable, the executor must first attempt a user-scoped installation from an official or trusted package source, outside the repository, and validate version/provenance before declaring the Work Order blocked. This never authorizes privilege elevation, credential extraction, secret persistence, authentication bypass or repository mutation outside the admitted scope. If the tool is functional but no usable authentication exists without user credential intervention, the terminal state is `BLOCKED_AUTH_ONLY`.

## Portability

GitHub is an acceleration/evidence platform. CORE product runtime must not silently require GitHub unless future product Scope explicitly admits that dependency.

## Activation gate

Changing repository rulesets is an administration mutation. This target was activated through the authorized `gh` GitHub administration surface and must remain bound to the returned ruleset id/configuration and the exact evidence bundle. Independent review, PR CI and checkpoint promotion remain separate gates.
