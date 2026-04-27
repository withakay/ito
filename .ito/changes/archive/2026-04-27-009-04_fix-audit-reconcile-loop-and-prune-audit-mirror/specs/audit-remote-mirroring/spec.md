<!-- ITO:START -->
## ADDED Requirements

### Requirement: Bounded Internal Audit Mirror

The internal audit branch SHALL store a bounded audit log by retaining only parseable audit events within 30 days of the newest merged event and no more than the newest 1000 retained events.

#### Scenario: Events older than retention window are pruned
- **WHEN** audit mirror sync merges events with timestamps older than 30 days relative to the newest event
- **THEN** the Git-stored audit log excludes those older events

#### Scenario: Event count exceeds cap
- **WHEN** audit mirror sync would store more than 1000 retained events
- **THEN** the Git-stored audit log keeps only the newest 1000 events

### Requirement: Aggregated Reconcile Noise

The internal audit branch SHALL aggregate adjacent equivalent `reconciled` events by storing a single event with `count` greater than 1 instead of repeated lines that differ only by timestamp or execution context.

#### Scenario: Equivalent reconcile events repeat sequentially
- **WHEN** audit mirror sync observes adjacent equivalent `reconciled` events
- **THEN** the Git-stored audit log increments the first event's `count`
- **AND** does not append a duplicate event line

#### Scenario: Different event separates reconcile events
- **WHEN** a different event occurs between two equivalent `reconciled` events
- **THEN** the later `reconciled` event is stored as a separate line
<!-- ITO:END -->
