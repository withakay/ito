use super::TaskItem;

pub(super) fn parse_checkbox_tasks(contents: &str) -> Vec<TaskItem> {
    let mut tasks: Vec<TaskItem> = Vec::new();
    for line in contents.lines() {
        let l = line.trim_start();
        let bytes = l.as_bytes();
        if bytes.len() < 6 {
            continue;
        }
        let bullet = bytes[0] as char;
        if bullet != '-' && bullet != '*' {
            continue;
        }
        if bytes[1] != b' ' || bytes[2] != b'[' || bytes[4] != b']' || bytes[5] != b' ' {
            continue;
        }

        let marker = bytes[3] as char;
        let (done, rest, status) = match marker {
            'x' | 'X' => (true, &l[6..], None),
            ' ' => (false, &l[6..], None),
            '~' | '>' => (false, &l[6..], Some("in-progress".to_string())),
            _ => continue,
        };
        tasks.push(TaskItem {
            id: (tasks.len() + 1).to_string(),
            description: rest.trim().to_string(),
            done,
            status,
        });
    }
    tasks
}

pub(super) fn looks_like_enhanced_tasks(contents: &str) -> bool {
    for line in contents.lines() {
        let l = line.trim_start();
        if l.starts_with("### Task ") {
            return true;
        }
    }
    false
}

pub(super) fn parse_enhanced_tasks(contents: &str) -> Vec<TaskItem> {
    let mut tasks: Vec<TaskItem> = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_desc: Option<String> = None;
    let mut current_done = false;
    let mut current_status: Option<String> = None;

    fn push_current(
        tasks: &mut Vec<TaskItem>,
        current_id: &mut Option<String>,
        current_desc: &mut Option<String>,
        current_done: &mut bool,
        current_status: &mut Option<String>,
    ) {
        let Some(desc) = current_desc.take() else {
            current_id.take();
            *current_done = false;
            *current_status = None;
            return;
        };
        let id = current_id
            .take()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| (tasks.len() + 1).to_string());
        tasks.push(TaskItem {
            id,
            description: desc,
            done: *current_done,
            status: current_status.take(),
        });
        *current_done = false;
    }

    for line in contents.lines() {
        let l = line.trim_start();

        if let Some(rest) = l.strip_prefix("### Task ") {
            push_current(
                &mut tasks,
                &mut current_id,
                &mut current_desc,
                &mut current_done,
                &mut current_status,
            );

            let (id, desc) = rest.split_once(':').unwrap_or((rest, ""));
            let id = id.trim();
            let desc = if desc.trim().is_empty() {
                rest.trim()
            } else {
                desc.trim()
            };

            current_id = Some(id.to_string());
            current_desc = Some(desc.to_string());
            current_done = false;
            current_status = Some("pending".to_string());
            continue;
        }

        if let Some(rest) = l.strip_prefix("- **Status**:") {
            let status = rest.trim();
            if let Some(status) = status
                .strip_prefix("[x]")
                .or_else(|| status.strip_prefix("[X]"))
            {
                current_done = true;
                current_status = Some(status.trim().to_string());
                continue;
            }
            if let Some(status) = status
                .strip_prefix("[>]")
                .or_else(|| status.strip_prefix("[~]"))
            {
                current_done = false;
                current_status = Some(status.trim().to_string());
                continue;
            }
            if let Some(status) = status.strip_prefix("[ ]") {
                current_done = false;
                current_status = Some(status.trim().to_string());
                continue;
            }
            if let Some(status) = status.strip_prefix("[-]") {
                current_done = false;
                current_status = Some(status.trim().to_string());
                continue;
            }
        }
    }

    push_current(
        &mut tasks,
        &mut current_id,
        &mut current_desc,
        &mut current_done,
        &mut current_status,
    );

    tasks
}

#[cfg(test)]
mod tests {
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
}
