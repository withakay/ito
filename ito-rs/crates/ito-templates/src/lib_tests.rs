use super::*;
use std::collections::BTreeSet;

#[test]
fn skill_inventory_contains_exactly_seven_entrypoints() {
    let entrypoints = skills_files()
        .into_iter()
        .filter(|file| file.relative_path.ends_with("/SKILL.md"))
        .filter_map(|file| file.relative_path.split('/').next().map(str::to_owned))
        .collect::<Vec<_>>();
    let actual = entrypoints.iter().cloned().collect::<BTreeSet<_>>();
    let expected = [
        "ito",
        "ito-proposal",
        "ito-research",
        "ito-apply",
        "ito-review",
        "ito-archive",
        "ito-loop",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();

    assert_eq!(
        entrypoints.len(),
        7,
        "expected one entrypoint per lifecycle skill"
    );
    assert_eq!(actual, expected);
}

#[test]
fn skill_inventory_keeps_canonical_lifecycle_order() {
    assert_eq!(
        LIFECYCLE_SKILL_NAMES,
        [
            "ito",
            "ito-proposal",
            "ito-research",
            "ito-apply",
            "ito-review",
            "ito-archive",
            "ito-loop",
        ]
    );

    let entrypoints = skills_files()
        .into_iter()
        .filter(|file| file.relative_path.ends_with("/SKILL.md"))
        .filter_map(|file| file.relative_path.split('/').next())
        .collect::<Vec<_>>();
    assert_eq!(entrypoints, LIFECYCLE_SKILL_NAMES);
}

#[test]
fn lifecycle_skill_content_preserves_retired_guidance() {
    let expectations: [(&str, &[&str]); 7] = [
        (
            "ito",
            &[
                "six lifecycle destinations",
                "installation error",
                "ito update",
                "ito agent instruction cleanup",
                "no wildcard",
            ],
        ),
        (
            "ito-proposal",
            &[
                "intake",
                "module confirmation",
                "brainstorming",
                "ito plan init",
                "research handoff",
            ],
        ),
        (
            "ito-research",
            &[
                "memory-search",
                "memory-query",
                ".ito/wiki/",
                "guidance, not the source of truth",
            ],
        ),
        (
            "ito-apply",
            &["main-first", "read-only", "worktree", "ito tasks", "commit"],
        ),
        (
            "ito-review",
            &[
                "acceptance criteria",
                "tests",
                "completion evidence",
                "independent",
            ],
        ),
        (
            "ito-archive",
            &[
                "explicit user confirmation",
                "spec promotion",
                "memory-capture",
                ".ito/wiki/",
                "cleanup",
            ],
        ),
        (
            "ito-loop",
            &[
                "ito ralph",
                "ito agent instruction orchestrate",
                "bounded",
                ".ito/user-prompts/orchestrate.md",
            ],
        ),
    ];

    for (skill, needles) in expectations {
        let path = format!("{skill}/SKILL.md");
        let contents = get_skill_file(&path).expect("retained lifecycle skill should exist");
        let contents = std::str::from_utf8(contents).expect("skill should be UTF-8");
        for needle in needles {
            assert!(
                contents.contains(needle),
                "{skill} is missing consolidated guidance: {needle}"
            );
        }
    }
}

#[test]
fn normalize_ito_dir_prefixes_dot() {
    assert_eq!(normalize_ito_dir(".ito"), ".ito");
    assert_eq!(normalize_ito_dir("ito"), ".ito");
    assert_eq!(normalize_ito_dir(".x"), ".x");
}

#[test]
fn render_rel_path_rewrites_ito_prefix() {
    assert_eq!(render_rel_path(".ito/AGENTS.md", ".ito"), ".ito/AGENTS.md");
    assert_eq!(render_rel_path(".ito/AGENTS.md", ".x"), ".x/AGENTS.md");
    assert_eq!(render_rel_path("AGENTS.md", ".x"), "AGENTS.md");
}

#[test]
fn render_bytes_rewrites_dot_ito_paths() {
    let b = render_bytes(b"see .ito/AGENTS.md", ".x");
    assert_eq!(std::str::from_utf8(&b).unwrap(), "see .x/AGENTS.md");
}

#[test]
fn extract_managed_block_returns_inner_content() {
    let s = "pre\n<!-- ITO:START -->\nhello\nworld\n<!-- ITO:END -->\npost\n";
    assert_eq!(extract_managed_block(s), Some("hello\nworld"));
}

#[test]
fn extract_managed_block_preserves_trailing_newline_from_content() {
    // Content ends with a newline, plus the TS separator newline before the end marker.
    let s = "pre\n<!-- ITO:START -->\nhello\nworld\n\n<!-- ITO:END -->\npost\n";
    assert_eq!(extract_managed_block(s), Some("hello\nworld\n"));
}

#[test]
fn default_project_files_contains_expected_files() {
    let files = default_project_files();
    assert!(!files.is_empty());

    let mut has_user_guidance = false;
    for EmbeddedFile {
        relative_path,
        contents,
    } in files
    {
        if relative_path == ".ito/user-guidance.md" {
            has_user_guidance = true;
            let contents = std::str::from_utf8(contents).expect("template should be UTF-8");
            assert!(contents.contains(ITO_START_MARKER));
            assert!(contents.contains(ITO_END_MARKER));
        }
    }

    assert!(
        has_user_guidance,
        "expected .ito/user-guidance.md in templates"
    );
}

#[test]
fn default_project_config_uses_pull_request_proposal_integration() {
    let config = default_project_files()
        .into_iter()
        .find(|file| file.relative_path == ".ito/config.json")
        .expect("default project config should be embedded");
    let value: serde_json::Value =
        serde_json::from_slice(config.contents).expect("default project config should be JSON");

    assert_eq!(
        value.pointer("/changes/proposal/integration_mode"),
        Some(&serde_json::json!("pull_request"))
    );
}

#[test]
fn default_home_files_returns_a_vec() {
    // The default home templates may be empty, but should still be loadable.
    let _ = default_home_files();
}

#[test]
fn schema_files_contains_builtins() {
    let files = schema_files();
    assert!(!files.is_empty());
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "spec-driven/schema.yaml")
    );
    assert!(files.iter().any(|f| f.relative_path == "tdd/schema.yaml"));
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "minimalist/schema.yaml")
    );
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "event-driven/schema.yaml")
    );
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "minimalist/UPSTREAM.md")
    );
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "event-driven/UPSTREAM.md")
    );
}

