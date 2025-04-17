# Using the `ci-local.sh` Script Locally

The `ci-local.sh` script simulates the CI pipeline locally. It performs the following steps:
1. Checks code formatting.
2. Builds the project.
3. Runs tests (if available).
4. Runs linter checks.
5. Generates API documentation.

---

## Prerequisites

Ensure the following tools are installed:
- **Rust toolchain**: Install via [rustup](https://rustup.rs/).
- **Cargo tools**:
  - `cargo fmt`
  - `cargo clippy`
  - `cargo nextest`
- **Bash shell**: Required to run the script.

---

## Steps to Run

1. **Navigate to the Project Directory**:
   ```bash
   cd /path/to/pr-summarizer
   ```
2. Make the Script Executable:
```bash
chmod +x ci-local.sh
```
3. Run the Script:
```bash
./ci-local.sh
```