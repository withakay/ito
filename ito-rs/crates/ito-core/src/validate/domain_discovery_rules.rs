//! Domain discovery validation rules for optional DDD handoff artifacts.
//!
//! These rules connect lightweight discovery outputs to proposal/spec/task text so
//! terminology, context boundaries, and lazily updated domain docs do not drift.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error_bridge::IntoCoreResult;

use super::rules_engine::rule_issue;
use super::{
    CoreResult, DomainChangeRepository, ValidationIssue, ValidationLevelYaml, ValidatorId,
};

pub(super) fn validate_ubiquitous_language_consistency_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let change = change_repo.get(change_id).into_core()?;
    let Some(discovery) = read_domain_discovery_markdown(&change.path) else {
        return Ok(Vec::new());
    };
    let handoff = parse_domain_discovery_handoff(&discovery);
    if handoff.rejected_aliases.is_empty() {
        return Ok(Vec::new());
    }

    let corpus = normalize_for_domain_match(&build_language_validation_corpus(&change));
    let mut issues = Vec::new();
    for alias in handoff.rejected_aliases {
        if !normalized_text_contains_term(&corpus, &alias.alias) {
            continue;
        }
        issues.push(rule_issue(
            ValidatorId::DeltaSpecsV1,
            "ubiquitous_language_consistency",
            level.as_level_str(),
            "domain-discovery.ubiquitous-language",
            format!(
                "Rejected alias '{}' appears in proposal, specs, or tasks; use canonical term '{}' instead",
                alias.alias, alias.canonical_term
            ),
        ));
    }

    Ok(issues)
}

pub(super) fn validate_domain_documentation_consistency_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    ito_path: &Path,
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let change = change_repo.get(change_id).into_core()?;
    let Some(discovery) = read_domain_discovery_markdown(&change.path) else {
        return Ok(Vec::new());
    };
    let handoff = parse_domain_discovery_handoff(&discovery);
    if handoff.terms.is_empty() {
        return Ok(Vec::new());
    }

    let mut issues = Vec::new();
    let project_root = ito_path.parent().map(Path::to_path_buf);
    for path in collect_domain_document_paths(&change.path)
        .into_iter()
        .chain(collect_existing_domain_document_paths(
            project_root.as_deref(),
            &change.path,
        ))
    {
        let Ok(markdown) = fs::read_to_string(&path) else {
            continue;
        };
        let doc_terms = parse_ubiquitous_language_terms(&markdown);
        for term in &handoff.terms {
            let Some(doc_term) = doc_terms
                .iter()
                .find(|candidate| same_domain_term(&candidate.term, &term.term))
            else {
                continue;
            };
            if same_definition(&doc_term.definition, &term.definition) {
                continue;
            }
            if terms_belong_to_different_contexts(term, doc_term) {
                continue;
            }

            let report_path =
                report_path_for_domain_document(&change.path, project_root.as_deref(), &path);
            issues.push(rule_issue(
                ValidatorId::DeltaSpecsV1,
                "domain_documentation_consistency",
                level.as_level_str(),
                report_path,
                format!(
                    "Proposed domain documentation defines '{}' differently from domain-discovery.md",
                    term.term
                ),
            ));
        }
    }

    Ok(issues)
}

pub(super) fn validate_context_boundary_consistency_rule(
    change_repo: &(impl DomainChangeRepository + ?Sized),
    change_id: &str,
    level: ValidationLevelYaml,
) -> CoreResult<Vec<ValidationIssue>> {
    let change = change_repo.get(change_id).into_core()?;
    let Some(discovery) = read_domain_discovery_markdown(&change.path) else {
        if corpus_suggests_cross_context(&build_language_validation_corpus(&change)) {
            return Ok(vec![rule_issue(
                ValidatorId::DeltaSpecsV1,
                "context_boundary_consistency",
                level.as_level_str(),
                "domain-discovery.bounded-context-map",
                "Cross-context proposal is missing domain discovery boundary framing",
            )]);
        }
        return Ok(Vec::new());
    };
    let handoff = parse_domain_discovery_handoff(&discovery);
    let corpus_requires_boundary_review =
        corpus_suggests_cross_context(&build_language_validation_corpus(&change));
    if !handoff.needs_context_boundary_review() && !corpus_requires_boundary_review {
        return Ok(Vec::new());
    }

    let mut missing = Vec::new();
    if corpus_requires_boundary_review && handoff.affected_context_count() < 2 {
        missing.push("affected contexts");
    }
    if !handoff.has_context_ownership() {
        missing.push("context ownership");
    }
    if !handoff.has_relationship_framing() {
        missing.push("relationship pattern or provisional unknown");
    }
    if !handoff.has_translation_boundary() {
        missing.push("translation boundary");
    }
    if missing.is_empty() {
        return Ok(Vec::new());
    }

    Ok(vec![rule_issue(
        ValidatorId::DeltaSpecsV1,
        "context_boundary_consistency",
        level.as_level_str(),
        "domain-discovery.bounded-context-map",
        format!(
            "Cross-context domain discovery is missing {}",
            missing.join(", ")
        ),
    )])
}

