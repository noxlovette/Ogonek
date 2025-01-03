DEFINE TABLE student TYPE RELATION FROM user TO user SCHEMAFULL PERMISSIONS none;
DEFINE FIELD notes 
    ON TABLE student 
    TYPE option<string>;
