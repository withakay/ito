# 029-02_agent-memory-abstraction

Make agent memory a first-class part of Ito via a loose, provider-agnostic abstraction. Configure which memory provider Ito uses by naming the store/search commands (or a delegated skill) instead of hard-coding any specific backend. Update apply and finish instruction templates to remind agents to capture useful memories at the end of work, and have finish refresh the archive/specs as part of wrap-up. No default provider.
