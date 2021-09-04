CREATE TABLE subcategories (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  sub_of INTEGER NOT NULL,
  FOREIGN KEY (sub_of) REFERENCES categories (id)
);
