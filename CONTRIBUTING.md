# Contributing to Naldom

We are thrilled that you're interested in contributing to the Naldom project! Your contributions, no matter how big or small, are incredibly valuable and help us achieve our goal of making programming more intuitive and accessible.

This document outlines guidelines for contributing to Naldom. Please take a moment to review it before making your first contribution.

## Code of Conduct

To ensure a welcoming and inclusive environment for everyone, we enforce our [Code of Conduct](CODE_OF_CONDUCT.md). By participating in this project, you are expected to uphold this code. Please report any unacceptable behavior to the project maintainers as outlined in the Code of Conduct.

## Getting Started

Before you start, make sure you have the necessary tools installed. Naldom's core components are written in Rust.

### Prerequisites

*   **Rust Toolchain:** Install Rust and Cargo (Rust's package manager) by following the instructions on [rustup.rs](https://rustup.rs/).
*   **Git:** For version control.

### Setting up your Development Environment

1.  **Fork the Repository:** Start by forking the `ADanMan/naldom-lang` repository to your GitHub account.
2.  **Clone your Fork:**
    ```bash
    git clone https://github.com/ADanMan/naldom-lang.git
    cd naldom-lang
    ```
3.  **Set up the LLM Server:** Naldom requires a locally running `llama.cpp` server. Please follow the **[LLM Server Setup Guide](docs/development-setup/llm-server-setup.md)** to get it running.
4.  **Install Dependencies:**
    ```bash
    # This command will build the project and download necessary dependencies
    cargo build
    ```

For more detailed setup instructions, including specific IDE configurations (like VS Code), please refer to our [Development Setup Guide](docs/development-setup/README.md) (will be created soon).

## Reporting Bugs

Found a bug? We appreciate your help in identifying and fixing issues!

1.  **Check existing issues:** Before submitting a new bug report, please check our [GitHub Issues](https://github.com/ADanMan/naldom-lang/issues) to see if the bug has already been reported.
2.  **Open a new issue:** If not, open a new issue and select the "Bug report" template.
3.  **Provide detailed information:**
    *   A clear and concise description of the bug.
    *   Steps to reproduce the behavior.
    *   Expected behavior vs. actual behavior.
    *   Any error messages or logs.
    *   Your operating system and Naldom version (once available).

## Suggesting Features or Enhancements

Have an idea for a new feature or an improvement? We'd love to hear it!

1.  **Check existing discussions/issues:** First, check [GitHub Discussions](https://github.com/ADanMan/naldom-lang/discussions) and [GitHub Issues](https://github.com/ADanMan/naldom-lang/issues) to see if your idea has already been discussed.
2.  **Open a new discussion or issue:**
    *   For general ideas or to gather feedback from the community, consider starting a discussion in the "Ideas" category.
    *   For well-defined feature requests, open a new issue and select the "Feature request" template.
3.  **Describe your proposal:** Clearly explain the problem your feature solves, the proposed solution, and any alternatives you've considered.

## Your First Contribution

If you're looking for an easy way to get started, look for issues labeled with `good first issue` on our [GitHub Issues page](https://github.com/ADanMan/naldom-lang/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22). These issues are specifically designed for new contributors and have been identified as relatively straightforward.

## Submitting Pull Requests

We welcome your code contributions! Here’s how to submit a Pull Request (PR):

1.  **Create a New Branch:** Always create a new branch for your changes, named descriptively (e.g., `feature/my-new-feature` or `fix/bug-description`).
    ```bash
    git checkout main
    git pull origin main
    git checkout -b feature/your-feature-name
    ```
2.  **Make Your Changes:** Implement your feature or bug fix.
3.  **Test Your Changes:** Ensure your changes are thoroughly tested. Write new unit and integration tests where appropriate.
    ```bash
    cargo test
    ```
4.  **Format Your Code:** We use `rustfmt` to maintain a consistent code style.
    ```bash
    cargo fmt
    ```
5.  **Lint Your Code:**
    ```bash
    cargo clippy
    ```
6.  **Write Clear Commit Messages:** Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification. A good commit message looks like:
    ```
    type(scope): subject

    [optional body]

    [optional footer(s)]
    ```
    Examples: `feat(parser): Add support for new Markdown extensions`, `fix(cli): Correct typo in help message`.

7.  **Push Your Branch:**
    ```bash
    git push origin feature/your-feature-name
    ```
8.  **Open a Pull Request:** Go to the `naldom-lang` repository on GitHub and open a new Pull Request from your branch.
    *   Fill out the PR template completely.
    *   Reference any relevant issues (e.g., `Fixes #123` or `Closes #456`).
    *   Provide a clear summary of your changes.

#### Pull Request Review Process

*   Your PR will be reviewed by maintainers. We might request changes or clarification.
*   Once approved and all Continuous Integration (CI) checks pass, your PR will be merged into the `main` branch.

## Communication and Support

*   **GitHub Discussions:** For general questions, ideas, and broader discussions, please use [GitHub Discussions](https://github.com/ADanMan/naldom-lang/discussions).
*   **GitHub Issues:** For bug reports and specific feature requests.

We aim to be responsive, but please understand that we are a small team (for now). Your patience is appreciated!

## Thank You!

Thank you for considering contributing to Naldom. We're excited to build this revolutionary language together!