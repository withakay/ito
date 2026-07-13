<!-- ITO:START -->
## ADDED Requirements

### Requirement: Update prunes retired managed skill surfaces safely
`ito update` and `ito init --upgrade` SHALL compare installed Ito-managed skill and command paths with the canonical lifecycle inventory and remove obsolete managed assets only when ownership and user-content checks permit safe deletion.

#### Scenario: Managed-only retired asset is removed
- **GIVEN** a retired skill or command contains only Ito-managed content at a known legacy path
- **WHEN** update cleanup runs
- **THEN** the obsolete file tree is removed
- **AND** empty Ito-managed parent directories are pruned

#### Scenario: Broken managed symlink is removed
- **GIVEN** a known retired asset path is a broken symlink
- **WHEN** update cleanup runs
- **THEN** symlink-aware metadata identifies and removes the obsolete link

#### Scenario: User content is preserved
- **GIVEN** a retired managed Markdown asset contains content outside its Ito-managed block
- **WHEN** update cleanup runs
- **THEN** user content is not deleted
- **AND** the command reports the path and retained lifecycle replacement

#### Scenario: Repeated update is stable
- **GIVEN** obsolete managed assets have been removed and the seven retained skills are current
- **WHEN** update runs again
- **THEN** no managed skill or command file changes
<!-- ITO:END -->
