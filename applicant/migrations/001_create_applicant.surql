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
DEFINE FIELD linkedin         ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD whatprogramming ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD whyprogramming  ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD start       ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD program      ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD project ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD proudwork       ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD futureskills    ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD stories   ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD strategies ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD support  ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD food     ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD weekend          ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;
DEFINE FIELD travel      ON TABLE apply TYPE option<string>                 PERMISSIONS FULL;

-- Enforce email uniqueness
DEFINE INDEX unique_email ON TABLE apply COLUMNS email UNIQUE;
