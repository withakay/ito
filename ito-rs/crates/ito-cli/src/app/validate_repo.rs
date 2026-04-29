//! CLI adapter for `ito validate repo`.
//!
//! Loads `ItoConfig`, runs the `ito-core::validate_repo` engine, renders
//! the resulting `ValidationReport` in human or JSON form, and maps the
//! outcome to documented exit codes:
//!
//! - **0** — no `ERROR` issues (and no `WARNING` issues under `--strict`).
//! - **1** — at least one `ERROR` issue (or any `WARNING` under `--strict`).
//! - **2** — usage error (mutually exclusive flags) or unloadable config.

use crate::cli::RepoValidateArgs;
use crate::cli_error::{CliError, CliResult, to_cli_error};
use crate::runtime::Runtime;
use ito_config::ConfigContext;
use ito_config::load_cascading_project_config;

use ito_config::types::ItoConfig;
use ito_core::process::SystemProcessRunner;
use ito_core::validate as core_validate;
use ito_core::validate_repo::{
    ActiveRule, RuleRegistry, StagedFiles, list_active_rules_for, run_repo_validation,
};

/// Top-level entry point invoked from `app::validate::handle_validate`.
pub(crate) fn handle_validate_repo(rt: &Runtime, args: &RepoValidateArgs) -> CliResult<()> {
    // The clap-derive layer enforces `--rule` vs `--no-rule` exclusivity;
    // we belt-and-braces the check here so the string-arg path is also
    // protected.
    if !args.rule.is_empty() && !args.no_rule.is_empty() {
        return Err(CliError::with_code(
            2,
            "`--rule` and `--no-rule` are mutually exclusive.",
        ));
    }

    let config = load_config(rt)?;

    if let Some(rule_id) = &args.explain {
        return explain_rule(&config, rule_id, args.json);
    }

    if args.list_rules {
        return list_rules(&config, args.json);
    }

    let project_root = rt.cwd();

    let runner = SystemProcessRunner;
    let staged = if args.staged {
        StagedFiles::from_git(&runner, project_root).map_err(to_cli_error)?
    } else {
        StagedFiles::empty()
    };

    let mut report = run_repo_validation(&config, project_root, &staged, &runner, args.strict);

    if !args.rule.is_empty() {
        report.issues.retain(|i| {
            i.rule_id
                .as_deref()
                .is_some_and(|id| args.rule.iter().any(|r| r == id))
        });
        recompute_summary(&mut report, args.strict);
    } else if !args.no_rule.is_empty() {
        report.issues.retain(|i| {
            i.rule_id
                .as_deref()
                .is_none_or(|id| !args.no_rule.iter().any(|r| r == id))
        });
        recompute_summary(&mut report, args.strict);
    }

    if args.json {
        print_report_json(&report)?;
    } else {
        print_report_human(&report);
    }

    // `report.valid` already accounts for `--strict` (warnings are
    // promoted to failures via [`core_validate::ValidationReport::new`]).
    if !report.valid {
        return Err(CliError::silent_with_code(1));
    }
    Ok(())
}

fn load_config(rt: &Runtime) -> CliResult<ItoConfig> {
    let ctx = ConfigContext::from_process_env();
    let cfg_value = load_cascading_project_config(rt.cwd(), rt.ito_path(), &ctx).merged;
    serde_json::from_value::<ItoConfig>(cfg_value).map_err(|e| {
        CliError::with_code(
            2,
            format!(
                "Cannot load `.ito/config.json` for `ito validate repo`.\n\
                 Why: configuration failed to deserialize.\n\
                 Details: {e}\n\
                 Fix: validate the JSON syntax (`cat .ito/config.json | jq .`) and confirm \
                 every field matches the schema.",
            ),
        )
    })
}

fn explain_rule(config: &ItoConfig, rule_id: &str, want_json: bool) -> CliResult<()> {
    let rules = list_active_rules_for(&RuleRegistry::built_in(), config);
    let Some(rule) = rules.iter().find(|r| r.rule_id.as_str() == rule_id) else {
        return Err(CliError::with_code(
            2,
            format!(
                "Unknown rule `{rule_id}`. Run `ito validate repo --list-rules` to see all rules.",
            ),
        ));
    };

    if want_json {
        let json = active_rule_json(rule);
        let body = serde_json::to_string_pretty(&json)
            .map_err(|e| CliError::with_code(2, e.to_string()))?;
        println!("{body}");
    } else {
        println!("rule: {}", rule.rule_id.as_str());
        println!("severity: {}", rule.severity.as_str());
        println!("active: {}", rule.active);
        if let Some(gate) = rule.gate {
            println!("gate: {gate}");
        }
        println!("description: {}", rule.description);
    }
    Ok(())
}

fn list_rules(config: &ItoConfig, want_json: bool) -> CliResult<()> {
    let rules = list_active_rules_for(&RuleRegistry::built_in(), config);

    if want_json {
        let arr: Vec<serde_json::Value> = rules.iter().map(active_rule_json).collect();
        let body = serde_json::to_string_pretty(&serde_json::json!({ "rules": arr }))
            .map_err(|e| CliError::with_code(2, e.to_string()))?;
        println!("{body}");
        return Ok(());
    }

    println!("Built-in repository validation rules:");
    println!();
    for rule in &rules {
        let marker = if rule.active { "[x]" } else { "[ ]" };
        println!(
            "  {marker} {id:<48} {sev:<8} {gate}",
            id = rule.rule_id.as_str(),
            sev = rule.severity.as_str(),
            gate = rule.gate.unwrap_or("(always active)"),
        );
    }
    Ok(())
}

fn active_rule_json(rule: &ActiveRule) -> serde_json::Value {
    serde_json::json!({
        "rule_id": rule.rule_id.as_str(),
        "severity": rule.severity.as_str(),
        "active": rule.active,
        "gate": rule.gate,
        "description": rule.description,
    })
}

fn print_report_json(report: &core_validate::ValidationReport) -> CliResult<()> {
    let body =
        serde_json::to_string_pretty(report).map_err(|e| CliError::with_code(2, e.to_string()))?;
    println!("{body}");
    Ok(())
}

fn print_report_human(report: &core_validate::ValidationReport) {
    if report.issues.is_empty() {
        println!("Repository validation passed.");
        return;
    }
    println!(
        "{}",
        crate::diagnostics::render_validation_issues(&report.issues),
    );
    println!();
    println!(
        "Summary: {} error(s), {} warning(s), {} info",
        report.summary.errors, report.summary.warnings, report.summary.info,
    );
}

/// Recompute `summary.{errors,warnings,info}` and `valid` after `--rule`
/// or `--no-rule` filtering, mirroring the logic in
/// [`core_validate::ValidationReport::new`].
///
/// `strict` is propagated so warnings are treated as failures when the
/// caller passed `--strict`.
fn recompute_summary(report: &mut core_validate::ValidationReport, strict: bool) {
    let mut errors = 0;
    let mut warnings = 0;
    let mut info = 0;
    for issue in &report.issues {
        match issue.level.as_str() {
            "ERROR" => errors += 1,
            "WARNING" => warnings += 1,
            "INFO" => info += 1,
            // ValidationLevel is a `&'static str` (not an enum), so we
            // cannot exhaustively match all variants. Unknown levels are
            // ignored rather than counted; a future enum-based level
            // refactor would let us drop this arm.
            _ => {}
        }
    }
    report.summary.errors = errors;
    report.summary.warnings = warnings;
    report.summary.info = info;
    report.valid = if strict {
        errors == 0 && warnings == 0
    } else {
        errors == 0
    };
}
