-- Your SQL goes here
ALTER TABLE auth_users
ADD permissions_bitwise VARCHAR NOT NULL default 0;