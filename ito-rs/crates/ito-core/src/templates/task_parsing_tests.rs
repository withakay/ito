use super::parse_enhanced_tasks;

#[test]
fn parse_enhanced_tasks_extracts_ids_status_and_done() {
    let contents = r#"### Task 1.1: First
 - **Status**: [x] complete

 ### Task 1.2: Second
 - **Status**: [>] in-progress
 "#;

    let tasks = parse_enhanced_tasks(contents);
    assert_eq!(tasks.len(), 2);
    assert_eq!(tasks[0].id, "1.1");
    assert_eq!(tasks[0].description, "First");
    assert!(tasks[0].done);
    assert_eq!(tasks[0].status.as_deref(), Some("complete"));

    assert_eq!(tasks[1].id, "1.2");
    assert_eq!(tasks[1].description, "Second");
    assert!(!tasks[1].done);
    assert_eq!(tasks[1].status.as_deref(), Some("in-progress"));
}