#[test]
fn get_schema_file_returns_contents() {
    let file = get_schema_file("spec-driven/schema.yaml").expect("schema should exist");
    let text = std::str::from_utf8(file).expect("schema should be utf8");
    assert!(text.contains("name: spec-driven"));
}

#[test]
fn presets_files_contains_orchestrate_builtins() {
    let files = presets_files();
    assert!(!files.is_empty());
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == "orchestrate/generic.yaml"),
        "expected orchestrate/generic.yaml in presets"
    );
}

#[test]
fn get_preset_file_returns_contents() {
    let file = get_preset_file("orchestrate/generic.yaml").expect("preset should exist");
    let text = std::str::from_utf8(file).expect("preset should be utf8");
    assert!(text.contains("name:"));
}

#[test]
fn loop_skill_template_includes_yaml_frontmatter() {
    let file = get_skill_file("ito-loop/SKILL.md").expect("loop skill should exist");
    let text = std::str::from_utf8(file).expect("skill should be utf8");
    assert!(text.starts_with("---\nname: ito-loop\n"));
    assert!(text.contains("description: Run bounded Ito Ralph iteration"));
    assert!(text.contains("at most two outer restarts"));
    assert!(text.contains("ito ralph --no-interactive --harness <harness> --change <change-id>"));
    assert!(text.contains("\n---\n\n<!-- ITO:START -->"));
}

