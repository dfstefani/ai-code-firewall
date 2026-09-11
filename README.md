# 🛡️ AI Code Firewall

> The enterprise-grade security and legal compliance engine for AI-generated code. Built in Rust.

[![CI/CD Pipeline](https://github.com/tuo-username/ai-code-firewall/actions/workflows/firewall-check.yml/badge.svg)](https://github.com/tuo-username/ai-code-firewall/actions)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

AI writes code fast, but it also introduces critical security vulnerabilities (RCE, SQLi, hardcoded secrets) and massive legal risks (copyright infringement and toxic copyleft licenses). **AI Code Firewall** acts as an unyielding gatekeeper at the local commit level and CI/CD pipeline stage.

## 🚀 Features

- **Blazing Fast Rust Core:** Zero-overhead memory safety and native binary compilation.
- **Deep Technical Scans:** Automatically blocks hardcoded secrets, SQL injections, and Remote Code Execution (RCE) patterns.
- **Legal & IP Compliance:** Detects potential copyright risks and unauthorized corporate IP leakage from AI generators.
- **Zero-Friction Enforcement:** Integrates seamlessly via Git pre-commit hooks and GitHub Actions.

---

## 📦 Getting Started

### Installation
Clone the repository and build the release binary:
```bash
git clone [https://github.com/dfstefani/ai-code-firewall.git](https://github.com/dfstefani/ai-code-firewall.git)
cd ai-code-firewall
cargo build --releases