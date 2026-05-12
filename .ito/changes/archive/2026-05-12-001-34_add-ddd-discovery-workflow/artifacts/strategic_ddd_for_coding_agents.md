# Strategic Domain-Driven Design Guide for Coding Agents

## Purpose

Use this guide when planning, decomposing, or implementing features in a domain-rich codebase. The goal of strategic Domain-Driven Design is not to create abstract architecture for its own sake. The goal is to preserve the business model in code, make feature boundaries explicit, and avoid accidental coupling between concepts that should evolve independently.

A coding agent should use this guide before writing or changing code whenever a task introduces new business behavior, changes workflow rules, touches multiple modules, or appears to span more than one domain concept.

---

## Core Principle

Strategic DDD is about deciding **where a concept belongs**, **what language should describe it**, and **which boundaries must be protected**.

Before implementation, answer:

1. What business capability is this feature part of?
2. Which bounded context owns the concepts being changed?
3. What words does the business use for these concepts?
4. Are we modifying an existing model, integrating two models, or accidentally mixing them?
5. What should be explicit at the boundary between contexts?

If those answers are unclear, pause and clarify the model before coding.

---

## 1. Start with Business Capability, Not Code Location

Do not begin by asking, “Which file should I edit?” Begin by asking, “What business capability is changing?”

Examples of business capabilities:

- Quoting
- Billing
- Fulfillment
- Scheduling
- Identity and access
- Inventory management
- Claims processing
- Risk assessment
- Customer onboarding

A feature should be planned around the capability it supports. Code structure should follow the domain boundary, not merely the current folder layout.

### Agent checklist

Before implementing:

- Identify the primary business capability.
- Identify secondary capabilities affected by the change.
- Avoid placing new behavior in a shared utility, generic service, or global model unless the concept is genuinely cross-cutting.
- Prefer adding behavior near the model that owns the decision.

### Warning signs

Be cautious if the plan says:

- “Add this to `common`.”
- “Put this in a helper.”
- “Reuse the existing `Order` object everywhere.”
- “Just add a flag.”
- “This service already has access to the data, so put it there.”

These often indicate boundary erosion.

---

## 2. Identify the Bounded Context

A bounded context is a boundary within which a particular domain model and language are valid.

The same word may mean different things in different contexts.

Example:

- In Sales, an “Order” may mean a customer intent to buy.
- In Fulfillment, an “Order” may mean a pick-pack-ship instruction.
- In Billing, an “Order” may mean a chargeable commercial commitment.

Do not force these into one universal `Order` model unless the business truly treats them as the same thing.

### Agent checklist

For every feature, determine:

- Which bounded context owns the primary behavior?
- Are any other contexts being referenced?
- Are we using another context’s model directly?
- Is translation needed at the boundary?
- Are names in the code valid within this context?

### Good agent behavior

Instead of:

> “The feature touches orders, so I will update the shared `Order` class.”

Prefer:

> “This changes fulfillment behavior. I need to determine whether Fulfillment owns its own representation of an order, or whether it is incorrectly depending on Sales’ order model.”

---

## 3. Use Ubiquitous Language Deliberately

Ubiquitous language is the shared language used by domain experts and developers inside a bounded context.

A coding agent should preserve and refine this language in code. Names are not cosmetic; they are part of the model.

### What to look for

Use domain terms for:

- Types
- Methods
- Events
- Commands
- Policies
- State transitions
- Invariants
- Test names

Avoid technical placeholders for domain concepts.

Prefer:

```text
ApproveClaim
QuoteExpired
ShipmentReadyForDispatch
CreditLimitExceeded
RenewSubscription
```

Avoid:

```text
ProcessData
HandleThing
DoOrderLogic
UpdateStatus
RunValidation
```

### Agent checklist

When adding or changing code:

- Use names from the feature request, product docs, tests, or domain conversations.
- Do not invent generic names if precise business terms are available.
- If two terms appear similar, do not merge them automatically.
- If one term is used inconsistently, call it out in the implementation notes.

### Rule of thumb

