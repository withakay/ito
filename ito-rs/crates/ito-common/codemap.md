[Codemap: ito-common]|L0 leaf: shared utils, zero workspace deps; no domain knowledge, no policy

[Entry Points]|src/fs.rs: FileSystem trait (testable I/O) |src/id/**: ID parsing (change/module/spec)
|src/io.rs: file op helpers |src/match_.rs: fuzzy matching |src/paths.rs: .ito path builders |src/git_url.rs: remote URL parsing

[Design]|boring+deterministic; add only when multiple crates need same primitive with no domain knowledge required; MUST NOT depend on any workspace crate

[Gotchas]|no workflow policy here |ID parser changes affect CLI UX, validation, and backend routes

[Tests]|cargo test -p ito-common
