-- Revert gif_zone:initial from pg

BEGIN;

DROP TABLE gif_tags;
DROP TABLE gif;
DROP TABLE tag;
DROP TYPE file_type;

COMMIT;
