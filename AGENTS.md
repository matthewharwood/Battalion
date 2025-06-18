<agent-spec>
  <objective>
    Battalion livestreams community resume reviews. Feedback is captured as
    votes and comments, exported to Google Sheets and social posts.
  </objective>
  <product-requirements>
    <section id="executive-summary">
      <situation>Need fast early-career resume evaluation and shareable content.</situation>
      <task>Create a public platform for live reviews.</task>
      <action>Host 90‑minute streams with real-time voting exported to Sheets.</action>
      <result>Candidates gain visibility, recruiters get filtered leads, hosts grow audiences.</result>
    </section>
    <section id="problem-statement">
      <situation>Junior resumes are indistinguishable and feedback is scarce.</situation>
      <task>Open the review process to the community.</task>
      <action>Allow viewers to score and comment live.</action>
      <result>Better differentiation and transparent hiring workflow.</result>
    </section>
    <section id="goals-metrics">
      <situation>Success needs measurable targets.</situation>
      <task>Define 12‑month KPIs.</task>
      <action>Track resumes reviewed, viewers, participation, new followers, offers and vote latency.</action>
      <result>Quantitative gauge of platform impact.</result>
    </section>
    <section id="user-personas">
      <situation>Users include engineers, managers, viewers and hosts.</situation>
      <task>Address each persona's goals.</task>
      <action>Provide feedback, shortlists, learning and shareable content.</action>
      <result>Utility across all stakeholders.</result>
    </section>
    <section id="functional-scope">
      <situation>The platform needs specific features.</situation>
      <task>Implement scheduler, job catalog, resume intake, voting, export and results pages.</task>
      <action>Build modular components.</action>
      <result>Operational system for evaluation and sharing.</result>
    </section>
    <section id="non-functional">
      <situation>Performance and quality attributes matter.</situation>
      <task>Set requirements for scale, latency, availability, security, accessibility and export.</task>
      <action>Design architecture to meet them.</action>
      <result>Reliable, compliant service.</result>
    </section>
    <section id="user-journey">
      <situation>Need a clear flow for events.</situation>
      <task>Outline steps before, during and after streaming.</task>
      <action>Host selects a job, schedules, collects resumes, runs the stream and posts results.</action>
      <result>Shared understanding of the experience.</result>
    </section>
    <section id="mvp">
      <situation>Focus on minimum viable value.</situation>
      <task>Prioritize features into M0, M1 and M2+.</task>
      <action>Ship scheduler, catalog, intake, voting and export first.</action>
      <result>Lean product ready for feedback.</result>
    </section>
    <section id="tech-stack">
      <situation>Need a dependable stack.</situation>
      <task>Use Rust/Axum, SurrealDB, Tera, WebSocket and CI/CD automation.</task>
      <action>Manage versions in the root Cargo.toml.</action>
      <result>Stable foundation for growth.</result>
    </section>
    <section id="analytics">
      <situation>Improvement relies on metrics.</situation>
      <task>Provide dashboards, weekly summaries and hiring funnel data.</task>
      <action>Aggregate event and user statistics.</action>
      <result>Data-driven platform evolution.</result>
    </section>
  </product-requirements>
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
    "code-style": "Guidelines for formatting, naming and comments.",
    "project-structure": "Layout for crates, migrations and templates.",
    "pr-messages": "Bullet summaries and test results in each PR.",
    "tech-stack": "Standard frameworks and libraries to use.",
    "programmatic-checks": "Commands ensuring code quality and tests pass.",
    "agent": "AI modifying this repository in line with AGENTS.md.",
    "scope": "Files and directories governed by an AGENTS.md.",
    "objective": "Overall mission of the project.",
    "star-section": {
      "type": "object",
      "properties": {
        "situation": {"type": "string"},
        "task": {"type": "string"},
        "action": {"type": "string"},
        "result": {"type": "string"}
      },
      "required": ["situation", "task", "action", "result"]
    }
  }
}
```
