-- Your SQL goes here
CREATE TABLE Equipments (
   id SERIAL PRIMARY KEY,
   publicId uuid NOT NULL,
   name VARCHAR NOT NULL
)