DEFINE TABLE apply SCHEMAFULL;

-- id can be generated or supplied as a string
DEFINE FIELD id               ON TABLE apply TYPE option<record<apply> | string>   PERMISSIONS FULL;

-- event & job links are optional
DEFINE FIELD event            ON TABLE apply TYPE option<record<event>   | string> PERMISSIONS FULL;
DEFINE FIELD job              ON TABLE apply TYPE option<record<job>     | string> PERMISSIONS FULL;

-- Required fields
DEFINE FIELD name             ON TABLE apply TYPE string                         PERMISSIONS FULL;
DEFINE FIELD github           ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD email            ON TABLE apply TYPE string                         PERMISSIONS FULL;
DEFINE FIELD resume           ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD portfolio        ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD code_base        ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD linkedin         ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD language         ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD what_programming ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD why_programming  ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD when_start       ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD who_program      ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD ultimate_project ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD proud_work       ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD future_skills    ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD oncall_stories   ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD focus_strategies ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD support_systems  ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD comfort_food     ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD weekend          ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD travel_wish      ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD truth1           ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD truth2           ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD lie              ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;

-- Enforce email uniqueness
DEFINE INDEX unique_email ON TABLE apply COLUMNS email UNIQUE;