If the code cannot be explained to a domain expert using its own names, the model is probably weak.

---

## 4. Distinguish Strategic Design from Tactical Patterns

Strategic DDD is not primarily about Entities, Value Objects, Aggregates, or Repositories. Those are tactical tools.

Strategic DDD asks:

- Which model applies here?
- Who owns this concept?
- Where is the boundary?
- How do contexts collaborate?
- Which language is valid in this part of the system?

Tactical DDD asks:

- Is this an Entity or Value Object?
- What is the Aggregate root?
- Where should persistence happen?
- Which invariant belongs inside the domain model?

Use tactical patterns only after the strategic boundary is clear.

### Agent rule

Do not introduce tactical DDD structures to compensate for unclear boundaries. First clarify the bounded context and ownership.

---

## 5. Prefer Model Ownership Over Data Ownership

A database table does not necessarily define the domain owner.

A context owns a concept if it owns the rules, decisions, language, and lifecycle for that concept.

Example:

A `customer` table may be used by Support, Billing, and Marketing. But each context may care about different meanings:

- Support: customer as a person needing help
- Billing: customer as a paying account
- Marketing: customer as a segmentable audience member

Do not let shared storage imply shared domain model.

### Agent checklist

When a feature involves existing data:

- Identify who owns the business decision, not just who owns the table.
- Avoid leaking one context’s internal model into another context.
- Prefer read models, projections, APIs, or events for cross-context access.
- Treat direct database access across contexts as a coupling smell.

---

## 6. Context Mapping: Choose the Relationship Explicitly

When two bounded contexts interact, define the relationship. Do not let accidental imports, shared DTOs, or database joins become the architecture.

Common context relationships:

### Customer/Supplier

One context provides data or behavior consumed by another. The upstream context influences the downstream model.

Use when:

- One team or module clearly owns a capability.
- Consumers adapt to the provider’s published contract.

Agent guidance:

- Depend on a stable API, event, or contract.
- Avoid depending on upstream internals.
- Add translation in the downstream context if terms differ.

### Conformist

A downstream context adopts the upstream model as-is.

Use sparingly, when:

- The upstream model is authoritative.
- There is little benefit in creating a separate model.
- The downstream context has limited domain complexity.

Agent guidance:

- Make the dependency explicit.
- Avoid pretending the downstream has an independent model.

### Anti-Corruption Layer

A boundary layer translates between an external or unsuitable model and the local domain model.

Use when:

- Integrating with legacy systems.
- Consuming third-party APIs.
- Protecting a rich domain model from another context’s language.
- The external model is unstable, awkward, or conceptually different.

Agent guidance:

- Do not spread external DTOs throughout the domain.
- Translate at the boundary.
- Keep mapping logic explicit and tested.

### Shared Kernel

Two contexts deliberately share a small part of the model.

Use rarely, when:

- The shared concepts are stable.
- The teams coordinate closely.
- The shared model is genuinely identical in both contexts.

Agent guidance:

- Keep the shared kernel small.
- Do not use it as a dumping ground.
- Any change should be treated as cross-context coordination.

### Separate Ways

Contexts do not integrate directly.

Use when:

- The cost of integration exceeds the value.
- Similar concepts do not need to be unified.
- Duplication is safer than coupling.

Agent guidance:

- Do not deduplicate merely because names look similar.
- Prefer independent models when behavior differs.

---

## 7. Protect Boundaries with Translation

Cross-context boundaries should translate concepts deliberately.

Translation may happen through:

- API adapters
- Event handlers
- Message consumers
- Application services
- Anti-corruption layers
- Read model projectors
- Import/export mappers

A boundary should answer:

- What does the upstream context call this?
- What does the local context call this?
- What assumptions are being converted?
- What data is ignored because it does not belong locally?
- What local invariants must be enforced after translation?

### Agent rule

Never pass another context’s domain object deep into the local domain model. Convert it into a local concept first.

---

## 8. Be Suspicious of Generic Models

Generic names often hide missing domain understanding.

Be cautious with:

