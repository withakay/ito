//! Tests for cycle detection in task/wave dependencies.

use super::cycle::find_cycle_path;

#[test]
fn find_cycle_path_detects_simple_two_node_cycle() {
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "a".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    assert!(path.contains("a"));
    assert!(path.contains("b"));
    assert!(path.contains(" -> "));
}

#[test]
fn find_cycle_path_detects_three_node_cycle() {
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
        ("c".to_string(), "a".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    assert!(path.contains("a"));
    assert!(path.contains("b"));
    assert!(path.contains("c"));
}

#[test]
fn find_cycle_path_detects_self_loop() {
    let edges = vec![("a".to_string(), "a".to_string())];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    assert!(path.contains("a"));
}

#[test]
fn find_cycle_path_returns_none_for_acyclic_graph() {
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
        ("c".to_string(), "d".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_none());
}

#[test]
fn find_cycle_path_returns_none_for_empty_graph() {
    let edges: Vec<(String, String)> = vec![];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_none());
}

#[test]
fn find_cycle_path_detects_cycle_in_complex_graph() {
    // Graph with both cyclic and acyclic components
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "c".to_string()),
        ("c".to_string(), "d".to_string()),
        ("d".to_string(), "e".to_string()),
        ("e".to_string(), "c".to_string()), // Cycle: c -> d -> e -> c
        ("f".to_string(), "g".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    // The cycle should involve c, d, e
    assert!(path.contains("c"));
    assert!(path.contains("d"));
    assert!(path.contains("e"));
}

#[test]
fn find_cycle_path_handles_multiple_cycles_returns_one() {
    // Two separate cycles
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("b".to_string(), "a".to_string()), // Cycle 1: a <-> b
        ("c".to_string(), "d".to_string()),
        ("d".to_string(), "c".to_string()), // Cycle 2: c <-> d
    ];
    let cycle = find_cycle_path(&edges);
    // Should find at least one cycle
    assert!(cycle.is_some());
}

#[test]
fn find_cycle_path_handles_diamond_pattern_without_cycle() {
    // Diamond: a -> b, a -> c, b -> d, c -> d (no cycle)
    let edges = vec![
        ("a".to_string(), "b".to_string()),
        ("a".to_string(), "c".to_string()),
        ("b".to_string(), "d".to_string()),
        ("c".to_string(), "d".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_none());
}

#[test]
fn find_cycle_path_handles_long_cycle() {
    // Long cycle: 1 -> 2 -> 3 -> 4 -> 5 -> 1
    let edges = vec![
        ("1".to_string(), "2".to_string()),
        ("2".to_string(), "3".to_string()),
        ("3".to_string(), "4".to_string()),
        ("4".to_string(), "5".to_string()),
        ("5".to_string(), "1".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    assert!(path.contains("1"));
    assert!(path.contains("5"));
}

#[test]
fn find_cycle_path_with_numeric_node_names() {
    // Simulating wave dependencies
    let edges = vec![
        ("1".to_string(), "2".to_string()),
        ("2".to_string(), "3".to_string()),
        ("3".to_string(), "1".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
}

#[test]
fn find_cycle_path_handles_special_characters_in_node_names() {
    let edges = vec![
        ("task-1.1".to_string(), "task-1.2".to_string()),
        ("task-1.2".to_string(), "task-1.1".to_string()),
    ];
    let cycle = find_cycle_path(&edges);
    assert!(cycle.is_some());
    let path = cycle.unwrap();
    assert!(path.contains("task-1.1"));
    assert!(path.contains("task-1.2"));
}
