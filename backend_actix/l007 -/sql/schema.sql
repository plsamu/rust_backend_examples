CREATE SCHEMA IF NOT EXISTS my_schema;

/*
BYTEA
The SQL standard defines a different binary string type, called BLOB or 
BINARY LARGE OBJECT. 
The input format is different from bytea, but the provided functions 
and operators are mostly the same.
 */
DROP TABLE
  IF EXISTS event;

DROP TABLE
  IF EXISTS my_schema.event;

CREATE TABLE
  IF NOT EXISTS my_schema.event (id SERIAL PRIMARY KEY, name TEXT NOT NULL, data BYTEA);

DROP TABLE
  IF EXISTS my_schema.user;

/* username TEXT NOT NULL, */
/* password TEXT NOT NULL */
CREATE TABLE
  IF NOT EXISTS my_schema.user (user_id TEXT PRIMARY KEY);