#[derive(Debug, Default)]
struct DomainDiscoveryHandoff {
    terms: Vec<DomainTerm>,
    rejected_aliases: Vec<RejectedAlias>,
    primary_context: Option<String>,
    supporting_contexts: Vec<String>,
    bounded_contexts: Vec<BoundedContext>,
    translation_required: Option<String>,
}

#[derive(Debug)]
struct DomainTerm {
    term: String,
    definition: String,
    owner_context: String,
}

#[derive(Debug)]
struct RejectedAlias {
    alias: String,
    canonical_term: String,
}

#[derive(Debug)]
struct BoundedContext {
    context: String,
    responsibilities: String,
    owner: String,
    owned_language: String,
    relationship_pattern: String,
}

impl DomainDiscoveryHandoff {
    fn needs_context_boundary_review(&self) -> bool {
        self.affected_context_count() > 1
    }

    fn affected_context_count(&self) -> usize {
        self.affected_contexts().len()
    }

    fn has_context_ownership(&self) -> bool {
        let affected_contexts = self.affected_contexts();
        let owned_contexts = self
            .bounded_contexts
            .iter()
            .filter(|context| {
                is_meaningful_domain_value(&context.context)
                    && is_meaningful_domain_value(&context.responsibilities)
                    && is_meaningful_domain_value(&context.owner)
                    && is_meaningful_domain_value(&context.owned_language)
            })
            .map(|context| normalize_for_domain_match(&context.context))
            .collect::<BTreeSet<_>>();
        !affected_contexts.is_empty() && affected_contexts.is_subset(&owned_contexts)
    }

    fn affected_contexts(&self) -> BTreeSet<String> {
        let mut contexts = BTreeSet::new();
        if let Some(context) = self.primary_context.as_deref() {
            insert_meaningful_context(&mut contexts, context);
        }
        for context in &self.supporting_contexts {
            insert_meaningful_context(&mut contexts, context);
        }
        for context in &self.bounded_contexts {
            insert_meaningful_context(&mut contexts, &context.context);
        }
        contexts
    }

    fn has_relationship_framing(&self) -> bool {
        let affected_contexts = self.affected_contexts();
        let framed_contexts = self
            .bounded_contexts
            .iter()
            .filter(|context| is_meaningful_domain_value(&context.relationship_pattern))
            .map(|context| normalize_for_domain_match(&context.context))
            .collect::<BTreeSet<_>>();
        !affected_contexts.is_empty() && affected_contexts.is_subset(&framed_contexts)
    }

    fn has_translation_boundary(&self) -> bool {
        self.translation_required
            .as_deref()
            .is_some_and(is_resolved_translation_boundary_value)
    }

    fn has_meaningful_content(&self) -> bool {
        !self.terms.is_empty()
            || !self.rejected_aliases.is_empty()
            || self
                .primary_context
                .as_deref()
                .is_some_and(is_meaningful_context)
            || !self.supporting_contexts.is_empty()
            || !self.bounded_contexts.is_empty()
            || self
                .translation_required
                .as_deref()
                .is_some_and(is_meaningful_domain_value)
    }

