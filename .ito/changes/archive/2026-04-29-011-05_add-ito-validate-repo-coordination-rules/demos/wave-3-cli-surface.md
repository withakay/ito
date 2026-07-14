# Wave 3: CLI Surface for ito validate repo

*2026-04-29T13:09:01Z by Showboat 0.6.1*
<!-- showboat-id: 25f68ad3-5c27-4a96-886c-10d5714e18b7 -->

Wave 3 (tasks 3.1, 3.2) ships the ito validate repo subcommand. Task 3.1 added the Repo(RepoValidateArgs) variant to ValidateCommand with --staged/--strict/--json/--rule/--no-rule/--list-rules/--explain flags. Task 3.2 implemented app/validate_repo.rs adapter with documented exit codes (0/1/2). CliError gained an exit_code field so usage errors and unloadable config exit 2 as the spec requires. 9 new integration tests in tests/validate_repo_cli.rs cover help discoverability, --list-rules, --explain, JSON envelope, --strict promotion, and exit-code matrix.

```bash
cargo run -p ito-cli --bin ito --quiet -- validate repo --list-rules 2>&1 | head -10
```

```output
Built-in repository validation rules:

  [x] coordination/branch-name-set                     WARNING  (always active)
  [x] coordination/gitignore-entries                   WARNING  changes.coordination_branch.storage == worktree
  [x] coordination/staged-symlinked-paths              ERROR    changes.coordination_branch.storage == worktree && staged context present
  [x] coordination/symlinks-wired                      ERROR    changes.coordination_branch.storage == worktree
  [x] worktrees/layout-consistent                      WARNING  worktrees.enabled == true
  [x] worktrees/no-write-on-control                    ERROR    worktrees.enabled == true
```

```bash
cargo run -p ito-cli --bin ito --quiet -- validate repo 2>&1 ; echo exit=$?
```

```output
Repository validation passed.
exit=0
```

```bash
cargo test -p ito-cli --test validate_repo_cli 2>&1 | tail -3
```

```output

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.63s

```
