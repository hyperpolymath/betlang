# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| < 1.0   | :x:                |

**Note:** betlang is currently in active development. We recommend using the latest commit from the `main` branch.

## Security Considerations for Probabilistic Code

betlang is a probabilistic programming DSL for modeling uncertainty. While the language itself has a minimal attack surface, users should be aware of these security considerations:

### 1. Randomness Quality

- **Issue:** Racket's built-in `random` function uses a pseudorandom number generator (PRNG)
- **Risk:** PRNGs are **not cryptographically secure**
- **Mitigation:**
  - Do NOT use betlang for cryptographic purposes
  - Do NOT use betlang for security-critical random number generation
  - For cryptographic randomness, use a dedicated crypto library

### 2. Denial of Service via Computation

- **Issue:** Probabilistic simulations can be computationally expensive
- **Risk:** Malicious or poorly written code could cause excessive CPU/memory usage
- **Mitigation:**
  - Be cautious when running untrusted bet code
  - Monitor resource usage for large simulations
  - Set reasonable limits on iteration counts (e.g., `bet-parallel` n parameter)

### 3. Floating Point Precision

- **Issue:** Statistical calculations involve floating-point arithmetic
- **Risk:** Precision errors, rounding issues, potential for numerical instability
- **Mitigation:**
  - Validate input ranges for statistical functions
  - Be aware of floating-point limitations in critical applications
  - Use appropriate precision for your domain

### 4. No Network or Filesystem Access

- **Good News:** betlang is **offline-first** and has no network dependencies
- **Risk Level:** LOW - No remote code execution, no data exfiltration, no network attacks

### 5. Memory Safety

- **Good News:** Racket has automatic garbage collection
- **Risk Level:** LOW - No buffer overflows, use-after-free, or memory corruption

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in betlang, please report it responsibly.

### How to Report

**For security vulnerabilities, please DO NOT open a public issue.**

Instead:

1. **Email:** Send details to [SECURITY_EMAIL - TO BE CONFIGURED]
   - Subject: "[SECURITY] betlang vulnerability report"
   - Include: description, steps to reproduce, potential impact

2. **GitHub Security Advisories (preferred):**
   - Go to the Security tab in the repository
   - Click "Report a vulnerability"
   - Fill out the advisory form

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact/severity
- Affected versions
- Suggested fix (if you have one)
- Your contact information (optional, for follow-up)

### Response Timeline

- **24 hours:** Acknowledgment of your report
- **7 days:** Initial assessment and severity classification
- **30 days:** Fix developed and tested (for confirmed vulnerabilities)
- **60 days:** Public disclosure (after fix is released)

### Responsible Disclosure

We follow these principles:

- We will acknowledge your report within 24 hours
- We will work with you to understand the issue
- We will keep you informed of our progress
- We will credit you in the security advisory (unless you prefer anonymity)
- We will publicly disclose the issue after a fix is available

### Hall of Fame

We recognize security researchers who responsibly disclose vulnerabilities:

- (No reports yet - you could be first!)

## Security Best Practices for Users

### When Using betlang in Production:

1. **Pin Your Version:**
   - Use a specific commit or tag, not `main` branch
   - Test thoroughly before upgrading

2. **Validate Inputs:**
   - Sanitize user-provided parameters to bet functions
   - Set reasonable bounds on iteration counts
   - Validate statistical assumptions

3. **Monitor Resource Usage:**
   - Track CPU and memory consumption
   - Set timeouts for long-running simulations
   - Implement circuit breakers for computational limits

4. **Review Dependencies:**
   - betlang has minimal dependencies (only Racket)
   - Keep Racket runtime updated
   - Monitor Racket security advisories

5. **Code Review:**
   - Review statistical correctness of probabilistic models
   - Verify randomness sources are appropriate for your use case
   - Audit third-party code before integration

## Scope

### In Scope

- Security vulnerabilities in betlang core code
- Security issues in standard library functions
- Documentation that could lead to insecure usage
- Build/deployment security issues

### Out of Scope

- Issues in user-written code using betlang
- Statistical correctness (report as bugs, not security issues)
- Performance issues (report as bugs, not security issues)
- Racket language vulnerabilities (report to Racket project)

## Security Tooling

### Static Analysis

Currently, betlang does not use automated security scanning tools. Contributions welcome for:

- Racket-specific static analysis integration
- Dependency vulnerability scanning
- Code quality tools

### Dependencies

betlang has minimal dependencies:

- **Racket:** Core language runtime (updated separately)
- **rackunit:** Testing framework (part of Racket distribution)

We do not use third-party libraries, minimizing supply chain risk.

## Known Limitations

1. **No Cryptographic Use:**
   - betlang's randomness is NOT suitable for cryptography
   - Use a dedicated cryptographic library instead

2. **No Input Validation:**
   - betlang does not sanitize user inputs by default
   - Users must validate inputs in their applications

3. **No Sandboxing:**
   - betlang code runs with full Racket VM permissions
   - Do not execute untrusted code

4. **No Resource Limits:**
   - No built-in limits on computation time or memory
   - Users must implement their own resource controls

## Security Roadmap

Future security enhancements under consideration:

- [ ] Cryptographically secure random number generator option
- [ ] Resource limit APIs (`bet-with-limits`, timeouts)
- [ ] Input validation helpers
- [ ] Security audit of statistical functions
- [ ] Formal verification of core primitives
- [ ] Supply chain security (SBOM, provenance)

## Compliance

betlang does not currently comply with specific security standards (e.g., FIPS, Common Criteria). If you need compliance for a specific standard, please open an issue to discuss.

## Contact

For non-security issues:
- Open an issue: [GitHub Issues](../../issues)
- Contribute: [CONTRIBUTING.md](CONTRIBUTING.md)

For security issues:
- Email: [SECURITY_EMAIL - TO BE CONFIGURED]
- GitHub Security Advisories: [Repository Security Tab](../../security)

---

**Last Updated:** 2025-11-22
**Version:** 1.0