    fn has_only_context_summary(&self) -> bool {
        self.terms.is_empty()
            && self.rejected_aliases.is_empty()
            && self.bounded_contexts.is_empty()
            && self.translation_required.is_none()
            && (self
                .primary_context
                .as_deref()
                .is_some_and(is_meaningful_context)
                || !self.supporting_contexts.is_empty())
    }
}

fn read_domain_discovery_markdown(change_path: &Path) -> Option<String> {
    let mut standalone_discovery = None;
    let mut embedded_discovery = String::new();
    let mut discovery = String::new();
    let standalone = change_path.join("domain-discovery.md");
    if let Ok(markdown) = fs::read_to_string(&standalone) {
        let handoff = parse_domain_discovery_handoff(&markdown);
        if handoff.has_meaningful_content() {
            standalone_discovery = Some((markdown, handoff.has_only_context_summary()));
        }
    }

    for path in embedded_domain_discovery_candidate_paths(change_path) {
        let Ok(markdown) = fs::read_to_string(path) else {
            continue;
        };
        let summary = markdown_section(&markdown, "Domain Discovery Summary");
        if summary.trim().is_empty() {
            continue;
        }
        let handoff = parse_domain_discovery_handoff(&markdown);
        if handoff.has_meaningful_content() {
            embedded_discovery.push_str(&markdown);
            embedded_discovery.push('\n');
        }
    }

    // Do not let a context-only standalone stub hide richer embedded handoff data.
    if let Some((markdown, has_only_context_summary)) = standalone_discovery
        && (embedded_discovery.is_empty() || !has_only_context_summary)
    {
        discovery.push_str(&markdown);
        discovery.push('\n');
    }
    discovery.push_str(&embedded_discovery);

    if !discovery.is_empty() {
        return Some(discovery);
    }

    None
}

fn parse_domain_discovery_handoff(markdown: &str) -> DomainDiscoveryHandoff {
    let summary = markdown_section(markdown, "Domain Discovery Summary");
    let summaries = markdown_sections(markdown, "Domain Discovery Summary");
    let ownership = markdown_section(markdown, "Model Ownership");
    let mut terms = parse_ubiquitous_language_terms(markdown);
    for summary in &summaries {
        terms.extend(parse_compact_domain_terms(summary));
    }
    dedup_domain_terms(&mut terms);
    let mut rejected_aliases = parse_rejected_aliases(markdown);
    for summary in &summaries {
        rejected_aliases.extend(parse_compact_rejected_aliases(summary));
    }
    dedup_rejected_aliases(&mut rejected_aliases);
    let mut bounded_contexts = parse_bounded_contexts(markdown);
    for summary in &summaries {
        bounded_contexts.extend(parse_compact_bounded_contexts(summary));
    }
    dedup_bounded_contexts(&mut bounded_contexts);
    DomainDiscoveryHandoff {
        terms,
        rejected_aliases,
        primary_context: parse_summary_field(&summary, "Primary bounded context"),
        supporting_contexts: parse_context_list_field(&summary, "Supporting contexts"),
        bounded_contexts,
        translation_required: parse_summary_field(&ownership, "Translation boundaries")
            .or_else(|| parse_summary_field(&ownership, "Translation required"))
            .or_else(|| parse_translation_boundary_field(&ownership, "Translation boundaries"))
            .or_else(|| parse_translation_boundary_field(&ownership, "Translation required"))
            .or_else(|| parse_translation_boundary_field(&summary, "Translation boundaries")),
    }
}

fn embedded_domain_discovery_candidate_paths(change_path: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for relative in ["proposal.md", "design.md", "tasks.md"] {
        paths.push(change_path.join(relative));
    }
    let mut spec_paths = Vec::new();
    collect_markdown_paths(&change_path.join("specs"), &mut spec_paths, 0);
    spec_paths.sort();
    paths.extend(spec_paths);
    paths
}

fn collect_markdown_paths(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > EMBEDDED_HANDOFF_MAX_DEPTH_INCLUSIVE {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            collect_markdown_paths(&path, out, depth + 1);
            continue;
        }
        if !metadata.is_file() || path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        out.push(path);
    }
}

fn parse_summary_field(markdown: &str, label: &str) -> Option<String> {
    parse_summary_field_matching(markdown, label, is_meaningful_domain_value)
}

