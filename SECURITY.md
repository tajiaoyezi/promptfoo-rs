# Security Policy

Thank you for helping keep promptfoo-rs safe.

## Supported versions

promptfoo-rs is pre-1.0. Security fixes target the latest `master` branch unless a release branch is explicitly documented later.

| Version | Supported |
|---|---|
| `master` | Yes |
| Tagged pre-release builds | Best effort |
| Older snapshots | No |

## Reporting a vulnerability

Do not open a public GitHub issue for a vulnerability.

Preferred channel:

1. Use GitHub Security Advisories for `tajiaoyezi/promptfoo-rs`.
2. Include affected commit or version, operating system, reproduction steps, expected impact, and any proof of concept.
3. Avoid including live provider tokens, registry credentials, or private service data. Redact secrets before attaching logs.

If a vulnerability is also present in upstream promptfoo, report it to the appropriate upstream project as well.

## Scope

In scope:

- Command injection, sandbox bypass, or unintended script execution.
- Secret leakage through logs, result artifacts, release gates, or compatibility reports.
- Path traversal or unsafe filesystem writes.
- Unintended upload, share, cloud mutation, or remote delete behavior.
- Vulnerabilities in config parsing, provider execution, output writing, local viewer data handling, or Node wrapper boundaries.

Out of scope:

- Missing public publication credentials.
- Known compatibility gaps already recorded in the matrix or release gates.
- Vulnerabilities in third-party services that promptfoo-rs only calls when the user explicitly configures them.
- Reports that require committing or sharing real secrets.

## Response expectations

This is a small maintainer project. Target response times:

| Step | Target |
|---|---|
| Initial acknowledgement | 72 hours |
| Triage | 7 days |
| Fix or mitigation plan for high severity issues | 30 days |
| Coordinated disclosure | After a fix is available, or within 90 days when appropriate |

## Security defaults

promptfoo-rs is local-first by default:

- Cloud/share/auth/delete compatibility commands fail closed unless explicitly implemented with authority.
- Custom script execution is disabled unless explicitly authorized.
- Release evidence must not store credentials.
- Provider/API key material must be redacted from artifacts and diagnostics.

See [docs/release.md](docs/release.md) and [docs/PROJECT-OVERVIEW.md](docs/PROJECT-OVERVIEW.md) for the release and security boundaries, and [PRIVACY.md](PRIVACY.md) for the privacy and telemetry statement.
