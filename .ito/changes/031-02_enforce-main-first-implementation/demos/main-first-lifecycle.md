# Main-First Implementation Readiness Demo

*2026-07-13T16:09:31Z by Showboat 0.6.1*
<!-- showboat-id: 2a53123c-7ac4-41b7-af45-4a1378e29781 -->

This executable walkthrough proves the main-first boundary: local proposal copies are rejected without mutation, a reviewed proposal on authoritative main passes prepare, a stale pre-integration branch fails execute, and Ito creates a verified implementation worktree where task mutations are allowed.

```bash
bash .ito/changes/031-02_enforce-main-first-implementation/demos/main-first-fixture.sh init .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture && echo "Created a local-only proposal on a pre-integration change branch."
```

```output
Switched to a new branch '031-02_enforce-main-first-implementation'
Created a local-only proposal on a pre-integration change branch.
```

A complete proposal in the working tree is not authority. Prepare must fail because main does not contain the proposal marker.

```bash
fixture="$PWD/.ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture"; ito="$PWD/target/debug/ito"; set +e; report=$(cd "$fixture" && "$ito" change preflight 031-02_enforce-main-first-implementation --for prepare --json); status=$?; set -e; printf "%s\n" "$report" | jq "{phase, ready, authority_ref: .authority.target_ref, failed: [.conditions[] | select(.passed == false) | .code]}"; test "$status" -eq 1
```

```output
{
  "phase": "prepare",
  "ready": false,
  "authority_ref": "refs/heads/main",
  "failed": [
    "change_target"
  ]
}
```

```bash
fixture="$PWD/.ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture"; ito="$PWD/target/debug/ito"; tasks="$fixture/.ito/changes/031-02_enforce-main-first-implementation/tasks.md"; before=$(git hash-object "$tasks"); set +e; report=$(cd "$fixture" && "$ito" tasks start 031-02_enforce-main-first-implementation 1.1 --json); status=$?; set -e; after=$(git hash-object "$tasks"); printf "%s\n" "$report" | jq "{phase, ready}"; printf "exit=%s task_state_unchanged=%s\n" "$status" "$([ "$before" = "$after" ] && echo true || echo false)"; test "$status" -eq 1; test "$before" = "$after"
```

```output
{
  "phase": "execute",
  "ready": false
}
exit=1 task_state_unchanged=true
```

After review, merge the proposal-only commit into main. Prepare now resolves one authoritative OID and discovers the integration commit from that history.

```bash
bash .ito/changes/031-02_enforce-main-first-implementation/demos/main-first-fixture.sh integrate .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture; fixture="$PWD/.ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture"; ito="$PWD/target/debug/ito"; (cd "$fixture" && "$ito" change preflight 031-02_enforce-main-first-implementation --for prepare --refresh --json) | jq "{phase, ready, authority_ref: .authority.target_ref, captured_oid: (.authority.oid != null), authority_matches_integration: (.authority.oid == .proposal_integration_oid)}"
```

```output
Switched to branch 'main'
Switched to branch '031-02_enforce-main-first-implementation'
{
  "phase": "prepare",
  "ready": true,
  "authority_ref": "refs/heads/main",
  "captured_oid": true,
  "authority_matches_integration": true
}
```

The pre-integration branch is still not a valid implementation checkout: it does not contain the merge commit that introduced the proposal on main.

```bash
fixture="$PWD/.ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture"; ito="$PWD/target/debug/ito"; set +e; report=$(cd "$fixture" && "$ito" change preflight 031-02_enforce-main-first-implementation --for execute --json); status=$?; set -e; printf "%s\n" "$report" | jq "{phase, ready, failed: [.conditions[] | select(.passed == false) | .code]}"; test "$status" -eq 1
```

```output
{
  "phase": "execute",
  "ready": false,
  "failed": [
    "implementation_ancestry"
  ]
}
```

Discard the stale branch and let Ito create the implementation worktree from the captured authority OID. The guarded command performs prepare before creation and execute before returning the path.

```bash
set -e; bash .ito/changes/031-02_enforce-main-first-implementation/demos/main-first-fixture.sh retire-old .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture; fixture=$(cd .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture && pwd -P); ito="$PWD/target/debug/ito"; worktree=$(cd "$fixture" && "$ito" worktree ensure --change 031-02_enforce-main-first-implementation); test -d "$worktree"; test "$(basename "$worktree")" = "031-02_enforce-main-first-implementation"; echo "verified_worktree_created=true"
```

```output
Switched to branch 'main'
verified_worktree_created=true
```

```bash
set -e; fixture=$(cd .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture && pwd -P); ito="$PWD/target/debug/ito"; worktree=$(cd "$fixture" && "$ito" worktree ensure --change 031-02_enforce-main-first-implementation); (cd "$worktree" && "$ito" change preflight 031-02_enforce-main-first-implementation --for execute --json) | jq "{phase, ready, ancestry: (.conditions[] | select(.code == \"implementation_ancestry\") | .passed), checkout_identity: (.conditions[] | select(.code == \"checkout_identity\") | .passed)}"
```

```output
{
  "phase": "execute",
  "ready": true,
  "ancestry": true,
  "checkout_identity": true
}
```

Once execute readiness passes, normal implementation and iteration features remain available. Task mutations now succeed in the verified worktree.

```bash
set -e; fixture=$(cd .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture && pwd -P); ito="$PWD/target/debug/ito"; worktree=$(cd "$fixture" && "$ito" worktree ensure --change 031-02_enforce-main-first-implementation); cd "$worktree"; "$ito" tasks start 031-02_enforce-main-first-implementation 1.1 --json | jq "{action, change_id, task_id, status}"
```

```output
{
  "action": "start",
  "change_id": "031-02_enforce-main-first-implementation",
  "task_id": "1.1",
  "status": "in_progress"
}
```

```bash
set -e; fixture=$(cd .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture && pwd -P); ito="$PWD/target/debug/ito"; worktree=$(cd "$fixture" && "$ito" worktree ensure --change 031-02_enforce-main-first-implementation); cd "$worktree"; "$ito" tasks complete 031-02_enforce-main-first-implementation 1.1 --json | jq "{action, change_id, task_id, status}"
```

```output
{
  "action": "complete",
  "change_id": "031-02_enforce-main-first-implementation",
  "task_id": "1.1",
  "status": "complete"
}
```

Orchestration persists and rechecks the same structured readiness gate before initial and resumed dispatch.

```bash
cargo test -q -p ito-core --test orchestrate_run_state orchestrate_ >/dev/null && echo "orchestration_readiness_and_resume_tests_passed=true"
```

```output
orchestration_readiness_and_resume_tests_passed=true
```

```bash
bash .ito/changes/031-02_enforce-main-first-implementation/demos/main-first-fixture.sh cleanup .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture; test ! -e .ito/changes/031-02_enforce-main-first-implementation/demos/.main-first-fixture; echo "fixture_cleaned=true"
```

```output
fixture_cleaned=true
```