fn parse_translation_boundary_field(markdown: &str, label: &str) -> Option<String> {
    parse_summary_field_matching(markdown, label, is_resolved_translation_boundary_value)
}

fn parse_summary_field_matching(
    markdown: &str,
    label: &str,
    is_resolved: impl Fn(&str) -> bool,
) -> Option<String> {
    for line in markdown.lines() {
        let line = line.trim();
        let Some(value) = parse_labeled_bullet(line, label) else {
            continue;
        };
        let value = clean_table_cell(value);
        if is_resolved(&value) {
            return Some(value);
        }
    }
    None
}

fn parse_labeled_bullet<'a>(line: &'a str, label: &str) -> Option<&'a str> {
    let line = line
        .strip_prefix("- ")
        .or_else(|| line.strip_prefix("* "))?;
    if let Some(line) = line.strip_prefix("**") {
        let (found, value) = line.split_once("**:")?;
        if found.trim().eq_ignore_ascii_case(label) {
            return Some(value);
        }
        return None;
    }

    let (found, value) = line.split_once(':')?;
    if found.trim().eq_ignore_ascii_case(label) {
        return Some(value);
    }
    None
}

fn parse_context_list_field(markdown: &str, label: &str) -> Vec<String> {
    let Some(value) = parse_summary_field(markdown, label) else {
        return Vec::new();
    };
    value
        .split([',', ';'])
        .map(clean_table_cell)
        .filter(|value| is_meaningful_context(value))
        .collect()
}

fn parse_bounded_contexts(markdown: &str) -> Vec<BoundedContext> {
    let section = markdown_section(markdown, "Bounded Context Map");
    let mut contexts = Vec::new();
    let mut columns = BoundedContextColumns::default();
    for row in markdown_table_rows(&section) {
        if row.len() < 4 {
            continue;
        }
        let context = clean_table_cell(&row[0]);
        if context.eq_ignore_ascii_case("context") {
            columns = BoundedContextColumns::from_header(&row);
            continue;
        }
        if context.is_empty() {
            continue;
        }
        contexts.push(BoundedContext {
            context,
            responsibilities: columns.cell(&row, columns.responsibilities),
            owner: columns.cell(&row, columns.owner),
            owned_language: columns.cell(&row, columns.owned_language),
            relationship_pattern: columns.cell(&row, columns.relationship_pattern),
        });
    }
    contexts
}

struct BoundedContextColumns {
    responsibilities: usize,
    owner: usize,
    owned_language: usize,
    relationship_pattern: usize,
}

impl Default for BoundedContextColumns {
    fn default() -> Self {
        Self {
            responsibilities: 1,
            owner: 2,
            owned_language: 3,
            relationship_pattern: 4,
        }
    }
}

impl BoundedContextColumns {
    fn from_header(header: &[String]) -> Self {
        let default = Self::default();
        Self {
            responsibilities: find_header_index(header, &["responsibilities"])
                .unwrap_or(default.responsibilities),
            owner: find_header_index(header, &["owner", "owning context"]).unwrap_or(default.owner),
            owned_language: find_header_index(header, &["owned language", "owned concepts"])
                .unwrap_or(default.owned_language),
            relationship_pattern: find_header_index(
                header,
                &["relationship pattern", "relationship"],
            )
            .unwrap_or(default.relationship_pattern),
        }
    }

    fn cell(&self, row: &[String], index: usize) -> String {
        row.get(index).map(clean_table_cell).unwrap_or_default()
    }
}

fn find_header_index(header: &[String], candidates: &[&str]) -> Option<usize> {
    header.iter().position(|cell| {
        let cell = normalize_for_domain_match(cell);
        candidates
            .iter()
            .any(|candidate| cell.contains(&normalize_for_domain_match(candidate)))
    })
}

fn parse_ubiquitous_language_terms(markdown: &str) -> Vec<DomainTerm> {
    let section = markdown_section(markdown, "Ubiquitous Language");
    let mut terms = Vec::new();
    for row in markdown_table_rows(&section) {
        if row.len() < 2 {
            continue;
        }
        let term = clean_table_cell(&row[0]);
        let definition = clean_table_cell(&row[1]);
        if term.is_empty() || definition.is_empty() || term.eq_ignore_ascii_case("term") {
            continue;
        }
        let owner_context = row.get(2).map(clean_table_cell).unwrap_or_default();
        terms.push(DomainTerm {
            term,
            definition,
            owner_context,
        });
    }
    terms
}

