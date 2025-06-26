DEFINE TABLE event SCHEMAFULL;

DEFINE FIELD id        ON TABLE event TYPE option<record<event> | string> PERMISSIONS FULL;
DEFINE FIELD title     ON TABLE event TYPE string                              PERMISSIONS FULL;
DEFINE FIELD description ON TABLE event TYPE string                             PERMISSIONS FULL;
DEFINE FIELD status    ON TABLE event TYPE string                              PERMISSIONS FULL;
DEFINE FIELD startDate ON TABLE event TYPE option<string>                      PERMISSIONS FULL;  
DEFINE FIELD endDate   ON TABLE event TYPE option<string>                      PERMISSIONS FULL;
DEFINE FIELD job       ON TABLE event TYPE option<string>                      PERMISSIONS FULL;