- `Item`
- `Record`
- `Object`
- `Entity`
- `Data`
- `Payload`
- `Status`
- `Type`
- `Manager`
- `Processor`
- `Handler`

These names may be acceptable at technical boundaries, but they should not dominate the domain model.

### Agent checklist

When encountering generic models:

- Look for domain-specific alternatives.
- Check whether one generic model is serving several contexts.
- Avoid adding more flags or branches if separate concepts are emerging.
- Consider whether a new bounded context, policy, or domain service is needed.

---

## 9. Watch for Boundary-Smell Feature Requests

Some feature requests sound simple but imply strategic design decisions.

### “Add a status”

Ask:

- Status of what, in which context?
- Is this lifecycle state or reporting state?
- Who is allowed to change it?
- Does this represent a new business process?
- Are different contexts using the same status differently?

### “Reuse the existing model”

Ask:

- Is the meaning identical?
- Are the invariants identical?
- Will both uses evolve together?
- Is reuse reducing duplication or creating coupling?

### “Just sync the data”

Ask:

- Which context is authoritative?
- Is this integration synchronous or asynchronous?
- What happens when data conflicts?
- Is eventual consistency acceptable?
- What language should the receiving context use?

### “Expose this field”

Ask:

- Is this field part of a public contract?
- Does exposing it leak internal model details?
- Should it be transformed into a local/read model concept?

### “Put it in shared”

Ask:

- Is it truly shared domain language?
- Is it technical infrastructure?
- Will multiple contexts need to coordinate every change?
- Would duplication be safer?

---

## 10. Plan Features as Domain Changes

For a non-trivial feature, produce a short strategic design note before coding.

### Recommended format

```text
Feature:

Business capability:

Primary bounded context:

Supporting contexts:

Key domain terms:

Owned concepts changed:

External concepts referenced:

Boundary relationships:

Translation required:

New or changed domain events:

Invariants / business rules:

Consistency requirements:

Open modeling questions:

Implementation implications:
```

### Example

```text
Feature:
Allow customers to reserve inventory before checkout.

Business capability:
Inventory reservation.

Primary bounded context:
Inventory.

Supporting contexts:
Checkout, Catalog.

Key domain terms:
Reservation, Available Stock, Reserved Stock, Reservation Expiry.

Owned concepts changed:
InventoryReservation, StockAvailability.

External concepts referenced:
Cart from Checkout, SKU from Catalog.

Boundary relationships:
Checkout requests a reservation from Inventory. Catalog provides SKU identity but does not own availability.

Translation required:
Checkout CartItem -> Inventory ReservationRequest.

New or changed domain events:
InventoryReserved, ReservationExpired, ReservationReleased.

Invariants / business rules:
Cannot reserve more than available stock. Reservation must expire after configured duration.

Consistency requirements:
Reservation can be eventually reflected in Checkout, but Inventory must enforce stock invariants transactionally.

Open modeling questions:
Can reservations be extended? Can partial reservations succeed?

Implementation implications:
Do not add reservation state to Checkout cart item as the source of truth. Add an Inventory-owned reservation model and expose a boundary API/event.
```

---

## 11. Use Events to Represent Cross-Context Facts

Domain events are useful when one context needs to announce that something meaningful happened.

Good event names are past-tense business facts:

```text
OrderPlaced
PaymentAuthorized
ClaimApproved
SubscriptionRenewed
InventoryReserved
InvoiceIssued
```

Avoid vague technical events:

```text
OrderUpdated
DataChanged
StatusModified
EntitySaved
```

### Agent checklist

When adding an event:

- Name it as a business fact.
- Emit it from the context that owns the fact.
- Do not include another context’s internal objects.
- Include enough information for consumers to react without depending on internals.
- Do not use events as a substitute for unclear ownership.

### Important distinction

A domain event says, “This business fact occurred.”

It should not say, “Please perform this technical action.”

---

## 12. Decide Consistency Requirements Strategically

Not every workflow needs immediate consistency across contexts.

Ask:

