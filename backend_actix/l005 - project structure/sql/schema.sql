/*
 BYTEA
 The SQL standard defines a different binary string type, called BLOB or 
 BINARY LARGE OBJECT. 
 The input format is different from bytea, but the provided functions 
 and operators are mostly the same.
 */
DROP TABLE IF EXISTS event;

CREATE TABLE
    IF NOT EXISTS event (id SERIAL PRIMARY KEY, name TEXT NOT NULL, data BYTEA);