-- the user that has the least privilege. They are students but are bound via the students table to the teacher

DEFINE TABLE user SCHEMAFULL
  PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD name ON user TYPE string ASSERT string::len($value) >= 2;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) VALUE string::lowercase($value);
DEFINE FIELD pass ON user TYPE string VALUE crypto::argon2::generate($value)
DEFINE FIELD OVERWRITE role ON user TYPE string 
    ASSERT $value IN ["student", "teacher"]
    DEFAULT "student"

DEFINE FIELD joined_at
    ON user 
    TYPE datetime
    DEFAULT time::now()
    READONLY;


DEFINE INDEX email ON user FIELDS email UNIQUE;
DEFINE INDEX username ON user FIELDS username UNIQUE;
