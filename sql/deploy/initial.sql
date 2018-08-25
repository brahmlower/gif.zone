-- Deploy gif_zone:initial to pg

BEGIN;

CREATE TABLE gif (
    id serial PRIMARY KEY,
    title varchar NOT NULL,
    ftype varchar NOT NULL,
    views integer NOT NULL
);

CREATE TABLE tag (
    id serial PRIMARY KEY,
    name varchar NOT NULL
);

COMMIT;
