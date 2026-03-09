use assert_struct::assert_struct;
use ito_core::tasks::{
    TaskStatus, TaskStatusSummary, TasksFormat, get_next_task_from_summary,
    get_task_status_from_repository,
};
use ito_domain::errors::DomainResult;
use ito_domain::tasks::{TaskRepository, TasksParseResult, parse_tasks_tracking_file};

struct FakeTaskRepo {
    parsed: TasksParseResult,
}

impl TaskRepository for FakeTaskRepo {
    fn load_tasks(&self, _change_id: &str) -> DomainResult<TasksParseResult> {
        Ok(self.parsed.clone())
    }
}

#[test]
fn repository_status_builds_summary_and_next_task() {
    let parsed = parse_tasks_tracking_file("# Tasks\n- [ ] First\n- [x] Done\n");
    let repo = FakeTaskRepo { parsed };

    let summary = get_task_status_from_repository(&repo, "025-02_demo").expect("status from repo");

    assert_struct!(summary, TaskStatusSummary {
        format: TasksFormat::Checkbox,
        progress.total: 2,
        progress.complete: 1,
        ..
    });
    assert!(!summary.ready.is_empty(), "expected ready tasks");

    let next = get_next_task_from_summary(&summary, "backend tasks")
        .expect("next from summary")
        .expect("next task");

    assert_struct!(
        next,
        ito_core::tasks::TaskItem {
            id: "1",
            status: TaskStatus::Pending,
            ..
        }
    );
}
