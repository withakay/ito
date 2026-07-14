use std::path::Path;

use crate::templates::{SchemaYaml, ValidationTrackingSourceYaml, ValidationYaml, ValidatorId};

use super::{
    ArtifactValidatorContext, CoreResult, ValidationReport, artifact_done, error, report,
    rules_engine,
};

/// Run schema-configured rules against a caller-controlled authoritative Ito tree.
pub(crate) fn validate_configured_schema_rules(
    ito_path: &Path,
    change_id: &str,
    schema: &SchemaYaml,
    validation: &ValidationYaml,
    strict: bool,
) -> CoreResult<ValidationReport> {
    let change_repo = crate::change_repository::FsChangeRepository::new(ito_path);
    let change_dir = ito_common::paths::change_dir(ito_path, change_id);
    let mut rep = report(strict);

    for (artifact_id, config) in &validation.artifacts {
        let Some(rules) = config.rules.as_ref().filter(|rules| !rules.is_empty()) else {
            continue;
        };
        let Some(artifact) = schema
            .artifacts
            .iter()
            .find(|artifact| artifact.id == *artifact_id)
        else {
            rep.push(error(
                "schema.validation",
                format!("validation.yaml references unknown artifact id '{artifact_id}'"),
            ));
            continue;
        };
        let Some(validator) = config.validate_as else {
            continue;
        };
        if !artifact_done(&change_dir, &artifact.generates)
            && validator != ValidatorId::DeltaSpecsV1
        {
            continue;
        }
        let context = ArtifactValidatorContext {
            ito_path,
            change_id,
            strict,
        };
        rules_engine::run_artifact_rules(
            &mut rep,
            &change_repo,
            context,
            validator,
            artifact_id,
            Some(rules),
        )?;
    }

    if let Some(config) = validation.proposal.as_ref()
        && change_dir.join("proposal.md").is_file()
        && let Some(rules) = config.rules.as_ref().filter(|rules| !rules.is_empty())
        && let Some(validator) = config.validate_as
    {
        let context = ArtifactValidatorContext {
            ito_path,
            change_id,
            strict,
        };
        match validator {
            ValidatorId::DeltaSpecsV1 => rules_engine::run_proposal_rules(
                &mut rep,
                &change_repo,
                context,
                validator,
                Some(rules),
            )?,
            ValidatorId::TasksTrackingV1 => rep.push(error(
                "schema.validation",
                "Validator 'ito.tasks-tracking.v1' is not valid for proposal artifacts",
            )),
        }
    }

    if let Some(config) = validation.tracking.as_ref()
        && let Some(rules) = config.rules.as_ref().filter(|rules| !rules.is_empty())
    {
        let ValidationTrackingSourceYaml::ApplyTracks = config.source;
        let Some(tracks) = schema
            .apply
            .as_ref()
            .and_then(|apply| apply.tracks.as_deref())
        else {
            return Ok(rep.finish());
        };
        if !ito_domain::tasks::is_safe_tracking_filename(tracks) {
            rep.push(error(
                "tracking",
                format!("Invalid tracking file path in apply.tracks: '{tracks}'"),
            ));
            return Ok(rep.finish());
        }
        let path = change_dir.join(tracks);
        if path.is_file() {
            let report_path = format!("changes/{change_id}/{tracks}");
            let context = ArtifactValidatorContext {
                ito_path,
                change_id,
                strict,
            };
            match config.validate_as {
                ValidatorId::TasksTrackingV1 => rules_engine::run_tracking_rules(
                    &mut rep,
                    &change_repo,
                    context,
                    ValidatorId::TasksTrackingV1,
                    &path,
                    &report_path,
                    Some(rules),
                )?,
                ValidatorId::DeltaSpecsV1 => rep.push(error(
                    "schema.validation",
                    "Validator 'ito.delta-specs.v1' is not valid for tracking files",
                )),
            }
        }
    }

    Ok(rep.finish())
}
