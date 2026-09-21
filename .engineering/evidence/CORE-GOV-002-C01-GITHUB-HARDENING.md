# CORE-GOV-002-C01 - GitHub CLI Bootstrap and Repository Hardening

## State

- Work Order: `CORE-GOV-002-C01`
- Repository: `KayzenRoot/core`
- Executor: Codex
- Current governance state: `ENFORCED_PENDING_REVIEW`
- Product scope: no M03 implementation or product-source change
- GitHub account used: `KayzenRoot`
- GitHub CLI: `gh version 2.101.0 (2026-09-15)`
- Bootstrap: existing user-scoped GitHub CLI package was upgraded through `winget` with the official `GitHub.cli` package; the installer reported successful archive hash verification. The executable was validated at `C:\Users\csn19\AppData\Local\Microsoft\WinGet\Packages\GitHub.cli_Microsoft.Winget.Source_8wekyb3d8bbwe\bin\gh.exe`.
- Authentication: `gh auth status -h github.com` succeeded for active account `KayzenRoot`; no token value was recorded.
- Repository permission readback: `admin=true`, `maintain=true`, `push=true`, `pull=true`, `triage=true`.

## Exact Git and dirty-state binding

- Original checkout: `D:\Projeto Codexx\core`
- Original branch: `main`
- Original HEAD at preflight: `fdb4dbe165e74b009c43df3874b6043c9b94710b`
- Original dirty state preserved without stash/reset/clean: `AGENTS.md` modified; `.gitattributes` untracked
- Current remote base: `origin/main` at `499042df143fc519e42c6904461a011b62d26125`
- Isolated worktree: `D:\Projeto Codexx\core-gov-002-c01`
- Governance branch: `governance/github-hardening`
- Worktree base was created cleanly from the current `origin/main`; the original dirty checkout was not used for edits.

## Pre-mutation GitHub state

Repository readback from `gh api repos/KayzenRoot/core`:

```text
default_branch=main
allow_auto_merge=false
allow_merge_commit=true
allow_rebase_merge=true
allow_squash_merge=true
allow_update_branch=false
delete_branch_on_merge=false
squash_merge_commit_title=COMMIT_OR_PR_TITLE
squash_merge_commit_message=COMMIT_MESSAGES
visibility=public
```

Rulesets readback before mutation: `[]`.

Workflow permissions readback: `default_workflow_permissions=read`, `can_approve_pull_request_reviews=false`.

The real check contexts observed on the current workflow/PR surface were:

```text
Governance
M01 (ubuntu-latest)
M01 (windows-latest)
M01 fuzz campaign
M02 workspace adapter (ubuntu-latest)
M02 workspace adapter (windows-latest)
M02 bounded fuzz campaign
```

The latest pre-mutation `main` run was `35601931550` at the exact base SHA above. Six jobs succeeded; `M01 (ubuntu-latest)` failed at `hostile_git_deadline_kills_reaps_and_does_not_poison_next_inspection` in `crates/core-workspace/tests/git_system.rs:254`. This existing failure is retained as a required gate and was not changed by this Work Order.

## Approved mutations

Repository settings were patched idempotently with `gh api -X PATCH repos/KayzenRoot/core` using only the approved fields:

```text
allow_squash_merge=true
allow_merge_commit=false
allow_auto_merge=true
allow_update_branch=true
delete_branch_on_merge=true
squash_merge_commit_title=PR_TITLE
squash_merge_commit_message=PR_BODY
```

The currently enabled post-mutation merge methods are `rebase` and `squash`; both are carried into the ruleset. Merge commit was disabled because the canonical governance target identifies squash as the preferred governed history and the C01 Work Order permits disabling merge commit when it does not conflict with canonical policy.

## Ruleset readback

- Name: `CORE main protection`
- Numeric id: `23769853`
- Source: `KayzenRoot/core` / `Repository`
- Target: `branch`
- Enforcement: `active`
- Conditions: `ref_name.include=["~DEFAULT_BRANCH"]`, `exclude=[]`
- Bypass actors: none (`[]`)

Sanitized ruleset readback:

