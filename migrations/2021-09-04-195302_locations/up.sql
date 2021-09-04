CREATE TABLE locations (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT
);

CREATE TABLE areas (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  location INTEGER NOT NULL,
  FOREIGN KEY (location) REFERENCES locations (id)
);
