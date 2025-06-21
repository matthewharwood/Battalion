# Vote to Applicant Relationship

Each `VoteRecord` links a viewer's evaluation to a specific applicant and event. The key fields are:

- `applicant_id` – `Thing` reference to the applicant's `apply` record.
- `event_id` – `Thing` reference to the `event` the applicant applied to.
- `session_id` – `Thing` representing the viewer session voting.
- `score` – integer rating.

When an event has a `job` reference, the applicant's vote implicitly ties back to that job through the event. This enables tallying results per job posting while keeping votes associated with individual applications.
