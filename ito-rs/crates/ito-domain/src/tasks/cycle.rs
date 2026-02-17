//! Graph cycle detection.
//!
//! Uses in-memory SQLite recursive CTEs to detect cycles in task or wave dependency graphs.

use rusqlite::OptionalExtension;

/// Detect a cycle in a graph of edges and return the cycle path.
///
/// Returns a formatted string `a -> b -> c -> a` if a cycle is found, or `None`.
/// Uses SQLite recursive CTEs for efficient path finding.
pub(super) fn find_cycle_path(edges: &[(String, String)]) -> Option<String> {
    if edges.is_empty() {
        return None;
    }

    let mut conn = rusqlite::Connection::open_in_memory().ok()?;
    conn.execute(
        "CREATE TABLE edge (src TEXT NOT NULL, dst TEXT NOT NULL);",
        [],
    )
    .ok()?;

    {
        let tx = conn.transaction().ok()?;
        {
            let mut stmt = tx
                .prepare("INSERT INTO edge (src, dst) VALUES (?1, ?2);")
                .ok()?;
            for (src, dst) in edges {
                stmt.execute(rusqlite::params![src, dst]).ok()?;
            }
        }
        tx.commit().ok()?;
    }

    // Detect a cycle and return a delimited path like: |a|b|c|a|
    let sql = r#"
  WITH RECURSIVE
    walk(start, current, path) AS (
      SELECT src, dst, '|' || src || '|' || dst || '|'
      FROM edge
      UNION ALL
      SELECT w.start, e.dst, w.path || e.dst || '|'
      FROM walk w
      JOIN edge e ON e.src = w.current
      WHERE w.current != w.start
        AND (
          e.dst = w.start
          OR instr(w.path, '|' || e.dst || '|') = 0
        )
    )
  SELECT path
  FROM walk
  WHERE start = current
  LIMIT 1;
  "#;

    let mut stmt = conn.prepare(sql).ok()?;
    let path: Option<String> = stmt.query_row([], |row| row.get(0)).optional().ok()?;
    path.map(|p| p.trim_matches('|').replace('|', " -> "))
}

#[cfg(test)]
#[path = "cycle_tests.rs"]
mod cycle_tests;