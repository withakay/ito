use super::{default_project_files, get_skill_file};

#[test]
fn proposal_skill_mentions_wiki_consultation() {
    let proposal = get_skill_file("ito-proposal/SKILL.md").expect("proposal skill should exist");
    let proposal = std::str::from_utf8(proposal).expect("skill should be utf8");
    assert!(proposal.contains("Inspect relevant brownfield specs"));
    assert!(proposal.contains(".ito/wiki/index.md"));
}

#[test]
fn research_and_archive_skills_include_wiki_follow_up() {
    let research = get_skill_file("ito-research/SKILL.md").expect("research skill should exist");
    let research = std::str::from_utf8(research).expect("skill should be utf8");
    assert!(research.contains("synthesized navigation"));
    assert!(research.contains("wiki topic/query artifacts"));

    let synthesize = get_skill_file("ito-research/research-synthesize.md")
        .expect("research synthesize template should exist");
    let synthesize = std::str::from_utf8(synthesize).expect("template should be utf8");
    assert!(synthesize.contains("## Wiki Follow-Up"));
    assert!(synthesize.contains("Cite this research summary from the wiki"));

    let archive = get_skill_file("ito-archive/SKILL.md").expect("archive skill should exist");
    let archive = std::str::from_utf8(archive).expect("skill should be utf8");
    assert!(archive.contains("refresh relevant `.ito/wiki/` topic/index/status material"));
    assert!(archive.contains("must not hide an archive failure"));
}

#[test]
fn wiki_skills_are_consolidated_into_lifecycle_phases() {
    assert!(get_skill_file("ito-wiki/SKILL.md").is_none());
    assert!(get_skill_file("ito-wiki-search/SKILL.md").is_none());

    let research = get_skill_file("ito-research/SKILL.md").expect("research skill");
    let research = std::str::from_utf8(research).expect("skill should be utf8");
    assert!(research.contains("Read `.ito/wiki/index.md`"));

    let archive = get_skill_file("ito-archive/SKILL.md").expect("archive skill");
    let archive = std::str::from_utf8(archive).expect("skill should be utf8");
    assert!(archive.contains("refresh relevant `.ito/wiki/`"));
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
