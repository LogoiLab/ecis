CREATE TABLE mount_styles (
  id SERIAL PRIMARY KEY,
  name TEXT
);

CREATE TABLE packages (
  id SERIAL PRIMARY KEY,
  pvalue TEXT
);

CREATE TABLE components (
  id SERIAL PRIMARY KEY,
  manufacturer INTEGER,
  FOREIGN KEY (manufacturer) REFERENCES manufacturers (id),
  purchase_date INTEGER,
  part_number TEXT NOT NULL,
  category INTEGER,
  FOREIGN KEY (category) REFERENCES categories (id),
  sub_category INTEGER,
  FOREIGN KEY (sub_category) REFERENCES subcategories (id),
  description TEXT,
  datasheet TEXT,
  cvalues TEXT,
  mount_style INTEGER,
  FOREIGN KEY (mount_style) REFERENCES mount_styles (id),
  package INTEGER,
  FOREIGN KEY (package) REFERENCES packages (id)
);
