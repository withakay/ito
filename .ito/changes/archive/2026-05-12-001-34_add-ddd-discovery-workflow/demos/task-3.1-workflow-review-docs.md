# Task 3.1: Workflow Review Documentation

*2026-05-11T20:01:27Z by Showboat 0.6.1*
<!-- showboat-id: ed985a41-6c67-4e88-8a3c-efd06e1e6210 -->

Updated peer-review guidance, the human agent workflow guide, and the workflow diagram so DDD discovery depth, domain language, bounded-context framing, validators, and approved domain-doc promotion are part of the lifecycle.

```bash
make docs
```

```output
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
   Generated /Users/jack/Code/withakay/ito/ito-worktrees/001-34_add-ddd-discovery-workflow/target/doc/ito_backend/index.html and 9 other files
rm -rf docs/rustdoc
cp -R target/doc docs/rustdoc
```
