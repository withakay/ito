use chrono::{TimeZone, Utc};
use ito_core::orchestrate::{
    ChangePlanInput, GatePolicy, OrchestrateChangeState, OrchestrateEvent, OrchestrateEventKind,
    OrchestrateGateRecord, OrchestrateRun, OrchestrateRunConfig, OrchestrateRunStatus, PlannedGate,
    append_orchestrate_event, build_run_plan, generate_orchestrate_run_id,
    init_orchestrate_run_state, load_orchestrate_change_state, load_orchestrate_plan,
    load_orchestrate_run, parse_max_parallel, remaining_gates_for_change,
    write_orchestrate_change_state,
};
use ito_domain::changes::ChangeOrchestrateMetadata;
use serde_yaml::Value;
use tempfile::TempDir;

#[test]
fn orchestrate_run_state_creates_expected_layout() {
    let tmp = TempDir::new().expect("tmp");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create .ito");

    let run_id = "20260424-120000-deadbeef";
    let plan = build_run_plan(
        run_id,
        "generic",
        OrchestrateRunConfig::default(),
        Vec::new(),
    )
    .expect("plan");

    let run = OrchestrateRun {
        run_id: run_id.to_string(),
        started_at: "2026-04-24T12:00:00Z".to_string(),
        finished_at: None,
        status: OrchestrateRunStatus::Running,
        preset: "generic".to_string(),
        max_parallel: plan.max_parallel,
        failure_policy: plan.failure_policy,
    };

    init_orchestrate_run_state(&ito_path, &run, &plan).expect("init state");

    let root = ito_path
        .join(".state")
        .join("orchestrate")
        .join("runs")
        .join(run_id);
    assert!(root.join("run.json").exists());
    assert!(root.join("plan.json").exists());
    assert!(root.join("events.jsonl").exists());
    assert!(root.join("changes").is_dir());

    // Sanity: load back.
    let loaded_run = load_orchestrate_run(&ito_path, run_id).expect("load run");
    assert_eq!(loaded_run.run_id, run_id);
    let loaded_plan = load_orchestrate_plan(&ito_path, run_id).expect("load plan");
    assert_eq!(loaded_plan.run_id, run_id);
}

#[test]
fn orchestrate_event_log_appends_without_truncation() {
    let tmp = TempDir::new().expect("tmp");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create .ito");
    let run_id = "20260424-120001-deadbeef";

    let plan = build_run_plan(
        run_id,
        "generic",
        OrchestrateRunConfig::default(),
        Vec::new(),
    )
    .expect("plan");
    let run = OrchestrateRun {
        run_id: run_id.to_string(),
        started_at: "2026-04-24T12:00:01Z".to_string(),
        finished_at: None,
        status: OrchestrateRunStatus::Running,
        preset: "generic".to_string(),
        max_parallel: plan.max_parallel,
        failure_policy: plan.failure_policy,
    };
    init_orchestrate_run_state(&ito_path, &run, &plan).expect("init");

    let e1 = OrchestrateEvent {
        ts: "2026-04-24T12:00:02Z".to_string(),
        kind: OrchestrateEventKind::RunStart {
            run_id: run_id.to_string(),
            preset: "generic".to_string(),
            max_parallel: 4,
            failure_policy: plan.failure_policy,
        },
    };
    append_orchestrate_event(&ito_path, run_id, &e1).expect("append 1");
    let events_path = ito_path
        .join(".state")
        .join("orchestrate")
        .join("runs")
        .join(run_id)
        .join("events.jsonl");
    let first = std::fs::read_to_string(&events_path).expect("read");
    assert_eq!(first.lines().count(), 1);

    let e2 = OrchestrateEvent {
        ts: "2026-04-24T12:00:03Z".to_string(),
        kind: OrchestrateEventKind::RunComplete {
            run_id: run_id.to_string(),
            status: OrchestrateRunStatus::Complete,
        },
    };
    append_orchestrate_event(&ito_path, run_id, &e2).expect("append 2");

    let contents = std::fs::read_to_string(&events_path).expect("read");
    assert_eq!(contents.lines().count(), 2);
    assert!(
        contents.starts_with(first.trim()),
        "expected append-only log"
    );

    for line in contents.lines() {
        serde_json::from_str::<serde_json::Value>(line).expect("valid json line");
    }
}

