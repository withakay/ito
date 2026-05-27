use super::{default_project_files, get_skill_file};

#[test]
fn proposal_skill_mentions_wiki_consultation() {
    let proposal = get_skill_file("ito-proposal/SKILL.md").expect("proposal skill should exist");
    let proposal = std::str::from_utf8(proposal).expect("skill should be utf8");
    assert!(proposal.contains("**Step 0.5: Consult the Ito wiki when present**"));
    assert!(proposal.contains(".ito/wiki/index.md"));
}

#[test]
fn research_and_archive_skills_include_wiki_follow_up() {
    let research = get_skill_file("ito-research/SKILL.md").expect("research skill should exist");
    let research = std::str::from_utf8(research).expect("skill should be utf8");
    assert!(research.contains("Research source artifacts and wiki synthesis have different jobs"));
    assert!(research.contains("$ITO_ROOT/wiki/queries/"));

    let synthesize = get_skill_file("ito-research/research-synthesize.md")
        .expect("research synthesize template should exist");
    let synthesize = std::str::from_utf8(synthesize).expect("template should be utf8");
    assert!(synthesize.contains("## Wiki Follow-Up"));
    assert!(synthesize.contains("Cite this research summary from the wiki"));

    let archive = get_skill_file("ito-archive/SKILL.md").expect("archive skill should exist");
    let archive = std::str::from_utf8(archive).expect("skill should be utf8");
    assert!(archive.contains("refresh relevant `.ito/wiki/` topic pages"));
    assert!(archive.contains("not an archive blocker"));
}

#[test]
fn wiki_skills_are_embedded() {
    let wiki = get_skill_file("ito-wiki/SKILL.md").expect("ito-wiki skill should exist");
    let wiki = std::str::from_utf8(wiki).expect("skill should be utf8");
    assert!(wiki.starts_with("---\nname: ito-wiki\n"));
    assert!(wiki.contains("## Maintenance Workflow"));
    assert!(wiki.contains("## Lint Checklist"));

    let search =
        get_skill_file("ito-wiki-search/SKILL.md").expect("ito-wiki-search skill should exist");
    let search = std::str::from_utf8(search).expect("skill should be utf8");
    assert!(search.starts_with("---\nname: ito-wiki-search\n"));
    assert!(search.contains("## Search Workflow"));
    assert!(search.contains("## Answer Rules"));
}

#[test]
fn default_project_agents_mentions_wiki_guidance() {
    let agents = default_project_files()
        .into_iter()
        .find(|f| f.relative_path == ".ito/AGENTS.md")
        .expect("expected .ito/AGENTS.md in templates");
    let text = std::str::from_utf8(agents.contents).expect("template should be UTF-8");

    assert!(text.contains(".ito/wiki/index.md"));
    assert!(text.contains("Refresh relevant `.ito/wiki/` topic pages"));
}