fn parse_compact_domain_terms(markdown: &str) -> Vec<DomainTerm> {
    let Some(value) = parse_summary_field(markdown, "Canonical terms") else {
        return Vec::new();
    };
    let owner_context =
        parse_summary_field(markdown, "Primary bounded context").unwrap_or_default();
    parse_arrow_pairs(&value)
        .into_iter()
        .map(|(term, definition)| DomainTerm {
            term,
            definition,
            owner_context: owner_context.clone(),
        })
        .collect()
}

fn dedup_domain_terms(terms: &mut Vec<DomainTerm>) {
    let mut seen = BTreeSet::new();
    terms.retain(|term| {
        seen.insert((
            normalize_for_domain_match(&term.term),
            normalize_for_domain_match(&term.definition),
            normalize_for_domain_match(&term.owner_context),
        ))
    });
}

fn parse_rejected_aliases(markdown: &str) -> Vec<RejectedAlias> {
    let section = markdown_section(markdown, "Rejected Aliases / Overloaded Terms");
    let mut aliases = Vec::new();
    for row in markdown_table_rows(&section) {
        if row.len() < 2 {
            continue;
        }
        let alias = clean_table_cell(&row[0]);
        let canonical_term = clean_table_cell(&row[1]);
        if alias.is_empty() || canonical_term.is_empty() || is_rejected_alias_header(&alias) {
            continue;
        }
        aliases.push(RejectedAlias {
            alias,
            canonical_term,
        });
    }
    aliases
}

fn is_rejected_alias_header(alias: &str) -> bool {
    matches!(
        normalize_for_domain_match(alias).as_str(),
        "term or alias" | "alias / term" | "alias" | "term" | "term or aliases"
    )
}

fn parse_compact_rejected_aliases(markdown: &str) -> Vec<RejectedAlias> {
    let Some(value) = parse_summary_field(markdown, "Rejected aliases / overloaded terms") else {
        return Vec::new();
    };
    parse_arrow_pairs(&value)
        .into_iter()
        .map(|(alias, canonical_term)| RejectedAlias {
            alias,
            canonical_term,
        })
        .collect()
}

fn dedup_rejected_aliases(aliases: &mut Vec<RejectedAlias>) {
    let mut seen = BTreeSet::new();
    aliases.retain(|alias| {
        seen.insert((
            normalize_for_domain_match(&alias.alias),
            normalize_for_domain_match(&alias.canonical_term),
        ))
    });
}

fn parse_compact_bounded_contexts(markdown: &str) -> Vec<BoundedContext> {
    let Some(value) = parse_summary_field(markdown, "Bounded contexts") else {
        return Vec::new();
    };
    let relationship_pattern =
        parse_summary_field(markdown, "Cross-context relationships").unwrap_or_default();
    // Compact entries use: `Context -> responsibilities, owner, owned language`.
    parse_arrow_pairs(&value)
        .into_iter()
        .map(|(context, description)| {
            let parts = description
                .split(',')
                .map(clean_table_cell)
                .collect::<Vec<_>>();
            let responsibilities = parts.first().cloned().unwrap_or_default();
            let owner = parts.get(1).cloned().unwrap_or_default();
            let owned_language = parts.get(2).cloned().unwrap_or_default();
            BoundedContext {
                context,
                responsibilities,
                owner,
                owned_language,
                relationship_pattern: relationship_pattern.clone(),
            }
        })
        .collect()
}

fn dedup_bounded_contexts(contexts: &mut Vec<BoundedContext>) {
    let mut seen = BTreeSet::new();
    contexts.retain(|context| {
        seen.insert((
            normalize_for_domain_match(&context.context),
            normalize_for_domain_match(&context.responsibilities),
            normalize_for_domain_match(&context.owner),
            normalize_for_domain_match(&context.owned_language),
            normalize_for_domain_match(&context.relationship_pattern),
        ))
    });
}

