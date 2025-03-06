# PR Summarizer with Jokes

A GitHub Action that automatically summarizes pull requests and adds a random programming joke to make code reviews more enjoyable.

## Features

- 📝 **PR Change Analysis**: Detects modified, added, and deleted files
- 📊 **Summary Generation**: Converts PR changes into short, meaningful descriptions
- 💬 **GitHub Commenting**: Posts the summary as a comment on the PR
- 😄 **Random Jokes**: Fetches and appends a programming joke to the comment
- 🔄 **External Repository Support**: Can be used across multiple GitHub repositories

## Example Output

```
## PR Summary:
- Implemented signup functionality using JWT authentication
- Refactored user service to support email validation
- Updated README.md with API documentation

## Affected files:
- [M] src/auth.rs
- [M] src/user_service.rs
- [M] README.md

### Here's something to lighten the mood:
Why do programmers prefer dark mode? Because light attracts bugs! 😆
```

## Usage

Add this to your repository's `.github/workflows/pr-summary.yml`:

```yaml
name: PR Summarizer

on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  summarize:
    runs-on: ubuntu-latest
    steps:
      - uses: yourusername/pr-summarizer@v1
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

- Rust 1.56+
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

[MIT](./LICENSE) License