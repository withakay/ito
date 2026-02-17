//! Relational validation for task and wave dependencies.
//!
//! Validates invariants like:
//! - No dependency cycles (tasks or waves).
//! - Dependencies must exist.
//! - Tasks cannot depend on tasks in other waves.
//! - Tasks cannot depend on shelved tasks (unless shelved themselves).
use std::collections::{BTreeMap, BTreeSet};

use super::cycle::find_cycle_path;
use super::{DiagnosticLevel, TaskDiagnostic, TaskItem, WaveInfo};

/// Validate relational invariants using an in-memory SQLite database.
///
/// Returns a list of diagnostics for any violations found.
pub(super) fn validate_relational(tasks: &[TaskItem], waves: &[WaveInfo]) -> Vec<TaskDiagnostic> {
    let mut diagnostics: Vec<TaskDiagnostic> = Vec::new();

    let Ok(conn) = rusqlite::Connection::open_in_memory() else {
        diagnostics.push(TaskDiagnostic {
            level: DiagnosticLevel::Error,
            message: "Relational validation failed: could not open in-memory SQLite".to_string(),
            task_id: None,
            line: None,
        });
        return diagnostics;
    };
    if conn.execute_batch("PRAGMA foreign_keys=ON;").is_err() {
        diagnostics.push(TaskDiagnostic {
            level: DiagnosticLevel::Error,
            message: "Relational validation failed: could not enable foreign_keys".to_string(),
            task_id: None,
            line: None,
        });
        return diagnostics;
    }

    if conn
        .execute_batch(
            r#"
CREATE TABLE wave (
  wave_num INTEGER PRIMARY KEY,
  source_line INTEGER NOT NULL
);

CREATE TABLE wave_dep (
  wave_num INTEGER NOT NULL REFERENCES wave(wave_num),
  dep_wave_num INTEGER NOT NULL REFERENCES wave(wave_num),
  source_line INTEGER NOT NULL,
  PRIMARY KEY (wave_num, dep_wave_num),
  CHECK (wave_num <> dep_wave_num)
);

CREATE TABLE task (
  id TEXT PRIMARY KEY,
  wave_num INTEGER NULL REFERENCES wave(wave_num),
  status TEXT NOT NULL CHECK (status IN ('pending','in-progress','complete','shelved')),
  source_line INTEGER NOT NULL
);

CREATE TABLE task_dep (
  task_id TEXT NOT NULL REFERENCES task(id),
  dep_task_id TEXT NOT NULL REFERENCES task(id),
  source_line INTEGER NOT NULL,
  PRIMARY KEY (task_id, dep_task_id),
  CHECK (task_id <> dep_task_id)
);
"#,
        )
        .is_err()
    {
        diagnostics.push(TaskDiagnostic {
            level: DiagnosticLevel::Error,
            message: "Relational validation failed: could not create SQLite schema".to_string(),
            task_id: None,
            line: None,
        });
        return diagnostics;
    }

    let wave_set: BTreeSet<u32> = waves.iter().map(|w| w.wave).collect();

    for w in waves {
        let source_line = (w.header_line_index + 1) as i64;
        let _ = conn.execute(
            "INSERT OR IGNORE INTO wave (wave_num, source_line) VALUES (?1, ?2)",
            rusqlite::params![w.wave as i64, source_line],
        );
    }

    for w in waves {
        let wave_num = w.wave as i64;
        let source_line = (w.depends_on_line_index.unwrap_or(w.header_line_index) + 1) as i64;
        for dep in &w.depends_on {
            if !wave_set.contains(dep) {
                continue;
            }
            if *dep == w.wave {
                continue;
            }
            let _ = conn.execute(
                "INSERT OR IGNORE INTO wave_dep (wave_num, dep_wave_num, source_line) VALUES (?1, ?2, ?3)",
                rusqlite::params![wave_num, *dep as i64, source_line],
            );
        }
    }

    let mut by_id: BTreeMap<&str, &TaskItem> = BTreeMap::new();
    for t in tasks {
        if by_id.contains_key(t.id.as_str()) {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Duplicate task id: {}", t.id),
                task_id: Some(t.id.clone()),
                line: Some(t.header_line_index + 1),
            });
            continue;
        }
        by_id.insert(t.id.as_str(), t);

        let wave_num = t.wave.map(|w| w as i64);
        let status = t.status.as_enhanced_label();
        let source_line = (t.header_line_index + 1) as i64;
        if conn
            .execute(
                "INSERT INTO task (id, wave_num, status, source_line) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![t.id, wave_num, status, source_line],
            )
            .is_err()
        {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Duplicate task id: {}", t.id),
                task_id: Some(t.id.clone()),
                line: Some(t.header_line_index + 1),
            });
        }
    }

    for t in tasks {
        let source_line = (t.header_line_index + 1) as i64;
        for dep in &t.dependencies {
            if dep.is_empty() || dep == "Checkpoint" {
                continue;
            }

            if dep == &t.id {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: "Task cannot depend on itself".to_string(),
                    task_id: Some(t.id.clone()),
                    line: Some(t.header_line_index + 1),
                });
                continue;
            }

            if !by_id.contains_key(dep.as_str()) {
                diagnostics.push(TaskDiagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Missing dependency: {dep}"),
                    task_id: Some(t.id.clone()),
                    line: Some(t.header_line_index + 1),
                });
                continue;
            }

            let _ = conn.execute(
                "INSERT OR IGNORE INTO task_dep (task_id, dep_task_id, source_line) VALUES (?1, ?2, ?3)",
                rusqlite::params![t.id, dep, source_line],
            );
        }
    }

    // Cross-wave task deps.
    {
        let Ok(mut stmt) = conn.prepare(
            r#"
SELECT td.task_id, td.dep_task_id, t.source_line
FROM task_dep td
JOIN task t ON t.id = td.task_id
JOIN task d ON d.id = td.dep_task_id
WHERE t.wave_num IS NOT d.wave_num
"#,
        ) else {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: "Relational validation failed: could not prepare cross-wave query"
                    .to_string(),
                task_id: None,
                line: None,
            });
            return diagnostics;
        };

        let Ok(mut rows) = stmt.query([]) else {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: "Relational validation failed: could not run cross-wave query".to_string(),
                task_id: None,
                line: None,
            });
            return diagnostics;
        };

        while let Ok(Some(row)) = rows.next() {
            let Ok(task_id) = row.get::<_, String>(0) else {
                break;
            };
            let Ok(dep_task_id) = row.get::<_, String>(1) else {
                break;
            };
            let Ok(line) = row.get::<_, i64>(2) else {
                break;
            };
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!(
                    "Cross-wave dependency not allowed: {} depends on {}",
                    task_id, dep_task_id
                ),
                task_id: Some(task_id),
                line: Some(line.max(1) as usize),
            });
        }
    }

    // No deps on shelved tasks (unless the depender itself is shelved).
    {
        let Ok(mut stmt) = conn.prepare(
            r#"
SELECT td.task_id, td.dep_task_id, t.source_line
FROM task_dep td
JOIN task t ON t.id = td.task_id
JOIN task d ON d.id = td.dep_task_id
WHERE t.status <> 'shelved' AND d.status = 'shelved'
"#,
        ) else {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: "Relational validation failed: could not prepare shelved-dep query"
                    .to_string(),
                task_id: None,
                line: None,
            });
            return diagnostics;
        };

        let Ok(mut rows) = stmt.query([]) else {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: "Relational validation failed: could not run shelved-dep query"
                    .to_string(),
                task_id: None,
                line: None,
            });
            return diagnostics;
        };

        while let Ok(Some(row)) = rows.next() {
            let Ok(task_id) = row.get::<_, String>(0) else {
                break;
            };
            let Ok(dep_task_id) = row.get::<_, String>(1) else {
                break;
            };
            let Ok(line) = row.get::<_, i64>(2) else {
                break;
            };
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Dependency is shelved: {dep_task_id}"),
                task_id: Some(task_id),
                line: Some(line.max(1) as usize),
            });
        }
    }

    // Cycle detection.
    {
        let mut edges: Vec<(String, String)> = Vec::new();
        let Ok(mut stmt) = conn.prepare("SELECT task_id, dep_task_id FROM task_dep") else {
            return diagnostics;
        };
        let Ok(mut rows) = stmt.query([]) else {
            return diagnostics;
        };
        while let Ok(Some(row)) = rows.next() {
            let Ok(src) = row.get::<_, String>(0) else {
                break;
            };
            let Ok(dst) = row.get::<_, String>(1) else {
                break;
            };
            edges.push((src, dst));
        }

        if let Some(path) = find_cycle_path(&edges) {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Dependency cycle detected: {path}"),
                task_id: None,
                line: None,
            });
        }
    }

    {
        let mut edges: Vec<(String, String)> = Vec::new();
        let Ok(mut stmt) = conn.prepare("SELECT wave_num, dep_wave_num FROM wave_dep") else {
            return diagnostics;
        };
        let Ok(mut rows) = stmt.query([]) else {
            return diagnostics;
        };
        while let Ok(Some(row)) = rows.next() {
            let Ok(src) = row.get::<_, i64>(0) else { break };
            let Ok(dst) = row.get::<_, i64>(1) else { break };
            edges.push((src.to_string(), dst.to_string()));
        }

        if let Some(path) = find_cycle_path(&edges) {
            diagnostics.push(TaskDiagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Wave dependency cycle detected: {path}"),
                task_id: None,
                line: None,
            });
        }
    }

    diagnostics
}

#[cfg(test)]
#[path = "relational_tests.rs"]
mod relational_tests;