fn parse_arrow_pairs(value: &str) -> Vec<(String, String)> {
    value
        .split(';')
        .filter_map(|entry| {
            let (left, right) = entry.split_once("->")?;
            let left = clean_table_cell(left);
            let right = clean_table_cell(right);
            if left.is_empty() || right.is_empty() {
                return None;
            }
            Some((left, right))
        })
        .collect()
}

fn markdown_section(markdown: &str, heading: &str) -> String {
    markdown_sections(markdown, heading).join("")
}

fn markdown_sections(markdown: &str, heading: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut section = String::new();
    let mut in_section = false;
    for line in markdown.lines() {
        let line = line.trim_end();
        if let Some(title) = line.trim().strip_prefix("## ") {
            if in_section && !section.is_empty() {
                sections.push(std::mem::take(&mut section));
            }
            in_section = title.trim().eq_ignore_ascii_case(heading);
            continue;
        }
        if in_section {
            section.push_str(line);
            section.push('\n');
        }
    }
    if in_section && !section.is_empty() {
        sections.push(section);
    }
    sections
}

fn markdown_table_rows(markdown: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    for line in markdown.lines() {
        let line = line.trim();
        if !line.starts_with('|') || !line.ends_with('|') {
            continue;
        }
        if line
            .chars()
            .all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace())
        {
            continue;
        }
        let row = line
            .trim_matches('|')
            .split('|')
            .map(clean_table_cell)
            .collect();
        rows.push(row);
    }
    rows
}

fn clean_table_cell(cell: impl AsRef<str>) -> String {
    let cell = cell.as_ref().trim();
    if cell.starts_with("<!--") && cell.ends_with("-->") {
        return String::new();
    }
    cell.trim_matches('`').trim().to_string()
}

fn insert_meaningful_context(contexts: &mut BTreeSet<String>, context: &str) {
    if !is_meaningful_context(context) {
        return;
    }
    contexts.insert(normalize_for_domain_match(context));
}

fn is_meaningful_context(value: &str) -> bool {
    is_meaningful_domain_value(value)
}

fn is_meaningful_domain_value(value: &str) -> bool {
    let value = value.trim();
    !value.is_empty()
        && !value.starts_with("<!--")
        && !value.eq_ignore_ascii_case("none")
        && !value.eq_ignore_ascii_case("n/a")
        && !value.eq_ignore_ascii_case("not applicable")
}

fn is_resolved_translation_boundary_value(value: &str) -> bool {
    let value = value.trim();
    // "None" is a resolved translation-boundary decision, not a placeholder.
    !value.is_empty()
        && !value.starts_with("<!--")
        && !value.eq_ignore_ascii_case("n/a")
        && !value.eq_ignore_ascii_case("not applicable")
}

fn build_language_validation_corpus(change: &ito_domain::changes::Change) -> String {
    let mut corpus = String::new();
    if let Some(proposal) = change.proposal.as_deref() {
        corpus.push_str(&strip_domain_discovery_sections(proposal));
        corpus.push('\n');
    }
    if let Some(design) = change.design.as_deref() {
        corpus.push_str(&strip_domain_discovery_sections(design));
        corpus.push('\n');
    }
    for spec in &change.specs {
        corpus.push_str(&strip_domain_discovery_sections(&spec.content));
        corpus.push('\n');
    }
    if let Ok(tasks) = fs::read_to_string(change.path.join("tasks.md")) {
        corpus.push_str(&strip_domain_discovery_sections(&tasks));
    }
    corpus
}

fn strip_domain_discovery_sections(markdown: &str) -> String {
    const DISCOVERY_SECTIONS: &[&str] = &[
        "Domain Discovery Summary",
        "Ubiquitous Language",
        "Rejected Aliases / Overloaded Terms",
        "Bounded Context Map",
        "Model Ownership",
        "Consistency Requirements",
        "Technique Fit",
        "Event Storming Snapshot",
        "Evidence Checked",
        "Proposed Documentation Updates",
    ];

    let mut stripped = String::new();
    let mut skipping = false;
    for line in markdown.lines() {
        if let Some(title) = line.trim().strip_prefix("## ") {
            skipping = DISCOVERY_SECTIONS
                .iter()
                .any(|section| title.trim().eq_ignore_ascii_case(section));
        }
        if skipping {
            continue;
        }
        stripped.push_str(line);
        stripped.push('\n');
    }
    stripped
}

