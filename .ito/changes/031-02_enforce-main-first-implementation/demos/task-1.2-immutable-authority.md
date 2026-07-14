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
    ) -> Result<String, ReadinessGitError>;

    /// List immutable tree entries below one literal repository-relative path.
    fn list_tree(
        &self,
        _repository_root: &Path,
        _authority_oid: &str,
        _path: &str,
    ) -> Result<Vec<GitTreeEntry>, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "authority tree listing is not implemented by this Git adapter",
        ))
    }

    /// Read one blob by object OID without consulting a checkout.
    fn read_blob(
        &self,
        _repository_root: &Path,
        _blob_oid: &str,
    ) -> Result<String, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "authority blob reading is not implemented by this Git adapter",
        ))
    }

    /// Find the newest first-parent target commit that introduced one literal marker path.
    fn find_introduction_commit(
        &self,
        _repository_root: &Path,
        _authority_oid: &str,
        _marker_path: &str,
    ) -> Result<String, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "proposal integration discovery is not implemented by this Git adapter",
        ))
    }

    /// Inspect checkout identity without reading proposal files from it.
    fn inspect_checkout(&self, _checkout: &Path) -> Result<CheckoutState, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "checkout identity inspection is not implemented by this Git adapter",
        ))
    }

    /// Test whether `ancestor_oid` is an ancestor of `descendant_oid`.
    fn is_ancestor(
        &self,
        _checkout: &Path,
        _ancestor_oid: &str,
        _descendant_oid: &str,
    ) -> Result<bool, ReadinessGitError> {
        Err(ReadinessGitError::new(
            "checkout ancestry inspection is not implemented by this Git adapter",
        ))
    }
}
```