- Which invariants must be enforced immediately?
- Which updates can be eventually consistent?
- What is the business impact of stale data?
- Who owns conflict resolution?
- What should happen if a downstream system is unavailable?

### Agent guidance

Use strong consistency inside a context for invariants owned by that context.

Use eventual consistency across contexts when possible.

Do not create distributed transactions across contexts unless the business truly requires it and the architecture supports it deliberately.

---

## 13. Avoid Anemic “Workflow Scripts” for Core Domain Logic

Application services can orchestrate use cases, but they should not become the only place business rules live.

If a rule expresses domain knowledge, it should usually live in the domain model, a policy, or a domain service owned by the context.

### Smell

```text
CheckoutService checks inventory, applies customer discount rules, validates payment risk, updates shipment state, and sends invoice.
```

This may indicate multiple contexts are being mixed into one transaction script.

### Better strategic split

- Checkout coordinates customer intent.
- Inventory owns stock reservation.
- Pricing owns discounts.
- Payments owns authorization.
- Fulfillment owns shipment preparation.
- Billing owns invoicing.

The implementation may still use an application service, but it should coordinate clear domain boundaries rather than absorb all domain logic.

---

## 14. Prefer Explicit Policies for Variable Business Rules

When business rules vary by market, customer type, plan, jurisdiction, or product line, model that variation explicitly.

Use terms like:

- EligibilityPolicy
- PricingPolicy
- RenewalPolicy
- CancellationPolicy
- RiskPolicy
- AllocationPolicy
- ApprovalPolicy

Avoid scattering conditional logic across handlers and controllers.

### Agent checklist

If adding conditionals, ask:

- Is this a named business rule?
- Does the business discuss this as a policy?
- Will it vary over time?
- Should it be isolated behind a domain concept?

---

## 15. Distinguish Commands, Queries, and Events

Strategic clarity improves when message types have clear intent.

### Command

A request to do something.

```text
ReserveInventory
ApproveClaim
CancelSubscription
```

Commands may fail because business rules may reject them.

### Query

A request to know something.

```text
GetAvailableStock
FindEligiblePlans
CalculateQuotePreview
```

Queries should not change domain state.

### Event

A statement that something happened.

```text
InventoryReserved
ClaimApproved
SubscriptionCancelled
```

Events are facts and should not be rejected by consumers.

### Agent rule

Do not name a command as an event or an event as a command. This confuses ownership and lifecycle.

---

## 16. Use Tests to Express the Domain Model

Tests should reinforce the ubiquitous language.

Prefer scenario names like:

```text
cannot_reserve_more_stock_than_is_available
expired_reservation_releases_stock
premium_customer_can_extend_reservation_once
invoice_is_issued_after_payment_is_captured
```

Avoid tests like:

```text
test_handler_updates_status
test_process_success
test_service_works
test_validation
```

### Agent checklist

When adding tests:

- Name tests using business behavior.
- Test context-owned invariants close to the domain model.
- Test boundary translation separately.
- Avoid asserting another context’s internal implementation details.

---

## 17. Refactoring Toward Strategic DDD

When existing code is not well-bounded, do not attempt a large rewrite unless asked. Improve boundaries incrementally.

Useful refactoring moves:

- Rename concepts to match domain language.
- Move behavior closer to the owning model.
- Introduce an anti-corruption layer around external models.
- Split a generic model into context-specific models.
- Extract policy objects for named business rules.
- Replace shared mutable state with published events or APIs.
- Add tests that capture domain rules before changing structure.

### Agent rule

Prefer small, safe boundary improvements that support the requested feature. Do not perform speculative architecture work unrelated to the task.

---

## 18. Decision Heuristics for Coding Agents

### When to create a new bounded context

Consider it when:

- The language differs from nearby code.
- The lifecycle differs.
- The business owner differs.
- The rules change for different reasons.
- Existing models require many flags or conditionals to support the feature.
- Teams or modules need to evolve independently.

Do not create one just because a new feature exists.

### When to reuse an existing model

Reuse when:

- The meaning is the same.
- The invariants are the same.
- The lifecycle is the same.
- The owner is the same.
- Future changes are likely to apply to both uses.

