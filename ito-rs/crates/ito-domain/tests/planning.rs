use ito_domain::planning;

#[test]
fn planning_paths_point_to_flexible_workspaces() {
    let root = std::path::Path::new(".ito");

    assert_eq!(planning::planning_dir(root), root.join("planning"));
    assert_eq!(planning::research_dir(root), root.join("research"));
}
