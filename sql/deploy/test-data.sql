-- Deploy gif_zone:test-data to pg

BEGIN;

INSERT INTO gif (id, title, ftype, fname, views) VALUES
    (DEFAULT, 'testing',            'Gif', 'patrick-board.gif',     4830),
    (DEFAULT, 'wheres the pizza?!', 'Gif', 'wheres-the-pizza.gif',  2893);

INSERT INTO tag (id, name) VALUES
    (DEFAULT, 'spongebob');

COMMIT;