### When to duplicate concepts

Duplicate when:

- Two contexts use similar data with different meaning.
- The cost of coupling is higher than the cost of duplication.
- Each context needs independent evolution.
- Translation is clearer than shared abstraction.

Duplication of data can be acceptable. Duplication of business rules may not be.

### When to introduce an anti-corruption layer

Introduce one when:

- External models leak into domain logic.
- Third-party terms are spreading through local code.
- Legacy structures do not match the local domain.
- Mapping rules are non-trivial or business-relevant.

### When to add a domain event

Add one when:

- A meaningful business fact occurred.
- Other contexts need to react.
- The producing context should not know all consumers.
- Eventual consistency is acceptable.

Do not add events merely to avoid calling a function inside the same cohesive model.

---

## 19. Agent Planning Prompt

Use this prompt before implementing a domain feature:

```text
Analyze this feature using strategic Domain-Driven Design before coding.

1. Identify the business capability.
2. Identify the primary bounded context and any supporting contexts.
3. List the key domain terms and ensure names match the local ubiquitous language.
4. Determine which context owns each concept, rule, and lifecycle change.
5. Identify any cross-context relationships and choose an explicit context mapping pattern.
6. Decide whether translation or an anti-corruption layer is needed.
7. Identify domain events, commands, queries, policies, and invariants.
8. Determine consistency requirements inside and across contexts.
9. Point out modeling ambiguity, boundary smells, or risky coupling.
10. Propose the smallest implementation plan that preserves the domain boundaries.

Do not begin coding until the strategic model is clear enough to avoid mixing contexts.
```

---

## 20. Agent Implementation Rules

When implementing:

1. Keep business behavior in the context that owns it.
2. Do not pass foreign domain objects deep into local domain logic.
3. Translate at boundaries.
4. Name code using the local ubiquitous language.
5. Prefer explicit policies over scattered conditionals.
6. Prefer events for cross-context facts, not technical notifications.
7. Avoid shared kernels unless the shared concept is truly stable and identical.
8. Do not infer ownership from database tables alone.
9. Do not add flags to generic models without checking whether a new concept is emerging.
10. Test business behavior using domain language.

---

## 21. Common Mistakes to Avoid

### Mistake: Treating DDD as folder structure

DDD is not achieved by creating folders named `domain`, `application`, and `infrastructure`. Boundaries and language matter more than folder names.

### Mistake: Creating one enterprise-wide model

Large systems usually need multiple models. A universal model often becomes vague, overloaded, and fragile.

### Mistake: Sharing DTOs across contexts

Shared DTOs can silently couple contexts. Prefer published contracts and local translation.

### Mistake: Making all integration synchronous

Synchronous calls across contexts can create temporal coupling and fragile workflows. Use events and eventual consistency where appropriate.

### Mistake: Hiding domain rules in controllers or handlers

Controllers, handlers, and application services should coordinate. They should not become the only place where business decisions live.

### Mistake: Using technical names for business concepts

Poor names make the model harder to reason about and easier to corrupt.

---

## 22. Final Pre-Code Review Checklist

Before writing code, the agent should be able to state:

```text
This feature belongs primarily to: <bounded context>

The business capability is: <capability>

The key domain concepts are: <terms>

This context owns: <rules/lifecycle/concepts>

Other contexts involved are: <contexts>

The relationship between contexts is: <context mapping pattern>

Translation is needed at: <boundary>

The main invariants are: <rules>

The consistency requirement is: <strong/eventual and why>

The implementation will avoid: <specific coupling risk>
```

If any of these are unknown, the agent should either ask for clarification or make the uncertainty explicit in the plan.

---

## Summary

Strategic DDD helps a coding agent avoid treating feature work as isolated code edits. It encourages the agent to preserve boundaries, use precise domain language, and model business capabilities explicitly.

The best implementation is not merely the one that passes tests today. It is the one that keeps the model understandable, protects context boundaries, and allows different parts of the business to evolve without corrupting each other.