```json
{
  "id": 23769853,
  "name": "CORE main protection",
  "target": "branch",
  "enforcement": "active",
  "conditions": {"ref_name": {"include": ["~DEFAULT_BRANCH"], "exclude": []}},
  "bypass_actors": [],
  "rules": [
    {"type": "deletion"},
    {"type": "non_fast_forward"},
    {
      "type": "pull_request",
      "parameters": {
        "allowed_merge_methods": ["rebase", "squash"],
        "dismiss_stale_reviews_on_push": true,
        "require_code_owner_review": false,
        "require_extra_approval_for_unattributed_changes": false,
        "require_last_push_approval": false,
        "required_approving_review_count": 0,
        "required_review_thread_resolution": true,
        "required_reviewers": []
      }
    },
    {
      "type": "required_status_checks",
      "parameters": {
        "do_not_enforce_on_create": false,
        "required_status_checks": [
          {"context": "Governance"},
          {"context": "M01 (ubuntu-latest)"},
          {"context": "M01 (windows-latest)"},
          {"context": "M01 fuzz campaign"},
          {"context": "M02 workspace adapter (ubuntu-latest)"},
          {"context": "M02 workspace adapter (windows-latest)"},
          {"context": "M02 bounded fuzz campaign"}
        ],
        "strict_required_status_checks_policy": true
      }
    }
  ]
}
```

The additional approval rule for unattributed Copilot pull requests was explicitly set to `false`; the configured approval count is also exactly zero. No reviewer, CODEOWNERS or last-push approval is required.

## Verification performed

Successful readback/verification commands:

```text
gh api repos/KayzenRoot/core
gh api repos/KayzenRoot/core/rulesets
gh api repos/KayzenRoot/core/rulesets/23769853
gh api repos/KayzenRoot/core/actions/permissions/workflow
gh ruleset view 23769853 -R KayzenRoot/core
gh ruleset check main -R KayzenRoot/core
```

`gh ruleset view` reported the four expected rules and `You can bypass: never`. `gh ruleset check main` reported all four rules configured from ruleset `23769853`.

Local documentation checks before commit:

```text
git diff --check = PASS
```

The repository validator and PR CI result are recorded only after execution on the exact governance branch/head. No secret scan result is claimed until it is run on the final delta.

## Risks and residuals

- The latest pre-mutation `main` CI had a pre-existing M01 Ubuntu failure. It remains in the required status-check set and was green on the governance PR run, so auto-merge cannot advance through any failing required gate.
- The exact governance PR run `35605615596` validated head `afc54df580386f46cee3d29e60b13a3d3a74f8e1`: 6 of 7 required contexts passed; `M02 workspace adapter (windows-latest)` failed in `Workspace and M02 tests` because `tests::multiple_leases_release_in_different_order_without_lost_notifications` reported an active capability lease and `ForceTerminate` at `crates/core-runtime/src/lib.rs:2205`. This is product/runtime scope and is not corrected here.
- Because one required context failed, the PR is not `READY_FOR_REVIEW`; the ruleset remains fail-closed.
- Ruleset administration is remote state; independent review and checkpoint promotion remain separate from this executor evidence.
- No product, M03, dependency, workflow or secret-bearing source change is included.

## Rollback

Do not run unless the active ruleset causes an operational lockout requiring immediate recovery. The exact rollback command is:

```text
gh api -X PUT repos/KayzenRoot/core/rulesets/23769853 --input <sanitized-ruleset-body-with-enforcement-disabled>
```

The rollback must preserve the ruleset id and change only `enforcement` to `disabled`; it must be followed by a fresh readback and new evidence. No rollback was executed.

## Checkpoint Delta

Record the objectively proven activation of `CORE main protection` id `23769853` on the current `origin/main` base, the exact settings/ruleset readback and the residual M01 Ubuntu failure. Do not record independent approval, merge or product completion.

## PR handoff

- PR: `#56` - https://github.com/KayzenRoot/core/pull/56
- Initial governance branch head validated: `afc54df580386f46cee3d29e60b13a3d3a74f8e1`
- Base head: `499042df143fc519e42c6904461a011b62d26125`
- Workflow run: `35605615596` - 6/7 required contexts passed; `M02 workspace adapter (windows-latest)` failed
- PR state: open, not merged; independent review and correction of the runtime failure remain pending
