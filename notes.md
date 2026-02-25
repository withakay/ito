### We should think about dating the skills for creating a proposal such that it respects the sch...

We should think about dating the skills for creating a proposal such that it respects the schemas and suggests an appropriate change schema, so a change type, I guess, from those that are available.


---

### Let's see how fast this is. Maybe we need to think about which model it uses. If it's fast, I...

Let's see how fast this is. Maybe we need to think about which model it uses. If it's fast, I would like it to actually possibly improve what's been said.


---

---
## 2026-02-25 07:17:32 UTC - Reply with exactly noted

After the tool succeeds, reply with exactly: noted


---

---
## 2026-02-25 07:21:21 UTC - One more check

Okay, one more check. It's probably something that I could have said here, but I'm going to. 

If empty or whitespace-only, output exactly: no text
If not empty, use the `note_append` tool and output exactly the tool result.


---

---
## 2026-02-25 07:21:46 UTC - user-note

Is this quicker? I think this is quicker.


---

### agent-instruction perf proposal

Change idea: 016-13_optimize-agent-instructions
- Goal: speed up `ito agent instruction apply` by removing default blocking `git fetch` and caching cascading config per invocation.
- Specs: agent-instructions, change-coordination-branch, cascading-config.
- Measured: apply ~1.35s vs others ~15ms; fetch is main cost.