#[test]
fn loop_command_template_uses_ito_loop_command_name() {
    let file = commands_files()
        .into_iter()
        .find(|f| f.relative_path == "ito-loop.md")
        .expect("loop command should exist");
    let text = std::str::from_utf8(file.contents).expect("command should be utf8");
    assert!(text.starts_with("---\nname: ito-loop\n"));
    assert!(text.contains("/ito-loop"));
    assert!(text.contains("continue ready work across the repo"));
}

#[test]
fn tmux_skill_and_scripts_are_not_embedded() {
    assert!(get_skill_file("ito-tmux/SKILL.md").is_none());
    assert!(get_skill_file("ito-tmux/scripts/wait-for-text.sh").is_none());
    assert!(get_skill_file("ito-tmux/scripts/find-sessions.sh").is_none());
    assert!(
        skills_files()
            .iter()
            .all(|file| !file.relative_path.contains("tmux")),
        "the embedded skill inventory must contain no tmux assets"
    );
}

#[test]
fn command_inventory_contains_only_lifecycle_entrypoints() {
    let actual = commands_files()
        .into_iter()
        .map(|file| file.relative_path)
        .collect::<BTreeSet<_>>();
    let expected = [
        "ito.md",
        "ito-proposal.md",
        "ito-research.md",
        "ito-apply.md",
        "ito-review.md",
        "ito-archive.md",
        "ito-loop.md",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

#[test]
fn orchestration_is_owned_by_loop_without_helper_surfaces() {
    assert!(get_skill_file("ito-orchestrate/SKILL.md").is_none());
    assert!(get_skill_file("ito-orchestrate-setup/SKILL.md").is_none());
    assert!(get_skill_file("ito-orchestrator-workflow/SKILL.md").is_none());
    assert!(get_command_file("ito-orchestrate.md").is_none());

    let loop_skill = get_skill_file("ito-loop/SKILL.md").expect("loop skill");
    let loop_skill = std::str::from_utf8(loop_skill).expect("utf8");
    assert!(loop_skill.contains("ito agent instruction orchestrate"));
    assert!(loop_skill.contains(".ito/user-prompts/orchestrate.md"));
}

#[test]
fn default_project_includes_orchestrate_user_prompt() {
    let files = default_project_files();
    assert!(
        files
            .iter()
            .any(|f| f.relative_path == ".ito/user-prompts/orchestrate.md"),
        "expected default project orchestrate.md prompt stub"
    );
}

#[test]
fn orchestrator_agent_templates_are_embedded_for_all_harnesses() {
    use crate::agents::{Harness, get_agent_files};

    for harness in Harness::all() {
        let files = get_agent_files(*harness);
        if *harness == Harness::Codex {
            assert!(files.is_empty(), "Codex roles must not become skills");
            continue;
        }
        if *harness == Harness::OpenCode {
            assert!(
                files.iter().any(|(name, _)| *name == "ito-orchestrator.md"),
                "expected ito-orchestrator.md in OpenCode agent templates"
            );
        }

        let expected = match harness {
            Harness::OpenCode | Harness::ClaudeCode | Harness::GitHubCopilot | Harness::Pi => [
                "ito-orchestrator.md",
                "ito-planner.md",
                "ito-researcher.md",
                "ito-worker.md",
                "ito-reviewer.md",
            ],
            Harness::Codex => unreachable!(),
        };

        for expected in expected {
            assert!(
                files.iter().any(|(name, _)| *name == expected),
                "expected {expected} in {harness:?} agent templates"
            );
        }
    }
}

#[test]
fn proposal_owns_intake_fix_feature_and_planning() {
    for retired in [
        "ito-proposal-intake/SKILL.md",
        "ito-fix/SKILL.md",
        "ito-feature/SKILL.md",
        "ito-plan/SKILL.md",
    ] {
        assert!(get_skill_file(retired).is_none(), "unexpected {retired}");
    }
    assert!(get_command_file("ito-plan.md").is_none());

    let proposal = get_skill_file("ito-proposal/SKILL.md").expect("proposal skill");
    let proposal = std::str::from_utf8(proposal).expect("utf8");
    for guidance in ["intake", "feature/fix", "brainstorming", "ito plan init"] {
        assert!(proposal.contains(guidance), "missing {guidance}");
    }
}

#[test]
fn memory_operations_are_owned_by_research_and_archive() {
    assert!(get_skill_file("ito-memory/SKILL.md").is_none());
    let research = get_skill_file("ito-research/SKILL.md").expect("research skill");
    let research = std::str::from_utf8(research).expect("utf8");
    assert!(research.contains("ito agent instruction memory-search"));
    assert!(research.contains("ito agent instruction memory-query"));

    let archive = get_skill_file("ito-archive/SKILL.md").expect("archive skill");
    let archive = std::str::from_utf8(archive).expect("utf8");
    assert!(archive.contains("ito agent instruction memory-capture"));
}

#[test]
fn default_project_agents_mentions_lifecycle_and_artifact_guidance() {
    let agents = default_project_files()
        .into_iter()
        .find(|f| f.relative_path == ".ito/AGENTS.md")
        .expect("expected .ito/AGENTS.md in templates");
    let text = std::str::from_utf8(agents.contents).expect("template should be UTF-8");

    assert!(text.contains("`ito-proposal`"));
    assert!(text.contains("`ito-research`"));
    assert!(text.contains("integrated into main"));
    assert!(text.contains("ito patch change <id> proposal"));
    assert!(text.contains("ito write change <id> design"));
}

#[test]
fn agent_templates_remind_harnesses_to_use_ito_patch_and_write_for_active_artifacts() {
    use crate::agents::{Harness, get_agent_files};

    let expected = ["ito-general", "ito-quick", "ito-thinking", "ito-worker"];

    for harness in Harness::all() {
        let files = get_agent_files(*harness);
        if *harness == Harness::Codex {
            assert!(files.is_empty(), "Codex roles must not become skills");
            continue;
        }
        for name in expected {
            let path = format!("{name}.md");
            let contents = files
                .iter()
                .find(|(file_name, _)| *file_name == path)
                .unwrap_or_else(|| {
                    panic!("missing agent template {path} for harness {:?}", harness)
                })
                .1;
            let text = std::str::from_utf8(contents).expect("template should be utf8");
            assert!(
                text.contains("ito patch"),
                "missing ito patch guidance in {path}"
            );
            assert!(
                text.contains("ito write"),
                "missing ito write guidance in {path}"
            );
        }
    }
}

#[test]
fn normalize_ito_dir_empty_defaults_to_dot_ito() {
    assert_eq!(normalize_ito_dir(""), ".ito");
}

#[test]
fn normalize_ito_dir_rejects_traversal_and_path_separators() {
    assert_eq!(normalize_ito_dir("../escape"), ".ito");
    assert_eq!(normalize_ito_dir("a/b"), ".ito");
    assert_eq!(normalize_ito_dir("a\\b"), ".ito");
}

#[test]
fn render_bytes_returns_borrowed_when_no_rewrite_needed() {
    let b = b"see .ito/AGENTS.md";
    let out = render_bytes(b, ".ito");
    assert_eq!(out.as_ref(), b);

    let b = b"no ito path";
    let out = render_bytes(b, ".x");
    assert_eq!(out.as_ref(), b);
}

#[test]
fn render_bytes_preserves_non_utf8() {
    let b = [0xff, 0x00, 0x41];
    let out = render_bytes(&b, ".x");
    assert_eq!(out.as_ref(), &b);
}

#[test]
fn extract_managed_block_rejects_inline_markers() {
    let s = "pre <!-- ITO:START -->\nhello\n<!-- ITO:END -->\n";
    assert_eq!(extract_managed_block(s), None);
}

#[test]
fn extract_managed_block_returns_empty_for_empty_inner() {
    let s = "<!-- ITO:START -->\n<!-- ITO:END -->\n";
    assert_eq!(extract_managed_block(s), Some(""));
}

// -------- stamp_version --------

#[test]
fn stamp_version_noop_without_marker() {
    let s = "no markers here\n";
    assert_eq!(stamp_version(s, "1.2.3"), s);
}

#[test]
fn stamp_version_inserts_when_missing() {
    let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3");
    assert_eq!(
        out,
        "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
    );
}

#[test]
fn stamp_version_idempotent_on_canonical_match() {
    let s = "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n";
    assert_eq!(stamp_version(s, "1.2.3"), s);
}

#[test]
fn stamp_version_rewrites_spaced_form_to_canonical() {
    let s = "<!-- ITO:START -->\n<!-- ITO:VERSION: 1.2.3 -->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3");
    assert_eq!(
        out,
        "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
    );
}

#[test]
fn stamp_version_rewrites_older_version() {
    let s = "<!-- ITO:START -->\n<!--ITO:VERSION:0.9.0-->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3");
    assert_eq!(
        out,
        "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
    );
}

