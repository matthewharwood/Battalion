DEFINE TABLE event SCHEMAFULL;
DEFINE FIELD id               ON event TYPE string;
DEFINE FIELD title            ON event TYPE string;
DEFINE FIELD description      ON event TYPE string;
DEFINE FIELD status           ON event TYPE string;
DEFINE FIELD start_date       ON event TYPE date;
DEFINE FIELD end_date         ON event TYPE date;
DEFINE FIELD spotlight_job_id ON event TYPE record<job>;
