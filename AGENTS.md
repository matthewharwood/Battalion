<agent-spec>
  <objective>
    Battalion is a live, community-driven platform that merges applicant tracking
    with social-media content creation. Each 90-minute livestream publicly
    reviews résumés, capturing votes and comments that export to Google Sheets
    and social posts.
  </objective>
  <code-style>
    Use Rust edition 2024 across all crates. Format code using `cargo fmt` with default settings before committing. Run `cargo clippy --workspace --all-targets -- -D warnings` to ensure no warnings. Follow snake_case for functions and variables and CamelCase for types. Include module-level and function-level doc comments where appropriate. Ensure files end with a single newline.
  </code-style>
  <project-structure>
    This repository is a cargo workspace with multiple crates such as `applicant`, `event`, `job`, `review`, `vote`, `website`, and `shared_macros`. Source code lives in each crate's `src` directory. Database migrations are stored in `migrations/` inside each crate. HTML templates reside in `templates/`. Shared macros are kept in `shared_macros`. Always keep workspaces modular, with lib.rs exposing the public interface, handlers.rs connecting business logic, models.rs structuring the data model, and routes.rs exposing the http and paths, Keep new crates under the workspace by editing the root `Cargo.toml`.
  </project-structure>
  <pr-messages>
    Summaries should be in bullet list form, highlighting user-facing changes. Include a section listing which commands were run for programmatic checks and whether they succeeded. Mention any new dependencies with versions. Keep messages concise but informative.
  </pr-messages>
  <tech-stack>
    The project uses Rust with Axum for the web framework, SurrealDB for persistence, Tera for templating, and Tokio for async runtime. All dependencies use workspace versions specified in the root `Cargo.toml` whenever possible.
  </tech-stack>
  <programmatic-checks>
    Run the following before committing:
    1. `cargo fmt --all -- --check`
    2. `cargo clippy --workspace --all-targets -- -D warnings`
    3. `cargo test --workspace`
  </programmatic-checks>
</agent-spec>

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "definitions": {
    "code-style": "Guidelines for source code formatting, naming, and general appearance.",
    "project-structure": "Directives on how files and directories should be organized.",
    "pr-messages": "Instructions for crafting effective Pull Request descriptions.",
    "tech-stack": "Specifications for programming languages, frameworks, and tools.",
    "dependencies": "Requirements for external libraries and their specific versions.",
    "programmatic-checks": "Commands or scripts to verify code quality, functionality, or adherence to standards.",
    "agent": "An AI entity designed to perform tasks, particularly code generation and modification.",
    "container": "A GitHub repository where the AGENTS.md file resides.",
    "scope": "The set of files and directories to which a particular AGENTS.md file's instructions apply.",
    "objective": "Mission statement summarizing the project's purpose."
  }
}
```
