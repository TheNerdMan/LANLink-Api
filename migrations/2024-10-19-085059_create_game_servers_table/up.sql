-- Your SQL goes here
CREATE TABLE game_servers (
    id SERIAL PRIMARY KEY,
    publicId uuid NOT NULL,
    game_server_title VARCHAR NOT NULL UNIQUE,
    game_type VARCHAR NOT NULL
)