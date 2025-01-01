DEFINE TABLE  user  SCHEMAFULL
  -- Only allow users to query their own record, including their password.
  PERMISSIONS
    FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD name ON user TYPE string;
DEFINE FIELD username ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD pass ON user TYPE string;

DEFINE INDEX email ON user FIELDS email UNIQUE;
DEFINE INDEX username ON user FIELDS username UNIQUE;

DEFINE ACCESS account ON DATABASE TYPE RECORD
  SIGNUP (
    CREATE user CONTENT {
      name: $name,
      username: $username,
      email: $email,
      pass: crypto::argon2::generate($pass) -- Use Argon2 to generate the hash.
    }
  )
  SIGNIN (
    SELECT * FROM user 
    WHERE (email = $email OR username = $username)
      AND crypto::argon2::compare(pass, $pass) -- Use Argon2 to compare the hashes.
  )
  DURATION FOR TOKEN 15m, FOR SESSION 12h;


-- not actually needed but possible to take advantage of later for OAuth
 DEFINE ACCESS token ON DATABASE TYPE RECORD WITH JWT ALGORITHM RS256 KEY "-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAlpEv7yCef6E9MyDFgHC1
bQ3VuHmT1kDy4s750PwDqbq4M+4GPWixRUD0xnc7R6jM41bqXc39UyzmGCyolXci
w6Ch3XCJjM7twBE3Gs2H3jdIf/loO91Slx6H/5m83oAl939yKWOafiFWhr+FI3Jj
Csr4UpIE22aOhZUXhkX8XYaW+fgpPzJzzxdG++HuVXRJqGf0hiW+AqWE5R/z5mmX
jj6RTvCXh00GrW42uQgcEYW9Gykr2ySY5JeswvGrYYnDWhTtPn9Ey8jhkrq3dymL
neho7V3F1wN2nbfHiXsEAVu4ba4RUjCKT8ltpsrD170iu1/CivzOAJM2544t1WDV
1QIDAQAB
-----END PUBLIC KEY-----";