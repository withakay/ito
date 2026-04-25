use std::collections::BTreeSet;
use std::sync::LazyLock;

use crate::error_bridge::IntoCoreResult;
use crate::show::read_change_proposal_markdown;
use ito_common::fs::StdFs;
use regex::Regex;

use super::rules_engine::rule_issue;
use super::{
    ArtifactValidatorContext, CoreResult, DomainChangeRepository, LEVEL_ERROR, LEVEL_INFO,
    LEVEL_WARNING, ValidationIssue, ValidationLevelYaml, ValidatorId, parse_change_show_json,
    read_change_delta_spec_files,
};

const MAX_SCENARIO_STEPS: usize = 8;
const CONTRACT_REF_SCHEMES: &[&str] = &["asyncapi", "cli", "config", "jsonschema", "openapi"];

static UI_MECHANICS_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    let mut patterns = Vec::new();
    for pattern in [
        r"(?i)\bclick\s+(?:on\s+|the\s+)?\w+",
        r"(?i)\bwait\s+\d+\s*(?:ms|millisecond|second|s)\b",
        r"(?i)\bsleep\s+\d+\b",
        r"(?i)\bselector\s*[:=]",
        r"(?i)\bcss\s+selector\b",
    ] {
        patterns.push(Regex::new(pattern).expect("valid UI mechanics regex"));
    }
    patterns
});

static INLINE_CODE_TOKEN_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"`([^`]+)`").expect("valid inline code regex"));

pub(super) fn artifact_rules() -> &'static [&'static str] {
    &["contract_refs", "scenario_grammar", "ui_mechanics"]
}

pub(super) fn proposal_rules() -> &'static [&'static str] {
    &["capabilities_consistency"]
}

pub(super) fn run_artifact_rule(
    rep: &mut super::ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    rule_name: &str,
    level: ValidationLevelYaml,
) -> CoreResult<()> {
    match rule_name {
        "scenario_grammar" => rep.extend(validate_scenario_grammar_rule(
            change_repo,
            ctx.change_id,
            level,
        )?),
        "ui_mechanics" => rep.extend(validate_ui_mechanics_rule(
            change_repo,
            ctx.change_id,
            level,
        )?),
        "contract_refs" => rep.extend(validate_contract_refs_rule(
            change_repo,
            ctx.change_id,
            level,
        )?),
        _ => {}
    }
    Ok(())
}

pub(super) fn run_proposal_rule(
    rep: &mut super::ReportBuilder,
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ctx: ArtifactValidatorContext<'_>,
    rule_name: &str,
    level: ValidationLevelYaml,
) -> CoreResult<()> {
    if rule_name == "capabilities_consistency" {
        rep.extend(validate_capabilities_consistency_rule(
            change_repo,
            ctx.ito_path,
            ctx.change_id,
            level,
        )?);
    }
    Ok(())
}

fn validate_scenario_grammar_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let show = parse_change_show_json(
        change_id,
        &read_change_delta_spec_files(change_repo, change_id)?,
    );
    let mut issues = Vec::new();

    for (delta_idx, delta) in show.deltas.iter().enumerate() {
        for (requirement_idx, requirement) in delta.requirements.iter().enumerate() {
            for (scenario_idx, scenario) in requirement.scenarios.iter().enumerate() {
                if scenario.raw_text.trim().is_empty() {
                    continue;
                }

                let steps = extract_scenario_steps(&scenario.raw_text);
                let has_given = steps.iter().any(|step| step.keyword == "GIVEN");
                let has_when = steps.iter().any(|step| step.keyword == "WHEN");
                let has_then = steps.iter().any(|step| step.keyword == "THEN");
                let path = format!(
                    "deltas[{delta_idx}].requirements[{requirement_idx}].scenarios[{scenario_idx}]"
                );

                if !has_when {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        level.as_level_str(),
                        &path,
                        "Scenario is missing WHEN step",
                    ));
                }
                if !has_then {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        level.as_level_str(),
                        &path,
                        "Scenario is missing THEN step",
                    ));
                }
                if !has_given {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        LEVEL_WARNING,
                        &path,
                        "Scenario is missing GIVEN step",
                    ));
                }
                if steps.len() > MAX_SCENARIO_STEPS {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "scenario_grammar",
                        LEVEL_WARNING,
                        &path,
                        format!(
                            "Scenario has more than {MAX_SCENARIO_STEPS} steps; consider splitting it"
                        ),
                    ));
                }
            }
        }
    }

    Ok(issues)
}

