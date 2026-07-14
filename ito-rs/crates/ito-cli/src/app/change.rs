use ito_core::implementation_readiness::{
    AuthoritativeChangeSource, ReadinessCondition, ReadinessPhase, ReadinessReport,
    ReadinessRequest, evaluate_readiness, materialize_authoritative_change, render_readiness_text,
};
use std::path::Path;

use crate::cli::{ChangeArgs, ChangeCommand, ChangePreflightArgs, ReadinessPhaseArg};
use crate::cli_error::{CliResult, silent_fail, to_cli_error};
use crate::runtime::Runtime;

pub(crate) fn handle_change_clap(rt: &Runtime, args: &ChangeArgs) -> CliResult<()> {
    match &args.command {
        ChangeCommand::Preflight(args) => handle_preflight(rt, args),
    }
}

fn handle_preflight(rt: &Runtime, args: &ChangePreflightArgs) -> CliResult<()> {
    let phase = match args.phase {
        ReadinessPhaseArg::Prepare => ReadinessPhase::Prepare,
        ReadinessPhaseArg::Execute => ReadinessPhase::Execute,
    };
    let report = evaluate_runtime_readiness(rt, &args.change_id, phase, args.refresh)?;

    emit_preflight_report(&report, args.json)?;
    if report.ready { Ok(()) } else { silent_fail() }
}

pub(crate) fn evaluate_runtime_readiness(
    rt: &Runtime,
    change_id: &str,
    phase: ReadinessPhase,
    refresh: bool,
) -> CliResult<ReadinessReport> {
    evaluate_runtime_readiness_at(rt, change_id, phase, refresh, None)
}

pub(crate) fn evaluate_runtime_readiness_at(
    rt: &Runtime,
    change_id: &str,
    phase: ReadinessPhase,
    refresh: bool,
    current_checkout: Option<&Path>,
) -> CliResult<ReadinessReport> {
    let mut request =
        ReadinessRequest::new(change_id, phase, rt.cwd()).with_refresh_authority(refresh);
    if phase == ReadinessPhase::Execute {
        request = request.with_current_checkout(current_checkout.unwrap_or_else(|| rt.cwd()));
    }
    let config = rt.typed_config().map_err(to_cli_error)?;
    Ok(evaluate_readiness(&request, &config))
}

pub(crate) fn require_runtime_readiness_at(
    rt: &Runtime,
    change_id: &str,
    phase: ReadinessPhase,
    json: bool,
    current_checkout: Option<&Path>,
) -> CliResult<ReadinessReport> {
    let report = evaluate_runtime_readiness_at(rt, change_id, phase, false, current_checkout)?;
    if report.ready {
        return Ok(report);
    }
    emit_preflight_report(&report, json)?;
    silent_fail()
}

pub(crate) fn require_runtime_readiness(
    rt: &Runtime,
    change_id: &str,
    phase: ReadinessPhase,
    json: bool,
) -> CliResult<ReadinessReport> {
    require_runtime_readiness_at(rt, change_id, phase, json, None)
}

pub(crate) fn require_authoritative_render_source(
    rt: &Runtime,
    prepare: &ReadinessReport,
    guidance_artifacts: &[&str],
    json: bool,
) -> CliResult<AuthoritativeChangeSource> {
    match materialize_authoritative_change(prepare, rt.cwd(), guidance_artifacts) {
        Ok(source) => Ok(source),
        Err(error) => {
            let mut report = prepare.clone();
            report.ready = false;
            report.conditions.push(ReadinessCondition {
                code: "authoritative_render_inputs".to_string(),
                passed: false,
                message: format!(
                    "Cannot load apply instructions exclusively from authority commit: {error}"
                ),
                remediation: Some(
                    "Restore the accepted proposal files as regular UTF-8 Git blobs on the authoritative target branch, then retry."
                        .to_string(),
                ),
                path: error.path().map(ToOwned::to_owned),
                validator_code: None,
            });
            emit_preflight_report(&report, json)?;
            silent_fail()
        }
    }
}

fn emit_preflight_report(report: &ReadinessReport, json: bool) -> CliResult<()> {
    if json {
        let rendered = serde_json::to_string_pretty(report).map_err(to_cli_error)?;
        println!("{rendered}");
        return Ok(());
    }

    let rendered = render_readiness_text(report);
    if report.ready {
        println!("{rendered}");
    } else {
        eprintln!("{rendered}");
    }
    Ok(())
}

#[cfg(test)]
#[path = "change_tests.rs"]
mod tests;
