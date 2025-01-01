DEFINE TABLE IF NOT EXISTS person SCHEMALESS
    PERMISSIONS FOR 
        CREATE, SELECT WHERE $auth,
        FOR UPDATE, DELETE WHERE created_by = $auth;
DEFINE FIELD IF NOT EXISTS name ON TABLE person TYPE string;
DEFINE FIELD IF NOT EXISTS created_by ON TABLE person VALUE $auth READONLY;

DEFINE INDEX IF NOT EXISTS unique_name ON TABLE user FIELDS name UNIQUE;
DEFINE ACCESS IF NOT EXISTS account ON DATABASE TYPE RECORD
SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
DURATION FOR TOKEN 15m, FOR SESSION 12h