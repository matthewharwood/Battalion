-- vote_record/migrations/001_create_vote_record.surql

DEFINE TABLE vote_record SCHEMAFULL;

-- Let SurrealDB auto‐generate id or accept a raw string
DEFINE FIELD id          ON TABLE vote_record TYPE option<record<vote_record> | string> PERMISSIONS FULL;

-- Now these will accept either a native record(e.g. event:foo) OR a string "event:foo"
DEFINE FIELD applicant_id ON TABLE vote_record TYPE option<record<apply> | string>   PERMISSIONS FULL;
DEFINE FIELD event_id     ON TABLE vote_record TYPE option<record<event> | string>   PERMISSIONS FULL;
DEFINE FIELD session_id   ON TABLE vote_record TYPE option<record<session> | string> PERMISSIONS FULL;

DEFINE FIELD name        ON TABLE vote_record TYPE option<string>                PERMISSIONS FULL;
DEFINE FIELD score       ON TABLE vote_record TYPE int                            PERMISSIONS FULL;
DEFINE FIELD timestamp   ON TABLE vote_record TYPE option<string>                PERMISSIONS FULL;
