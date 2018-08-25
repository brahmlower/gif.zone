-- Revert gif_zone:initial from pg

BEGIN;

DROP TABLE gif;
DROP TABLE tag;

COMMIT;
