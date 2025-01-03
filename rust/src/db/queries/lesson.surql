-- lessons
DEFINE TABLE OVERWRITE lesson SCHEMAFULL
    PERMISSIONS
        for select WHERE assignee = $auth.id
        for select, update, delete WHERE created_by = $auth.id

;

DEFINE FIELD markdown
    ON TABLE lesson
    TYPE string;

DEFINE FIELD time
    ON TABLE lesson
    TYPE object;

DEFINE FIELD title
    ON TABLE lesson
    TYPE string;

DEFINE FIELD topic
    ON TABLE lesson
    TYPE string;

DEFINE FIELD time.custom_at
    ON TABLE lesson
    TYPE option<datetime>;

DEFINE FIELD time.created_at
    ON TABLE lesson
    TYPE datetime
    DEFAULT time::now()
    READONLY
    ;

DEFINE FIELD time.updated_at
    ON TABLE lesson
    TYPE datetime
    VALUE time::now()
    ;

DEFINE FIELD created_by
    ON TABLE lesson
    TYPE record<user>;

DEFINE FIELD assignee
    ON TABLE lesson
    TYPE record<user>