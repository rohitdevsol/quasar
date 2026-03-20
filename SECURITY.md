# Security Policy

> **Quasar has not been audited.** Do not use it in production with real funds until an audit is complete. There is no bug bounty program at this time.

## Reporting a Vulnerability

If you discover a security vulnerability, **report it privately** — do not open a public issue.

**Email:** [leo@blueshift.gg](mailto:leo@blueshift.gg)

Include:
- Description of the vulnerability
- Steps to reproduce
- Affected crate(s)
- Suggested fix (if any)

We will acknowledge receipt within 48 hours.

## Scope

This policy covers:

- `quasar-lang` — framework primitives, zero-copy access, CPI builder
- `quasar-derive` — proc macros
- `quasar-spl` — SPL Token integration

## Unsafe Code

Quasar uses `unsafe` for zero-copy access, CPI syscalls, and pointer casts. Every `unsafe` block has a documented soundness invariant and is validated by Miri under Tree Borrows with symbolic alignment checking.

If you find an `unsafe` block that lacks a soundness argument or can be triggered to produce undefined behavior, that qualifies as a security vulnerability.