#[test]
fn stamp_version_preserves_frontmatter() {
    let s = "---\nname: foo\n---\n\n<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3");
    assert_eq!(
        out,
        "---\nname: foo\n---\n\n<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
    );
}

#[test]
fn stamp_version_preserves_trailing_content() {
    let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\nepilogue line\n";
    let out = stamp_version(s, "9.9.9");
    assert!(out.ends_with("<!-- ITO:END -->\nepilogue line\n"));
    assert!(out.contains("<!--ITO:VERSION:9.9.9-->"));
}

#[test]
fn stamp_version_handles_prerelease_semver() {
    let s = "<!-- ITO:START -->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3-asd");
    assert!(out.contains("<!--ITO:VERSION:1.2.3-asd-->"));
}

#[test]
fn stamp_version_idempotent_on_canonical_with_trailing_whitespace() {
    // Trailing CR / spaces / tabs after the canonical stamp should still
    // be treated as canonical so the file is not rewritten.
    let s = "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->  \nbody\n<!-- ITO:END -->\n";
    assert_eq!(stamp_version(s, "1.2.3"), s);
}

#[test]
fn stamp_version_canonical_with_leading_whitespace_is_rewritten() {
    // Leading whitespace makes the line non-canonical even if the version
    // matches; the writer normalises to the tight form on next run.
    let s = "<!-- ITO:START -->\n  <!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n";
    let out = stamp_version(s, "1.2.3");
    assert_eq!(
        out,
        "<!-- ITO:START -->\n<!--ITO:VERSION:1.2.3-->\nbody\n<!-- ITO:END -->\n"
    );
    // And then it's stable.
    assert_eq!(stamp_version(&out, "1.2.3"), out);
}

