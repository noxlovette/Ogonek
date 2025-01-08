-- DEFINE RELATIONS
DEFINE TABLE student TYPE RELATION FROM user TO user SCHEMAFULL;
DEFINE FIELD notes 
    ON TABLE student 
    TYPE option<string>;

DEFINE TABLE bookmark TYPE RELATION FROM user TO lesson SCHEMAFULL 
    PERMISSIONS
        FOR select, update, delete WHERE in = $auth.id;
        FOR create FULL

DEFINE FIELD notes
    ON TABLE bookmark 
    TYPE option<string>;
DEFINE FIELD added_at
    ON TABLE bookmark
    TYPE datetime
    DEFAULT time::now();


DEFINE TABLE bookmark TYPE RELATION FROM user TO lesson SCHEMAFULL 
    PERMISSIONS
        FOR select, update, delete WHERE in = $auth.id;
        FOR create FULL

DEFINE FIELD notes
    ON TABLE bookmark 
    TYPE option<string>;
DEFINE FIELD added_at
    ON TABLE bookmark
    TYPE datetime
    DEFAULT time::now();

-- DEFINE TABLES
-- LESSON
DEFINE TABLE OVERWRITE lesson SCHEMAFULL
    PERMISSIONS
        for select WHERE assignee = $auth.id
        for select, update, delete, create WHERE created_by = $auth.id

;

DEFINE FIELD markdown
    ON TABLE lesson
    TYPE string;

DEFINE FIELD time
    ON TABLE lesson
    TYPE object
    DEFAULT {
			created_at: time::now(),
			updated_at: time::now()
		}
        ;

DEFINE FIELD title
    ON TABLE lesson
    TYPE string
    ;

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

-- USER
DEFINE TABLE user SCHEMAFULL

DEFINE FIELD name ON user TYPE string ASSERT string::len($value) >= 2;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) VALUE string::lowercase($value);
DEFINE FIELD pass ON user TYPE string VALUE crypto::argon2::generate($value)
DEFINE FIELD OVERWRITE role ON user TYPE string 
    ASSERT $value IN ["student", "teacher"]
    DEFAULT "student"

DEFINE FIELD joined
    ON user 
    TYPE datetime
    DEFAULT time::now()
    READONLY;


-- INDEXES
DEFINE INDEX email ON user FIELDS email UNIQUE;
DEFINE INDEX username ON user FIELDS username UNIQUE;

