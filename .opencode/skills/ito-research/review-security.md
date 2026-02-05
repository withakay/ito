# Security Review

## Objective

Find security vulnerabilities in the proposed changes for: **{{change_id}}**

## Perspective

You are a security researcher. Assume attackers are sophisticated and motivated.
Find ways to exploit, bypass, or abuse the proposed system.

## Process

1. Read the proposal and affected specs
1. Map the attack surface
1. Identify vulnerabilities by category:
   - Authentication/authorization bypasses
   - Injection points (SQL, XSS, command, template)
   - Data exposure risks
   - CSRF/SSRF vulnerabilities
   - Cryptographic weaknesses
   - Race conditions
   - Supply chain risks

## Output Format

# Security Review: {{change_id}}

## Attack Surface

List entry points and trust boundaries.

## Findings

### \[CRITICAL/HIGH/MEDIUM/LOW\]: Finding Title

- **Location**: File/component affected
- **Attack Vector**: How an attacker could exploit this
- **Impact**: What damage could be done
- **Proof of Concept**: Example attack (if applicable)
- **Mitigation**: Required fix
- **Status**: [ ] Not addressed

## Recommendations

Proactive security improvements beyond specific findings.

## Verdict

- [ ] Approved for implementation
- [ ] Requires changes before implementation
- [ ] Needs significant redesign