#[test]
fn stamp_version_handles_crlf_line_endings() {
    let s = "<!-- ITO:START -->\r\nbody\r\n<!-- ITO:END -->\r\n";
    let out = stamp_version(s, "1.2.3");
    assert!(out.contains("<!--ITO:VERSION:1.2.3-->"));
    // Re-stamp must be a no-op even though the surrounding line endings
    // are CRLF.
    assert_eq!(stamp_version(&out, "1.2.3"), out);
}

#[test]
fn stamp_version_round_trip_on_real_skill() {
    let bytes = get_skill_file("ito-proposal/SKILL.md").expect("ito-proposal skill exists");
    let text = std::str::from_utf8(bytes).expect("skill is utf8");
    let stamped = stamp_version(text, "1.2.3");
    let restamped = stamp_version(&stamped, "1.2.3");
    assert_eq!(stamped, restamped, "stamping must be idempotent");
    assert!(stamped.contains("<!--ITO:VERSION:1.2.3-->"));
    assert_eq!(
        stamped.matches("<!--ITO:VERSION:").count(),
        1,
        "exactly one stamp must be present"
    );
}

// -------- bundle invariants --------

#[test]
fn every_shipped_skill_has_ito_prefix() {
    let mut violations: Vec<&'static str> = Vec::new();
    for f in skills_files() {
        let Some(top) = f.relative_path.split('/').next() else {
            continue;
        };
        if top == "ito" || top.starts_with("ito-") {
            continue;
        }
        violations.push(f.relative_path);
    }
    assert!(
        violations.is_empty(),
        "skills missing `ito-` prefix: {violations:?}"
    );
}