fn normalized_text_contains_term(text: &str, term: &str) -> bool {
    let term = normalize_for_domain_match(term);
    if term.is_empty() {
        return false;
    }

    for (idx, _) in text.match_indices(&term) {
        let before = text
            .get(..idx)
            .and_then(|prefix| prefix.chars().next_back());
        let after = text
            .get(idx + term.len()..)
            .and_then(|suffix| suffix.chars().next());
        if is_term_boundary(before) && is_term_boundary(after) {
            return true;
        }
    }
    false
}

fn corpus_suggests_cross_context(corpus: &str) -> bool {
    let normalized = normalize_for_domain_match(corpus);
    let strong_signal = [
        "cross-context",
        "cross context",
        "multiple bounded contexts",
        "more than one bounded context",
        "spans multiple bounded contexts",
        "spans multiple contexts",
        "between bounded contexts",
        "across bounded contexts",
    ]
    .iter()
    .any(|needle| normalized.contains(needle));
    if strong_signal {
        return true;
    }

    [
        "coordinates with",
        "coordinate with",
        "integrates with",
        "integrate with",
        "collaborates with",
        "collaborate with",
        "communicates with",
        "communicate with",
        "depends on",
        "shared between",
        "shared across",
    ]
    .iter()
    .any(|needle| named_context_sentence_contains(corpus, needle))
}

fn named_context_sentence_contains(corpus: &str, needle: &str) -> bool {
    corpus.split(['.', '\n']).any(|sentence| {
        let normalized_sentence = normalize_for_domain_match(sentence);
        normalized_sentence.contains(needle)
            && !mentions_external_integration(&normalized_sentence)
            && (named_context_count(sentence) >= 2
                || relationship_phrase_has_domain_terms(&normalized_sentence, needle))
    })
}

fn mentions_external_integration(sentence: &str) -> bool {
    sentence.contains("external")
        || sentence.contains("third-party")
        || sentence.contains("third party")
        || sentence.contains("vendor")
        || sentence.contains("stripe")
}

fn relationship_phrase_has_domain_terms(sentence: &str, needle: &str) -> bool {
    let Some((left, right)) = sentence.split_once(needle) else {
        return false;
    };
    last_domainish_word(left).is_some() && first_domainish_word(right).is_some()
}

fn last_domainish_word(value: &str) -> Option<&str> {
    value
        .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .rev()
        .find(|word| !is_ignored_relationship_word(word))
        .filter(|word| is_domainish_word(word))
}

fn first_domainish_word(value: &str) -> Option<&str> {
    value
        .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .find(|word| !is_ignored_relationship_word(word))
        .filter(|word| is_domainish_word(word))
}

fn is_domainish_word(word: &str) -> bool {
    word_looks_like_domain_context(word) && !is_ignored_relationship_word(word)
}

fn is_ignored_relationship_word(word: &str) -> bool {
    let word = normalize_for_domain_match(word);
    if word.is_empty() {
        return true;
    }
    matches!(
        word.as_str(),
        "and"
            | "are"
            | "but"
            | "for"
            | "from"
            | "need"
            | "needs"
            | "not"
            | "the"
            | "this"
            | "through"
            | "with"
    )
}

fn word_looks_like_domain_context(word: &str) -> bool {
    let word = normalize_for_domain_match(word);
    word.len() > 2
        && (word.ends_with('s')
            || word.ends_with("ing")
            || word.ends_with("ment")
            || word.ends_with("tion")
            || word.ends_with("ship")
            || matches!(
                word.as_str(),
                "auth"
                    | "catalog"
                    | "checkout"
                    | "fulfillment"
                    | "inventory"
                    | "invoice"
                    | "payment"
                    | "support"
                    | "user"
                    | "workspace"
            ))
}

