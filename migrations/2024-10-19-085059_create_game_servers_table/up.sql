-- Your SQL goes here
CREATE TABLE GameServers (
    id SERIAL PRIMARY KEY,
    publicId uuid NOT NULL,
    game_server_title VARCHAR NOT NULL UNIQUE,
    game_type VARCHAR NOT NULL
)