#[test]
fn every_shipped_command_has_ito_prefix() {
    let mut violations: Vec<&'static str> = Vec::new();
    for f in commands_files() {
        let Some(name) = f.relative_path.split('/').next_back() else {
            continue;
        };
        let stem = name.strip_suffix(".md").unwrap_or(name);
        if stem == "ito" || stem.starts_with("ito-") {
            continue;
        }
        violations.push(f.relative_path);
    }
    assert!(
        violations.is_empty(),
        "commands missing `ito-` prefix: {violations:?}"
    );
}

#[test]
fn every_shipped_agent_has_ito_prefix() {
    let mut violations: Vec<String> = Vec::new();
    let agent_dirs = AGENTS_DIR.dirs();
    for harness_dir in agent_dirs {
        for entry_file in harness_dir.files() {
            let Some(name) = entry_file.path().file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            let stem = name
                .strip_suffix(".md")
                .or_else(|| name.strip_suffix(".md.j2"))
                .unwrap_or(name);
            if stem.starts_with("ito-") {
                continue;
            }
            violations.push(entry_file.path().display().to_string());
        }
        for nested in harness_dir.dirs() {
            let Some(name) = nested.path().file_name().and_then(|s| s.to_str()) else {
                continue;
            };
            if name.starts_with("ito-") {
                continue;
            }
            violations.push(nested.path().display().to_string());
        }
    }
    assert!(
        violations.is_empty(),
        "agents missing `ito-` prefix: {violations:?}"
    );
}

fn count_marker_on_own_line(text: &str, marker: &str) -> usize {
    let mut from = 0;
    let mut count = 0;
    while let Some(idx) = find_marker_index(text, marker, from) {
        count += 1;
        from = idx + marker.len();
    }
    count
}

#[test]
fn every_shipped_markdown_has_exactly_one_marker_pair() {
    let mut violations: Vec<String> = Vec::new();
    let mut all = Vec::new();
    all.extend(default_project_files());
    all.extend(default_home_files());
    all.extend(skills_files());
    all.extend(adapters_files());
    all.extend(commands_files());
    all.extend(schema_files());
    all.extend(presets_files());
    all.extend(dir_files(&AGENTS_DIR));
    for f in all {
        if !f.relative_path.ends_with(".md") {
            continue;
        }
        let Ok(text) = std::str::from_utf8(f.contents) else {
            continue;
        };
        let starts = count_marker_on_own_line(text, ITO_START_MARKER);
        let ends = count_marker_on_own_line(text, ITO_END_MARKER);
        if starts != 1 || ends != 1 {
            violations.push(format!("{}: starts={starts} ends={ends}", f.relative_path));
        }
    }
    assert!(
        violations.is_empty(),
        "expected exactly one ITO:START and one ITO:END per shipped markdown:\n  {}",
        violations.join("\n  ")
    );
}

#[test]
fn every_shipped_markdown_has_managed_markers() {
    let mut missing_start: Vec<&'static str> = Vec::new();
    let mut missing_end: Vec<&'static str> = Vec::new();

    let bundles: [&[EmbeddedFile]; 0] = [];
    let _ = bundles;

    let collect = || -> Vec<EmbeddedFile> {
        let mut all = Vec::new();
        all.extend(default_project_files());
        all.extend(default_home_files());
        all.extend(skills_files());
        all.extend(adapters_files());
        all.extend(commands_files());
        all.extend(schema_files());
        all.extend(presets_files());
        all.extend(dir_files(&AGENTS_DIR));
        all
    };

    for f in collect() {
        if !f.relative_path.ends_with(".md") {
            continue;
        }
        let Ok(text) = std::str::from_utf8(f.contents) else {
            continue;
        };
        if find_marker_index(text, ITO_START_MARKER, 0).is_none() {
            missing_start.push(f.relative_path);
        }
        if find_marker_index(text, ITO_END_MARKER, 0).is_none() {
            missing_end.push(f.relative_path);
        }
    }

    assert!(
        missing_start.is_empty() && missing_end.is_empty(),
        "markdown assets missing managed markers — start: {missing_start:?}, end: {missing_end:?}"
    );
}
