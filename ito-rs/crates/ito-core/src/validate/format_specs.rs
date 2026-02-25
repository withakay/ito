#[derive(Debug, Clone, Copy)]
pub(crate) struct FormatSpecRef {
    pub validator_id: &'static str,
    pub spec_path: &'static str,
}

pub(crate) const DELTA_SPECS_V1: FormatSpecRef = FormatSpecRef {
    validator_id: "ito.delta-specs.v1",
    spec_path: ".ito/specs/delta-specs/spec.md",
};

pub(crate) const TASKS_TRACKING_V1: FormatSpecRef = FormatSpecRef {
    validator_id: "ito.tasks-tracking.v1",
    spec_path: ".ito/specs/tasks-tracking/spec.md",
};