fn validate_ui_mechanics_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let show = parse_change_show_json(
        change_id,
        &read_change_delta_spec_files(change_repo, change_id)?,
    );
    let mut issues = Vec::new();

    for (delta_idx, delta) in show.deltas.iter().enumerate() {
        for (requirement_idx, requirement) in delta.requirements.iter().enumerate() {
            if requirement.tags.iter().any(|tag| tag == "ui") {
                continue;
            }

            for (scenario_idx, scenario) in requirement.scenarios.iter().enumerate() {
                if scenario.raw_text.trim().is_empty() {
                    continue;
                }

                let Some(pattern) = UI_MECHANICS_PATTERNS
                    .iter()
                    .find(|pattern| pattern.is_match(&scenario.raw_text))
                else {
                    continue;
                };

                issues.push(rule_issue(
                    ValidatorId::DeltaSpecsV1,
                    "ui_mechanics",
                    ui_mechanics_issue_level(level, LEVEL_WARNING),
                    format!(
                        "deltas[{delta_idx}].requirements[{requirement_idx}].scenarios[{scenario_idx}]"
                    ),
                    format!(
                        "Scenario may be describing UI mechanics rather than behavior (matched pattern: {})",
                        pattern.as_str()
                    ),
                ));
            }
        }
    }

    Ok(issues)
}

fn ui_mechanics_issue_level(
    configured_level: ValidationLevelYaml,
    table_level: &'static str,
) -> &'static str {
    // Opt-in rule severity is a floor: users can turn rule errors down to warnings,
    // but setting the rule to `error` does not promote advisory rows into errors.
    if configured_level == ValidationLevelYaml::Warning && table_level == LEVEL_ERROR {
        return LEVEL_WARNING;
    }
    table_level
}

#[derive(Debug, Clone)]
struct ScenarioStep {
    keyword: &'static str,
}

fn extract_scenario_steps(raw_text: &str) -> Vec<ScenarioStep> {
    let mut steps = Vec::new();
    for line in raw_text.lines() {
        let line = line.trim_start();
        let upper = line.to_ascii_uppercase();
        if upper.starts_with("- **GIVEN**") {
            steps.push(ScenarioStep { keyword: "GIVEN" });
            continue;
        }
        if upper.starts_with("- **WHEN**") {
            steps.push(ScenarioStep { keyword: "WHEN" });
            continue;
        }
        if upper.starts_with("- **THEN**") {
            steps.push(ScenarioStep { keyword: "THEN" });
            continue;
        }
        if upper.starts_with("- **AND**") {
            steps.push(ScenarioStep { keyword: "AND" });
        }
    }
    steps
}

fn validate_contract_refs_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let show = parse_change_show_json(
        change_id,
        &read_change_delta_spec_files(change_repo, change_id)?,
    );
    let proposal = read_change_proposal_markdown(change_repo, change_id)?;
    let public_contract_schemes = proposal
        .as_deref()
        .map(parse_public_contract_schemes)
        .unwrap_or_default();
    let mut referenced_schemes: BTreeSet<String> = BTreeSet::new();
    let mut has_any_contract_ref = false;
    let mut issues = Vec::new();

    for (delta_idx, delta) in show.deltas.iter().enumerate() {
        for (requirement_idx, requirement) in delta.requirements.iter().enumerate() {
            for contract_ref in &requirement.contract_refs {
                has_any_contract_ref = true;
                let path =
                    format!("deltas[{delta_idx}].requirements[{requirement_idx}].contract_refs");

                let Some(scheme) = contract_ref.scheme.as_deref() else {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "contract_refs",
                        level.as_level_str(),
                        &path,
                        format!("Invalid contract ref '{}'", contract_ref.raw),
                    ));
                    continue;
                };
                let Some(identifier) = contract_ref.identifier.as_deref() else {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "contract_refs",
                        level.as_level_str(),
                        &path,
                        format!("Invalid contract ref '{}'", contract_ref.raw),
                    ));
                    continue;
                };
                if !CONTRACT_REF_SCHEMES.contains(&scheme) {
                    issues.push(rule_issue(
                        ValidatorId::DeltaSpecsV1,
                        "contract_refs",
                        level.as_level_str(),
                        &path,
                        format!(
                            "Unknown contract ref scheme '{scheme}' in '{raw}'. Supported schemes: {{{supported}}}",
                            raw = contract_ref.raw,
                            supported = CONTRACT_REF_SCHEMES.join(", "),
                        ),
                    ));
                    continue;
                }
                if !identifier.is_empty() {
                    referenced_schemes.insert(scheme.to_string());
                }
            }
        }
    }

    if has_any_contract_ref && !contract_discovery_is_configured() {
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "contract_refs",
            LEVEL_INFO,
            "proposal.contracts",
            "Contract refs are present, but contract resolution is not configured for this project yet",
        ));
    }

    for scheme in public_contract_schemes {
        if scheme == "none" || referenced_schemes.contains(&scheme) {
            continue;
        }
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "contract_refs",
            LEVEL_WARNING,
            "proposal.contracts",
            format!(
                "Public Contract facet '{scheme}' is declared in Change Shape but no requirement references {scheme}:..."
            ),
        ));
    }

    Ok(issues)
}

fn contract_discovery_is_configured() -> bool {
    false
}

