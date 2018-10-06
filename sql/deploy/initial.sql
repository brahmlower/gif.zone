-- Deploy gif_zone:initial to pg

BEGIN;

CREATE TYPE file_type AS ENUM (
    'Gif',
    'Webm'
);

CREATE TABLE gif (
    id      serial PRIMARY KEY,
    title   varchar NOT NULL,
    ftype   file_type NOT NULL,
    fname   varchar NOT NULL,
    views   integer NOT NULL
);

CREATE TABLE tag (
    id serial PRIMARY KEY,
    label varchar NOT NULL
);

CREATE TABLE gif_tags (
    tag integer NOT NULL REFERENCES tag (id),
    gif integer NOT NULL REFERENCES gif (id)
);

COMMIT;
