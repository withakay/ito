---
title: Template Bundle Retrofit
summary: Plain markdown files under ito-templates/assets were retrofitted with ITO start/end markers; pre-marked files remained unchanged and no adapter sample was modified.
tags: []
related: []
keywords: []
createdAt: '2026-04-24T21:05:41.424Z'
updatedAt: '2026-04-24T21:05:41.424Z'
---
## Reason
Document marker retrofit for template assets markdown files

## Raw Concept
**Task:**
Retrofit template bundle markdown assets with ITO markers

**Changes:**
- Added <!-- ITO:START --> and <!-- ITO:END --> markers to all plain markdown files in assets
- Skipped files that were already pre-marked
- Confirmed assets/adapters contained no unmarked plain markdown file to modify

**Files:**
- ito-rs/crates/ito-templates/assets
- ito-rs/crates/ito-templates/assets/adapters

**Flow:**
scan assets -> add markers to plain markdown -> leave pre-marked files unchanged -> verify adapter sample status

**Timestamp:** 2026-04-24

**Author:** Template bundle retrofit verification

## Narrative
### Structure
This update applies across the template bundle assets directory, with special handling for pre-marked markdown files and adapter samples.

### Dependencies
Depends on distinguishing plain markdown files from files that already include ITO markers.

### Highlights
The retrofit standardized marker presence without rewriting already compliant files. Verification found no additional adapter markdown requiring modification.

### Rules
Only plain .md files under assets should be retrofitted with markers; already marked files must remain unchanged.

## Facts
- **template_markers**: All plain .md files under ito-rs/crates/ito-templates/assets now carry <!-- ITO:START --> and <!-- ITO:END --> markers. [project]
- **pre_marked_files**: Existing pre-marked files were left unchanged. [project]
- **adapter_sample_verification**: The only plain markdown file under assets/adapters was already marked, so no adapter sample was modified during verification. [project]