fn parse_public_contract_schemes(markdown: &str) -> Vec<String> {
    let mut in_change_shape = false;
    for line in markdown.lines() {
        let line = line.trim_end();
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("## ") {
            in_change_shape = title.trim().eq_ignore_ascii_case("Change Shape");
            continue;
        }
        if !in_change_shape {
            continue;
        }
        let Some(rest) = trimmed
            .strip_prefix("- **Public Contract**:")
            .map(str::trim)
        else {
            continue;
        };
        let mut schemes = Vec::new();
        for value in rest.split(',') {
            let value = value.trim().to_ascii_lowercase();
            if value.is_empty() {
                continue;
            }
            schemes.push(value);
        }
        return schemes;
    }

    Vec::new()
}

fn validate_capabilities_consistency_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ito_path: &std::path::Path,
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let Some(proposal) = read_change_proposal_markdown(change_repo, change_id)? else {
        return Ok(Vec::new());
    };

    let parsed = parse_proposal_capabilities(&proposal);
    let delta_specs = read_change_delta_spec_files(change_repo, change_id)?;
    let mut delta_names: BTreeSet<String> = BTreeSet::new();
    for spec in delta_specs {
        delta_names.insert(spec.spec);
    }
    let baseline_names_raw =
        ito_domain::discovery::list_spec_dir_names(&StdFs, ito_path).into_core()?;
    let mut baseline_names: BTreeSet<String> = BTreeSet::new();
    for baseline_name in baseline_names_raw {
        baseline_names.insert(baseline_name);
    }

    let mut issues = Vec::new();
    for warning_message in parsed.warnings {
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "capabilities_consistency",
            LEVEL_WARNING,
            "proposal.capabilities",
            warning_message,
        ));
    }

    for capability in &parsed.new_capabilities {
        if !delta_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed in the proposal but no delta spec exists at specs/{capability}/spec.md"
                ),
            ));
        }
        if baseline_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed as new but already exists in .ito/specs/{capability}/"
                ),
            ));
        }
    }

    for capability in &parsed.modified_capabilities {
        if !delta_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed in the proposal but no delta spec exists at specs/{capability}/spec.md"
                ),
            ));
        }
        if !baseline_names.contains(capability) {
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "capabilities_consistency",
                level.as_level_str(),
                "proposal.capabilities",
                format!(
                    "Capability '{capability}' is listed as modified but no baseline spec exists in .ito/specs/{capability}/"
                ),
            ));
        }
    }

    let mut declared: BTreeSet<String> = BTreeSet::new();
    for capability in &parsed.new_capabilities {
        declared.insert(capability.clone());
    }
    for capability in &parsed.modified_capabilities {
        declared.insert(capability.clone());
    }
    for capability in delta_names {
        if declared.contains(&capability) {
            continue;
        }
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "capabilities_consistency",
            level.as_level_str(),
            "proposal.capabilities",
            format!("Delta capability '{capability}' is not listed in the proposal"),
        ));
    }

    Ok(issues)
}

#[derive(Debug, Default)]
struct ParsedProposalCapabilities {
    new_capabilities: Vec<String>,
    modified_capabilities: Vec<String>,
    warnings: Vec<String>,
}

fn parse_proposal_capabilities(markdown: &str) -> ParsedProposalCapabilities {
    let mut parsed = ParsedProposalCapabilities::default();
    let mut in_capabilities = false;

    enum CapabilitySection {
        None,
        New,
        Modified,
    }

    let mut section = CapabilitySection::None;
    for line in markdown.lines() {
        let line = line.trim_end();
        let trimmed = line.trim();

        if let Some(title) = trimmed.strip_prefix("## ") {
            in_capabilities = title.trim().eq_ignore_ascii_case("Capabilities");
            if !in_capabilities {
                section = CapabilitySection::None;
            }
            continue;
        }
        if !in_capabilities {
            continue;
        }

        if let Some(title) = trimmed.strip_prefix("### ") {
            section = if title.trim().eq_ignore_ascii_case("New Capabilities") {
                CapabilitySection::New
            } else if title.trim().eq_ignore_ascii_case("Modified Capabilities") {
                CapabilitySection::Modified
            } else {
                CapabilitySection::None
            };
            continue;
        }

        let Some(rest) = trimmed
            .strip_prefix("- ")
            .or_else(|| trimmed.strip_prefix("* "))
            .map(str::trim)
        else {
            continue;
        };
        if rest.is_empty() || rest.starts_with("<!--") {
            continue;
        }

        let Some(capability) = extract_first_inline_code_token(rest) else {
            parsed.warnings.push(format!(
                "Capability bullet is missing an inline-code token: {rest}"
            ));
            continue;
        };
        if capability.starts_with('<') && capability.ends_with('>') {
            continue;
        }

        match section {
            CapabilitySection::New => parsed.new_capabilities.push(capability),
            CapabilitySection::Modified => parsed.modified_capabilities.push(capability),
            CapabilitySection::None => {}
        }
    }

    parsed
}

fn extract_first_inline_code_token(line: &str) -> Option<String> {
    let captures = INLINE_CODE_TOKEN_RE.captures(line)?;
    let token = captures.get(1)?.as_str().trim();
    if token.is_empty() {
        return None;
    }
    Some(token.to_string())
}
