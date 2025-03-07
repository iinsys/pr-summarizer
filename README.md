# PR Summarizer with Jokes

![GitHub Test Actions Status](https://github.com/bansikah22/pr-summarizer/actions/workflows/test.yml/badge.svg)
![GitHub Test Actions Status](https://github.com/bansikah22/pr-summarizer/actions/workflows/release.yml/badge.svg)
![GitHub Issues](https://img.shields.io/github/issues/bansikah22/pr-summarizer?color=red)
![Open Source](https://img.shields.io/github/license/bansikah22/pr-summarizer?color=green)

A GitHub Action that automatically summarizes pull requests and adds a random programming joke to make code reviews more enjoyable.

## Features

- 📝 **PR Change Analysis**: Detects modified, added, and deleted files
- 📊 **Summary Generation**: Converts PR changes into short, meaningful descriptions
- 💬 **GitHub Commenting**: Posts the summary as a comment on the PR
- 😄 **Random Jokes**: Fetches and appends a programming joke to the comment
- 🔄 **External Repository Support**: Can be used across multiple GitHub repositories

## Example Output

```
🚀 PR Summary
📝 Changes Overview
✨ Implement user authentication system
✅ Added JWT token generation and validation
✅ Created user registration endpoint
🧪 Added tests for auth endpoints
📚 Updated API documentation
📂 Affected Files

🟢 [+] src/auth/jwt.rs
🟢 [+] src/auth/middleware.rs
🔵 [M] src/routes/users.rs
🔵 [M] src/models/user.rs
🟢 [+] tests/auth_tests.rs
🔵 [M] README.md

😄 Code Humor
Why do programmers prefer dark mode? Because light attracts bugs! 🤓

This summary was automatically generated by PR Summarizer ⚡
```

## Usage

Add this to your repository's `.github/workflows/pr-summary.yml`:

```yaml
name: PR Summarizer

on:
  pull_request:
    types: [opened, synchronize, reopened]

permissions:
  pull-requests: write  # Required to comment on PRs
  issues: write        # Required for PR comments via issues API

jobs:
  summarize:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout repository
        uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Ensures full history for diff analysis

      - name: Run PR Summarizer
        uses: bansikah22/pr-summarizer@v1.0.1  # Uses the lastest actions tag
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
```

## How It Works

1. When a pull request is opened or updated, the action is triggered
2. It analyzes the changes made in the PR
3. It generates a concise summary of the changes
4. It fetches a programming joke from several APIs
5. It posts a comment on the PR with the summary and joke

## Development

### Prerequisites

- Rust 1.83.+
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```
## 🤝 Contributors

We appreciate the efforts of all contributors who help improve this project. 

| Contributor        | Role                         |
|-------------------|-----------------------------|
| **Noel Bansikah** | Author & Maintainer         |

Contributions are welcomed! 🎉 Feel free to submit issues, feature requests, or pull requests to help enhance this project.

## 📜 License

[MIT](./LICENSE) License
