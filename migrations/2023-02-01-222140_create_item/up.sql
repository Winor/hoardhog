-- Your SQL goes here
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  quantity int NOT NULL DEFAULT 1,
  unit VARCHAR NOT NULL DEFAULT 'Piece',
  about TEXT,
  instock BOOLEAN NOT NULL DEFAULT TRUE
)