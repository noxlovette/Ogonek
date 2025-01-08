DEFINE TABLE task SCHEMAFULL;

DEFINE FIELD markdown
    ON TABLE task
    TYPE string;

DEFINE FIELD due
    ON TABLE task
    TYPE datetime;

DEFINE FIELD title
    ON TABLE task
    TYPE string;

DEFINE FIELD file
    ON TABLE task
    TYPE option<string>;

DEFINE FIELD completed
    ON TABLE task
    TYPE bool
    DEFAULT false;

DEFINE FIELD priority
    ON TABLE task
    TYPE number
    ASSERT $value >= 1 AND $value <= 3
    DEFAULT 1
    ;

DEFINE FIELD assignee
    ON TABLE task  
    TYPE record<user>;

DEFINE FIELD created_by
ON TABLE task  
TYPE record<user>;