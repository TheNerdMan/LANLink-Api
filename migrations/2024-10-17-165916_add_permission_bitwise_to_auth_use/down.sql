-- This file should undo anything in `up.sql`
ALTER TABLE auth_users
DROP COLUMN permissions_bitwise;