#[test]
fn orchestrate_change_state_is_written_and_readable() {
    let tmp = TempDir::new().expect("tmp");
    let ito_path = tmp.path().join(".ito");
    std::fs::create_dir_all(&ito_path).expect("create .ito");
    let run_id = "20260424-120002-deadbeef";

    let plan = build_run_plan(
        run_id,
        "generic",
        OrchestrateRunConfig::default(),
        Vec::new(),
    )
    .expect("plan");
    let run = OrchestrateRun {
        run_id: run_id.to_string(),
        started_at: "2026-04-24T12:00:00Z".to_string(),
        finished_at: None,
        status: OrchestrateRunStatus::Running,
        preset: "generic".to_string(),
        max_parallel: plan.max_parallel,
        failure_policy: plan.failure_policy,
    };
    init_orchestrate_run_state(&ito_path, &run, &plan).expect("init");

    let state = OrchestrateChangeState {
        change_id: "001-01_demo".to_string(),
        gates: vec![OrchestrateGateRecord {
            gate: "tests".to_string(),
            outcome: ito_core::orchestrate::GateOutcome::Pass,
            finished_at: "2026-04-24T12:01:00Z".to_string(),
            error: None,
        }],
        updated_at: "2026-04-24T12:01:01Z".to_string(),
    };
    write_orchestrate_change_state(&ito_path, run_id, &state).expect("write");

    let loaded = load_orchestrate_change_state(&ito_path, run_id, "001-01_demo")
        .expect("load")
        .expect("exists");
    assert_eq!(loaded, state);
}

#[test]
fn orchestrate_dependency_cycle_is_rejected() {
    let run_id = "20260424-120003-deadbeef";
    let cfg = OrchestrateRunConfig::default();
    let changes = vec![
        ChangePlanInput {
            id: "A".to_string(),
            orchestrate: ChangeOrchestrateMetadata {
                depends_on: vec!["B".to_string()],
                preferred_gates: Vec::new(),
            },
        },
        ChangePlanInput {
            id: "B".to_string(),
            orchestrate: ChangeOrchestrateMetadata {
                depends_on: vec!["A".to_string()],
                preferred_gates: Vec::new(),
            },
        },
    ];

    let err = build_run_plan(run_id, "generic", cfg, changes).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("cycle"), "unexpected error: {msg}");
}

#[test]
fn orchestrate_max_parallel_aliases_resolve() {
    assert_eq!(
        parse_max_parallel(Some(Value::String("serial".to_string())), 4).unwrap(),
        1
    );
    assert_eq!(
        parse_max_parallel(Some(Value::String("parallel".to_string())), 4).unwrap(),
        4
    );
    assert_eq!(
        parse_max_parallel(Some(Value::Number(2.into())), 4).unwrap(),
        2
    );
}

#[test]
fn orchestrate_run_id_generation_matches_expected_format() {
    let id = generate_orchestrate_run_id(Utc.with_ymd_and_hms(2026, 4, 24, 12, 0, 0).unwrap());
    assert_eq!(id.len(), 24, "unexpected run id: {id}");
    assert_eq!(&id[8..9], "-");
    assert_eq!(&id[15..16], "-");

    let date = &id[0..8];
    let time = &id[9..15];
    let suffix = &id[16..24];
    assert!(date.chars().all(|c| c.is_ascii_digit()));
    assert!(time.chars().all(|c| c.is_ascii_digit()));
    assert!(suffix.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn orchestrate_resume_skips_terminal_gates() {
    let planned = vec![
        PlannedGate {
            name: "apply-complete".to_string(),
            policy: GatePolicy::Run,
        },
        PlannedGate {
            name: "tests".to_string(),
            policy: GatePolicy::Run,
        },
        PlannedGate {
            name: "code-review".to_string(),
            policy: GatePolicy::Run,
        },
    ];

    let state = OrchestrateChangeState {
        change_id: "001-01_demo".to_string(),
        gates: vec![
            OrchestrateGateRecord {
                gate: "apply-complete".to_string(),
                outcome: ito_core::orchestrate::GateOutcome::Pass,
                finished_at: "2026-04-24T12:00:00Z".to_string(),
                error: None,
            },
            OrchestrateGateRecord {
                gate: "tests".to_string(),
                outcome: ito_core::orchestrate::GateOutcome::Pass,
                finished_at: "2026-04-24T12:00:01Z".to_string(),
                error: None,
            },
        ],
        updated_at: "2026-04-24T12:00:02Z".to_string(),
    };

    let remaining = remaining_gates_for_change(&planned, Some(&state));
    let names: Vec<String> = remaining.into_iter().map(|g| g.name).collect();
    assert_eq!(names, vec!["code-review".to_string()]);

    let state_fail = OrchestrateChangeState {
        change_id: "001-01_demo".to_string(),
        gates: vec![
            OrchestrateGateRecord {
                gate: "apply-complete".to_string(),
                outcome: ito_core::orchestrate::GateOutcome::Pass,
                finished_at: "2026-04-24T12:00:00Z".to_string(),
                error: None,
            },
            OrchestrateGateRecord {
                gate: "tests".to_string(),
                outcome: ito_core::orchestrate::GateOutcome::Fail,
                finished_at: "2026-04-24T12:00:01Z".to_string(),
                error: Some("boom".to_string()),
            },
        ],
        updated_at: "2026-04-24T12:00:02Z".to_string(),
    };
    let remaining = remaining_gates_for_change(&planned, Some(&state_fail));
    let names: Vec<String> = remaining.into_iter().map(|g| g.name).collect();
    assert_eq!(names, vec!["tests".to_string(), "code-review".to_string()]);
}
