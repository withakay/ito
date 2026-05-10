# Cli Research Specification

## Purpose

Define the `cli-research` capability, including required behavior and validation scenarios, so it remains stable and testable.


## Requirements

### Requirement: Research initialization

The CLI SHALL initialize the `.ito/research/` directory structure with templates for structured domain investigation.

#### Scenario: Initialize research directory

- **WHEN** executing `ito x-research init`
- **THEN** create the `.ito/research/` directory if it does not exist
- **AND** create the `.ito/research/investigations/` subdirectory
- **AND** create `SUMMARY.md` template with sections for key findings, stack recommendations, feature prioritization, architecture considerations, pitfalls to avoid, and roadmap implications
- **AND** create `stack-analysis.md` template in investigations/ with sections for requirements, options evaluated, recommendation, and alternatives
- **AND** create `feature-landscape.md` template in investigations/ with sections for table stakes, differentiators, and competitive analysis
- **AND** create `architecture.md` template in investigations/ with sections for system design, data flow, and integration considerations
- **AND** create `pitfalls.md` template in investigations/ with sections for common mistakes, mitigations, and lessons learned
- **AND** display a success message indicating the research structure has been initialized
- **AND** skip creating any files that already exist to preserve existing content

### Requirement: Research status display

The CLI SHALL display the current state of research artifacts, indicating which investigations have been completed.

#### Scenario: Show research status

- **WHEN** executing `ito x-research status`
- **THEN** check for existence of `.ito/research/investigations/*.md` files
- **AND** display a table showing each investigation's name, status (complete/incomplete/missing), and last modified timestamp
- **AND** indicate whether SUMMARY.md exists and has content
- **AND** print a hint to run `ito x-research init` if the directory structure is missing
- **AND** suggest running specific investigations if they are incomplete

### Requirement: Research command templates

The CLI SHALL provide command templates that can be loaded by AI tools to guide structured research investigations.

#### Scenario: Generate stack analysis command template

- **WHEN** generating research command templates
- **THEN** create a template file with instructions for:
  - Identifying the domain and key technical requirements
  - Researching current best practices using web search
  - Evaluating library ecosystem and maturity
  - Documenting trade-offs between options
- **AND** specify that findings should be written to `.ito/research/investigations/stack-analysis.md`
- **AND** include a template structure with sections for Requirements, Options Evaluated (table with Option, Pros, Cons, Maturity), Recommendation, and Alternatives

#### Scenario: Generate feature landscape command template

- **WHEN** generating research command templates
- **THEN** create a template file with instructions for:
  - Identifying table stakes features (must-have functionality)
  - Identifying differentiators (competitive advantages)
  - Analyzing competitive landscape
  - Prioritizing features for roadmap
- **AND** specify that findings should be written to `.ito/research/investigations/feature-landscape.md`
- **AND** include a template structure with sections for Table Stakes, Differentiators, and Competitive Analysis

#### Scenario: Generate architecture command template

- **WHEN** generating research command templates
- **THEN** create a template file with instructions for:
  - Analyzing system architecture requirements
  - Identifying integration points and dependencies
  - Evaluating architectural patterns and trade-offs
  - Documenting design decisions and rationale
- **AND** specify that findings should be written to `.ito/research/investigations/architecture.md`
- **AND** include a template structure with sections for System Design, Data Flow, Integration Considerations, and Design Decisions

#### Scenario: Generate pitfalls command template

- **WHEN** generating research command templates
- **THEN** create a template file with instructions for:
  - Identifying common mistakes and failure modes in the domain
  - Researching lessons learned from similar projects
  - Identifying security, performance, and usability pitfalls
  - Recommending mitigations and best practices
- **AND** specify that findings should be written to `.ito/research/investigations/pitfalls.md`
- **AND** include a template structure with sections for Common Pitfalls, Mitigations, Security Considerations, and Lessons Learned

### Requirement: Research synthesis

The CLI SHALL provide guidance for synthesizing individual investigations into a cohesive summary.

#### Scenario: Generate summary template guidance

- **WHEN** generating research command templates
- **THEN** create a template file with instructions for:
  - Reading all investigation files in `.ito/research/investigations/`
  - Extracting key findings from each investigation
  - Synthesizing findings into a cohesive summary
  - Identifying implications for roadmap and execution
- **AND** specify that the summary should be written to `.ito/research/SUMMARY.md`
- **AND** include a template structure with sections for Key Findings, Stack Recommendations, Feature Prioritization, Architecture Considerations, Pitfalls to Avoid, and Implications for Roadmap

### Requirement: Research workflow integration

The CLI SHALL integrate research capabilities with the broader Ito workflow, enabling research to precede proposal creation.

#### Scenario: Research before proposal workflow

- **WHEN** a user begins planning a complex change that requires domain investigation
- **THEN** suggest running `ito x-research init` to create research structure
- **AND** provide guidance on which investigations to complete based on the change type
- **AND** indicate that research findings should inform the change proposal's "Why" and "What Changes" sections
- **AND** recommend referencing `.ito/research/SUMMARY.md` in the proposal for context

### Requirement: Error handling

The CLI SHALL provide clear error messages and recovery suggestions when research commands encounter issues.

#### Scenario: Research directory cannot be created

- **WHEN** the `.ito/research/` directory cannot be created due to permissions or filesystem errors
- **THEN** display an error message explaining the failure
- **AND** suggest checking directory permissions and disk space
- **AND** exit with code 1

#### Scenario: Investigation files are missing

- **WHEN** executing `ito x-research status` and investigation files are missing
- **THEN** display a warning that investigations are incomplete
- **AND** suggest running `ito x-research init` to create templates
- **AND** list which investigation files are missing

### Requirement: Template quality

The CLI SHALL generate high-quality templates that provide clear guidance for structured research and follow Ito conventions.

#### Scenario: Investigation templates follow best practices

- **WHEN** generating investigation templates
- **THEN** structure each template with clear sections and headings
- **AND** provide guidance on what content to include in each section
- **AND** include placeholder questions or prompts to guide the research process
- **AND** follow the format documented in project-planning-research-proposal.md
