BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS users (
  id INTEGER PRIMARY KEY,
  name TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL
);


CREATE TABLE IF NOT EXISTS images (
  id TEXT PRIMARY KEY,
  content_type TEXT NOT NULL,
  image_data BLOB NOT NULL,
  uploaded_by INTEGER NOT NULL,

  FOREIGN KEY (uploaded_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS products (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  description TEXT,
  price REAL NOT NULL, 
  published_by INTEGER NOT NULL,

  FOREIGN KEY (published_by) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS product_images (
  product_id INTEGER NOT NULL,
  image_id INTEGER NOT NULL,
  display_order INTEGER DEFAULT 0,

  PRIMARY KEY (product_id, image_id),

  FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
  FOREIGN KEY (image_id) REFERENCES images(id) ON DELETE CASCADE
);

COMMIT;
