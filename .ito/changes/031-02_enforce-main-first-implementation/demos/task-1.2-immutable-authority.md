# Immutable proposal authority

*2026-07-13T14:28:48Z by Showboat 0.6.1*
<!-- showboat-id: 75533f0e-be5d-4e4d-b2a2-8e83f073bd1a -->

Pull-request mode derives authority from the configured target branch's real tracked upstream; it does not infer origin/main.

```bash
git for-each-ref --format='%(refname)|%(upstream)|%(upstream:remotename)|%(upstream:remoteref)' refs/heads/main
```

```output
refs/heads/main|refs/remotes/origin/main|origin|refs/heads/main
```

Direct-merge and pull-request authority are each resolved to a commit OID. These happen to agree in this checkout, but the readiness report retains the selected ref and captured OID.

```bash
git rev-parse --verify --end-of-options 'refs/heads/main^{commit}'
```

```output
813a8d0ac50d1c7b1ee5f592933f59037de60693
```

```bash
git rev-parse --verify --end-of-options 'refs/remotes/origin/main^{commit}'
```

```output
813a8d0ac50d1c7b1ee5f592933f59037de60693
```

Explicit refresh uses one force refspec and suppresses FETCH_HEAD writes before resolving the captured authority.

```bash
sed -n '90,145p' ito-rs/crates/ito-core/src/implementation_readiness/git.rs
```

```output

        Ok(TrackedUpstream {
            tracking_ref: tracking_ref.to_string(),
            remote: remote.to_string(),
            remote_ref: remote_ref.to_string(),
        })
    }

    fn refresh_upstream(
        &self,
        repository_root: &Path,
        upstream: &TrackedUpstream,
    ) -> Result<(), ReadinessGitError> {
        let refspec = format!("+{}:{}", upstream.remote_ref, upstream.tracking_ref);
        run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "fetch",
                "--no-tags",
                "--no-write-fetch-head",
                upstream.remote.as_str(),
                refspec.as_str(),
            ],
            "refresh target branch upstream",
        )?;
        Ok(())
    }

    fn resolve_commit(
        &self,
        repository_root: &Path,
        target_ref: &str,
    ) -> Result<String, ReadinessGitError> {
        let commit_ref = format!("{target_ref}^{{commit}}");
        let output = run_git(
            &SystemProcessRunner,
            repository_root,
            [
                "rev-parse",
                "--verify",
                "--end-of-options",
                commit_ref.as_str(),
            ],
            "resolve authority commit",
        )?;
        let oid = output.trim();
        if !matches!(oid.len(), 40 | 64) || !oid.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(ReadinessGitError::new(format!(
                "authority resolution returned an invalid commit OID: '{oid}'"
            )));
        }
        Ok(oid.to_ascii_lowercase())
    }
}

```
