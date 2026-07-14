# 029-02_agent-memory-abstraction

Make agent memory a first-class part of Ito via a loose, provider-agnostic abstraction. Configure `capture`, `search`, and `query` independently, with each operation choosing either a delegated skill or a rendered command template instead of hard-coding any specific backend. Update apply and finish instruction templates to remind agents to capture useful memories at the end of work, and have finish refresh archive/spec/doc follow-up without duplicating the existing archive prompt. No default provider.