fn named_context_count(sentence: &str) -> usize {
    sentence
        .split(|ch: char| !ch.is_alphanumeric() && ch != '_')
        .filter(|word| {
            let Some(first) = word.chars().next() else {
                return false;
            };
            first.is_ascii_uppercase()
                && word_looks_like_domain_context(word)
                && !matches!(
                    *word,
                    "A" | "An" | "And" | "But" | "For" | "If" | "In" | "The" | "This"
                )
        })
        .count()
}

fn is_term_boundary(ch: Option<char>) -> bool {
    let Some(ch) = ch else {
        return true;
    };
    !ch.is_alphanumeric() && ch != '_'
}

fn same_domain_term(a: &str, b: &str) -> bool {
    normalize_for_domain_match(a) == normalize_for_domain_match(b)
}

fn same_definition(a: &str, b: &str) -> bool {
    normalize_for_domain_match(a) == normalize_for_domain_match(b)
}

fn terms_belong_to_different_contexts(a: &DomainTerm, b: &DomainTerm) -> bool {
    is_meaningful_domain_value(&a.owner_context)
        && is_meaningful_domain_value(&b.owner_context)
        && !same_domain_term(&a.owner_context, &b.owner_context)
}

fn normalize_for_domain_match(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    for word in value.split_whitespace() {
        if !normalized.is_empty() {
            normalized.push(' ');
        }
        normalized.push_str(word);
    }
    normalized.make_ascii_lowercase();
    normalized
}

fn collect_domain_document_paths(change_path: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect_domain_document_paths_inner(change_path, &mut out, 0);
    out.sort();
    out
}

fn collect_existing_domain_document_paths(
    project_root: Option<&Path>,
    change_path: &Path,
) -> Vec<PathBuf> {
    let Some(project_root) = project_root else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_existing_domain_document_paths_inner(project_root, change_path, &mut out, 0, false);
    for root_name in ["docs", "adr", "adrs", "decisions"] {
        collect_existing_domain_document_paths_inner(
            &project_root.join(root_name),
            change_path,
            &mut out,
            0,
            true,
        );
    }
    out.sort();
    out.dedup();
    out
}

const EMBEDDED_HANDOFF_MAX_DEPTH_INCLUSIVE: usize = 4;
const DOMAIN_DOCUMENT_MAX_DEPTH_INCLUSIVE: usize = 4;

fn collect_domain_document_paths_inner(dir: &Path, out: &mut Vec<PathBuf>, depth: usize) {
    if depth > DOMAIN_DOCUMENT_MAX_DEPTH_INCLUSIVE {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            collect_domain_document_paths_inner(&path, out, depth + 1);
            continue;
        }
        if !metadata.is_file() || !is_domain_document_path(&path) {
            continue;
        }
        out.push(path);
    }
}

fn collect_existing_domain_document_paths_inner(
    dir: &Path,
    change_path: &Path,
    out: &mut Vec<PathBuf>,
    depth: usize,
    recursive: bool,
) {
    if depth > DOMAIN_DOCUMENT_MAX_DEPTH_INCLUSIVE || dir.starts_with(change_path) {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            if !recursive {
                continue;
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            if matches!(
                name,
                ".git" | ".brv" | ".ito" | "target" | "node_modules" | "vendor" | "dist"
            ) {
                continue;
            }
            collect_existing_domain_document_paths_inner(&path, change_path, out, depth + 1, true);
            continue;
        }
        if !metadata.is_file() || !is_domain_document_path(&path) {
            continue;
        }
        out.push(path);
    }
}

fn is_domain_document_path(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let name = name.to_ascii_lowercase();
    if name == "context.md" || name == "context-map.md" {
        return true;
    }

    let Some(rest) = name.strip_prefix("adr") else {
        return false;
    };
    if rest == ".md" || rest.starts_with('-') || rest.starts_with('_') {
        return name.ends_with(".md");
    }
    rest.chars().next().is_some_and(|ch| ch.is_ascii_digit()) && name.ends_with(".md")
}

fn report_path_for_domain_document(
    change_path: &Path,
    project_root: Option<&Path>,
    path: &Path,
) -> String {
    if let Ok(path) = path.strip_prefix(change_path) {
        return path.display().to_string();
    }
    if let Some(project_root) = project_root
        && let Ok(path) = path.strip_prefix(project_root)
    {
        return path.display().to_string();
    }
    path.display().to_string()
}
