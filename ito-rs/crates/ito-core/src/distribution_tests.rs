use super::*;

#[test]
fn pi_manifests_includes_adapter_skills_and_commands() {
    let root = Path::new("/tmp/project");
    let manifests = pi_manifests(root);

    // Must contain the adapter extension.
    let adapter = manifests
        .iter()
        .find(|m| m.asset_type == AssetType::Adapter);
    assert!(adapter.is_some(), "pi_manifests must include the adapter");
    let adapter = adapter.unwrap();
    assert_eq!(adapter.source, "pi/ito-skills.ts");
    assert!(
        adapter.dest.ends_with(".pi/extensions/ito-skills.ts"),
        "adapter dest should end with .pi/extensions/ito-skills.ts, got {:?}",
        adapter.dest
    );

    // Must contain skill entries under .pi/skills/.
    let skills: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .collect();
    assert!(
        !skills.is_empty(),
        "pi_manifests must include skill entries"
    );
    for skill in &skills {
        let dest_str = skill.dest.to_string_lossy();
        assert!(
            dest_str.contains(".pi/skills/"),
            "skill dest should be under .pi/skills/, got: {}",
            dest_str
        );
    }

    // Must contain command entries under .pi/commands/.
    let commands: Vec<_> = manifests
        .iter()
        .filter(|m| m.asset_type == AssetType::Command)
        .collect();
    assert!(
        !commands.is_empty(),
        "pi_manifests must include command entries"
    );
    for cmd in &commands {
        let dest_str = cmd.dest.to_string_lossy();
        assert!(
            dest_str.contains(".pi/commands/"),
            "command dest should be under .pi/commands/, got: {}",
            dest_str
        );
    }
}

#[test]
fn pi_adapter_asset_exists_in_embedded_templates() {
    let contents = ito_templates::get_adapter_file("pi/ito-skills.ts");
    assert!(
        contents.is_some(),
        "pi/ito-skills.ts must be present in embedded adapter assets"
    );
    let bytes = contents.unwrap();
    assert!(!bytes.is_empty());
    let text = std::str::from_utf8(bytes).expect("adapter should be valid UTF-8");
    assert!(
        text.contains("ExtensionAPI"),
        "Pi adapter should import the Pi ExtensionAPI type"
    );
    assert!(
        text.contains(r#""--tool", "pi""#),
        "Pi adapter must request bootstrap with --tool pi (not opencode or other)"
    );
    assert!(
        !text.contains(r#""--tool", "opencode""#),
        "Pi adapter must not reference opencode tool type"
    );
    // Verify unused imports are not present.
    assert!(
        !text.contains("import path from"),
        "Pi adapter should not have unused path import"
    );
    assert!(
        !text.contains("import fs from"),
        "Pi adapter should not have unused fs import"
    );
}

#[test]
fn pi_manifests_skills_match_opencode_skills() {
    // Pi and OpenCode should install the same set of skills from the
    // shared embedded source — only the destination directory differs.
    let root = Path::new("/home/user/myproject");
    let pi = pi_manifests(root);
    let oc_dir = root.join(".opencode");
    let oc = opencode_manifests(&oc_dir);

    let pi_skill_sources: std::collections::BTreeSet<_> = pi
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .map(|m| m.source.clone())
        .collect();
    let oc_skill_sources: std::collections::BTreeSet<_> = oc
        .iter()
        .filter(|m| m.asset_type == AssetType::Skill)
        .map(|m| m.source.clone())
        .collect();

    assert_eq!(
        pi_skill_sources, oc_skill_sources,
        "Pi and OpenCode should install identical skill sources"
    );
}

#[test]
fn pi_agent_templates_discoverable() {
    use ito_templates::agents::{Harness, get_agent_files};
    let files = get_agent_files(Harness::Pi);
    let names: Vec<_> = files.iter().map(|(name, _)| *name).collect();
    assert!(
        names.contains(&"ito-quick.md"),
        "Pi agent templates must include ito-quick.md, got: {:?}",
        names
    );
    assert!(
        names.contains(&"ito-general.md"),
        "Pi agent templates must include ito-general.md, got: {:?}",
        names
    );
    assert!(
        names.contains(&"ito-thinking.md"),
        "Pi agent templates must include ito-thinking.md, got: {:?}",
        names
    );
}

#[test]
fn pi_manifests_commands_match_opencode_commands() {
    // Pi and OpenCode should install the same set of commands from the
    // shared embedded source — only the destination directory differs.
    let root = Path::new("/home/user/myproject");
    let pi = pi_manifests(root);
    let oc_dir = root.join(".opencode");
    let oc = opencode_manifests(&oc_dir);

    let pi_cmd_sources: std::collections::BTreeSet<_> = pi
        .iter()
        .filter(|m| m.asset_type == AssetType::Command)
        .map(|m| m.source.clone())
        .collect();
    let oc_cmd_sources: std::collections::BTreeSet<_> = oc
        .iter()
        .filter(|m| m.asset_type == AssetType::Command)
        .map(|m| m.source.clone())
        .collect();

    assert_eq!(
        pi_cmd_sources, oc_cmd_sources,
        "Pi and OpenCode should install identical command sources"
    );
}

#[cfg(unix)]
#[test]
fn ensure_manifest_script_is_executable_only_adds_execute_bits() {
    let td = tempfile::tempdir().unwrap();
    let dest = td.path().join("skills/demo/scripts/run.sh");
    std::fs::create_dir_all(dest.parent().unwrap()).unwrap();
    std::fs::write(&dest, "#!/usr/bin/env bash\n").unwrap();

    let mut permissions = std::fs::metadata(&dest).unwrap().permissions();
    permissions.set_mode(0o600);
    std::fs::set_permissions(&dest, permissions).unwrap();

    let manifest = FileManifest {
        source: "demo/scripts/run.sh".to_string(),
        dest: dest.clone(),
        asset_type: AssetType::Skill,
    };

    ensure_manifest_script_is_executable(&manifest).unwrap();

    let mode = std::fs::metadata(&dest).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o711);
}
