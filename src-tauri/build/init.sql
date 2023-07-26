DEFINE NAMESPACE TuringApp;
USE NS TuringApp;
DEFINE DATABASE TuringDB;
USE DB TuringDB;

DEFINE TABLE user SCHEMAFULL
    PERMISSIONS 
        FOR select, update WHERE id = $auth.id, 
        FOR create, delete NONE;
  DEFINE FIELD user ON user TYPE string;
  DEFINE FIELD pass ON user TYPE string;
  DEFINE FIELD role ON user TYPE string;
  DEFINE INDEX idx_user ON user COLUMNS user UNIQUE;

DEFINE SCOPE TuringScope
    SESSION 1h
    SIGNUP ( CREATE user SET user = $user, pass = crypto::argon2::generate($pass), role = $role )
    SIGNIN ( SELECT * FROM user WHERE user = $user AND crypto::argon2::compare(pass, $pass) );

DEFINE TABLE role SCHEMAFULL
    PERMISSIONS
      FOR create, update NONE,
      FOR select WHERE $auth.roles containsany [role:human, role:interrogator, role:computer],
      FOR delete NONE;
  create role:human;
  create role:computer;
  create role:interrogator;

DEFINE TABLE human SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:human];
  DEFINE FIELD name ON human TYPE string;
  DEFINE FIELD age ON human TYPE int;
  DEFINE FIELD gender ON human TYPE string;
  DEFINE FIELD nationality ON human TYPE string;

DEFINE TABLE interrogator SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator];
  DEFINE FIELD name ON interrogator TYPE string;
  DEFINE FIELD age ON interrogator TYPE int;
  DEFINE FIELD gender ON interrogator TYPE string;
  DEFINE FIELD nationality ON interrogator TYPE string;

DEFINE TABLE verdict SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator];
  DEFINE FIELD correct ON verdict TYPE bool;

DEFINE TABLE session SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator];
  DEFINE FIELD time_frame ON session TYPE datetime;
  DEFINE FIELD time_spent ON session TYPE datetime;

DEFINE TABLE question SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:interrogator];
  DEFINE FIELD text ON question TYPE string;

DEFINE TABLE computer SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:computer];
  DEFINE FIELD model ON computer TYPE string;
  DEFINE FIELD developed_by ON computer TYPE string;

DEFINE TABLE answer SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE true, 
        FOR create, delete, update WHERE id = $auth.id AND $auth.role containsany [role:computer, role:human];
  DEFINE FIELD text ON answer TYPE string;


DEFINE TABLE mentions;
DEFINE TABLE asks;
DEFINE TABLE gives;
DEFINE TABLE includes;
DEFINE TABLE participateIn;
DEFINE TABLE follows;
DEFINE TABLE answeredBy;

DEFINE FIELD order ON follows TYPE int;
DEFINE FIELD answeredBy ON follows TYPE int;

DEFINE INDEX mentions ON TABLE mentions COLUMNS in, out;
DEFINE INDEX asks ON TABLE asks COLUMNS in, out;
DEFINE INDEX gives ON TABLE gives COLUMNS in, out;
DEFINE INDEX includes ON TABLE includes COLUMNS in, out;
DEFINE INDEX participateIn ON TABLE participateIn COLUMNS in, out;
DEFINE INDEX follows ON TABLE follows COLUMNS in, out;
DEFINE INDEX answeredBy ON TABLE answeredBy COLUMNS in, out;
