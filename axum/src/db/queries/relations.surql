DEFINE TABLE OVERWRITE bookmark TYPE RELATION FROM user TO lesson SCHEMAFULL 
    PERMISSIONS
        FOR select, update, delete WHERE in = $auth.id;
        FOR create FULL

DEFINE FIELD OVERWRITE notes
    ON TABLE bookmark 
    TYPE option<string>;
DEFINE FIELD OVERWRITE added_at
    ON TABLE bookmark
    TYPE datetime
    DEFAULT time::now();


DEFINE TABLE student TYPE RELATION FROM user TO user SCHEMAFULL;
DEFINE FIELD notes 
    ON TABLE student 
    TYPE option<string>;
